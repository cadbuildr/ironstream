// FILE: rw_step_reader.rs
//! `rw_step_reader` — high-level STEP reader and work-session stubs mirroring
//! OpenCascade's `STEPControl_Reader` and `IFSelect_WorkSession`.
//!
//! # Scope
//!
//! * [`StepReader`]   — convenience reader that owns a file path and a list of
//!   transferred shape labels; mirrors `STEPControl_Reader`.
//! * [`WorkSession`]  — session object that tracks a loaded model by name and
//!   exposes its entity table; mirrors `IFSelect_WorkSession`.
//! * [`read_step`]    — free function: open a STEP file path and return all
//!   shape labels as a `Vec<String>`.
//!
//! # Design notes
//!
//! This is a pure-Rust, zero-dependency stub.  No file I/O is performed beyond
//! recording the supplied path string — no external crates are required and
//! `std` is the only dependency.  The API surface follows OCCT's public
//! interfaces so that callers can be ported to a full implementation later
//! without changing call sites.
//!
//! # OCCT correspondence
//!
//! | OCCT type                  | IronStream type     |
//! |----------------------------|---------------------|
//! | `STEPControl_Reader`       | [`StepReader`]      |
//! | `IFSelect_WorkSession`     | [`WorkSession`]     |

// ---------------------------------------------------------------------------
// StepReader
// ---------------------------------------------------------------------------

/// High-level STEP reader that records the file path it was asked to load,
/// collects transferred shape labels, and exposes a small inspection API.
///
/// Mirrors the public interface of OCCT's `STEPControl_Reader`:
///
/// | OCCT method               | IronStream equivalent     |
/// |---------------------------|---------------------------|
/// | `ReadFile(path)`          | `read_file(path)`         |
/// | `TransferRoots()`         | `transfer_roots()`        |
/// | `OneShape()`              | `one_shape()`             |
/// | `NbRootsForTransfer()`    | `root_count()`            |
// occt: STEPControl_Reader
pub struct StepReader {
    /// Path of the STEP file that was (or will be) loaded.
    pub file_path: String,
    /// Shape labels produced by `transfer_roots`.  Each entry is a
    /// human-readable label such as `"shape_1"`, `"shape_2"`, … that
    /// identifies a transferred root shape.
    pub shapes: Vec<String>,
    /// Number of root entities found after loading.  Set by `read_file`.
    pub num_roots: usize,
}

impl StepReader {
    /// Create a new, empty reader.
    ///
    /// Equivalent to constructing `STEPControl_Reader` with its default
    /// constructor; no file is loaded until `read_file` is called.
    pub fn new() -> Self {
        Self {
            file_path: String::new(),
            shapes: Vec::new(),
            num_roots: 0,
        }
    }

    /// Record `path` as the target file and perform a stub "load".
    ///
    /// In a full implementation this method would open `path` on the file
    /// system, parse the ISO-10303-21 payload, and populate the internal
    /// entity model.  Here it records the path and seeds `num_roots` with a
    /// placeholder value of `1` so that callers can exercise the transfer
    /// workflow.
    ///
    /// Returns `Ok(())` for any non-empty path, or `Err` when `path` is empty.
    ///
    /// Mirrors `STEPControl_Reader::ReadFile(filename)`.
    pub fn read_file(&mut self, path: &str) -> Result<(), String> {
        if path.is_empty() {
            return Err("read_file: path must not be empty".to_string());
        }
        self.file_path = path.to_string();
        self.shapes.clear();
        // Stub: pretend the file contains one root shape.
        self.num_roots = 1;
        Ok(())
    }

    /// Transfer all root entities and return the number of shapes produced.
    ///
    /// After a successful `read_file` call this method populates `self.shapes`
    /// with one label per root (`"shape_1"`, `"shape_2"`, …) and returns that
    /// count.  If no file has been loaded the method is a no-op and returns 0.
    ///
    /// Mirrors `STEPControl_Reader::TransferRoots()`.
    pub fn transfer_roots(&mut self) -> usize {
        if self.file_path.is_empty() {
            return 0;
        }
        self.shapes.clear();
        for i in 1..=self.num_roots {
            self.shapes.push(format!("shape_{}", i));
        }
        self.shapes.len()
    }

    /// Return a single label that represents the union of all transferred
    /// shapes.
    ///
    /// When shapes are available they are joined with `" + "` to form a
    /// compound label (e.g. `"shape_1 + shape_2"`).  When no shapes have been
    /// transferred an empty `String` is returned.
    ///
    /// Mirrors `STEPControl_Reader::OneShape()` which produces a single
    /// `TopoDS_Compound` from all transferred roots.
    pub fn one_shape(&self) -> String {
        self.shapes.join(" + ")
    }

    /// Number of root entities available for transfer.
    ///
    /// Mirrors `STEPControl_Reader::NbRootsForTransfer()`.
    pub fn root_count(&self) -> usize {
        self.num_roots
    }
}

impl Default for StepReader {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// WorkSession
// ---------------------------------------------------------------------------

/// Session object that owns a model name and a flat list of entity labels.
///
/// Mirrors OCCT's `IFSelect_WorkSession`, which manages the lifecycle of a
/// loaded interface model (STEP, IGES, …) and provides access to its entity
/// table.
///
/// | OCCT method                    | IronStream equivalent         |
/// |--------------------------------|-------------------------------|
/// | Constructor / `SetModel`       | `new()` + `load_file(path)`   |
/// | `NbStartingEntities()`         | `entity_count()`              |
/// | `StartingEntity(rank)`         | `entity_at(i)`                |
// occt: IFSelect_WorkSession
pub struct WorkSession {
    /// Human-readable name of the model that was loaded (derived from `path`).
    pub model_name: String,
    /// Flat list of entity labels in load order.
    pub entities: Vec<String>,
}

impl WorkSession {
    /// Create an empty work session with no model loaded.
    pub fn new() -> Self {
        Self {
            model_name: String::new(),
            entities: Vec::new(),
        }
    }

    /// Load a STEP file and populate the entity table.
    ///
    /// In a full implementation this would open `path`, parse the ISO-10303-21
    /// payload via the transfer framework, and build the entity table.  This
    /// stub records `path` as the model name and seeds the entity list with a
    /// single placeholder entry (`"entity_1"`) so that callers can exercise the
    /// session API.
    ///
    /// Returns `Ok(())` for any non-empty path, or `Err` when `path` is empty.
    ///
    /// Mirrors `IFSelect_WorkSession` loading via `XSControl_WorkSession::SelectNorm`
    /// + `ReadFile`.
    pub fn load_file(&mut self, path: &str) -> Result<(), String> {
        if path.is_empty() {
            return Err("load_file: path must not be empty".to_string());
        }
        self.model_name = path.to_string();
        self.entities.clear();
        // Stub: seed with a single entity label.
        self.entities.push("entity_1".to_string());
        Ok(())
    }

    /// Number of entities in the loaded model.
    ///
    /// Mirrors `IFSelect_WorkSession::NbStartingEntities()`.
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Access an entity label by zero-based index.
    ///
    /// Returns `None` when `i` is out of range.
    ///
    /// Mirrors `IFSelect_WorkSession::StartingEntity(rank)` (OCCT uses
    /// 1-based ranks; this API uses 0-based indices for idiomatic Rust).
    pub fn entity_at(&self, i: usize) -> Option<&String> {
        self.entities.get(i)
    }
}

impl Default for WorkSession {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// read_step — convenience free function
// ---------------------------------------------------------------------------

/// Open a STEP file at `path` and return the transferred shape labels.
///
/// This is a thin convenience wrapper around [`StepReader`]:
///
/// ```text
/// let shapes = read_step("part.step")?;
/// // shapes == ["shape_1"]
/// ```
///
/// Returns `Ok(Vec<String>)` with one label per transferred root, or
/// `Err(String)` when the path is empty or the reader fails.
pub fn read_step(path: &str) -> Result<Vec<String>, String> {
    let mut reader = StepReader::new();
    reader.read_file(path)?;
    reader.transfer_roots();
    Ok(reader.shapes)
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- StepReader ----------------------------------------------------------

    #[test]
    fn step_reader_new_is_empty() {
        let r = StepReader::new();
        assert!(r.file_path.is_empty());
        assert!(r.shapes.is_empty());
        assert_eq!(r.num_roots, 0);
    }

    #[test]
    fn step_reader_read_file_records_path() {
        let mut r = StepReader::new();
        assert!(r.read_file("part.step").is_ok());
        assert_eq!(r.file_path, "part.step");
    }

    #[test]
    fn step_reader_read_file_empty_path_is_err() {
        let mut r = StepReader::new();
        assert!(r.read_file("").is_err());
    }

    #[test]
    fn step_reader_root_count_after_load() {
        let mut r = StepReader::new();
        r.read_file("model.stp").unwrap();
        assert_eq!(r.root_count(), 1);
    }

    #[test]
    fn step_reader_transfer_roots_populates_shapes() {
        let mut r = StepReader::new();
        r.read_file("model.stp").unwrap();
        let n = r.transfer_roots();
        assert_eq!(n, 1);
        assert_eq!(r.shapes.len(), 1);
        assert_eq!(r.shapes[0], "shape_1");
    }

    #[test]
    fn step_reader_transfer_roots_without_load_returns_zero() {
        let mut r = StepReader::new();
        assert_eq!(r.transfer_roots(), 0);
        assert!(r.shapes.is_empty());
    }

    #[test]
    fn step_reader_one_shape_joins_labels() {
        let mut r = StepReader::new();
        r.read_file("assembly.step").unwrap();
        r.num_roots = 3;
        r.transfer_roots();
        assert_eq!(r.one_shape(), "shape_1 + shape_2 + shape_3");
    }

    #[test]
    fn step_reader_one_shape_empty_when_no_transfer() {
        let r = StepReader::new();
        assert!(r.one_shape().is_empty());
    }

    #[test]
    fn step_reader_default_equals_new() {
        let r: StepReader = Default::default();
        assert!(r.file_path.is_empty());
        assert_eq!(r.num_roots, 0);
    }

    // --- WorkSession ---------------------------------------------------------

    #[test]
    fn work_session_new_is_empty() {
        let s = WorkSession::new();
        assert!(s.model_name.is_empty());
        assert_eq!(s.entity_count(), 0);
    }

    #[test]
    fn work_session_load_file_sets_model_name() {
        let mut s = WorkSession::new();
        assert!(s.load_file("part.step").is_ok());
        assert_eq!(s.model_name, "part.step");
    }

    #[test]
    fn work_session_load_file_empty_path_is_err() {
        let mut s = WorkSession::new();
        assert!(s.load_file("").is_err());
    }

    #[test]
    fn work_session_entity_count_after_load() {
        let mut s = WorkSession::new();
        s.load_file("model.stp").unwrap();
        assert_eq!(s.entity_count(), 1);
    }

    #[test]
    fn work_session_entity_at_in_range() {
        let mut s = WorkSession::new();
        s.load_file("model.stp").unwrap();
        assert_eq!(s.entity_at(0), Some(&"entity_1".to_string()));
    }

    #[test]
    fn work_session_entity_at_out_of_range() {
        let mut s = WorkSession::new();
        s.load_file("model.stp").unwrap();
        assert!(s.entity_at(99).is_none());
    }

    #[test]
    fn work_session_default_equals_new() {
        let s: WorkSession = Default::default();
        assert!(s.model_name.is_empty());
        assert_eq!(s.entity_count(), 0);
    }

    // --- read_step -----------------------------------------------------------

    #[test]
    fn read_step_returns_shape_labels() {
        let shapes = read_step("part.step").unwrap();
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0], "shape_1");
    }

    #[test]
    fn read_step_empty_path_returns_err() {
        assert!(read_step("").is_err());
    }
}
