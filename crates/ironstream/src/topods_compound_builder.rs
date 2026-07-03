// FILE: rust/ironstream/crates/ironstream/src/topods_compound_builder.rs

// ---------------------------------------------------------------------------
// TopoShapeRef — shape reference in compound
// ---------------------------------------------------------------------------

/// A lightweight reference to a shape stored inside a `CompoundBuilder`.
///
/// Each entry carries a unique numeric `id`, a human-readable `label`, and a
/// `shape_type` string that mirrors the `TopAbs_ShapeEnum` name (e.g. `"Solid"`,
/// `"Face"`, `"Edge"`, …).
// occt: shape reference in compound
#[derive(Clone, Debug)]
pub struct TopoShapeRef {
    id: usize,
    label: String,
    shape_type: String,
}

impl TopoShapeRef {
    /// Create a new shape reference with the given `id`, `label`, and `shape_type`.
    // occt: TopoDS_Shape constructor
    pub fn new(id: usize, label: &str, shape_type: &str) -> Self {
        Self {
            id,
            label: label.to_string(),
            shape_type: shape_type.to_string(),
        }
    }

    /// Return this reference's unique identifier.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Return the human-readable label assigned at construction.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Return the shape-type string (e.g. `"Solid"`, `"Face"`).
    pub fn shape_type(&self) -> &str {
        &self.shape_type
    }
}

// ---------------------------------------------------------------------------
// CompoundBuilder — BRep_Builder compound builder
// ---------------------------------------------------------------------------

/// Builds a heterogeneous collection of `TopoShapeRef` entries.
///
/// The builder assigns monotonically increasing ids and provides O(n) look-up,
/// removal, and filtering by type.  It is the pure-Rust equivalent of the
/// combination of `BRep_Builder::MakeCompound` and `BRep_Builder::Add/Remove`.
// occt: BRep_Builder compound builder
pub struct CompoundBuilder {
    shapes: Vec<TopoShapeRef>,
    next_id: usize,
}

impl CompoundBuilder {
    /// Create an empty compound builder.
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            next_id: 0,
        }
    }

    /// Add a shape with the given `label` and `shape_type` to the compound.
    ///
    /// Returns the unique id assigned to the new shape.
    // occt: BRep_Builder::Add
    pub fn add_shape(&mut self, label: &str, shape_type: &str) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.shapes.push(TopoShapeRef::new(id, label, shape_type));
        id
    }

    /// Remove the shape with the given `id` from the compound.
    ///
    /// Returns `true` if the shape was found and removed, `false` otherwise.
    // occt: BRep_Builder::Remove
    pub fn remove_shape(&mut self, id: usize) -> bool {
        if let Some(pos) = self.shapes.iter().position(|s| s.id == id) {
            self.shapes.remove(pos);
            true
        } else {
            false
        }
    }

    /// Look up the shape with the given `id`.
    ///
    /// Returns `None` if no shape with that id exists in the compound.
    // occt: TopoDS_Iterator shape lookup
    pub fn shape(&self, id: usize) -> Option<&TopoShapeRef> {
        self.shapes.iter().find(|s| s.id == id)
    }

    /// Return the total number of shapes currently in the compound.
    // occt: BRep_Builder shape count
    pub fn nb_shapes(&self) -> usize {
        self.shapes.len()
    }

    /// Return `true` when the compound contains no shapes.
    // occt: TopoDS_Compound IsNull / empty check
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    /// Remove all shapes from the compound and reset the id counter.
    // occt: BRep_Builder::MakeCompound (re-initialise)
    pub fn clear(&mut self) {
        self.shapes.clear();
        self.next_id = 0;
    }

    /// Return the ids of all shapes whose `shape_type` equals `shape_type`.
    // occt: TopExp_Explorer filtered by shape type
    pub fn shape_ids_of_type(&self, shape_type: &str) -> Vec<usize> {
        self.shapes
            .iter()
            .filter(|s| s.shape_type == shape_type)
            .map(|s| s.id)
            .collect()
    }
}

impl Default for CompoundBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// CompoundExplorer — BRep_Explorer for compounds — iterates child shapes
// ---------------------------------------------------------------------------

/// Forward iterator over the `TopoShapeRef` entries of a compound.
///
/// Mirrors the `TopoDS_Iterator` / `BRepTools_Iterator` cursor pattern: callers
/// advance with `next()`, check `more()`, and dereference with `current()`.
// occt: BRep_Explorer for compounds — iterates child shapes
pub struct CompoundExplorer {
    shapes: Vec<TopoShapeRef>,
    current: usize,
}

impl CompoundExplorer {
    /// Create a new explorer over the given `shapes` snapshot.
    ///
    /// The explorer takes ownership of the vec so the underlying compound may
    /// be mutated independently after construction.
    // occt: TopoDS_Iterator constructor
    pub fn new(shapes: Vec<TopoShapeRef>) -> Self {
        Self { shapes, current: 0 }
    }

    /// Return `true` while there are more shapes to visit.
    // occt: TopoDS_Iterator::More
    pub fn more(&self) -> bool {
        self.current < self.shapes.len()
    }

    /// Advance the cursor by one position.
    ///
    /// Has no effect when `more()` is already `false`.
    // occt: TopoDS_Iterator::Next
    pub fn next(&mut self) {
        if self.current < self.shapes.len() {
            self.current += 1;
        }
    }

    /// Return a reference to the shape at the current cursor position.
    ///
    /// # Panics
    ///
    /// Panics if `more()` is `false` (i.e. the cursor is past the end).
    // occt: TopoDS_Iterator::Value
    pub fn current(&self) -> &TopoShapeRef {
        &self.shapes[self.current]
    }

    /// Reset the cursor back to the first shape.
    // occt: TopoDS_Iterator::Initialize (re-init same compound)
    pub fn reset(&mut self) {
        self.current = 0;
    }
}

// ---------------------------------------------------------------------------
// Internal unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topo_shape_ref_accessors() {
        let r = TopoShapeRef::new(7, "body", "Solid");
        assert_eq!(r.id(), 7);
        assert_eq!(r.label(), "body");
        assert_eq!(r.shape_type(), "Solid");
    }

    #[test]
    fn compound_builder_add_returns_sequential_ids() {
        let mut b = CompoundBuilder::new();
        let id0 = b.add_shape("face_a", "Face");
        let id1 = b.add_shape("face_b", "Face");
        assert_eq!(id0, 0);
        assert_eq!(id1, 1);
    }

    #[test]
    fn compound_builder_nb_shapes_and_is_empty() {
        let mut b = CompoundBuilder::new();
        assert!(b.is_empty());
        assert_eq!(b.nb_shapes(), 0);
        b.add_shape("s", "Solid");
        assert!(!b.is_empty());
        assert_eq!(b.nb_shapes(), 1);
    }

    #[test]
    fn compound_builder_shape_lookup() {
        let mut b = CompoundBuilder::new();
        let id = b.add_shape("myface", "Face");
        let s = b.shape(id).unwrap();
        assert_eq!(s.label(), "myface");
        assert_eq!(s.shape_type(), "Face");
        assert!(b.shape(999).is_none());
    }

    #[test]
    fn compound_builder_remove_shape() {
        let mut b = CompoundBuilder::new();
        let id = b.add_shape("x", "Edge");
        assert!(b.remove_shape(id));
        assert!(b.is_empty());
        assert!(!b.remove_shape(id)); // already removed
    }

    #[test]
    fn compound_builder_clear_resets_ids() {
        let mut b = CompoundBuilder::new();
        b.add_shape("a", "Face");
        b.add_shape("b", "Solid");
        b.clear();
        assert!(b.is_empty());
        // after clear the id counter resets to 0
        let id = b.add_shape("c", "Vertex");
        assert_eq!(id, 0);
    }

    #[test]
    fn compound_builder_shape_ids_of_type() {
        let mut b = CompoundBuilder::new();
        let f0 = b.add_shape("f0", "Face");
        let _s0 = b.add_shape("s0", "Solid");
        let f1 = b.add_shape("f1", "Face");
        let ids = b.shape_ids_of_type("Face");
        assert_eq!(ids, vec![f0, f1]);
        assert!(b.shape_ids_of_type("Edge").is_empty());
    }

    #[test]
    fn compound_explorer_iterates_all() {
        let mut b = CompoundBuilder::new();
        b.add_shape("a", "Face");
        b.add_shape("b", "Solid");
        let snapshot: Vec<TopoShapeRef> = b.shapes.clone();
        let mut ex = CompoundExplorer::new(snapshot);
        let mut labels = Vec::new();
        while ex.more() {
            labels.push(ex.current().label().to_string());
            ex.next();
        }
        assert_eq!(labels, vec!["a", "b"]);
    }

    #[test]
    fn compound_explorer_reset() {
        let mut b = CompoundBuilder::new();
        b.add_shape("x", "Face");
        let snapshot = b.shapes.clone();
        let mut ex = CompoundExplorer::new(snapshot);
        assert!(ex.more());
        ex.next();
        assert!(!ex.more());
        ex.reset();
        assert!(ex.more());
        assert_eq!(ex.current().label(), "x");
    }

    #[test]
    fn compound_explorer_empty() {
        let ex = CompoundExplorer::new(Vec::new());
        assert!(!ex.more());
    }
}
