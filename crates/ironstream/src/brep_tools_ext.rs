// FILE: src/brep_tools_ext.rs
//! Extended BRep-tools utilities mirroring `BRepTools_Quilt`,
//! `BRepTools_ReShape`, and `BRepTools_WireExplorer` from OpenCASCADE.
//!
//! All types are pure-Rust, zero-dependency stubs that preserve the public
//! surface area and intent of the corresponding OCCT classes.

// ---------------------------------------------------------------------------
// Quilt
// ---------------------------------------------------------------------------

/// A collection of surfaces that can be stitched (quilted) into a single
/// shell.
///
/// Mirrors `BRepTools_Quilt`.  In OCCT a Quilt accepts faces/surfaces and
/// builds a unified shell by binding shared edges; here the payload is kept
/// as plain strings so the stub can compile with zero dependencies.
// occt: BRepTools_Quilt
pub struct Quilt {
    /// Names of surfaces that have been added to this quilt.
    pub surfaces: Vec<String>,
    /// Pairs of surface names that share a boundary and are therefore bound
    /// together, mirroring `BRepTools_Quilt::Bind(edge1, edge2)`.
    pub bindings: Vec<(String, String)>,
}

impl Quilt {
    /// Create an empty quilt.
    ///
    /// Mirrors `BRepTools_Quilt()`.
    pub fn new() -> Self {
        Self {
            surfaces: Vec::new(),
            bindings: Vec::new(),
        }
    }

    /// Add a surface (by name) to the quilt.
    ///
    /// Mirrors `BRepTools_Quilt::Add(face)`.
    pub fn add(&mut self, s: &str) {
        self.surfaces.push(s.to_string());
    }

    /// Declare that surface `s1` and surface `s2` share a boundary edge.
    ///
    /// Mirrors `BRepTools_Quilt::Bind(edge_on_s1, edge_on_s2)` — in the full
    /// OCCT implementation this performs topological identification of the two
    /// edges; here we simply record the intent as a string pair.
    pub fn bind(&mut self, s1: &str, s2: &str) {
        self.bindings.push((s1.to_string(), s2.to_string()));
    }

    /// Return the number of surfaces currently held in the quilt.
    ///
    /// Mirrors querying the shell produced by `BRepTools_Quilt::Shells()`.
    pub fn surface_count(&self) -> usize {
        self.surfaces.len()
    }
}

impl Default for Quilt {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// ReShape
// ---------------------------------------------------------------------------

/// Records substitutions (old-shape → new-shape) and applies them to shape
/// names on demand.
///
/// Mirrors `BRepTools_ReShape`.  The OCCT class performs topological
/// replacement of sub-shapes inside a compound; this stub models the same
/// contract using plain strings.
// occt: BRepTools_ReShape
pub struct ReShape {
    /// Ordered list of (old, new) substitution pairs.
    pub substitutions: Vec<(String, String)>,
}

impl ReShape {
    /// Create a new, empty reshape context.
    ///
    /// Mirrors `BRepTools_ReShape()`.
    pub fn new() -> Self {
        Self {
            substitutions: Vec::new(),
        }
    }

    /// Record that `old` should be replaced by `new` wherever it appears.
    ///
    /// Mirrors `BRepTools_ReShape::Replace(old_shape, new_shape)`.
    pub fn replace(&mut self, old: &str, new: &str) {
        self.substitutions.push((old.to_string(), new.to_string()));
    }

    /// Apply all recorded substitutions to `shape` and return the result.
    ///
    /// Substitutions are applied in registration order; if the same name is
    /// matched by more than one rule the first match wins (OCCT semantics:
    /// once replaced a shape is not re-examined).
    ///
    /// Mirrors `BRepTools_ReShape::Apply(shape)`.
    pub fn apply(&self, shape: &str) -> String {
        for (old, new) in &self.substitutions {
            if shape == old.as_str() {
                return new.clone();
            }
        }
        shape.to_string()
    }

    /// Return `true` if `s` is registered as an *old* shape (i.e. it will be
    /// replaced by a call to [`apply`]).
    ///
    /// Mirrors `BRepTools_ReShape::IsRecorded(shape)`.
    pub fn is_recorded(&self, s: &str) -> bool {
        self.substitutions.iter().any(|(old, _)| old.as_str() == s)
    }
}

impl Default for ReShape {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// WireExplorer
// ---------------------------------------------------------------------------

/// Ordered traversal of the edges of a wire represented as a sequence of
/// names.
///
/// Mirrors `BRepTools_WireExplorer`.  In OCCT the explorer walks the edges of
/// a `TopoDS_Wire` in connectivity order (end of edge N = start of edge N+1).
/// This string-based stub preserves the same iterator protocol so callers can
/// be written against the same API shape.
// occt: BRepTools_WireExplorer
pub struct WireExplorer {
    /// The ordered sequence of edge names that make up the wire.
    pub wire: Vec<String>,
    /// Current position within `wire`.
    pub index: usize,
}

impl WireExplorer {
    /// Create a new explorer initialised with the given edge sequence.
    ///
    /// Mirrors `BRepTools_WireExplorer::Init(wire)`.
    pub fn new(wire: Vec<String>) -> Self {
        Self { wire, index: 0 }
    }

    /// Return `true` if there is a current edge to examine.
    ///
    /// Mirrors `BRepTools_WireExplorer::More()`.
    pub fn more(&self) -> bool {
        self.index < self.wire.len()
    }

    /// Advance to the next edge and return a reference to it, or `None` if
    /// already exhausted.
    ///
    /// Note: unlike OCCT's separate `Next()` / `Current()` pair, this method
    /// advances *and* returns in a single call, matching the Rust iterator
    /// idiom.  See [`current_edge`] for the non-advancing accessor.
    ///
    /// Mirrors `BRepTools_WireExplorer::Next()` + `Current()`.
    pub fn next(&mut self) -> Option<&String> {
        if self.index < self.wire.len() {
            let edge = &self.wire[self.index];
            self.index += 1;
            Some(edge)
        } else {
            None
        }
    }

    /// Return a reference to the current edge without advancing.
    ///
    /// Returns `None` if the explorer is exhausted.
    ///
    /// Mirrors `BRepTools_WireExplorer::Current()`.
    pub fn current_edge(&self) -> Option<&String> {
        self.wire.get(self.index)
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Quilt ----------------------------------------------------------------

    #[test]
    fn quilt_new_is_empty() {
        let q = Quilt::new();
        assert_eq!(q.surface_count(), 0);
        assert!(q.bindings.is_empty());
    }

    #[test]
    fn quilt_add_increments_count() {
        let mut q = Quilt::new();
        q.add("face_A");
        q.add("face_B");
        assert_eq!(q.surface_count(), 2);
    }

    #[test]
    fn quilt_bind_records_pair() {
        let mut q = Quilt::new();
        q.add("face_A");
        q.add("face_B");
        q.bind("face_A", "face_B");
        assert_eq!(q.bindings.len(), 1);
        assert_eq!(q.bindings[0], ("face_A".to_string(), "face_B".to_string()));
    }

    #[test]
    fn quilt_default_equals_new() {
        let q: Quilt = Default::default();
        assert_eq!(q.surface_count(), 0);
    }

    // ---- ReShape --------------------------------------------------------------

    #[test]
    fn reshape_new_has_no_substitutions() {
        let r = ReShape::new();
        assert!(r.substitutions.is_empty());
    }

    #[test]
    fn reshape_apply_known_shape() {
        let mut r = ReShape::new();
        r.replace("edge_old", "edge_new");
        assert_eq!(r.apply("edge_old"), "edge_new");
    }

    #[test]
    fn reshape_apply_unknown_shape_returns_input() {
        let mut r = ReShape::new();
        r.replace("edge_old", "edge_new");
        assert_eq!(r.apply("edge_other"), "edge_other");
    }

    #[test]
    fn reshape_first_match_wins() {
        let mut r = ReShape::new();
        r.replace("A", "B");
        r.replace("A", "C");
        assert_eq!(r.apply("A"), "B");
    }

    #[test]
    fn reshape_is_recorded_true_for_registered() {
        let mut r = ReShape::new();
        r.replace("shape_x", "shape_y");
        assert!(r.is_recorded("shape_x"));
    }

    #[test]
    fn reshape_is_recorded_false_for_new() {
        let mut r = ReShape::new();
        r.replace("shape_x", "shape_y");
        assert!(!r.is_recorded("shape_y"));
        assert!(!r.is_recorded("shape_z"));
    }

    #[test]
    fn reshape_default_equals_new() {
        let r: ReShape = Default::default();
        assert!(r.substitutions.is_empty());
    }

    // ---- WireExplorer ---------------------------------------------------------

    #[test]
    fn wire_explorer_empty_wire() {
        let ex = WireExplorer::new(vec![]);
        assert!(!ex.more());
        assert!(ex.current_edge().is_none());
    }

    #[test]
    fn wire_explorer_more_and_next() {
        let mut ex = WireExplorer::new(vec!["e1".to_string(), "e2".to_string(), "e3".to_string()]);
        assert!(ex.more());
        assert_eq!(ex.next().unwrap(), "e1");
        assert!(ex.more());
        assert_eq!(ex.next().unwrap(), "e2");
        assert!(ex.more());
        assert_eq!(ex.next().unwrap(), "e3");
        assert!(!ex.more());
        assert!(ex.next().is_none());
    }

    #[test]
    fn wire_explorer_current_edge_does_not_advance() {
        let mut ex = WireExplorer::new(vec!["e1".to_string(), "e2".to_string()]);
        assert_eq!(ex.current_edge().unwrap(), "e1");
        assert_eq!(ex.current_edge().unwrap(), "e1");
        ex.next();
        assert_eq!(ex.current_edge().unwrap(), "e2");
    }

    #[test]
    fn wire_explorer_index_field_matches_position() {
        let mut ex = WireExplorer::new(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(ex.index, 0);
        ex.next();
        assert_eq!(ex.index, 1);
        ex.next();
        assert_eq!(ex.index, 2);
    }

    #[test]
    fn wire_explorer_wire_field_preserved() {
        let edges = vec!["x".to_string(), "y".to_string()];
        let ex = WireExplorer::new(edges.clone());
        assert_eq!(ex.wire, edges);
    }
}
