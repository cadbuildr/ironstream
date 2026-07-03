// FILE: rust/ironstream/crates/ironstream/src/brep_shell_maker.rs
//! Shell-construction stubs mirroring `BRepBuilderAPI_Sewing` and
//! `BRepBuilderAPI_MakeShell` from OCCT.
//!
//! `Sewing` collects faces/shells and merges their shared edges within a
//! tolerance to produce a single sewn shell.  `ShellMaker` wraps a surface
//! handle and exposes a ready-made shell from it.

// ---------------------------------------------------------------------------
// Sewing
// ---------------------------------------------------------------------------

// occt: BRepBuilderAPI_Sewing
/// Stub mirroring `BRepBuilderAPI_Sewing`.
///
/// Collects input shapes (faces or shells expressed as labels), performs a
/// topological sewing pass, and exposes the sewn result together with
/// diagnostic counts.
///
/// # Typical usage
///
/// ```
/// use ironstream::brep_shell_maker::Sewing;
///
/// let mut s = Sewing::new(1e-6);
/// s.add("face_1");
/// s.add("face_2");
/// s.perform();
/// let shell = s.sewed_shape();
/// assert!(!shell.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct Sewing {
    // occt: BRepBuilderAPI_Sewing::SetTolerance / GetTolerance
    /// Sewing tolerance used to identify coincident edges.
    pub tolerance: f64,
    // occt: BRepBuilderAPI_Sewing::Add — accumulated input shape labels
    /// Labels of the shapes that have been added via [`Sewing::add`].
    pub shapes: Vec<String>,
    // occt: BRepBuilderAPI_Sewing — free (boundary) edge labels after Perform
    /// Labels of free (unmatched) edges discovered during [`Sewing::perform`].
    pub free_edges: Vec<String>,

    /// Sewn result label, populated by [`Sewing::perform`].
    sewed_shape: String,
    /// Number of faces removed as duplicates during sewing.
    nb_deleted_faces: usize,
    /// Shapes whose topology was altered during sewing.
    modified: Vec<String>,
    /// Whether [`Sewing::perform`] has been called.
    performed: bool,
}

impl Sewing {
    // occt: BRepBuilderAPI_Sewing(Standard_Real tolerance, ...)
    /// Create a new `Sewing` builder with the given tolerance.
    ///
    /// Mirrors `BRepBuilderAPI_Sewing(Standard_Real tolerance, ...)`.
    pub fn new(tol: f64) -> Self {
        Self {
            tolerance: tol,
            shapes: Vec::new(),
            free_edges: Vec::new(),
            sewed_shape: String::new(),
            nb_deleted_faces: 0,
            modified: Vec::new(),
            performed: false,
        }
    }

    // occt: BRepBuilderAPI_Sewing::Add(const TopoDS_Shape& aShape)
    /// Add a shape (face or shell label) to be sewn.
    ///
    /// Mirrors `BRepBuilderAPI_Sewing::Add(const TopoDS_Shape&)`.
    pub fn add(&mut self, shape: &str) {
        if !shape.is_empty() {
            self.shapes.push(shape.to_string());
        }
    }

    // occt: BRepBuilderAPI_Sewing::Perform(const Message_ProgressRange&)
    /// Execute the sewing operation.
    ///
    /// Populates [`Sewing::free_edges`], [`Sewing::sewed_shape`] and the
    /// deleted-face counter.  This stub uses a deterministic heuristic:
    ///
    /// * A single input shape produces two free boundary edges.
    /// * Two or more shapes are considered fully sewn with no free edges.
    /// * Any shape whose label starts with `"modified_"` is recorded as
    ///   modified.
    ///
    /// Mirrors `BRepBuilderAPI_Sewing::Perform()`.
    pub fn perform(&mut self) {
        self.performed = true;
        let n = self.shapes.len();

        // Build the sewn-shape label.
        if n == 0 {
            self.sewed_shape = String::new();
            self.free_edges.clear();
            self.nb_deleted_faces = 0;
            self.modified.clear();
            return;
        }

        self.sewed_shape = format!(
            "shell(sewed=[{}],tol={})",
            self.shapes.join(","),
            self.tolerance
        );

        // Free-edge heuristic: a single face has two dangling boundary edges.
        self.free_edges.clear();
        if n == 1 {
            self.free_edges.push(format!("free_edge({}:0)", self.shapes[0]));
            self.free_edges.push(format!("free_edge({}:1)", self.shapes[0]));
        }

        // Deleted-face heuristic: one duplicate assumed for every three inputs.
        self.nb_deleted_faces = n / 3;

        // Collect shapes that were topologically modified.
        self.modified = self
            .shapes
            .iter()
            .filter(|s| s.starts_with("modified_"))
            .cloned()
            .collect();
    }

    // occt: BRepBuilderAPI_Sewing::SewedShape() -> const TopoDS_Shape&
    /// Return the sewn shape label.
    ///
    /// Returns an empty string when [`Sewing::perform`] has not been called or
    /// no input shapes were added.
    ///
    /// Mirrors `BRepBuilderAPI_Sewing::SewedShape()`.
    pub fn sewed_shape(&self) -> String {
        self.sewed_shape.clone()
    }

    // occt: BRepBuilderAPI_Sewing::NbFreeEdges() -> Standard_Integer
    /// Return the number of free (boundary) edges found during sewing.
    ///
    /// Mirrors `BRepBuilderAPI_Sewing::NbFreeEdges()`.
    pub fn nb_free_edges(&self) -> usize {
        self.free_edges.len()
    }

    // occt: BRepBuilderAPI_Sewing::NbDeletedFaces() -> Standard_Integer
    /// Return the number of faces that were deleted (merged as duplicates).
    ///
    /// Mirrors `BRepBuilderAPI_Sewing::NbDeletedFaces()`.
    pub fn nb_deleted_faces(&self) -> usize {
        self.nb_deleted_faces
    }

    // occt: BRepBuilderAPI_Sewing::IsModified(const TopoDS_Shape&) -> Standard_Boolean
    /// Return `true` when `shape` was topologically modified during sewing.
    ///
    /// Mirrors `BRepBuilderAPI_Sewing::IsModified(const TopoDS_Shape&)`.
    pub fn is_modified(&self, shape: &str) -> bool {
        self.modified.iter().any(|s| s == shape)
    }
}

// ---------------------------------------------------------------------------
// ShellMaker
// ---------------------------------------------------------------------------

// occt: BRepBuilderAPI_MakeShell
/// Stub mirroring `BRepBuilderAPI_MakeShell`.
///
/// Builds a topological shell directly from a surface, i.e. a single-face
/// shell covering the natural parametric bounds of the surface.
///
/// # Typical usage
///
/// ```
/// use ironstream::brep_shell_maker::ShellMaker;
///
/// let m = ShellMaker::from_surface("geom_sphere_1");
/// assert!(m.is_done());
/// let shell = m.shell();
/// assert!(shell.contains("geom_sphere_1"));
/// ```
#[derive(Debug, Clone)]
pub struct ShellMaker {
    // occt: BRepBuilderAPI_MakeShell — underlying Geom_Surface handle (stub: label)
    /// Label of the underlying surface.
    pub surface: String,
    // occt: BRepBuilderAPI_MakeShell::IsDone
    /// Whether shell construction succeeded.
    pub done: bool,
}

impl ShellMaker {
    // occt: BRepBuilderAPI_MakeShell(const Handle(Geom_Surface)& S, Standard_Boolean Segment)
    /// Construct a shell from a surface label.
    ///
    /// Succeeds for any non-empty surface label.  Mirrors
    /// `BRepBuilderAPI_MakeShell(const Handle(Geom_Surface)&, Standard_Boolean)`.
    pub fn from_surface(surf: &str) -> Self {
        let done = !surf.is_empty();
        Self {
            surface: surf.to_string(),
            done,
        }
    }

    // occt: BRepBuilderAPI_MakeShell::Shell() -> const TopoDS_Shell&
    /// Return the shell label.
    ///
    /// Returns an empty string when construction did not succeed.
    ///
    /// Mirrors `BRepBuilderAPI_MakeShell::Shell()`.
    pub fn shell(&self) -> String {
        if self.done {
            format!("shell(surface={})", self.surface)
        } else {
            String::new()
        }
    }

    // occt: BRepBuilderAPI_MakeShell::IsDone() -> Standard_Boolean
    /// Return `true` when the shell was successfully constructed.
    ///
    /// Mirrors `BRepBuilderAPI_MakeShell::IsDone()`.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ---------------------------------------------------------------------------
// Free-function helper
// ---------------------------------------------------------------------------

/// Sew a slice of shape labels together at the given tolerance and return the
/// sewn-shape label.
///
/// Convenience wrapper around [`Sewing`] that mirrors the typical single-call
/// pattern: create a `BRepBuilderAPI_Sewing`, add shapes, call `Perform()`,
/// return `SewedShape()`.
pub fn sew_shapes(shapes: &[&str], tol: f64) -> String {
    let mut s = Sewing::new(tol);
    for &shape in shapes {
        s.add(shape);
    }
    s.perform();
    s.sewed_shape()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Sewing ---------------------------------------------------------------

    #[test]
    fn sewing_new_is_empty() {
        let s = Sewing::new(1e-6);
        assert_eq!(s.tolerance, 1e-6);
        assert!(s.shapes.is_empty());
        assert!(s.free_edges.is_empty());
    }

    #[test]
    fn sewing_add_accumulates() {
        let mut s = Sewing::new(1e-6);
        s.add("face_a");
        s.add("face_b");
        assert_eq!(s.shapes.len(), 2);
    }

    #[test]
    fn sewing_add_ignores_empty() {
        let mut s = Sewing::new(1e-6);
        s.add("");
        assert!(s.shapes.is_empty());
    }

    #[test]
    fn sewing_perform_no_shapes() {
        let mut s = Sewing::new(1e-6);
        s.perform();
        assert!(s.sewed_shape().is_empty());
        assert_eq!(s.nb_free_edges(), 0);
        assert_eq!(s.nb_deleted_faces(), 0);
    }

    #[test]
    fn sewing_perform_single_shape_has_free_edges() {
        let mut s = Sewing::new(1e-6);
        s.add("face_1");
        s.perform();
        assert!(!s.sewed_shape().is_empty());
        assert_eq!(s.nb_free_edges(), 2);
    }

    #[test]
    fn sewing_perform_multiple_shapes_no_free_edges() {
        let mut s = Sewing::new(1e-6);
        s.add("face_1");
        s.add("face_2");
        s.add("face_3");
        s.perform();
        let result = s.sewed_shape();
        assert!(result.contains("face_1"));
        assert!(result.contains("face_2"));
        assert!(result.contains("face_3"));
        assert_eq!(s.nb_free_edges(), 0);
    }

    #[test]
    fn sewing_deleted_faces_heuristic() {
        let mut s = Sewing::new(1e-6);
        for i in 0..6 {
            s.add(&format!("face_{}", i));
        }
        s.perform();
        // 6 / 3 == 2
        assert_eq!(s.nb_deleted_faces(), 2);
    }

    #[test]
    fn sewing_is_modified_prefix() {
        let mut s = Sewing::new(1e-6);
        s.add("face_1");
        s.add("modified_face_2");
        s.perform();
        assert!(!s.is_modified("face_1"));
        assert!(s.is_modified("modified_face_2"));
    }

    #[test]
    fn sewing_sewed_shape_contains_tolerance() {
        let mut s = Sewing::new(0.001);
        s.add("f1");
        s.perform();
        assert!(s.sewed_shape().contains("tol=0.001"));
    }

    // -- ShellMaker -----------------------------------------------------------

    #[test]
    fn shell_maker_is_done_with_surface() {
        let m = ShellMaker::from_surface("geom_sphere_1");
        assert!(m.is_done());
        let shell = m.shell();
        assert!(shell.contains("geom_sphere_1"));
    }

    #[test]
    fn shell_maker_empty_surface_fails() {
        let m = ShellMaker::from_surface("");
        assert!(!m.is_done());
        assert!(m.shell().is_empty());
    }

    #[test]
    fn shell_maker_label_format() {
        let m = ShellMaker::from_surface("surf_42");
        assert_eq!(m.shell(), "shell(surface=surf_42)");
    }

    // -- sew_shapes free function ---------------------------------------------

    #[test]
    fn sew_shapes_empty_slice() {
        let result = sew_shapes(&[], 1e-6);
        assert!(result.is_empty());
    }

    #[test]
    fn sew_shapes_two_faces() {
        let result = sew_shapes(&["face_a", "face_b"], 1e-4);
        assert!(result.contains("face_a"));
        assert!(result.contains("face_b"));
        assert!(result.contains("tol=0.0001"));
    }

    #[test]
    fn sew_shapes_single_face() {
        let result = sew_shapes(&["only_face"], 1e-6);
        assert!(result.contains("only_face"));
    }
}
