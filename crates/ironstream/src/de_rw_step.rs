// FILE: rust/ironstream/crates/ironstream/src/de_rw_step.rs
//! `de_rw_step` — high-level STEP file exchange provider, mirroring
//! OpenCascade's `DE_Wrapper` / `RWStep_Reader` exchange infrastructure
//! (`src/DataExchange/TKDE/DE/` and `src/DataExchange/TKSTEP/RWStep/`).
//!
//! This module provides a zero-dependency, pure-Rust stub for registering and
//! driving STEP file I/O at the data-exchange layer.  It sits one level above
//! `STEPControl_Reader` / `STEPControl_Writer` (see `rw_step.rs`), exposing a
//! provider interface that the `DE_Wrapper` registry can dispatch to.
//!
//! # Scope
//!
//! * [`DeRwStep`]          — the STEP exchange provider (read + write).
//! * [`StepFileInfo`]      — lightweight metadata extracted from a STEP header.
//! * [`detect_step_schema`] — probe a path for its STEP schema tag without a
//!   full parse.
//!
//! # Design
//!
//! No file-system access beyond `std::fs` is performed; no external crates are
//! used.  The `read` / `write` methods operate on shape labels represented as
//! plain `String` tokens — sufficient for a stub / test harness.
//!
//! # OCCT correspondence
//!
//! | OCCT type            | IronStream type    |
//! |----------------------|--------------------|
//! | `DE_Wrapper`         | [`DeRwStep`]       |
//! | `RWStep_Reader`      | [`DeRwStep`]       |
//! | *(header metadata)*  | [`StepFileInfo`]   |

use std::fs;
use std::path::Path;

// ---------------------------------------------------------------------------
// Known STEP schemas
// ---------------------------------------------------------------------------

/// Default STEP schema used when none is specified.
const DEFAULT_SCHEMA: &str = "AP214";

/// All schema strings that this provider recognises.
const KNOWN_SCHEMAS: &[&str] = &[
    "AP203",
    "AP214",
    "AP242",
    "CONFIG_CONTROL_DESIGN",
    "AUTOMOTIVE_DESIGN",
    "AP203E2",
];

// ---------------------------------------------------------------------------
// DeRwStep
// ---------------------------------------------------------------------------

/// High-level STEP exchange provider.
///
/// Mirrors the combined responsibilities of OCCT's `DE_Wrapper` (provider
/// registry / dispatch) and `RWStep_Reader` (STEP-specific read logic).
/// A single instance handles both reading and writing for one configured
/// schema variant.
///
/// # Quick start
///
/// ```rust,ignore
/// let provider = DeRwStep::new();
/// let shapes   = provider.read("part.step").unwrap();
/// provider.write(&shapes, "out.step").unwrap();
/// ```
// occt-ref: DE_Wrapper
// occt-ref: RWStep_Reader
#[derive(Debug, Clone)]
pub struct DeRwStep {
    /// Human-readable version string for this provider (e.g. `"1.0"`).
    pub version: String,
    /// Active STEP schema (e.g. `"AP214"`, `"AP203"`, `"AP242"`).
    pub schema: String,
}

impl DeRwStep {
    /// Create a provider with default version `"1.0"` and schema `"AP214"`.
    ///
    /// Mirrors `DE_Wrapper::GlobalWrapper()` factory semantics — the caller
    /// receives a ready-to-use provider without further configuration.
    pub fn new() -> Self {
        Self {
            version: "1.0".to_string(),
            schema: DEFAULT_SCHEMA.to_string(),
        }
    }

    /// Create a provider with the given schema tag.
    ///
    /// `s` is normalised to uppercase before storage.  Unknown schema strings
    /// are accepted without error; the provider will embed them verbatim in
    /// written STEP files.
    pub fn with_schema(s: &str) -> Self {
        Self {
            version: "1.0".to_string(),
            schema: s.to_uppercase(),
        }
    }

    /// Read a STEP file at `path` and return the list of shape labels it
    /// contains.
    ///
    /// In a full implementation, shape labels would be `TopoDS_Shape` handles.
    /// Here they are the raw `PRODUCT` name strings extracted from the DATA
    /// section.  Returns `Err` when the file cannot be opened or does not
    /// appear to be a valid STEP file.
    ///
    /// Mirrors the read half of `DE_Wrapper::Read` / `RWStep_Reader::ReadFile`.
    pub fn read(&self, path: &str) -> Result<Vec<String>, String> {
        let data = fs::read_to_string(path)
            .map_err(|e| format!("DeRwStep::read: cannot open '{}': {}", path, e))?;

        if !is_step_content(&data) {
            return Err(format!(
                "DeRwStep::read: '{}' does not contain a valid ISO-10303-21 header",
                path
            ));
        }

        let info = StepFileInfo::parse_header(&data);
        Ok(info.product_names)
    }

    /// Write `shapes` (as shape-label strings) to a STEP file at `path`.
    ///
    /// Generates a minimal but well-formed ISO-10303-21 file whose DATA
    /// section contains one `PRODUCT` entity per shape label.  Returns `Err`
    /// when the file cannot be written.
    ///
    /// Mirrors `DE_Wrapper::Write` / `STEPControl_Writer::Write`.
    pub fn write(&self, shapes: &[String], path: &str) -> Result<(), String> {
        if shapes.is_empty() {
            return Err("DeRwStep::write: no shapes to write".to_string());
        }

        let content = self.build_step_content(shapes);
        fs::write(path, content)
            .map_err(|e| format!("DeRwStep::write: cannot write '{}': {}", path, e))
    }

    /// Return `true` when `path` has a file extension recognised by this
    /// provider (`.stp` or `.step`, case-insensitive).
    ///
    /// Mirrors `DE_Provider::CheckExtension` / the extension filter used by
    /// `DE_Wrapper` when selecting a provider for a given file.
    pub fn can_read(&self, path: &str) -> bool {
        let p = Path::new(path);
        match p.extension().and_then(|e| e.to_str()) {
            Some(ext) => {
                let ext_lo = ext.to_lowercase();
                ext_lo == "stp" || ext_lo == "step"
            }
            None => false,
        }
    }

    // --- Private helpers -----------------------------------------------------

    /// Serialise `shapes` into a minimal ISO-10303-21 string.
    fn build_step_content(&self, shapes: &[String]) -> String {
        let mut s = String::new();

        // HEADER
        s.push_str("ISO-10303-21;\nHEADER;\n");
        s.push_str("FILE_DESCRIPTION(('IronStream STEP export'),'2;1');\n");
        s.push_str("FILE_NAME('','',('IronStream'),(''),'IronStream','','');\n");
        s.push_str(&format!("FILE_SCHEMA(('{}'));\n", self.schema));
        s.push_str("ENDSEC;\n");

        // DATA
        s.push_str("DATA;\n");
        for (i, shape) in shapes.iter().enumerate() {
            let id = i as u64 + 1;
            // Emit a stub PRODUCT entity for each shape label.
            s.push_str(&format!(
                "#{id} = PRODUCT('{shape}','{shape}','',(#__ctx__));\n",
            ));
        }
        s.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
        s
    }
}

impl Default for DeRwStep {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// StepFileInfo
// ---------------------------------------------------------------------------

/// Lightweight metadata extracted from a STEP file's HEADER section.
///
/// Mirrors the information that OCCT's `RWStep_Reader` / `StepData_StepModel`
/// make available after loading: the schema name, the total number of DATA
/// entities, and the product names found in `PRODUCT(…)` records.
// occt-note: RWStep_Reader  (header inspection helpers)
#[derive(Debug, Clone, Default)]
pub struct StepFileInfo {
    /// Schema tag from `FILE_SCHEMA`, e.g. `"AP214"` or `"CONFIG_CONTROL_DESIGN"`.
    pub schema: String,
    /// Total number of `#N = …` entity lines found in the DATA section.
    pub entity_count: usize,
    /// Product names extracted from `PRODUCT('name',…)` entities.
    pub product_names: Vec<String>,
}

impl StepFileInfo {
    /// Construct an empty `StepFileInfo`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse STEP text supplied in `data` and populate a `StepFileInfo`.
    ///
    /// Only the HEADER and DATA sections are inspected at a surface level; no
    /// full entity-graph resolution is performed.  Suitable for quick metadata
    /// probing without a complete `STEPControl_Reader` load cycle.
    pub fn parse_header(data: &str) -> StepFileInfo {
        let mut info = StepFileInfo::new();

        // --- Schema from HEADER ----------------------------------------------
        info.schema = extract_file_schema(data).unwrap_or_default();

        // --- DATA section scanning -------------------------------------------
        let data_body = extract_data_section(data);
        for line in data_body.lines() {
            let line = line.trim();
            // Count entity lines (start with '#').
            if line.starts_with('#') && line.contains('=') {
                info.entity_count += 1;
            }
            // Collect PRODUCT names.
            if let Some(name) = extract_product_name(line) {
                if !name.is_empty() {
                    info.product_names.push(name);
                }
            }
        }

        info
    }
}

// ---------------------------------------------------------------------------
// detect_step_schema
// ---------------------------------------------------------------------------

/// Probe the file at `path` for its STEP schema tag without a full parse.
///
/// Returns `Some(schema)` when the file is readable and contains a
/// `FILE_SCHEMA` record; returns `None` otherwise.
///
/// Mirrors the lightweight schema-detection pass that `DE_Wrapper` performs
/// when selecting a provider for an unknown file.
pub fn detect_step_schema(path: &str) -> Option<String> {
    let data = fs::read_to_string(path).ok()?;
    if !is_step_content(&data) {
        return None;
    }
    extract_file_schema(&data)
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Return `true` when `data` begins with the ISO-10303-21 sentinel.
fn is_step_content(data: &str) -> bool {
    data.trim_start().starts_with("ISO-10303-21;")
}

/// Extract the schema string from a `FILE_SCHEMA(('…'))` line.
///
/// Handles both single-schema `FILE_SCHEMA(('AP214'))` and multi-schema
/// `FILE_SCHEMA(('AP214','AP203'))` by returning the first entry.  The
/// returned string is normalised to uppercase.
fn extract_file_schema(data: &str) -> Option<String> {
    // Find the FILE_SCHEMA keyword within the HEADER section.
    let header_start = data.find("HEADER;")?;
    let header_end = data[header_start..]
        .find("ENDSEC;")
        .map(|p| header_start + p)
        .unwrap_or(data.len());
    let header = &data[header_start..header_end];

    let kw_pos = header.find("FILE_SCHEMA")?;
    let after = &header[kw_pos + "FILE_SCHEMA".len()..];

    // Find the first single-quoted string after FILE_SCHEMA.
    let q1 = after.find('\'')?;
    let q2 = after[q1 + 1..].find('\'')? + q1 + 1;
    let raw = &after[q1 + 1..q2];

    // Map known long-form schema names to short canonical tags.
    let canonical = match raw.to_uppercase().as_str() {
        "CONFIG_CONTROL_DESIGN" => "AP203".to_string(),
        "AUTOMOTIVE_DESIGN" => "AP214".to_string(),
        other => other.to_string(),
    };
    Some(canonical)
}

/// Return the slice of text between `DATA;` and `ENDSEC;`.
fn extract_data_section(data: &str) -> &str {
    let start = match data.find("DATA;") {
        Some(p) => p + 5,
        None => return "",
    };
    let end = data[start..]
        .find("ENDSEC;")
        .map(|p| start + p)
        .unwrap_or(data.len());
    &data[start..end]
}

/// Extract the first single-quoted argument of a `PRODUCT(…)` entity line.
///
/// The STEP pattern is: `#N = PRODUCT('name','description','',(…));`
/// This function handles the common cases without a full recursive parser.
fn extract_product_name(line: &str) -> Option<String> {
    // Must be an entity line referencing PRODUCT.
    if !line.contains("PRODUCT(") {
        return None;
    }
    let after_kw = &line[line.find("PRODUCT(")?  + "PRODUCT(".len()..];
    // First argument is the id string: 'name'
    let q1 = after_kw.find('\'')?;
    let q2 = after_kw[q1 + 1..].find('\'')? + q1 + 1;
    Some(after_kw[q1 + 1..q2].to_string())
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- DeRwStep ------------------------------------------------------------

    #[test]
    fn new_has_default_schema() {
        let p = DeRwStep::new();
        assert_eq!(p.schema, "AP214");
        assert_eq!(p.version, "1.0");
    }

    #[test]
    fn with_schema_normalises_to_uppercase() {
        let p = DeRwStep::with_schema("ap203");
        assert_eq!(p.schema, "AP203");
    }

    #[test]
    fn can_read_accepts_step_extensions() {
        let p = DeRwStep::new();
        assert!(p.can_read("model.step"));
        assert!(p.can_read("model.stp"));
        assert!(p.can_read("MODEL.STEP"));
        assert!(p.can_read("MODEL.STP"));
    }

    #[test]
    fn can_read_rejects_other_extensions() {
        let p = DeRwStep::new();
        assert!(!p.can_read("model.iges"));
        assert!(!p.can_read("model.obj"));
        assert!(!p.can_read("noextension"));
    }

    #[test]
    fn read_returns_err_for_missing_file() {
        let p = DeRwStep::new();
        let result = p.read("/nonexistent/path/to/file.step");
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(msg.contains("cannot open"));
    }

    #[test]
    fn write_returns_err_for_empty_shapes() {
        let p = DeRwStep::new();
        let result = p.write(&[], "/tmp/out.step");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("no shapes"));
    }

    #[test]
    fn build_step_content_round_trips_product_names() {
        let p = DeRwStep::new();
        let shapes = vec!["PartA".to_string(), "PartB".to_string()];
        let content = p.build_step_content(&shapes);

        assert!(content.starts_with("ISO-10303-21;"));
        assert!(content.contains("FILE_SCHEMA(('AP214'))"));
        assert!(content.contains("PRODUCT('PartA'"));
        assert!(content.contains("PRODUCT('PartB'"));
        assert!(content.trim_end().ends_with("END-ISO-10303-21;"));
    }

    #[test]
    fn build_step_content_uses_schema() {
        let p = DeRwStep::with_schema("AP203");
        let content = p.build_step_content(&["S".to_string()]);
        assert!(content.contains("FILE_SCHEMA(('AP203'))"));
    }

    // --- StepFileInfo --------------------------------------------------------

    fn sample_step(schema: &str, products: &[&str]) -> String {
        let mut s = String::new();
        s.push_str("ISO-10303-21;\nHEADER;\n");
        s.push_str("FILE_DESCRIPTION(('test'),'2;1');\n");
        s.push_str("FILE_NAME('','',(''),(''),'','','');\n");
        s.push_str(&format!("FILE_SCHEMA(('{}'));\n", schema));
        s.push_str("ENDSEC;\nDATA;\n");
        for (i, name) in products.iter().enumerate() {
            let id = i as u64 + 1;
            s.push_str(&format!(
                "#{id} = PRODUCT('{name}','{name}','',(#__ctx__));\n"
            ));
        }
        s.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
        s
    }

    #[test]
    fn step_file_info_new_is_empty() {
        let info = StepFileInfo::new();
        assert!(info.schema.is_empty());
        assert_eq!(info.entity_count, 0);
        assert!(info.product_names.is_empty());
    }

    #[test]
    fn parse_header_extracts_schema() {
        let data = sample_step("AP214", &[]);
        let info = StepFileInfo::parse_header(&data);
        // AP214 is the canonical tag; AUTOMOTIVE_DESIGN maps to AP214.
        assert_eq!(info.schema, "AP214");
    }

    #[test]
    fn parse_header_extracts_product_names() {
        let data = sample_step("AP214", &["Bracket", "Bolt"]);
        let info = StepFileInfo::parse_header(&data);
        assert_eq!(info.product_names, vec!["Bracket", "Bolt"]);
    }

    #[test]
    fn parse_header_counts_entities() {
        let data = sample_step("AP214", &["A", "B", "C"]);
        let info = StepFileInfo::parse_header(&data);
        assert_eq!(info.entity_count, 3);
    }

    #[test]
    fn parse_header_on_empty_data_returns_default() {
        let info = StepFileInfo::parse_header("");
        assert!(info.schema.is_empty());
        assert_eq!(info.entity_count, 0);
    }

    // --- detect_step_schema --------------------------------------------------

    #[test]
    fn detect_step_schema_returns_none_for_missing_file() {
        assert!(detect_step_schema("/no/such/file.step").is_none());
    }

    // --- helpers -------------------------------------------------------------

    #[test]
    fn is_step_content_accepts_valid_header() {
        assert!(is_step_content("ISO-10303-21;\nHEADER;\n"));
        assert!(is_step_content("  ISO-10303-21;\n"));
    }

    #[test]
    fn is_step_content_rejects_other_content() {
        assert!(!is_step_content("NOT-ISO\n"));
        assert!(!is_step_content(""));
    }

    #[test]
    fn extract_file_schema_maps_long_names() {
        let data = "ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('CONFIG_CONTROL_DESIGN'));\nENDSEC;\n";
        let schema = extract_file_schema(data).unwrap();
        assert_eq!(schema, "AP203");

        let data2 = "ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\n";
        let schema2 = extract_file_schema(data2).unwrap();
        assert_eq!(schema2, "AP214");
    }

    #[test]
    fn extract_product_name_parses_correctly() {
        let line = "#5 = PRODUCT('Bracket','Bracket long name','',(#1));";
        let name = extract_product_name(line).unwrap();
        assert_eq!(name, "Bracket");
    }

    #[test]
    fn extract_product_name_returns_none_for_other_entities() {
        assert!(extract_product_name("#1 = CARTESIAN_POINT('origin',(0.,0.,0.));").is_none());
    }

    #[test]
    fn known_schemas_list_is_non_empty() {
        assert!(!KNOWN_SCHEMAS.is_empty());
        assert!(KNOWN_SCHEMAS.contains(&"AP214"));
        assert!(KNOWN_SCHEMAS.contains(&"AP203"));
    }
}
