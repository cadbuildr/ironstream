// FILE: rust/ironstream/crates/ironstream/src/topo_builder.rs
//! Zero-dependency stub for low-level topological shape construction.
//!
//! Models the public interfaces of `BRep_Builder` and `TopoDS_Builder` from
//! OCCT.  Shapes are represented as opaque string tokens (e.g. `"vertex:0"`,
//! `"edge:v0..v1"`); no geometric data is carried.  All formatting is done with
//! `std` primitives only — no external crates are used.

// ─────────────────────────────────────────────────────────────────────────────
// TopoBuilder — static factory methods
// ─────────────────────────────────────────────────────────────────────────────

/// Static factory that creates opaque shape tokens for each topological kind.
///
/// Each method mirrors the corresponding low-level builder call in OCCT and
/// returns a `String` token that encodes the shape's kind and its inputs.
/// The token format is intentionally human-readable so that unit tests and
/// debug output remain self-documenting.
// occt: TopoDS_Builder
// occt: BRep_Builder
pub struct TopoBuilder;

impl TopoBuilder {
    /// Create a vertex at the given 3-D point and return its token.
    ///
    /// The token encodes the three coordinates rounded to six decimal places.
    ///
    /// # Example
    /// ```
    /// # use ironstream::topo_builder::TopoBuilder;
    /// let v = TopoBuilder::make_vertex([1.0, 2.0, 3.0]);
    /// assert!(v.starts_with("vertex:("));
    /// ```
    // occt: BRep_Builder::MakeVertex / TopoDS_Builder::MakeVertex
    pub fn make_vertex(pt: [f64; 3]) -> String {
        format!(
            "vertex:({:.6},{:.6},{:.6})",
            pt[0], pt[1], pt[2]
        )
    }

    /// Create an edge between two vertex tokens and return its token.
    ///
    /// # Example
    /// ```
    /// # use ironstream::topo_builder::TopoBuilder;
    /// let v0 = TopoBuilder::make_vertex([0.0, 0.0, 0.0]);
    /// let v1 = TopoBuilder::make_vertex([1.0, 0.0, 0.0]);
    /// let e  = TopoBuilder::make_edge(&v0, &v1);
    /// assert!(e.starts_with("edge:["));
    /// ```
    // occt: BRep_Builder::MakeEdge / TopoDS_Builder::MakeEdge
    pub fn make_edge(v1: &str, v2: &str) -> String {
        format!("edge:[{v1}..{v2}]")
    }

    /// Create a wire from an ordered slice of edge tokens and return its token.
    ///
    /// An empty edge list produces the token `"wire:[]"`.
    // occt: BRep_Builder::MakeWire / TopoDS_Builder::MakeWire
    pub fn make_wire(edges: &[String]) -> String {
        let inner = edges.join(",");
        format!("wire:[{inner}]")
    }

    /// Create a face bounded by a wire token and return its token.
    // occt: BRep_Builder::MakeFace / TopoDS_Builder::MakeFace
    pub fn make_face(wire: &str) -> String {
        format!("face:[{wire}]")
    }

    /// Create a shell from an ordered slice of face tokens and return its token.
    ///
    /// An empty face list produces the token `"shell:[]"`.
    // occt: BRep_Builder::MakeShell / TopoDS_Builder::MakeShell
    pub fn make_shell(faces: &[String]) -> String {
        let inner = faces.join(",");
        format!("shell:[{inner}]")
    }

    /// Create a solid bounded by a shell token and return its token.
    // occt: BRep_Builder::MakeSolid / TopoDS_Builder::MakeSolid
    pub fn make_solid(shell: &str) -> String {
        format!("solid:[{shell}]")
    }

    /// Create a compound from an ordered slice of shape tokens and return its token.
    ///
    /// An empty shape list produces the token `"compound:[]"`.
    // occt: BRep_Builder::MakeCompound / TopoDS_Builder::MakeCompound
    pub fn make_compound(shapes: &[String]) -> String {
        let inner = shapes.join(",");
        format!("compound:[{inner}]")
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BrepBuilder — incremental B-Rep mesh builder
// ─────────────────────────────────────────────────────────────────────────────

/// Incremental builder for a lightweight B-Rep mesh (vertex soup + edge list).
///
/// Vertices are stored as 3-D coordinate triples; edges reference vertices by
/// their zero-based index into the `vertices` vector.  Call [`build`] to
/// serialise the accumulated geometry into a summary token.
///
/// This mirrors the incremental add/commit pattern found in OCCT's
/// `BRep_Builder` / `BRepBuilderAPI_*` family: add primitives one at a time
/// and materialise the result only when `build` is called.
// occt: BRep_Builder
// occt: TopoDS_Builder
pub struct BrepBuilder {
    /// Ordered list of vertex positions.
    pub vertices: Vec<[f64; 3]>,
    /// Ordered list of edges, each referencing two vertex indices.
    pub edges: Vec<(usize, usize)>,
}

impl BrepBuilder {
    /// Create an empty builder with no vertices or edges.
    // occt: BRep_Builder (constructor)
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Append a vertex at position `p` and return its index.
    ///
    /// Indices are zero-based and stable: the first vertex added has index `0`,
    /// the second has index `1`, and so on.
    // occt: BRep_Builder::MakeVertex — adds a vertex to the shape
    pub fn add_vertex(&mut self, p: [f64; 3]) -> usize {
        let idx = self.vertices.len();
        self.vertices.push(p);
        idx
    }

    /// Append an edge connecting the vertices at indices `v1` and `v2`.
    ///
    /// # Panics
    ///
    /// Panics (debug build only) if either index is out of range.
    // occt: BRep_Builder::MakeEdge — adds an edge between two vertices
    pub fn add_edge(&mut self, v1: usize, v2: usize) {
        debug_assert!(v1 < self.vertices.len(), "v1 index out of range");
        debug_assert!(v2 < self.vertices.len(), "v2 index out of range");
        self.edges.push((v1, v2));
    }

    /// Serialise the accumulated B-Rep data into a summary string.
    ///
    /// The returned token encodes the vertex count and the edge list so that
    /// callers (and tests) can verify the structure without inspecting internal
    /// fields.
    ///
    /// Format: `"brep:V<n_verts>E[(<i0>,<i1>),...]"`.
    // occt: BRep_Builder — finalises the BRep shape
    pub fn build(&self) -> String {
        let n = self.vertices.len();

        if self.edges.is_empty() {
            return format!("brep:V{n}E[]");
        }

        let edge_tokens: Vec<String> = self
            .edges
            .iter()
            .map(|(a, b)| format!("({a},{b})"))
            .collect();

        format!("brep:V{n}E[{}]", edge_tokens.join(","))
    }
}

impl Default for BrepBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // TopoBuilder::make_vertex ────────────────────────────────────────────────

    #[test]
    fn make_vertex_encodes_coordinates() {
        let v = TopoBuilder::make_vertex([1.0, -2.5, 0.0]);
        assert_eq!(v, "vertex:(1.000000,-2.500000,0.000000)");
    }

    #[test]
    fn make_vertex_origin() {
        let v = TopoBuilder::make_vertex([0.0, 0.0, 0.0]);
        assert_eq!(v, "vertex:(0.000000,0.000000,0.000000)");
    }

    // TopoBuilder::make_edge ──────────────────────────────────────────────────

    #[test]
    fn make_edge_encodes_vertices() {
        let v0 = TopoBuilder::make_vertex([0.0, 0.0, 0.0]);
        let v1 = TopoBuilder::make_vertex([1.0, 0.0, 0.0]);
        let e = TopoBuilder::make_edge(&v0, &v1);
        assert!(e.starts_with("edge:[vertex:"));
        assert!(e.contains(".."));
    }

    #[test]
    fn make_edge_format() {
        let e = TopoBuilder::make_edge("A", "B");
        assert_eq!(e, "edge:[A..B]");
    }

    // TopoBuilder::make_wire ──────────────────────────────────────────────────

    #[test]
    fn make_wire_empty() {
        let w = TopoBuilder::make_wire(&[]);
        assert_eq!(w, "wire:[]");
    }

    #[test]
    fn make_wire_single_edge() {
        let e = "edge:[A..B]".to_string();
        let w = TopoBuilder::make_wire(&[e.clone()]);
        assert_eq!(w, format!("wire:[{e}]"));
    }

    #[test]
    fn make_wire_multiple_edges() {
        let edges: Vec<String> = vec!["e0".into(), "e1".into(), "e2".into()];
        let w = TopoBuilder::make_wire(&edges);
        assert_eq!(w, "wire:[e0,e1,e2]");
    }

    // TopoBuilder::make_face ──────────────────────────────────────────────────

    #[test]
    fn make_face_encodes_wire() {
        let wire = "wire:[e0,e1]";
        let f = TopoBuilder::make_face(wire);
        assert_eq!(f, "face:[wire:[e0,e1]]");
    }

    // TopoBuilder::make_shell ─────────────────────────────────────────────────

    #[test]
    fn make_shell_empty() {
        let s = TopoBuilder::make_shell(&[]);
        assert_eq!(s, "shell:[]");
    }

    #[test]
    fn make_shell_two_faces() {
        let faces: Vec<String> = vec!["face:[w0]".into(), "face:[w1]".into()];
        let sh = TopoBuilder::make_shell(&faces);
        assert_eq!(sh, "shell:[face:[w0],face:[w1]]");
    }

    // TopoBuilder::make_solid ─────────────────────────────────────────────────

    #[test]
    fn make_solid_encodes_shell() {
        let shell = "shell:[face:[w0]]";
        let sol = TopoBuilder::make_solid(shell);
        assert_eq!(sol, "solid:[shell:[face:[w0]]]");
    }

    // TopoBuilder::make_compound ──────────────────────────────────────────────

    #[test]
    fn make_compound_empty() {
        let c = TopoBuilder::make_compound(&[]);
        assert_eq!(c, "compound:[]");
    }

    #[test]
    fn make_compound_mixed_shapes() {
        let shapes: Vec<String> = vec!["solid:[s0]".into(), "solid:[s1]".into()];
        let c = TopoBuilder::make_compound(&shapes);
        assert_eq!(c, "compound:[solid:[s0],solid:[s1]]");
    }

    // BrepBuilder::new / default ──────────────────────────────────────────────

    #[test]
    fn new_builder_is_empty() {
        let b = BrepBuilder::new();
        assert!(b.vertices.is_empty());
        assert!(b.edges.is_empty());
    }

    #[test]
    fn default_equals_new() {
        let b: BrepBuilder = Default::default();
        assert!(b.vertices.is_empty());
        assert!(b.edges.is_empty());
    }

    // BrepBuilder::add_vertex ─────────────────────────────────────────────────

    #[test]
    fn add_vertex_returns_sequential_indices() {
        let mut b = BrepBuilder::new();
        assert_eq!(b.add_vertex([0.0, 0.0, 0.0]), 0);
        assert_eq!(b.add_vertex([1.0, 0.0, 0.0]), 1);
        assert_eq!(b.add_vertex([1.0, 1.0, 0.0]), 2);
    }

    #[test]
    fn add_vertex_stores_coordinates() {
        let mut b = BrepBuilder::new();
        let idx = b.add_vertex([3.0, 4.0, 5.0]);
        assert_eq!(b.vertices[idx], [3.0, 4.0, 5.0]);
    }

    // BrepBuilder::add_edge ───────────────────────────────────────────────────

    #[test]
    fn add_edge_stores_index_pair() {
        let mut b = BrepBuilder::new();
        let v0 = b.add_vertex([0.0, 0.0, 0.0]);
        let v1 = b.add_vertex([1.0, 0.0, 0.0]);
        b.add_edge(v0, v1);
        assert_eq!(b.edges, vec![(0, 1)]);
    }

    #[test]
    fn add_multiple_edges() {
        let mut b = BrepBuilder::new();
        let v0 = b.add_vertex([0.0, 0.0, 0.0]);
        let v1 = b.add_vertex([1.0, 0.0, 0.0]);
        let v2 = b.add_vertex([1.0, 1.0, 0.0]);
        b.add_edge(v0, v1);
        b.add_edge(v1, v2);
        b.add_edge(v2, v0);
        assert_eq!(b.edges, vec![(0, 1), (1, 2), (2, 0)]);
    }

    // BrepBuilder::build ──────────────────────────────────────────────────────

    #[test]
    fn build_empty_builder() {
        let b = BrepBuilder::new();
        assert_eq!(b.build(), "brep:V0E[]");
    }

    #[test]
    fn build_vertices_only() {
        let mut b = BrepBuilder::new();
        b.add_vertex([0.0, 0.0, 0.0]);
        b.add_vertex([1.0, 0.0, 0.0]);
        assert_eq!(b.build(), "brep:V2E[]");
    }

    #[test]
    fn build_with_edges() {
        let mut b = BrepBuilder::new();
        let v0 = b.add_vertex([0.0, 0.0, 0.0]);
        let v1 = b.add_vertex([1.0, 0.0, 0.0]);
        let v2 = b.add_vertex([0.0, 1.0, 0.0]);
        b.add_edge(v0, v1);
        b.add_edge(v1, v2);
        b.add_edge(v2, v0);
        assert_eq!(b.build(), "brep:V3E[(0,1),(1,2),(2,0)]");
    }

    #[test]
    fn build_single_edge() {
        let mut b = BrepBuilder::new();
        let v0 = b.add_vertex([0.0, 0.0, 0.0]);
        let v1 = b.add_vertex([5.0, 0.0, 0.0]);
        b.add_edge(v0, v1);
        assert_eq!(b.build(), "brep:V2E[(0,1)]");
    }
}
