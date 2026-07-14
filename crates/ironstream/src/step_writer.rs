// FILE: step_writer.rs
//! `step_writer` — high-level STEP file writer, modelled after OCCT's
//! `STEPControl_Writer` and `IFSelect_ReturnStatus`.
//!
//! This module provides a higher-level facade over raw STEP serialisation:
//! shapes are recorded as opaque strings (representing serialised topology),
//! and the writer emits a well-formed ISO-10303-21 file when `write` is
//! called.  No external crates are used; only `std` is required.

use std::fmt;
use std::fs;
use std::io;

// ---------------------------------------------------------------------------
// StepReturnStatus
// ---------------------------------------------------------------------------

/// Return status for STEP writer operations.
///
/// Mirrors OCCT's `IFSelect_ReturnStatus` enumeration, adding an `Error`
/// variant that carries a diagnostic message for failures that originate
/// inside IronStream (e.g. filesystem errors).
// occt-ref: IFSelect_ReturnStatus
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepReturnStatus {
    /// The operation completed successfully.
    Done,
    /// The operation failed without a specific error message.
    Fail,
    /// Nothing was attempted (e.g. no shapes staged, empty write).
    Void,
    /// The operation failed with a descriptive error message.
    Error(String),
}

impl StepReturnStatus {
    /// Returns `true` when the status indicates success (`Done`).
    ///
    /// All other variants — `Fail`, `Void`, and `Error(_)` — return `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironstream::step_writer::StepReturnStatus;
    ///
    /// assert!(StepReturnStatus::Done.is_ok());
    /// assert!(!StepReturnStatus::Fail.is_ok());
    /// assert!(!StepReturnStatus::Void.is_ok());
    /// assert!(!StepReturnStatus::Error("bad".to_string()).is_ok());
    /// ```
    pub fn is_ok(&self) -> bool {
        matches!(self, StepReturnStatus::Done)
    }
}

impl fmt::Display for StepReturnStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepReturnStatus::Done        => write!(f, "Done"),
            StepReturnStatus::Fail        => write!(f, "Fail"),
            StepReturnStatus::Void        => write!(f, "Void"),
            StepReturnStatus::Error(msg)  => write!(f, "Error({})", msg),
        }
    }
}

// ---------------------------------------------------------------------------
// StepWriter
// ---------------------------------------------------------------------------

/// High-level STEP file writer.
///
/// Shapes are added as opaque string tokens (they represent serialised
/// topology in a full OCCT pipeline; here they are stored verbatim and
/// emitted inside the DATA section of the output file).
///
/// The schema field controls the `FILE_SCHEMA` header entry.  Two common
/// values are `"CONFIG_CONTROL_DESIGN"` (AP203) and
/// `"AUTOMOTIVE_DESIGN"` (AP214).  The default is `"CONFIG_CONTROL_DESIGN"`.
///
/// # Correspondence with OCCT
///
/// | OCCT method / concept             | IronStream equivalent        |
/// |-----------------------------------|------------------------------|
/// | `STEPControl_Writer::Transfer()`  | `StepWriter::transfer()`     |
/// | `STEPControl_Writer::Write()`     | `StepWriter::write()`        |
/// | `IFSelect_ReturnStatus`           | `StepReturnStatus`           |
// occt-ref: STEPControl_Writer
#[derive(Debug, Clone)]
pub struct StepWriter {
    /// Staged shape tokens.  Each string is written verbatim as one DATA-
    /// section line when `write` is called.
    // occt-ref: STEPControl_Writer // (internal shape list)
    pub shapes: Vec<String>,
    /// STEP schema name embedded in the `FILE_SCHEMA` header.
    pub schema: String,
}

impl StepWriter {
    /// Create a writer with the default AP203 schema.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironstream::step_writer::StepWriter;
    ///
    /// let w = StepWriter::new();
    /// assert_eq!(w.schema, "CONFIG_CONTROL_DESIGN");
    /// assert!(w.shapes.is_empty());
    /// ```
    // occt-ref: STEPControl_Writer // ::STEPControl_Writer()
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            schema: "CONFIG_CONTROL_DESIGN".to_string(),
        }
    }

    /// Create a writer with a custom schema name.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironstream::step_writer::StepWriter;
    ///
    /// let w = StepWriter::with_schema("AUTOMOTIVE_DESIGN");
    /// assert_eq!(w.schema, "AUTOMOTIVE_DESIGN");
    /// ```
    // occt-ref: STEPControl_Writer // (schema configuration)
    pub fn with_schema(schema: &str) -> Self {
        Self {
            shapes: Vec::new(),
            schema: schema.to_string(),
        }
    }

    /// Stage a shape for inclusion in the next `write` call.
    ///
    /// The string `s` is appended verbatim to the internal shape list.
    /// To transfer and immediately validate, use [`transfer`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironstream::step_writer::StepWriter;
    ///
    /// let mut w = StepWriter::new();
    /// w.add_shape("CLOSED_SHELL('box',())");
    /// assert_eq!(w.shape_count(), 1);
    /// ```
    // occt-ref: STEPControl_Writer // ::Transfer() — staging phase
    pub fn add_shape(&mut self, s: &str) {
        self.shapes.push(s.to_string());
    }

    /// Transfer (validate and stage) a shape string.
    ///
    /// Returns `StepReturnStatus::Void` when `s` is empty, and
    /// `StepReturnStatus::Done` otherwise.  The shape is staged for the
    /// next `write` call only when `Done` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironstream::step_writer::{StepWriter, StepReturnStatus};
    ///
    /// let mut w = StepWriter::new();
    /// assert_eq!(w.transfer("CLOSED_SHELL('box',())"), StepReturnStatus::Done);
    /// assert_eq!(w.transfer(""), StepReturnStatus::Void);
    /// assert_eq!(w.shape_count(), 1);
    /// ```
    // occt-ref: STEPControl_Writer // ::Transfer(shape, mode, compgraph)
    pub fn transfer(&mut self, s: &str) -> StepReturnStatus {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return StepReturnStatus::Void;
        }
        self.shapes.push(trimmed.to_string());
        StepReturnStatus::Done
    }

    /// Serialise all staged shapes to a STEP file at `path`.
    ///
    /// Returns `StepReturnStatus::Void` when no shapes have been staged,
    /// `StepReturnStatus::Done` on success, and
    /// `StepReturnStatus::Error(msg)` when the file cannot be written.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ironstream::step_writer::{StepWriter, StepReturnStatus};
    ///
    /// let mut w = StepWriter::new();
    /// w.add_shape("CLOSED_SHELL('box',())");
    /// let status = w.write("/tmp/out.step");
    /// assert!(status.is_ok());
    /// ```
    // occt-ref: STEPControl_Writer // ::Write(filename)
    pub fn write(&self, path: &str) -> StepReturnStatus {
        if self.shapes.is_empty() {
            return StepReturnStatus::Void;
        }
        let content = self.build_step_content();
        match fs::write(path, content) {
            Ok(()) => StepReturnStatus::Done,
            Err(e) => StepReturnStatus::Error(io_error_message(e)),
        }
    }

    /// Number of shapes currently staged.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironstream::step_writer::StepWriter;
    ///
    /// let mut w = StepWriter::new();
    /// assert_eq!(w.shape_count(), 0);
    /// w.add_shape("CLOSED_SHELL('s',())");
    /// w.add_shape("CLOSED_SHELL('t',())");
    /// assert_eq!(w.shape_count(), 2);
    /// ```
    // occt-ref: STEPControl_Writer // (no direct equivalent; related to NbShapes)
    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    // --- Private helpers -----------------------------------------------------

    /// Build the full ISO-10303-21 string from the staged shapes.
    fn build_step_content(&self) -> String {
        let mut out = String::new();

        // ISO-10303-21 mandatory sentinel
        out.push_str("ISO-10303-21;\n");

        // HEADER section
        out.push_str("HEADER;\n");
        out.push_str("FILE_DESCRIPTION(('IronStream STEP export'),'2;1');\n");
        out.push_str("FILE_NAME('','',(''),(''),'IronStream','IronStream','');\n");
        out.push_str(&format!("FILE_SCHEMA(('{}'));\n", self.schema));
        out.push_str("ENDSEC;\n");

        // DATA section — each shape occupies one numbered entity line
        out.push_str("DATA;\n");
        for (i, shape) in self.shapes.iter().enumerate() {
            let id = i + 1;
            // Emit as a raw entity line; if the shape already has the
            // KEYWORD(params) form, wrap it in the standard #N = ...; syntax.
            out.push_str(&format!("#{id} = {shape};\n"));
        }
        out.push_str("ENDSEC;\n");
        out.push_str("END-ISO-10303-21;\n");

        out
    }
}

impl Default for StepWriter {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Free-function convenience API
// ---------------------------------------------------------------------------

/// Write a collection of shape strings to a STEP file at `path`.
///
/// This is a thin convenience wrapper over `StepWriter` for callers that do
/// not need to configure the schema or stage shapes incrementally.
///
/// Returns `StepReturnStatus::Void` when `shapes` is empty,
/// `StepReturnStatus::Done` on success, and
/// `StepReturnStatus::Error(msg)` on I/O failure.
///
/// # Examples
///
/// ```no_run
/// use ironstream::step_writer::{write_step_file, StepReturnStatus};
///
/// let shapes = vec!["CLOSED_SHELL('box',())".to_string()];
/// let status = write_step_file(&shapes, "/tmp/assembly.step");
/// assert!(status.is_ok());
/// ```
// occt-ref: STEPControl_Writer // (utility usage pattern)
pub fn write_step_file(shapes: &[String], path: &str) -> StepReturnStatus {
    if shapes.is_empty() {
        return StepReturnStatus::Void;
    }
    let mut w = StepWriter::new();
    for s in shapes {
        let status = w.transfer(s);
        if !status.is_ok() {
            return status;
        }
    }
    w.write(path)
}

// ---------------------------------------------------------------------------
// Private utility
// ---------------------------------------------------------------------------

/// Convert a `std::io::Error` into a plain `String` without allocating via
/// any external formatting library.
fn io_error_message(e: io::Error) -> String {
    // `io::Error` implements `Display` through the standard library.
    format!("{}", e)
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- StepReturnStatus ----------------------------------------------------

    #[test]
    fn status_done_is_ok() {
        assert!(StepReturnStatus::Done.is_ok());
    }

    #[test]
    fn status_fail_is_not_ok() {
        assert!(!StepReturnStatus::Fail.is_ok());
    }

    #[test]
    fn status_void_is_not_ok() {
        assert!(!StepReturnStatus::Void.is_ok());
    }

    #[test]
    fn status_error_is_not_ok() {
        assert!(!StepReturnStatus::Error("oops".to_string()).is_ok());
    }

    #[test]
    fn status_display_done() {
        assert_eq!(StepReturnStatus::Done.to_string(), "Done");
    }

    #[test]
    fn status_display_fail() {
        assert_eq!(StepReturnStatus::Fail.to_string(), "Fail");
    }

    #[test]
    fn status_display_void() {
        assert_eq!(StepReturnStatus::Void.to_string(), "Void");
    }

    #[test]
    fn status_display_error() {
        assert_eq!(
            StepReturnStatus::Error("bad path".to_string()).to_string(),
            "Error(bad path)"
        );
    }

    #[test]
    fn status_clone_and_eq() {
        let s = StepReturnStatus::Error("x".to_string());
        assert_eq!(s.clone(), s);
    }

    // --- StepWriter::new / with_schema ---------------------------------------

    #[test]
    fn new_has_default_schema() {
        let w = StepWriter::new();
        assert_eq!(w.schema, "CONFIG_CONTROL_DESIGN");
        assert!(w.shapes.is_empty());
    }

    #[test]
    fn with_schema_sets_schema() {
        let w = StepWriter::with_schema("AUTOMOTIVE_DESIGN");
        assert_eq!(w.schema, "AUTOMOTIVE_DESIGN");
        assert!(w.shapes.is_empty());
    }

    #[test]
    fn default_equals_new() {
        let a = StepWriter::new();
        let b = StepWriter::default();
        assert_eq!(a.schema, b.schema);
        assert_eq!(a.shapes, b.shapes);
    }

    // --- add_shape / shape_count ---------------------------------------------

    #[test]
    fn add_shape_increments_count() {
        let mut w = StepWriter::new();
        assert_eq!(w.shape_count(), 0);
        w.add_shape("CLOSED_SHELL('a',())");
        assert_eq!(w.shape_count(), 1);
        w.add_shape("CLOSED_SHELL('b',())");
        assert_eq!(w.shape_count(), 2);
    }

    #[test]
    fn add_shape_stores_exact_string() {
        let mut w = StepWriter::new();
        w.add_shape("MY_ENTITY('x')");
        assert_eq!(w.shapes[0], "MY_ENTITY('x')");
    }

    // --- transfer ------------------------------------------------------------

    #[test]
    fn transfer_non_empty_returns_done_and_stages() {
        let mut w = StepWriter::new();
        let status = w.transfer("CLOSED_SHELL('box',())");
        assert_eq!(status, StepReturnStatus::Done);
        assert_eq!(w.shape_count(), 1);
    }

    #[test]
    fn transfer_empty_string_returns_void_and_does_not_stage() {
        let mut w = StepWriter::new();
        let status = w.transfer("");
        assert_eq!(status, StepReturnStatus::Void);
        assert_eq!(w.shape_count(), 0);
    }

    #[test]
    fn transfer_whitespace_only_returns_void() {
        let mut w = StepWriter::new();
        let status = w.transfer("   \t\n");
        assert_eq!(status, StepReturnStatus::Void);
        assert_eq!(w.shape_count(), 0);
    }

    #[test]
    fn transfer_trims_shape_string() {
        let mut w = StepWriter::new();
        w.transfer("  ENTITY('x')  ");
        assert_eq!(w.shapes[0], "ENTITY('x')");
    }

    // --- write ---------------------------------------------------------------

    #[test]
    fn write_empty_writer_returns_void() {
        let w = StepWriter::new();
        assert_eq!(w.write("/tmp/never_created.step"), StepReturnStatus::Void);
    }

    #[test]
    fn write_bad_path_returns_error() {
        let mut w = StepWriter::new();
        w.add_shape("CLOSED_SHELL('s',())");
        let status = w.write("/no_such_dir_ironstream/out.step");
        assert!(matches!(status, StepReturnStatus::Error(_)));
    }

    #[test]
    fn write_produces_valid_step_header() {
        let mut w = StepWriter::new();
        w.add_shape("CLOSED_SHELL('s',())");
        let path = "/tmp/ironstream_step_writer_test.step";
        let status = w.write(path);
        assert_eq!(status, StepReturnStatus::Done);

        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.starts_with("ISO-10303-21;"));
        assert!(content.contains("HEADER;"));
        assert!(content.contains("DATA;"));
        assert!(content.contains("ENDSEC;"));
        assert!(content.trim_end().ends_with("END-ISO-10303-21;"));
        // Clean up
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn write_embeds_schema_in_header() {
        let mut w = StepWriter::with_schema("AUTOMOTIVE_DESIGN");
        w.add_shape("ENTITY('x')");
        let path = "/tmp/ironstream_step_writer_schema_test.step";
        w.write(path);
        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("FILE_SCHEMA(('AUTOMOTIVE_DESIGN'))"));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn write_numbers_entities_sequentially() {
        let mut w = StepWriter::new();
        w.add_shape("A('x')");
        w.add_shape("B('y')");
        let path = "/tmp/ironstream_step_writer_seq_test.step";
        w.write(path);
        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("#1 = A('x');"));
        assert!(content.contains("#2 = B('y');"));
        let _ = std::fs::remove_file(path);
    }

    // --- write_step_file free function ---------------------------------------

    #[test]
    fn write_step_file_empty_slice_returns_void() {
        let status = write_step_file(&[], "/tmp/never.step");
        assert_eq!(status, StepReturnStatus::Void);
    }

    #[test]
    fn write_step_file_bad_path_returns_error() {
        let shapes = vec!["ENTITY('x')".to_string()];
        let status = write_step_file(&shapes, "/no_such_dir/out.step");
        assert!(matches!(status, StepReturnStatus::Error(_)));
    }

    #[test]
    fn write_step_file_success() {
        let shapes = vec![
            "CLOSED_SHELL('a',())".to_string(),
            "CLOSED_SHELL('b',())".to_string(),
        ];
        let path = "/tmp/ironstream_write_step_file_test.step";
        let status = write_step_file(&shapes, path);
        assert_eq!(status, StepReturnStatus::Done);
        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("CLOSED_SHELL('a',())"));
        assert!(content.contains("CLOSED_SHELL('b',())"));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn write_step_file_skips_empty_shape_strings() {
        // Empty strings inside the slice are treated as Void by transfer()
        // and cause write_step_file to return Void immediately.
        let shapes = vec!["".to_string()];
        let status = write_step_file(&shapes, "/tmp/never2.step");
        assert_eq!(status, StepReturnStatus::Void);
    }
}
