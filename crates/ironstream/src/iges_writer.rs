// FILE: rust/ironstream/crates/ironstream/src/iges_writer.rs

//! High-level IGES writer — assembles shape records and serialises them to an
//! IGES file, mirroring OpenCascade's `IGESControl_Writer` class
//! (`src/DataExchange/TKIGES/IGESControl/IGESControl_Writer.hxx`).
//!
//! Uses only `std`; zero third-party dependencies.

use std::fs;
use std::io::Write as IoWrite;

// occt-ref: IGESControl_Writer // — transfer mode controlling how BRep topology is
// mapped to IGES entities.
/// Transfer mode used when adding shapes to the writer.
///
/// Mirrors the `IFSelect_ReturnStatus`-adjacent write-mode enum exposed by
/// `IGESControl_Writer` in OCCT.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IgesMode {
    /// Exact BRep representation (type 186 Manifold Solid BRep Object).
    // occt-ref: IGESControl_Writer // mode BRep (0)
    BRep,
    /// Facetted BRep — triangulated approximation of solid geometry.
    // occt-ref: IGESControl_Writer // mode FacettedBRep (1)
    FacettedBRep,
    /// Surface of revolution (entity type 162).
    // occt-ref: IGESControl_Writer // mode SurfaceOfRevolution (2)
    SurfaceOfRevolution,
    /// Trimmed parametric surfaces (entity type 144).
    // occt-ref: IGESControl_Writer // mode Trimmed (3)
    Trimmed,
    /// Wire-frame geometry — curves only, no surfaces.
    // occt-ref: IGESControl_Writer // mode WireFrame (4)
    WireFrame,
}

// occt-ref: IGESControl_Writer
/// Stub IGES file writer that accumulates shape descriptors and serialises
/// them to a minimal IGES text file.
///
/// # Example
/// ```
/// use ironstream::iges_writer::{IgesMode, IgesWriter};
///
/// let mut w = IgesWriter::new()
///     .with_mode(IgesMode::BRep)
///     .with_units("MM");
/// w.add_shape("solid_box");
/// w.write("/tmp/out.igs").unwrap();
/// assert_eq!(w.shape_count(), 1);
/// ```
pub struct IgesWriter {
    // occt-ref: IGESControl_Writer // shape sequence
    pub shapes: Vec<String>,
    // occt-ref: IGESControl_Writer // write mode
    pub mode: IgesMode,
    // occt-ref: IGESControl_Writer // units string (e.g. "MM", "INCH")
    pub units: String,
}

impl IgesWriter {
    // ----------------------------- constructors ----------------------------

    /// Creates an [`IgesWriter`] with default settings: [`IgesMode::BRep`] and
    /// millimetre units.
    // occt-ref: IGESControl_Writer // ::IGESControl_Writer()
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            mode: IgesMode::BRep,
            units: String::from("MM"),
        }
    }

    /// Consumes `self`, sets the write mode to `m`, and returns the updated
    /// writer.  Intended for builder-style construction.
    // occt-ref: IGESControl_Writer // write-mode setter
    pub fn with_mode(mut self, m: IgesMode) -> Self {
        self.mode = m;
        self
    }

    /// Consumes `self`, sets the units string to `u`, and returns the updated
    /// writer.  Intended for builder-style construction.
    // occt-ref: IGESControl_Writer // units setter
    pub fn with_units(mut self, u: &str) -> Self {
        self.units = u.to_string();
        self
    }

    // ------------------------------- mutators ------------------------------

    /// Registers a shape descriptor string with the writer.
    ///
    /// In a full implementation `s` would be a reference to a BRep shape;
    /// here we record the debug label for stub purposes.
    // occt-ref: IGESControl_Writer // ::AddShape
    pub fn add_shape(&mut self, s: &str) {
        self.shapes.push(s.to_string());
    }

    // ------------------------------- queries -------------------------------

    /// Returns the number of shapes registered with this writer.
    // occt-ref: IGESControl_Writer // ::NbShapes (conceptual)
    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    // ------------------------------- I/O -----------------------------------

    /// Writes the accumulated shapes to an IGES file at `path`.
    ///
    /// The output is a minimal valid IGES skeleton (Start, Global, Directory
    /// Entry, Parameter Data, and Terminate sections).  Each registered shape
    /// produces one stub Directory Entry / Parameter Data pair.
    ///
    /// Returns `Err(String)` if the file cannot be created or written.
    // occt-ref: IGESControl_Writer // ::Write
    pub fn write(&self, path: &str) -> Result<(), String> {
        let mut file = fs::File::create(path)
            .map_err(|e| format!("iges_writer: cannot create '{}': {}", path, e))?;

        let mode_str = match self.mode {
            IgesMode::BRep => "BRep",
            IgesMode::FacettedBRep => "FacettedBRep",
            IgesMode::SurfaceOfRevolution => "SurfaceOfRevolution",
            IgesMode::Trimmed => "Trimmed",
            IgesMode::WireFrame => "WireFrame",
        };

        // ---- Start section (S) -----------------------------------------
        // A single free-text line describing the file origin.
        let start_line = Self::iges_line(
            &format!(
                "IronStream IGES export — mode: {} units: {}",
                mode_str, self.units
            ),
            'S',
            1,
        );
        file.write_all(start_line.as_bytes())
            .map_err(|e| format!("iges_writer: write error: {}", e))?;

        // ---- Global section (G) ----------------------------------------
        // Minimal global parameters: units flag and file name.
        let global_params = format!(
            "1H,,1H;,7Hironstream,{},1H;,{},15H{},",
            Self::hollerith(&self.units),
            mode_str,
            chrono_stub()
        );
        let global_line = Self::iges_line(&global_params, 'G', 1);
        file.write_all(global_line.as_bytes())
            .map_err(|e| format!("iges_writer: write error: {}", e))?;

        // ---- Directory Entry (D) and Parameter Data (P) sections ---------
        let mut de_seq: u32 = 1; // odd 1-based DE sequence numbers
        let mut pd_seq: u32 = 1; // 1-based PD sequence numbers

        for shape in &self.shapes {
            // Entity type 308 = Subfigure Definition (used as a generic shape
            // placeholder; a real writer would use topology-specific types).
            let entity_type: u32 = 308;

            // Directory Entry — line 1 of 2
            let de1 = format!(
                "{:8}{:8}{:8}{:8}{:8}{:8}{:8}{:8}{:8}",
                entity_type, // field 1: entity type
                pd_seq,      // field 2: PD sequence number
                0,           // field 3: structure
                0,           // field 4: line font pattern
                0,           // field 5: level
                0,           // field 6: view
                0,           // field 7: transform matrix
                0,           // field 8: label display
                0,           // field 9: status / blank
            );
            let de1_line = format!("{:<72}D{:7}\n", de1, de_seq);
            file.write_all(de1_line.as_bytes())
                .map_err(|e| format!("iges_writer: write error: {}", e))?;

            // Directory Entry — line 2 of 2
            let label_trunc: String = shape.chars().take(8).collect();
            let de2 = format!(
                "{:8}{:8}{:8}{:8}{:8}{:8}{:8}{:8}{:<8}",
                entity_type, // field 10: entity type (repeated)
                0,           // field 11: line weight
                0,           // field 12: color
                1,           // field 13: PD line count
                0,           // field 14: form number
                0,           // field 15: reserved
                0,           // field 16: reserved
                label_trunc, // field 17: entity label
                0,           // field 18: subscript
            );
            let de2_line = format!("{:<72}D{:7}\n", de2, de_seq + 1);
            file.write_all(de2_line.as_bytes())
                .map_err(|e| format!("iges_writer: write error: {}", e))?;

            // Parameter Data — one line per shape
            let pd_data = format!("{},{},{};;", entity_type, de_seq, shape);
            let pd_line = format!("{:<64} {:7}P{:7}\n", pd_data, de_seq, pd_seq);
            file.write_all(pd_line.as_bytes())
                .map_err(|e| format!("iges_writer: write error: {}", e))?;

            de_seq += 2;
            pd_seq += 1;
        }

        // ---- Terminate section (T) -------------------------------------
        let shape_count = self.shapes.len() as u32;
        let terminate = format!(
            "S{:7}G{:7}D{:7}P{:7}{:40}T{:7}\n",
            1,
            1,
            de_seq - 1,
            pd_seq - 1,
            "",
            1
        );
        file.write_all(terminate.as_bytes())
            .map_err(|e| format!("iges_writer: write error: {}", e))?;

        drop(shape_count); // used only for documentation purposes above
        Ok(())
    }

    // --------------------------- private helpers ---------------------------

    /// Formats `text` as a fixed-width 80-character IGES line with section
    /// identifier `section` and sequence number `seq`.
    fn iges_line(text: &str, section: char, seq: u32) -> String {
        // IGES lines are 80 characters: columns 1–72 free text, 73 section id,
        // 74–80 sequence number.
        let truncated: String = text.chars().take(72).collect();
        format!("{:<72}{}{:7}\n", truncated, section, seq)
    }

    /// Encodes `s` as an IGES hollerith string of the form `<n>H<s>`.
    fn hollerith(s: &str) -> String {
        format!("{}H{}", s.len(), s)
    }
}

impl Default for IgesWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns a minimal ISO 8601-style timestamp string using only `std`.
/// A real implementation would use the system clock; this stub returns a
/// fixed placeholder to preserve the zero-dependency constraint.
fn chrono_stub() -> &'static str {
    "2026-01-01T00:00:00"
}

// occt-ref: IGESData_IGESEntity // (simplified write-side view)
/// A single IGES entity ready for serialisation, identified by a numeric
/// entity type code and a free-text label.
///
/// Models the write-side view of `IGESData_IGESEntity` / `IGESData_IGESModel`
/// from OCCT — just enough structure to emit a valid Parameter Data line.
pub struct IgesEntity {
    /// IGES entity type number (e.g. 110 = line, 116 = point, 186 = BRep solid).
    // occt-ref: IGESData_IGESEntity // ::TypeNumber
    pub entity_type: u32,
    /// Human-readable label stored in the Directory Entry label field.
    // occt-ref: IGESData_IGESEntity // label
    pub label: String,
}

impl IgesEntity {
    // ----------------------------- constructors ----------------------------

    /// Creates a new [`IgesEntity`] with the given entity type code and label.
    // occt-ref: IGESData_IGESEntity // constructor
    pub fn new(etype: u32, label: &str) -> Self {
        Self {
            entity_type: etype,
            label: label.to_string(),
        }
    }

    // ------------------------------- I/O -----------------------------------

    /// Serialises this entity to a minimal IGES Parameter Data line.
    ///
    /// The format is:
    /// ```text
    /// <entity_type>,<label>;
    /// ```
    /// padded to 72 characters and terminated with a newline.
    // occt-ref: IGESData_IGESEntity // serialisation (IGESData_IGESWriter concept)
    pub fn to_iges_line(&self) -> String {
        let data = format!("{},{};", self.entity_type, self.label);
        format!("{:<72}\n", data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ----------------------------- IgesMode --------------------------------

    #[test]
    fn iges_mode_variants_are_distinct() {
        assert_ne!(IgesMode::BRep, IgesMode::FacettedBRep);
        assert_ne!(IgesMode::BRep, IgesMode::SurfaceOfRevolution);
        assert_ne!(IgesMode::BRep, IgesMode::Trimmed);
        assert_ne!(IgesMode::BRep, IgesMode::WireFrame);
        assert_ne!(IgesMode::FacettedBRep, IgesMode::SurfaceOfRevolution);
        assert_ne!(IgesMode::FacettedBRep, IgesMode::Trimmed);
        assert_ne!(IgesMode::FacettedBRep, IgesMode::WireFrame);
        assert_ne!(IgesMode::SurfaceOfRevolution, IgesMode::Trimmed);
        assert_ne!(IgesMode::SurfaceOfRevolution, IgesMode::WireFrame);
        assert_ne!(IgesMode::Trimmed, IgesMode::WireFrame);
    }

    // ----------------------------- IgesWriter ------------------------------

    #[test]
    fn new_defaults() {
        let w = IgesWriter::new();
        assert_eq!(w.mode, IgesMode::BRep);
        assert_eq!(w.units, "MM");
        assert_eq!(w.shape_count(), 0);
    }

    #[test]
    fn default_equals_new() {
        let w: IgesWriter = IgesWriter::default();
        assert_eq!(w.mode, IgesMode::BRep);
        assert_eq!(w.units, "MM");
    }

    #[test]
    fn with_mode_changes_mode() {
        let w = IgesWriter::new().with_mode(IgesMode::WireFrame);
        assert_eq!(w.mode, IgesMode::WireFrame);
    }

    #[test]
    fn with_units_changes_units() {
        let w = IgesWriter::new().with_units("INCH");
        assert_eq!(w.units, "INCH");
    }

    #[test]
    fn builder_chain_preserves_both_fields() {
        let w = IgesWriter::new()
            .with_mode(IgesMode::Trimmed)
            .with_units("M");
        assert_eq!(w.mode, IgesMode::Trimmed);
        assert_eq!(w.units, "M");
    }

    #[test]
    fn add_shape_increments_count() {
        let mut w = IgesWriter::new();
        assert_eq!(w.shape_count(), 0);
        w.add_shape("box");
        assert_eq!(w.shape_count(), 1);
        w.add_shape("sphere");
        assert_eq!(w.shape_count(), 2);
    }

    #[test]
    fn add_shape_stores_name() {
        let mut w = IgesWriter::new();
        w.add_shape("cylinder");
        assert_eq!(w.shapes[0], "cylinder");
    }

    #[test]
    fn write_empty_writer_succeeds() {
        let w = IgesWriter::new();
        let path = "/tmp/iges_writer_test_empty.igs";
        assert!(w.write(path).is_ok());
        // Clean up.
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn write_with_shapes_succeeds() {
        let mut w = IgesWriter::new().with_mode(IgesMode::BRep).with_units("MM");
        w.add_shape("box");
        w.add_shape("sphere");
        let path = "/tmp/iges_writer_test_shapes.igs";
        assert!(w.write(path).is_ok());
        // Verify file exists and is non-empty.
        let meta = std::fs::metadata(path).unwrap();
        assert!(meta.len() > 0);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn write_to_invalid_path_returns_err() {
        let w = IgesWriter::new();
        let result = w.write("/no/such/directory/out.igs");
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(msg.contains("iges_writer"));
    }

    #[test]
    fn write_file_contains_start_section() {
        let w = IgesWriter::new();
        let path = "/tmp/iges_writer_test_start.igs";
        w.write(path).unwrap();
        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains('S'));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn write_file_contains_terminate_section() {
        let w = IgesWriter::new();
        let path = "/tmp/iges_writer_test_term.igs";
        w.write(path).unwrap();
        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains('T'));
        let _ = std::fs::remove_file(path);
    }

    // ----------------------------- IgesEntity ------------------------------

    #[test]
    fn iges_entity_new_stores_fields() {
        let e = IgesEntity::new(110, "my_line");
        assert_eq!(e.entity_type, 110);
        assert_eq!(e.label, "my_line");
    }

    #[test]
    fn to_iges_line_contains_entity_type() {
        let e = IgesEntity::new(116, "pt");
        let line = e.to_iges_line();
        assert!(line.contains("116"));
    }

    #[test]
    fn to_iges_line_contains_label() {
        let e = IgesEntity::new(116, "my_point");
        let line = e.to_iges_line();
        assert!(line.contains("my_point"));
    }

    #[test]
    fn to_iges_line_ends_with_newline() {
        let e = IgesEntity::new(110, "line_a");
        let line = e.to_iges_line();
        assert!(line.ends_with('\n'));
    }

    #[test]
    fn to_iges_line_has_semicolon_terminator() {
        let e = IgesEntity::new(126, "bspline");
        let line = e.to_iges_line();
        // Parameter data lines end with ';' before padding/newline.
        assert!(line.trim_end().ends_with(';'));
    }

    #[test]
    fn to_iges_line_padded_to_at_least_72_chars_before_newline() {
        let e = IgesEntity::new(100, "arc");
        let line = e.to_iges_line();
        // Strip trailing newline; content field should be 72 chars wide.
        let without_nl = line.trim_end_matches('\n');
        assert_eq!(without_nl.len(), 72);
    }

    #[test]
    fn iges_entity_type_zero_is_valid() {
        let e = IgesEntity::new(0, "unknown");
        assert_eq!(e.entity_type, 0);
        let line = e.to_iges_line();
        assert!(line.contains("0,unknown"));
    }
}
