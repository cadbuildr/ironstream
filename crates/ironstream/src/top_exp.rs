// FILE: top_exp.rs
//! Topology traversal utilities porting OCCT's TopExp and TopExp_Explorer.
//!
//! This module defines a rich, ID-keyed topology type system
//! (`TopoDS_Shape` and friends) on top of which the explorer and shape-map
//! utilities operate.  The types are intentionally similar to OCCT's
//! `TopoDS` hierarchy but kept self-contained here so that the lower-level
//! `topods` module (which is a tessellating solid representation) remains
//! independent.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Rich topology types (ID-keyed, suitable for traversal)
// ---------------------------------------------------------------------------

/// A topological vertex with a stable numeric id and a position.
// occt-ref: TopoDS_Vertex
#[derive(Clone, Debug, PartialEq)]
pub struct TopoDS_Vertex {
    pub id: u64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A topological edge connecting two vertices.
// occt-ref: TopoDS_Edge
#[derive(Clone, Debug, PartialEq)]
pub struct TopoDS_Edge {
    pub id: u64,
    pub vertices: Vec<TopoDS_Vertex>,
}

/// A closed loop of edges.
// occt-ref: TopoDS_Wire
#[derive(Clone, Debug, PartialEq)]
pub struct TopoDS_Wire {
    pub id: u64,
    pub edges: Vec<TopoDS_Edge>,
}

/// A bounded surface region, bounded by one or more wires.
// occt-ref: TopoDS_Face
#[derive(Clone, Debug, PartialEq)]
pub struct TopoDS_Face {
    pub id: u64,
    pub wires: Vec<TopoDS_Wire>,
}

/// A connected set of faces.
// occt-ref: TopoDS_Shell
#[derive(Clone, Debug, PartialEq)]
pub struct TopoDS_Shell {
    pub id: u64,
    pub faces: Vec<TopoDS_Face>,
}

/// A solid bounded by one or more shells.
// occt-ref: TopoDS_Solid
#[derive(Clone, Debug, PartialEq)]
pub struct TopoDS_Solid {
    pub id: u64,
    pub shells: Vec<TopoDS_Shell>,
}

/// A connected solid (rarely used; here for hierarchy completeness).
// occt: TopoDS_CompSolid
#[derive(Clone, Debug, PartialEq)]
pub struct TopoDS_CompSolid {
    pub id: u64,
    pub solids: Vec<TopoDS_Solid>,
}

/// A heterogeneous collection of shapes.
// occt-ref: TopoDS_Compound
#[derive(Clone, Debug, PartialEq)]
pub struct TopoDS_Compound {
    pub id: u64,
    pub shapes: Vec<TopoDS_Shape>,
}

/// A discriminated union over all topology levels.
// occt-ref: TopoDS_Shape
#[derive(Clone, Debug, PartialEq)]
pub enum TopoDS_Shape {
    Vertex(TopoDS_Vertex),
    Edge(TopoDS_Edge),
    Wire(TopoDS_Wire),
    Face(TopoDS_Face),
    Shell(TopoDS_Shell),
    Solid(TopoDS_Solid),
    CompSolid(TopoDS_CompSolid),
    Compound(TopoDS_Compound),
}

// ---------------------------------------------------------------------------
// Shape type enumeration
// ---------------------------------------------------------------------------

/// Shape type enumeration matching OCCT's TopAbs_ShapeEnum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeEnum {
    Compound,
    CompSolid,
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Vertex,
    Shape,
}

impl ShapeEnum {
    /// Returns the rank/dimension of the shape type (lower = simpler)
    pub fn rank(self) -> usize {
        match self {
            ShapeEnum::Vertex => 0,
            ShapeEnum::Edge => 1,
            ShapeEnum::Wire => 2,
            ShapeEnum::Face => 3,
            ShapeEnum::Shell => 4,
            ShapeEnum::Solid => 5,
            ShapeEnum::CompSolid => 6,
            ShapeEnum::Compound => 7,
            ShapeEnum::Shape => 8,
        }
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Returns the ShapeEnum variant for a given TopoDS_Shape
fn shape_type(shape: &TopoDS_Shape) -> ShapeEnum {
    match shape {
        TopoDS_Shape::Vertex(_) => ShapeEnum::Vertex,
        TopoDS_Shape::Edge(_) => ShapeEnum::Edge,
        TopoDS_Shape::Wire(_) => ShapeEnum::Wire,
        TopoDS_Shape::Face(_) => ShapeEnum::Face,
        TopoDS_Shape::Shell(_) => ShapeEnum::Shell,
        TopoDS_Shape::Solid(_) => ShapeEnum::Solid,
        TopoDS_Shape::CompSolid(_) => ShapeEnum::CompSolid,
        TopoDS_Shape::Compound(_) => ShapeEnum::Compound,
    }
}

/// Returns direct children of a TopoDS_Shape
fn direct_children(shape: &TopoDS_Shape) -> Vec<TopoDS_Shape> {
    match shape {
        TopoDS_Shape::Compound(c) => c.shapes.iter().cloned().collect(),
        TopoDS_Shape::CompSolid(cs) => cs.solids.iter().map(|s| TopoDS_Shape::Solid(s.clone())).collect(),
        TopoDS_Shape::Solid(s) => s.shells.iter().map(|sh| TopoDS_Shape::Shell(sh.clone())).collect(),
        TopoDS_Shape::Shell(sh) => sh.faces.iter().map(|f| TopoDS_Shape::Face(f.clone())).collect(),
        TopoDS_Shape::Face(f) => f.wires.iter().map(|w| TopoDS_Shape::Wire(w.clone())).collect(),
        TopoDS_Shape::Wire(w) => w.edges.iter().map(|e| TopoDS_Shape::Edge(e.clone())).collect(),
        TopoDS_Shape::Edge(e) => e.vertices.iter().map(|v| TopoDS_Shape::Vertex(v.clone())).collect(),
        TopoDS_Shape::Vertex(_) => vec![],
    }
}

/// Produce a stable u64 key for a shape based on its identity.
fn shape_key(shape: &TopoDS_Shape) -> u64 {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    let mut h = DefaultHasher::new();
    shape_hash(shape, &mut h);
    h.finish()
}

fn shape_hash<H: std::hash::Hasher>(shape: &TopoDS_Shape, h: &mut H) {
    use std::hash::Hash;
    match shape {
        TopoDS_Shape::Vertex(v) => {
            0u8.hash(h);
            v.id.hash(h);
        }
        TopoDS_Shape::Edge(e) => {
            1u8.hash(h);
            e.id.hash(h);
        }
        TopoDS_Shape::Wire(w) => {
            2u8.hash(h);
            w.id.hash(h);
        }
        TopoDS_Shape::Face(f) => {
            3u8.hash(h);
            f.id.hash(h);
        }
        TopoDS_Shape::Shell(sh) => {
            4u8.hash(h);
            sh.id.hash(h);
        }
        TopoDS_Shape::Solid(s) => {
            5u8.hash(h);
            s.id.hash(h);
        }
        TopoDS_Shape::CompSolid(cs) => {
            6u8.hash(h);
            cs.id.hash(h);
        }
        TopoDS_Shape::Compound(c) => {
            7u8.hash(h);
            c.id.hash(h);
        }
    }
}

// ---------------------------------------------------------------------------
// occt: TopExp
// ---------------------------------------------------------------------------

/// Topology exploration utilities — static helpers for building shape maps.
pub struct TopExp;

impl TopExp {
    /// Collect all sub-shapes of `to_find` type within `shape`, storing them
    /// in `map` (keyed by index, value = shape).  Equivalent to
    /// `TopExp::MapShapes` in OCCT.
    pub fn map_shapes(
        shape: &TopoDS_Shape,
        to_find: ShapeEnum,
        map: &mut IndexedShapeMap,
    ) {
        let st = shape_type(shape);
        if st == to_find {
            map.add(shape.clone());
            return;
        }
        // Only recurse if to_find is a sub-type of the current shape
        if st.rank() > to_find.rank() {
            for child in direct_children(shape) {
                Self::map_shapes(&child, to_find, map);
            }
        }
    }

    /// Like `map_shapes`, but also records parent shapes (ancestors).
    /// For each sub-shape of type `to_find`, stores a list of ancestors of
    /// type `to_ancestor` that contain it.  Equivalent to
    /// `TopExp::MapShapesAndAncestors` in OCCT.
    pub fn map_shapes_and_ancestors(
        shape: &TopoDS_Shape,
        to_find: ShapeEnum,
        to_ancestor: ShapeEnum,
        map: &mut AncestorMap,
    ) {
        // Collect all ancestor-type shapes first
        let mut ancestor_map = IndexedShapeMap::new();
        Self::map_shapes(shape, to_ancestor, &mut ancestor_map);

        // For each ancestor, collect its sub-shapes of to_find type
        for (ancestor_idx, ancestor) in ancestor_map.shapes.iter().enumerate() {
            let mut sub_map = IndexedShapeMap::new();
            Self::map_shapes(ancestor, to_find, &mut sub_map);
            for sub in &sub_map.shapes {
                let idx = map.find_map.add(sub.clone());
                map.ancestors.entry(idx).or_insert_with(Vec::new).push(ancestor_idx);
                map.ancestor_shapes.entry(ancestor_idx).or_insert_with(|| ancestor.clone());
            }
        }
    }
}

// ---------------------------------------------------------------------------
// IndexedShapeMap
// ---------------------------------------------------------------------------

/// An ordered, deduplicated map of shapes (indexed from 0).
/// Shapes are identified by structural equality.
pub struct IndexedShapeMap {
    pub shapes: Vec<TopoDS_Shape>,
    // index lookup: shape hash -> position in shapes
    index: HashMap<u64, usize>,
}

impl IndexedShapeMap {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            index: HashMap::new(),
        }
    }

    /// Add a shape, returning its index (existing if already present).
    pub fn add(&mut self, shape: TopoDS_Shape) -> usize {
        let key = shape_key(&shape);
        if let Some(&idx) = self.index.get(&key) {
            return idx;
        }
        let idx = self.shapes.len();
        self.index.insert(key, idx);
        self.shapes.push(shape);
        idx
    }

    /// Find the index of a shape, or None.
    pub fn find(&self, shape: &TopoDS_Shape) -> Option<usize> {
        let key = shape_key(shape);
        self.index.get(&key).copied()
    }

    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }
}

impl Default for IndexedShapeMap {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// AncestorMap
// ---------------------------------------------------------------------------

/// Map from sub-shape index to list of ancestor indices, plus the ancestor shapes themselves.
pub struct AncestorMap {
    pub find_map: IndexedShapeMap,
    /// sub-shape idx -> list of ancestor indices (into ancestor_shapes)
    pub ancestors: HashMap<usize, Vec<usize>>,
    pub ancestor_shapes: HashMap<usize, TopoDS_Shape>,
}

impl AncestorMap {
    pub fn new() -> Self {
        Self {
            find_map: IndexedShapeMap::new(),
            ancestors: HashMap::new(),
            ancestor_shapes: HashMap::new(),
        }
    }

    /// Return the list of ancestors for a given sub-shape, or empty slice.
    pub fn ancestors_of(&self, shape: &TopoDS_Shape) -> Vec<&TopoDS_Shape> {
        if let Some(idx) = self.find_map.find(shape) {
            if let Some(anc_indices) = self.ancestors.get(&idx) {
                return anc_indices
                    .iter()
                    .filter_map(|i| self.ancestor_shapes.get(i))
                    .collect();
            }
        }
        vec![]
    }
}

impl Default for AncestorMap {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// occt: TopExp_Explorer
// ---------------------------------------------------------------------------

/// Iterator over sub-shapes of a given type within a root shape.
///
/// Mirrors `TopExp_Explorer` from OCCT.  Call `init` to start a traversal,
/// then loop with `more` / `current` / `next`.
///
/// The optional `to_avoid` type acts as a pruning barrier: when the explorer
/// encounters a shape of that type it does not descend into it (even if
/// sub-shapes of `to_find` might be inside).
pub struct TopExpExplorer {
    /// Stack of (shape, child_index) for DFS traversal
    stack: Vec<(TopoDS_Shape, usize)>,
    /// The shape type we are looking for
    to_find: ShapeEnum,
    /// Optional shape type to avoid descending into
    to_avoid: Option<ShapeEnum>,
    /// The current matching shape, if any
    current: Option<TopoDS_Shape>,
    /// Whether the explorer has been initialized
    initialized: bool,
}

impl TopExpExplorer {
    /// Create a new, uninitialized explorer.
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            to_find: ShapeEnum::Shape,
            to_avoid: None,
            current: None,
            initialized: false,
        }
    }

    /// Initialize (or re-initialize) the explorer.
    ///
    /// - `shape` — root shape to traverse
    /// - `to_find` — type of sub-shapes to collect
    /// - `to_avoid` — if `Some(t)`, shapes of type `t` are not descended into
    pub fn init(&mut self, shape: TopoDS_Shape, to_find: ShapeEnum, to_avoid: Option<ShapeEnum>) {
        self.stack.clear();
        self.to_find = to_find;
        self.to_avoid = to_avoid;
        self.current = None;
        self.initialized = true;
        // Push the root
        self.stack.push((shape, 0));
        // Advance to first match
        self.advance();
    }

    /// Returns `true` if there is a current shape to examine.
    pub fn more(&self) -> bool {
        self.current.is_some()
    }

    /// Returns the current shape.  Panics if `more()` is false.
    pub fn current(&self) -> &TopoDS_Shape {
        self.current.as_ref().expect("TopExpExplorer: no current shape (call more() first)")
    }

    /// Advance to the next matching shape.
    pub fn next(&mut self) {
        // Pop current match and continue
        self.current = None;
        // We need to continue from where we were.  The stack top is the node
        // we just matched; we need to decide whether to descend or move on.
        // After popping, advance() handles the rest.
        self.advance();
    }

    /// Returns the depth of the current shape in the traversal stack.
    pub fn depth(&self) -> usize {
        // depth = number of frames on stack (the current match is not on stack)
        self.stack.len()
    }

    /// Reset location (no-op in this pure-topology implementation; present for
    /// API compatibility).
    pub fn reset_location(&mut self) {
        // In OCCT this resets the accumulated location transform.
        // In our pure-topology model there are no locations to reset.
    }

    /// Internal DFS advance: walk the stack until we find a shape of `to_find`
    /// type, or exhaust the traversal.
    fn advance(&mut self) {
        while let Some((shape, child_idx)) = self.stack.last_mut() {
            let st = shape_type(shape);

            // First visit: check the node itself for a match.
            // Always mark as visited (child_idx = 1) so the descend logic below
            // correctly maps child_idx → child_pos via (child_idx - 1).
            if *child_idx == 0 {
                *child_idx = 1;
                let is_avoided = self.to_avoid.map_or(false, |av| av == st);
                if !is_avoided && st == self.to_find {
                    self.current = Some(shape.clone());
                    return;
                }
            }

            // Determine whether we should descend into children.
            let is_avoided = self.to_avoid.map_or(false, |av| av == st);
            let should_descend = !is_avoided && st.rank() > self.to_find.rank();

            if should_descend {
                let children = direct_children(shape);
                let idx = *child_idx; // guaranteed >= 1 here
                let child_pos = idx - 1; // idx=1 → child[0], idx=2 → child[1], …
                if child_pos < children.len() {
                    *child_idx = idx + 1;
                    let child = children[child_pos].clone();
                    self.stack.push((child, 0));
                } else {
                    self.stack.pop();
                }
            } else {
                self.stack.pop();
            }
        }
        self.current = None;
    }

    /// Collect all matching shapes into a Vec (consumes remaining iterations).
    pub fn collect_all(&mut self) -> Vec<TopoDS_Shape> {
        let mut result = Vec::new();
        while self.more() {
            result.push(self.current().clone());
            self.next();
        }
        result
    }
}

impl Default for TopExpExplorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal vertex
    fn make_vertex(id: u64) -> TopoDS_Shape {
        TopoDS_Shape::Vertex(TopoDS_Vertex { id, x: 0.0, y: 0.0, z: 0.0 })
    }

    /// Build an edge with two vertices
    fn make_edge(id: u64, v0: u64, v1: u64) -> TopoDS_Shape {
        TopoDS_Shape::Edge(TopoDS_Edge {
            id,
            vertices: vec![
                TopoDS_Vertex { id: v0, x: 0.0, y: 0.0, z: 0.0 },
                TopoDS_Vertex { id: v1, x: 1.0, y: 0.0, z: 0.0 },
            ],
        })
    }

    /// Build a wire from edge ids (reuses make_edge with sequential vertex ids)
    fn make_wire_with_edges(wire_id: u64, edge_ids: &[(u64, u64, u64)]) -> TopoDS_Wire {
        let edges = edge_ids.iter().map(|(eid, v0, v1)| {
            TopoDS_Edge {
                id: *eid,
                vertices: vec![
                    TopoDS_Vertex { id: *v0, x: 0.0, y: 0.0, z: 0.0 },
                    TopoDS_Vertex { id: *v1, x: 1.0, y: 0.0, z: 0.0 },
                ],
            }
        }).collect();
        TopoDS_Wire { id: wire_id, edges }
    }

    fn make_face(face_id: u64, wire: TopoDS_Wire) -> TopoDS_Face {
        TopoDS_Face { id: face_id, wires: vec![wire] }
    }

    fn make_shell(shell_id: u64, faces: Vec<TopoDS_Face>) -> TopoDS_Shell {
        TopoDS_Shell { id: shell_id, faces }
    }

    fn make_solid(solid_id: u64, shell: TopoDS_Shell) -> TopoDS_Solid {
        TopoDS_Solid { id: solid_id, shells: vec![shell] }
    }

    fn make_compound(compound_id: u64, shapes: Vec<TopoDS_Shape>) -> TopoDS_Shape {
        TopoDS_Shape::Compound(TopoDS_Compound { id: compound_id, shapes })
    }

    // ---- shape_type tests ----

    #[test]
    fn test_shape_type_vertex() {
        let v = make_vertex(1);
        assert_eq!(shape_type(&v), ShapeEnum::Vertex);
    }

    #[test]
    fn test_shape_type_edge() {
        let e = make_edge(1, 10, 11);
        assert_eq!(shape_type(&e), ShapeEnum::Edge);
    }

    #[test]
    fn test_shape_type_compound() {
        let c = make_compound(99, vec![]);
        assert_eq!(shape_type(&c), ShapeEnum::Compound);
    }

    // ---- ShapeEnum rank ----

    #[test]
    fn test_shape_enum_ranks_ordered() {
        assert!(ShapeEnum::Vertex.rank() < ShapeEnum::Edge.rank());
        assert!(ShapeEnum::Edge.rank() < ShapeEnum::Wire.rank());
        assert!(ShapeEnum::Wire.rank() < ShapeEnum::Face.rank());
        assert!(ShapeEnum::Face.rank() < ShapeEnum::Shell.rank());
        assert!(ShapeEnum::Shell.rank() < ShapeEnum::Solid.rank());
        assert!(ShapeEnum::Solid.rank() < ShapeEnum::Compound.rank());
    }

    // ---- IndexedShapeMap ----

    #[test]
    fn test_indexed_shape_map_deduplication() {
        let mut map = IndexedShapeMap::new();
        let v1 = make_vertex(1);
        let v2 = make_vertex(1); // same id → same shape
        let v3 = make_vertex(2); // different id

        let i0 = map.add(v1.clone());
        let i1 = map.add(v2.clone()); // should be same index
        let i2 = map.add(v3.clone());

        assert_eq!(i0, 0);
        assert_eq!(i1, 0); // dedup
        assert_eq!(i2, 1);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_indexed_shape_map_find() {
        let mut map = IndexedShapeMap::new();
        let v = make_vertex(42);
        map.add(v.clone());
        assert_eq!(map.find(&v), Some(0));
        let absent = make_vertex(99);
        assert_eq!(map.find(&absent), None);
    }

    // ---- TopExp::map_shapes ----

    #[test]
    fn test_map_shapes_vertices_from_edge() {
        let edge = make_edge(1, 10, 11);
        let mut map = IndexedShapeMap::new();
        TopExp::map_shapes(&edge, ShapeEnum::Vertex, &mut map);
        assert_eq!(map.len(), 2, "edge should have 2 vertices");
    }

    #[test]
    fn test_map_shapes_edges_from_wire() {
        let wire = make_wire_with_edges(1, &[(10, 100, 101), (11, 102, 103), (12, 104, 105)]);
        let shape = TopoDS_Shape::Wire(wire);
        let mut map = IndexedShapeMap::new();
        TopExp::map_shapes(&shape, ShapeEnum::Edge, &mut map);
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_map_shapes_vertices_from_wire() {
        // Wire with 2 edges, each sharing a vertex (ids 100, 101, 100) → 2 distinct vertices
        let wire = make_wire_with_edges(1, &[(10, 100, 101), (11, 101, 102)]);
        let shape = TopoDS_Shape::Wire(wire);
        let mut map = IndexedShapeMap::new();
        TopExp::map_shapes(&shape, ShapeEnum::Vertex, &mut map);
        // 3 vertex objects but only 2 distinct ids (101 shared)
        assert_eq!(map.len(), 3); // topological dedup by id: 100, 101, 102
    }

    #[test]
    fn test_map_shapes_vertex_itself() {
        let v = make_vertex(5);
        let mut map = IndexedShapeMap::new();
        TopExp::map_shapes(&v, ShapeEnum::Vertex, &mut map);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_map_shapes_no_match_higher_type() {
        // Ask for faces from a vertex — should be empty
        let v = make_vertex(1);
        let mut map = IndexedShapeMap::new();
        TopExp::map_shapes(&v, ShapeEnum::Face, &mut map);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_shapes_compound_of_edges() {
        let e1 = make_edge(1, 10, 11);
        let e2 = make_edge(2, 12, 13);
        let e3 = make_edge(3, 14, 15);
        let compound = make_compound(99, vec![e1, e2, e3]);

        let mut edge_map = IndexedShapeMap::new();
        TopExp::map_shapes(&compound, ShapeEnum::Edge, &mut edge_map);
        assert_eq!(edge_map.len(), 3);

        let mut vertex_map = IndexedShapeMap::new();
        TopExp::map_shapes(&compound, ShapeEnum::Vertex, &mut vertex_map);
        assert_eq!(vertex_map.len(), 6); // 3 edges × 2 distinct vertices each
    }

    #[test]
    fn test_map_shapes_nested_compound() {
        let e1 = make_edge(1, 10, 11);
        let e2 = make_edge(2, 12, 13);
        let inner = make_compound(10, vec![e1]);
        let outer = make_compound(20, vec![inner, e2]);

        let mut map = IndexedShapeMap::new();
        TopExp::map_shapes(&outer, ShapeEnum::Edge, &mut map);
        assert_eq!(map.len(), 2);
    }

    // ---- TopExp::map_shapes_and_ancestors ----

    #[test]
    fn test_map_shapes_and_ancestors_edge_to_vertex() {
        let wire = make_wire_with_edges(1, &[(10, 100, 101), (11, 101, 102)]);
        let shape = TopoDS_Shape::Wire(wire);

        let mut amap = AncestorMap::new();
        TopExp::map_shapes_and_ancestors(&shape, ShapeEnum::Vertex, ShapeEnum::Edge, &mut amap);

        // 3 distinct vertices, each should have at least 1 ancestor edge
        assert_eq!(amap.find_map.len(), 3);

        // vertex 101 is shared between edge 10 and edge 11 → 2 ancestors
        let v101 = TopoDS_Shape::Vertex(TopoDS_Vertex { id: 101, x: 0.0, y: 0.0, z: 0.0 });
        let ancs = amap.ancestors_of(&v101);
        assert_eq!(ancs.len(), 2, "vertex 101 should have 2 ancestor edges");
    }

    #[test]
    fn test_map_shapes_and_ancestors_face_to_edge() {
        let w1 = make_wire_with_edges(1, &[(10, 100, 101), (11, 101, 102), (12, 102, 100)]);
        let w2 = make_wire_with_edges(2, &[(20, 200, 201), (21, 201, 202), (22, 202, 200)]);
        let f1 = make_face(1, w1);
        let f2 = make_face(2, w2);
        let shell = make_shell(1, vec![f1, f2]);
        let shape = TopoDS_Shape::Shell(shell);

        let mut amap = AncestorMap::new();
        TopExp::map_shapes_and_ancestors(&shape, ShapeEnum::Edge, ShapeEnum::Face, &mut amap);

        // 6 distinct edges total
        assert_eq!(amap.find_map.len(), 6);
    }

    // ---- TopExpExplorer ----

    #[test]
    fn test_explorer_vertices_from_edge() {
        let edge = make_edge(1, 10, 11);
        let mut ex = TopExpExplorer::new();
        ex.init(edge, ShapeEnum::Vertex, None);

        let mut count = 0;
        while ex.more() {
            let s = ex.current();
            assert_eq!(shape_type(s), ShapeEnum::Vertex);
            count += 1;
            ex.next();
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_explorer_edges_from_wire() {
        let wire = make_wire_with_edges(1, &[(10, 100, 101), (11, 102, 103), (12, 104, 105)]);
        let shape = TopoDS_Shape::Wire(wire);
        let mut ex = TopExpExplorer::new();
        ex.init(shape, ShapeEnum::Edge, None);

        let results = ex.collect_all();
        assert_eq!(results.len(), 3);
        for s in &results {
            assert_eq!(shape_type(s), ShapeEnum::Edge);
        }
    }

    #[test]
    fn test_explorer_vertices_from_wire() {
        let wire = make_wire_with_edges(1, &[(10, 100, 101), (11, 102, 103)]);
        let shape = TopoDS_Shape::Wire(wire);
        let mut ex = TopExpExplorer::new();
        ex.init(shape, ShapeEnum::Vertex, None);

        let results = ex.collect_all();
        assert_eq!(results.len(), 4);
    }

    #[test]
    fn test_explorer_empty_compound() {
        let compound = make_compound(1, vec![]);
        let mut ex = TopExpExplorer::new();
        ex.init(compound, ShapeEnum::Edge, None);
        assert!(!ex.more());
    }

    #[test]
    fn test_explorer_single_vertex() {
        let v = make_vertex(42);
        let mut ex = TopExpExplorer::new();
        ex.init(v, ShapeEnum::Vertex, None);
        assert!(ex.more());
        let s = ex.current();
        assert_eq!(shape_type(s), ShapeEnum::Vertex);
        ex.next();
        assert!(!ex.more());
    }

    #[test]
    fn test_explorer_no_match() {
        let v = make_vertex(1);
        let mut ex = TopExpExplorer::new();
        ex.init(v, ShapeEnum::Edge, None);
        assert!(!ex.more(), "vertex has no edges");
    }

    #[test]
    fn test_explorer_compound_of_edges() {
        let e1 = make_edge(1, 10, 11);
        let e2 = make_edge(2, 12, 13);
        let compound = make_compound(99, vec![e1, e2]);
        let mut ex = TopExpExplorer::new();
        ex.init(compound, ShapeEnum::Edge, None);
        let results = ex.collect_all();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_explorer_compound_edges_then_vertices() {
        let e1 = make_edge(1, 10, 11);
        let e2 = make_edge(2, 12, 13);
        let compound = make_compound(99, vec![e1, e2]);

        // vertices
        let mut ex = TopExpExplorer::new();
        ex.init(compound.clone(), ShapeEnum::Vertex, None);
        let verts = ex.collect_all();
        assert_eq!(verts.len(), 4);

        // edges (re-init)
        ex.init(compound, ShapeEnum::Edge, None);
        let edges = ex.collect_all();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_explorer_to_avoid_stops_descent() {
        // Compound → Edge → Vertex, but avoid Edge: should not find vertices inside edges
        let e1 = make_edge(1, 10, 11);
        let compound = make_compound(99, vec![e1]);
        let mut ex = TopExpExplorer::new();
        // Find vertices, but avoid descending into edges
        ex.init(compound, ShapeEnum::Vertex, Some(ShapeEnum::Edge));
        assert!(!ex.more(), "should not find vertices when edges are avoided");
    }

    #[test]
    fn test_explorer_to_avoid_does_not_skip_target_itself() {
        // If to_find == to_avoid, we should not yield anything (avoidance takes priority)
        let e1 = make_edge(1, 10, 11);
        let compound = make_compound(99, vec![e1]);
        let mut ex = TopExpExplorer::new();
        ex.init(compound, ShapeEnum::Edge, Some(ShapeEnum::Edge));
        // Compound descends to edge, edge is to_avoid so not yielded
        assert!(!ex.more());
    }

    #[test]
    fn test_explorer_depth() {
        let wire = make_wire_with_edges(1, &[(10, 100, 101)]);
        let shape = TopoDS_Shape::Wire(wire);
        let mut ex = TopExpExplorer::new();
        ex.init(shape, ShapeEnum::Edge, None);
        assert!(ex.more());
        // depth is non-zero when we have a current shape
        let _ = ex.depth(); // just ensure it doesn't panic
    }

    #[test]
    fn test_explorer_nested_compound() {
        let e1 = make_edge(1, 10, 11);
        let e2 = make_edge(2, 12, 13);
        let inner = make_compound(10, vec![e1]);
        let outer = make_compound(20, vec![inner, e2]);

        let mut ex = TopExpExplorer::new();
        ex.init(outer, ShapeEnum::Edge, None);
        let edges = ex.collect_all();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_explorer_solid_to_faces() {
        let w1 = make_wire_with_edges(1, &[(10, 100, 101), (11, 101, 102), (12, 102, 100)]);
        let w2 = make_wire_with_edges(2, &[(20, 200, 201), (21, 201, 202), (22, 202, 200)]);
        let f1 = make_face(1, w1);
        let f2 = make_face(2, w2);
        let shell = make_shell(1, vec![f1, f2]);
        let solid = make_solid(1, shell);
        let shape = TopoDS_Shape::Solid(solid);

        let mut ex = TopExpExplorer::new();
        ex.init(shape.clone(), ShapeEnum::Face, None);
        let faces = ex.collect_all();
        assert_eq!(faces.len(), 2);

        ex.init(shape.clone(), ShapeEnum::Wire, None);
        let wires = ex.collect_all();
        assert_eq!(wires.len(), 2);

        ex.init(shape, ShapeEnum::Edge, None);
        let edges = ex.collect_all();
        assert_eq!(edges.len(), 6);
    }

    #[test]
    fn test_explorer_reinit_resets_state() {
        let e1 = make_edge(1, 10, 11);
        let compound = make_compound(1, vec![e1]);

        let mut ex = TopExpExplorer::new();
        ex.init(compound.clone(), ShapeEnum::Edge, None);
        // Exhaust
        while ex.more() { ex.next(); }
        assert!(!ex.more());

        // Reinit should restart
        ex.init(compound, ShapeEnum::Edge, None);
        assert!(ex.more());
        let results = ex.collect_all();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_map_shapes_vertex_count_with_sharing() {
        // Two edges sharing a vertex (same id)
        let wire = make_wire_with_edges(1, &[(1, 10, 11), (2, 11, 12)]);
        let shape = TopoDS_Shape::Wire(wire);
        let mut map = IndexedShapeMap::new();
        TopExp::map_shapes(&shape, ShapeEnum::Vertex, &mut map);
        // 3 unique vertex ids: 10, 11, 12
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_explorer_collect_all_cleans_up() {
        let wire = make_wire_with_edges(1, &[(1, 10, 11), (2, 12, 13), (3, 14, 15)]);
        let shape = TopoDS_Shape::Wire(wire);
        let mut ex = TopExpExplorer::new();
        ex.init(shape, ShapeEnum::Vertex, None);
        let all = ex.collect_all();
        assert_eq!(all.len(), 6);
        // After collect_all, more() should be false
        assert!(!ex.more());
    }
}
