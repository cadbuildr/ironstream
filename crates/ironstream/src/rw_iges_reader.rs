// FILE: rw_iges_reader.rs
//! `rw_iges_reader` — high-level IGES reader and curve/surface transfer stubs
//! mirroring OpenCascade's `IGESControl_Reader` and
//! `IGESToBRep_CurveAndSurface`.
//!
//! # Scope
//!
//! * [`IgesReader`]   — convenience reader that owns a file path and a list of
//!   transferred shape labels; mirrors `IGESControl_Reader`.
//! * [`IgesToBrep`]   — converter that transfers IGES curve and surface entities
//!   to BRep representations; mirrors `IGESToBRep_CurveAndSurface`.
//! * [`read_iges`]    — free function: open an IGES file path and return all
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
//! | OCCT type                       | IronStream type   |
//! |---------------------------------|-------------------|
//! | `IGESControl_Reader`            | [`IgesReader`]    |
//! | `IGESToBRep_CurveAndSurface`    | [`IgesToBrep`]    |

// ---------------------------------------------------------------------------
// IgesReader
// ---------------------------------------------------------------------------

/// High-level IGES reader that records the file path it was asked to load,
/// collects transferred shape labels, and exposes a small inspection API.
///
/// Mirrors the public interface of OCCT's `IGESControl_Reader`:
///
/// | OCCT method               | IronStream equivalent     |
/// |---------------------------|---------------------------|
/// | `ReadFile(path)`          | `read_file(path)`         |
/// | `TransferRoots()`         | `transfer_roots()`        |
/// | `OneShape()`              | `one_shape()`             |
/// | `NbRootsForTransfer()`    | `root_count()`            |
/// | `Clear()`                 | `clear()`                 |
// occt: IGESControl_Reader
pub struct IgesReader {
    /// Path of the IGES file that was (or will be) loaded.
    pub file_path: String,
    /// Shape labels produced by `transfer_roots`.  Each entry is a
    /// human-readable label such as `"shape_1"`, `"shape_2"`, … that
    /// identifies a transferred root shape.
    pub shapes: Vec<String>,
    /// Number of root entities found after loading.  Set by `read_file`.
    pub num_roots: usize,
}

impl IgesReader {
    /// Create a new, empty reader.
    ///
    /// Equivalent to constructing `IGESControl_Reader` with its default
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
    /// system, parse the ASCII-encoded IGES payload (per ASME Y14.26M), and
    /// populate the internal entity model.  Here it records the path and seeds
    /// `num_roots` with a placeholder value of `1` so that callers can exercise
    /// the transfer workflow.
    ///
    /// Returns `Ok(())` for any non-empty path, or `Err` when `path` is empty.
    ///
    /// Mirrors `IGESControl_Reader::ReadFile(filename)`.
    pub fn read_file(&mut self, path: &str) -> Result<(), String> {
        if path.is_empty() {
            return Err("read_file: path must not be empty".to_string());
        }
        self.file_path = path.to_string();
        self.shapes.clear();
        // Stub: pretend the file contains one root entity.
        self.num_roots = 1;
        Ok(())
    }

    /// Transfer all root entities and return the number of shapes produced.
    ///
    /// After a successful `read_file` call this method populates `self.shapes`
    /// with one label per root (`"shape_1"`, `"shape_2"`, …) and returns that
    /// count.  If no file has been loaded the method is a no-op and returns 0.
    ///
    /// Mirrors `IGESControl_Reader::TransferRoots()`.
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
    /// Mirrors `IGESControl_Reader::OneShape()` which produces a single
    /// `TopoDS_Compound` from all transferred roots.
    pub fn one_shape(&self) -> String {
        self.shapes.join(" + ")
    }

    /// Number of root entities available for transfer.
    ///
    /// Mirrors `IGESControl_Reader::NbRootsForTransfer()`.
    pub fn root_count(&self) -> usize {
        self.num_roots
    }

    /// Clear all loaded data and reset the reader to its initial state.
    ///
    /// After calling `clear` the reader behaves as if `new()` had just been
    /// called: `file_path` is empty, `shapes` is empty, and `num_roots` is 0.
    ///
    /// Mirrors `IGESControl_Reader::Clear()` / `ClearShapes()`.
    pub fn clear(&mut self) {
        self.file_path.clear();
        self.shapes.clear();
        self.num_roots = 0;
    }
}

impl Default for IgesReader {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// IgesToBrep
// ---------------------------------------------------------------------------

/// Converter that transfers individual IGES curve and surface entities into
/// BRep (boundary-representation) shape labels.
///
/// Mirrors the public interface of OCCT's `IGESToBRep_CurveAndSurface`:
///
/// | OCCT method                            | IronStream equivalent       |
/// |----------------------------------------|-----------------------------|
/// | Constructor with tolerance             | `new(tol)`                  |
/// | `TransferSurface(entity, …)`           | `transfer_surface(entity)`  |
/// | `TransferCurve(entity, …)`             | `transfer_curve(entity)`    |
// occt: IGESToBRep_CurveAndSurface
pub struct IgesToBrep {
    /// General (3-D) transfer tolerance used when approximating curves and
    /// surfaces.  Corresponds to `GetEpsGeom()` / `SetEpsGeom()` in OCCT.
    pub tolerance: f64,
    /// Surface-specific tolerance; used during surface triangulation and
    /// UV-parameter mapping.  Corresponds to `GetSurfaceCurve()` in OCCT.
    pub surface_tol: f64,
}

impl IgesToBrep {
    /// Create a new converter with the given tolerance.
    ///
    /// `surface_tol` is set to `tol * 1e-2` as a sensible default (matching
    /// the relative scale used by OCCT's default surface curve tolerance).
    ///
    /// Mirrors `IGESToBRep_CurveAndSurface(tol, surfcurve, …)`.
    pub fn new(tol: f64) -> Self {
        Self {
            tolerance: tol,
            surface_tol: tol * 1.0e-2,
        }
    }

    /// Transfer an IGES surface entity to a BRep shape label.
    ///
    /// In a full implementation this method would interpret `entity` as an
    /// IGES surface record (e.g. entity type 128 – B-Spline Surface, type 114
    /// – Parametric Spline Surface, …), approximate it within `self.tolerance`,
    /// and return the resulting `TopoDS_Shape` label.  This stub returns a
    /// deterministic placeholder string so that callers can exercise the API.
    ///
    /// Mirrors `IGESToBRep_CurveAndSurface::TransferSurface(entity, …)`.
    pub fn transfer_surface(&self, entity: &str) -> String {
        format!("brep_surface({})", entity)
    }

    /// Transfer an IGES curve entity to a BRep shape label.
    ///
    /// In a full implementation this method would interpret `entity` as an
    /// IGES curve record (e.g. entity type 126 – Rational B-Spline Curve,
    /// type 110 – Line, type 100 – Circular Arc, …), approximate it within
    /// `self.tolerance`, and return the resulting `TopoDS_Edge` label.  This
    /// stub returns a deterministic placeholder string so that callers can
    /// exercise the API.
    ///
    /// Mirrors `IGESToBRep_CurveAndSurface::TransferCurve(entity, …)`.
    pub fn transfer_curve(&self, entity: &str) -> String {
        format!("brep_curve({})", entity)
    }
}

impl Default for IgesToBrep {
    fn default() -> Self {
        Self::new(1.0e-7)
    }
}

// ---------------------------------------------------------------------------
// read_iges — convenience free function
// ---------------------------------------------------------------------------

/// Open an IGES file at `path` and return the transferred shape labels.
///
/// This is a thin convenience wrapper around [`IgesReader`]:
///
/// ```text
/// let shapes = read_iges("part.igs")?;
/// // shapes == ["shape_1"]
/// ```
///
/// Returns `Ok(Vec<String>)` with one label per transferred root, or
/// `Err(String)` when the path is empty or the reader fails.
pub fn read_iges(path: &str) -> Result<Vec<String>, String> {
    let mut reader = IgesReader::new();
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

    // --- IgesReader ----------------------------------------------------------

    #[test]
    fn iges_reader_new_is_empty() {
        let r = IgesReader::new();
        assert!(r.file_path.is_empty());
        assert!(r.shapes.is_empty());
        assert_eq!(r.num_roots, 0);
    }

    #[test]
    fn iges_reader_read_file_records_path() {
        let mut r = IgesReader::new();
        assert!(r.read_file("part.igs").is_ok());
        assert_eq!(r.file_path, "part.igs");
    }

    #[test]
    fn iges_reader_read_file_empty_path_is_err() {
        let mut r = IgesReader::new();
        assert!(r.read_file("").is_err());
    }

    #[test]
    fn iges_reader_root_count_after_load() {
        let mut r = IgesReader::new();
        r.read_file("model.igs").unwrap();
        assert_eq!(r.root_count(), 1);
    }

    #[test]
    fn iges_reader_transfer_roots_populates_shapes() {
        let mut r = IgesReader::new();
        r.read_file("model.igs").unwrap();
        let n = r.transfer_roots();
        assert_eq!(n, 1);
        assert_eq!(r.shapes.len(), 1);
        assert_eq!(r.shapes[0], "shape_1");
    }

    #[test]
    fn iges_reader_transfer_roots_without_load_returns_zero() {
        let mut r = IgesReader::new();
        assert_eq!(r.transfer_roots(), 0);
        assert!(r.shapes.is_empty());
    }

    #[test]
    fn iges_reader_one_shape_joins_labels() {
        let mut r = IgesReader::new();
        r.read_file("assembly.igs").unwrap();
        r.num_roots = 3;
        r.transfer_roots();
        assert_eq!(r.one_shape(), "shape_1 + shape_2 + shape_3");
    }

    #[test]
    fn iges_reader_one_shape_empty_when_no_transfer() {
        let r = IgesReader::new();
        assert!(r.one_shape().is_empty());
    }

    #[test]
    fn iges_reader_clear_resets_all_fields() {
        let mut r = IgesReader::new();
        r.read_file("part.igs").unwrap();
        r.transfer_roots();
        r.clear();
        assert!(r.file_path.is_empty());
        assert!(r.shapes.is_empty());
        assert_eq!(r.num_roots, 0);
    }

    #[test]
    fn iges_reader_read_file_after_clear_works() {
        let mut r = IgesReader::new();
        r.read_file("first.igs").unwrap();
        r.clear();
        assert!(r.read_file("second.igs").is_ok());
        assert_eq!(r.file_path, "second.igs");
    }

    #[test]
    fn iges_reader_default_equals_new() {
        let r: IgesReader = Default::default();
        assert!(r.file_path.is_empty());
        assert_eq!(r.num_roots, 0);
    }

    // --- IgesToBrep ----------------------------------------------------------

    #[test]
    fn iges_to_brep_new_sets_tolerance() {
        let c = IgesToBrep::new(1.0e-5);
        assert!((c.tolerance - 1.0e-5).abs() < f64::EPSILON);
    }

    #[test]
    fn iges_to_brep_new_sets_surface_tol_relative() {
        let c = IgesToBrep::new(1.0e-5);
        let expected = 1.0e-5 * 1.0e-2;
        assert!((c.surface_tol - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn iges_to_brep_transfer_surface_returns_label() {
        let c = IgesToBrep::new(1.0e-7);
        let label = c.transfer_surface("iges_surf_128");
        assert_eq!(label, "brep_surface(iges_surf_128)");
    }

    #[test]
    fn iges_to_brep_transfer_curve_returns_label() {
        let c = IgesToBrep::new(1.0e-7);
        let label = c.transfer_curve("iges_curve_126");
        assert_eq!(label, "brep_curve(iges_curve_126)");
    }

    #[test]
    fn iges_to_brep_default_tolerance() {
        let c: IgesToBrep = Default::default();
        assert!((c.tolerance - 1.0e-7).abs() < f64::EPSILON);
    }

    #[test]
    fn iges_to_brep_transfer_surface_empty_entity() {
        let c = IgesToBrep::new(1.0e-7);
        assert_eq!(c.transfer_surface(""), "brep_surface()");
    }

    #[test]
    fn iges_to_brep_transfer_curve_empty_entity() {
        let c = IgesToBrep::new(1.0e-7);
        assert_eq!(c.transfer_curve(""), "brep_curve()");
    }

    // --- read_iges -----------------------------------------------------------

    #[test]
    fn read_iges_returns_shape_labels() {
        let shapes = read_iges("part.igs").unwrap();
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0], "shape_1");
    }

    #[test]
    fn read_iges_empty_path_returns_err() {
        assert!(read_iges("").is_err());
    }
}
