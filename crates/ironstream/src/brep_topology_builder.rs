// FILE: rust/ironstream/crates/ironstream/src/brep_topology_builder.rs

// ---------------------------------------------------------------------------
// BrepTopoShapeType — shape kind discriminant
// ---------------------------------------------------------------------------

/// Discriminant for the topological shape type stored in a `BrepTopoShape`.
///
/// Mirrors the ordered set of kinds in OCCT so that code ported from
/// `TopAbs_ShapeEnum` maps one-to-one.
// occt-ref: TopAbs_ShapeEnum // for builder
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrepTopoShapeType {
    Vertex,
    Edge,
    Wire,
    Face,
    Shell,
    Solid,
    CompSolid,
    Compound,
}

// ---------------------------------------------------------------------------
// BrepTopoShape — topology node
// ---------------------------------------------------------------------------

/// A single node in a BRep topology graph.
///
/// Each shape has a unique numeric `id`, a `shape_type`, a human-readable
/// `label`, an ordered list of child shape ids, and an orientation flag
/// (`0` = Forward, `1` = Reversed).
// occt-note: BRep shape stub
#[derive(Clone, Debug)]
pub struct BrepTopoShape {
    id: usize,
    shape_type: BrepTopoShapeType,
    label: String,
    children: Vec<usize>,
    orientation: u8,
}

impl BrepTopoShape {
    /// Create a new shape with the given `id`, `shape_type`, and `label`.
    /// Orientation defaults to `0` (Forward) and the child list is empty.
    pub fn new(id: usize, shape_type: BrepTopoShapeType, label: &str) -> Self {
        Self {
            id,
            shape_type,
            label: label.to_string(),
            children: Vec::new(),
            orientation: 0,
        }
    }

    /// Return this shape's unique identifier.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Return the topological kind of this shape.
    pub fn shape_type(&self) -> BrepTopoShapeType {
        self.shape_type
    }

    /// Return the human-readable label assigned at construction.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Append `child_id` to the ordered list of children.
    pub fn add_child(&mut self, child_id: usize) {
        self.children.push(child_id);
    }

    /// Return the ordered slice of child shape ids.
    pub fn children(&self) -> &[usize] {
        &self.children
    }

    /// Return the orientation byte (`0` = Forward, `1` = Reversed).
    pub fn orientation(&self) -> u8 {
        self.orientation
    }

    /// Overwrite the orientation byte.
    pub fn set_orientation(&mut self, orientation: u8) {
        self.orientation = orientation;
    }
}

// ---------------------------------------------------------------------------
// BrepTopoBuilder — topology graph builder
// ---------------------------------------------------------------------------

/// Builds a BRep topology graph from primitive make_* calls.
///
/// Each `make_*` method allocates a new `BrepTopoShape`, wires the supplied
/// children into it, stores it in the internal arena, and returns the new
/// shape's `id`.  The builder owns all shapes; callers navigate the graph
/// through ids.
// occt-ref: BRep_Builder // — builds BRep topology
pub struct BrepTopoBuilder {
    shapes: Vec<BrepTopoShape>,
    next_id: usize,
}

impl BrepTopoBuilder {
    /// Create an empty builder.
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            next_id: 0,
        }
    }

    // Internal helper: allocate the next id and push a shape.
    fn alloc(&mut self, shape_type: BrepTopoShapeType, label: &str) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.shapes.push(BrepTopoShape::new(id, shape_type, label));
        id
    }

    /// Create a `Vertex` shape and return its id.
    // occt-ref: BRep_Builder // ::MakeVertex
    pub fn make_vertex(&mut self, label: &str) -> usize {
        self.alloc(BrepTopoShapeType::Vertex, label)
    }

    /// Create an `Edge` whose two endpoint vertices are `v1` and `v2`.
    /// Returns the new edge's id.
    // occt-ref: BRep_Builder // ::MakeEdge
    pub fn make_edge(&mut self, v1: usize, v2: usize) -> usize {
        let id = self.alloc(BrepTopoShapeType::Edge, "edge");
        let shape = &mut self.shapes[id];
        shape.add_child(v1);
        shape.add_child(v2);
        id
    }

    /// Create a `Wire` from an ordered slice of edge ids.
    /// Returns the new wire's id.
    // occt-ref: BRep_Builder // ::MakeWire
    pub fn make_wire(&mut self, edges: &[usize]) -> usize {
        let id = self.alloc(BrepTopoShapeType::Wire, "wire");
        let edge_ids: Vec<usize> = edges.to_vec();
        for e in edge_ids {
            self.shapes[id].add_child(e);
        }
        id
    }

    /// Create a `Face` bounded by the wire with id `wire`.
    /// Returns the new face's id.
    // occt-ref: BRep_Builder // ::MakeFace
    pub fn make_face(&mut self, wire: usize) -> usize {
        let id = self.alloc(BrepTopoShapeType::Face, "face");
        self.shapes[id].add_child(wire);
        id
    }

    /// Create a `Shell` from an ordered slice of face ids.
    /// Returns the new shell's id.
    // occt-ref: BRep_Builder // ::MakeShell
    pub fn make_shell(&mut self, faces: &[usize]) -> usize {
        let id = self.alloc(BrepTopoShapeType::Shell, "shell");
        let face_ids: Vec<usize> = faces.to_vec();
        for f in face_ids {
            self.shapes[id].add_child(f);
        }
        id
    }

    /// Create a `Solid` bounded by the shell with id `shell`.
    /// Returns the new solid's id.
    // occt-ref: BRep_Builder // ::MakeSolid
    pub fn make_solid(&mut self, shell: usize) -> usize {
        let id = self.alloc(BrepTopoShapeType::Solid, "solid");
        self.shapes[id].add_child(shell);
        id
    }

    /// Look up the shape with the given `id`.  Returns `None` if the id has
    /// not been allocated yet.
    pub fn shape(&self, id: usize) -> Option<&BrepTopoShape> {
        self.shapes.get(id)
    }

    /// Return the total number of shapes currently held by the builder.
    pub fn nb_shapes(&self) -> usize {
        self.shapes.len()
    }
}

impl Default for BrepTopoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Internal unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_vertex_returns_sequential_ids() {
        let mut b = BrepTopoBuilder::new();
        let v0 = b.make_vertex("v0");
        let v1 = b.make_vertex("v1");
        assert_eq!(v0, 0);
        assert_eq!(v1, 1);
    }

    #[test]
    fn make_vertex_shape_type_and_label() {
        let mut b = BrepTopoBuilder::new();
        let id = b.make_vertex("P");
        let s = b.shape(id).unwrap();
        assert_eq!(s.shape_type(), BrepTopoShapeType::Vertex);
        assert_eq!(s.label(), "P");
        assert_eq!(s.orientation(), 0);
        assert!(s.children().is_empty());
    }

    #[test]
    fn make_edge_links_vertices() {
        let mut b = BrepTopoBuilder::new();
        let v0 = b.make_vertex("v0");
        let v1 = b.make_vertex("v1");
        let e = b.make_edge(v0, v1);
        let s = b.shape(e).unwrap();
        assert_eq!(s.shape_type(), BrepTopoShapeType::Edge);
        assert_eq!(s.children(), &[v0, v1]);
    }

    #[test]
    fn make_wire_links_edges() {
        let mut b = BrepTopoBuilder::new();
        let v0 = b.make_vertex("v0");
        let v1 = b.make_vertex("v1");
        let v2 = b.make_vertex("v2");
        let e0 = b.make_edge(v0, v1);
        let e1 = b.make_edge(v1, v2);
        let w = b.make_wire(&[e0, e1]);
        let s = b.shape(w).unwrap();
        assert_eq!(s.shape_type(), BrepTopoShapeType::Wire);
        assert_eq!(s.children(), &[e0, e1]);
    }

    #[test]
    fn make_face_links_wire() {
        let mut b = BrepTopoBuilder::new();
        let v0 = b.make_vertex("v0");
        let v1 = b.make_vertex("v1");
        let e = b.make_edge(v0, v1);
        let w = b.make_wire(&[e]);
        let f = b.make_face(w);
        let s = b.shape(f).unwrap();
        assert_eq!(s.shape_type(), BrepTopoShapeType::Face);
        assert_eq!(s.children(), &[w]);
    }

    #[test]
    fn make_shell_links_faces() {
        let mut b = BrepTopoBuilder::new();
        let v0 = b.make_vertex("v0");
        let v1 = b.make_vertex("v1");
        let e = b.make_edge(v0, v1);
        let w = b.make_wire(&[e]);
        let f0 = b.make_face(w);
        let f1 = b.make_face(w);
        let sh = b.make_shell(&[f0, f1]);
        let s = b.shape(sh).unwrap();
        assert_eq!(s.shape_type(), BrepTopoShapeType::Shell);
        assert_eq!(s.children(), &[f0, f1]);
    }

    #[test]
    fn make_solid_links_shell() {
        let mut b = BrepTopoBuilder::new();
        let v0 = b.make_vertex("v0");
        let v1 = b.make_vertex("v1");
        let e = b.make_edge(v0, v1);
        let w = b.make_wire(&[e]);
        let f = b.make_face(w);
        let sh = b.make_shell(&[f]);
        let sol = b.make_solid(sh);
        let s = b.shape(sol).unwrap();
        assert_eq!(s.shape_type(), BrepTopoShapeType::Solid);
        assert_eq!(s.children(), &[sh]);
    }

    #[test]
    fn nb_shapes_tracks_allocations() {
        let mut b = BrepTopoBuilder::new();
        assert_eq!(b.nb_shapes(), 0);
        b.make_vertex("a");
        assert_eq!(b.nb_shapes(), 1);
        b.make_vertex("b");
        assert_eq!(b.nb_shapes(), 2);
    }

    #[test]
    fn shape_returns_none_for_unknown_id() {
        let b = BrepTopoBuilder::new();
        assert!(b.shape(99).is_none());
    }

    #[test]
    fn set_orientation_round_trips() {
        let mut b = BrepTopoBuilder::new();
        let id = b.make_vertex("v");
        b.shape(id).unwrap(); // confirm exists
        let s = b.shapes.get_mut(id).unwrap();
        s.set_orientation(1);
        assert_eq!(s.orientation(), 1);
    }

    #[test]
    fn shape_type_enum_variants_distinct() {
        assert_ne!(BrepTopoShapeType::Vertex, BrepTopoShapeType::Edge);
        assert_ne!(BrepTopoShapeType::Wire, BrepTopoShapeType::Face);
        assert_ne!(BrepTopoShapeType::Shell, BrepTopoShapeType::Solid);
        assert_ne!(BrepTopoShapeType::CompSolid, BrepTopoShapeType::Compound);
    }
}
