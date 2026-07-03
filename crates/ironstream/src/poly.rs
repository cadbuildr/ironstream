// FILE: poly.rs
//! `Poly` — polygon triangulation types, mirroring OpenCascade's `Poly` package.
//!
//! Provides the fundamental mesh primitives used by the meshing pipeline:
//! a single triangle by vertex indices (`Poly_Triangle`), a complete triangulated
//! surface (`Poly_Triangulation`), a fixed-size 1-based array of triangles
//! (`Poly_Array1OfTriangle`), and a topological adjacency helper (`Poly_Connect`).
//!
//! Indexing throughout follows the OCCT convention: **1-based** vertex and
//! triangle indices; violations panic with `Standard_OutOfRange`-style messages.

use crate::gp::{Pnt, Vec3};
use crate::gp2d::Pnt2d;

// ─── Poly_Triangle ───────────────────────────────────────────────────────────

/// A single triangle defined by three **1-based** vertex indices into a
/// companion `Poly_Triangulation` node array.
///
/// Mirrors `Poly_Triangle` from OpenCascade's `Poly` package.
// occt: Poly_Triangle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Poly_Triangle {
    /// Vertex indices (1-based), stored as `[n1, n2, n3]`.
    nodes: [usize; 3],
}

impl Poly_Triangle {
    /// Construct a triangle with all three node indices set to 0 (invalid).
    /// Mirrors the default `Poly_Triangle()` constructor.
    #[inline]
    pub fn new() -> Self {
        Self { nodes: [0, 0, 0] }
    }

    /// Construct a triangle from three 1-based vertex indices.
    /// Mirrors `Poly_Triangle(n1, n2, n3)`.
    ///
    /// # Panics
    /// Panics when any index is 0.
    #[inline]
    pub fn from_nodes(n1: usize, n2: usize, n3: usize) -> Self {
        assert!(n1 > 0 && n2 > 0 && n3 > 0,
            "Poly_Triangle: node indices must be >= 1 (got {n1}, {n2}, {n3})");
        Self { nodes: [n1, n2, n3] }
    }

    /// Set all three vertex indices at once.
    /// Mirrors `Set(n1, n2, n3)`.
    ///
    /// # Panics
    /// Panics when any index is 0.
    #[inline]
    pub fn set(&mut self, n1: usize, n2: usize, n3: usize) {
        assert!(n1 > 0 && n2 > 0 && n3 > 0,
            "Poly_Triangle::set: node indices must be >= 1 (got {n1}, {n2}, {n3})");
        self.nodes = [n1, n2, n3];
    }

    /// Return all three vertex indices as a tuple `(n1, n2, n3)`.
    /// Mirrors `Get(n1, n2, n3)`.
    #[inline]
    pub fn get(&self) -> (usize, usize, usize) {
        (self.nodes[0], self.nodes[1], self.nodes[2])
    }

    /// Return the vertex index at position `i` (1-based, `i` in `[1, 3]`).
    /// Mirrors `Value(i)`.
    ///
    /// # Panics
    /// Panics when `i` is not in `[1, 3]`.
    #[inline]
    pub fn value(&self, i: usize) -> usize {
        assert!(i >= 1 && i <= 3,
            "Poly_Triangle::value: index {i} out of [1, 3]");
        self.nodes[i - 1]
    }

    /// Set the vertex index at position `i` (1-based, `i` in `[1, 3]`).
    /// Mirrors `ChangeValue(i)` / `SetValue(i, n)`.
    ///
    /// # Panics
    /// Panics when `i` is not in `[1, 3]` or `n == 0`.
    #[inline]
    pub fn set_value(&mut self, i: usize, n: usize) {
        assert!(i >= 1 && i <= 3,
            "Poly_Triangle::set_value: index {i} out of [1, 3]");
        assert!(n > 0,
            "Poly_Triangle::set_value: node index must be >= 1 (got {n})");
        self.nodes[i - 1] = n;
    }

    /// Return true if all three node indices are non-zero (i.e. the triangle
    /// has been initialised with real vertex indices).
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.nodes[0] > 0 && self.nodes[1] > 0 && self.nodes[2] > 0
    }
}

impl Default for Poly_Triangle {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Poly_Array1OfTriangle ───────────────────────────────────────────────────

/// A 1-D, fixed-size, **1-based** array of `Poly_Triangle`.
///
/// Mirrors `Poly_Array1OfTriangle` (an `NCollection_Array1<Poly_Triangle>`
/// specialisation) from OpenCascade.
// occt: Poly_Array1OfTriangle
#[derive(Clone, Debug)]
pub struct Poly_Array1OfTriangle {
    data: Vec<Poly_Triangle>,
    lower: usize,
}

impl Poly_Array1OfTriangle {
    /// Construct a 1-based array of `n` triangles, all default-initialised.
    ///
    /// # Panics
    /// Panics when `n == 0`.
    pub fn new(n: usize) -> Self {
        assert!(n > 0, "Poly_Array1OfTriangle: size must be >= 1");
        Self {
            data: vec![Poly_Triangle::new(); n],
            lower: 1,
        }
    }

    /// Construct with explicit lower/upper bounds, analogous to
    /// `Poly_Array1OfTriangle(lower, upper)`.
    ///
    /// # Panics
    /// Panics when `upper < lower`.
    pub fn with_bounds(lower: usize, upper: usize) -> Self {
        assert!(upper >= lower,
            "Poly_Array1OfTriangle: upper ({upper}) < lower ({lower})");
        let n = upper - lower + 1;
        Self {
            data: vec![Poly_Triangle::new(); n],
            lower,
        }
    }

    /// Lower index (1 by default).
    #[inline] pub fn lower(&self) -> usize { self.lower }

    /// Upper index (`lower + len - 1`).
    #[inline] pub fn upper(&self) -> usize { self.lower + self.data.len() - 1 }

    /// Number of triangles.
    #[inline] pub fn length(&self) -> usize { self.data.len() }

    /// True when the array has no elements.
    #[inline] pub fn is_empty(&self) -> bool { self.data.is_empty() }

    /// Return triangle at 1-based index `i`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn value(&self, i: usize) -> Poly_Triangle {
        self.check_bounds(i);
        self.data[i - self.lower]
    }

    /// Mutable reference to triangle at 1-based index `i`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn value_mut(&mut self, i: usize) -> &mut Poly_Triangle {
        self.check_bounds(i);
        let idx = i - self.lower;
        &mut self.data[idx]
    }

    /// Set triangle at 1-based index `i`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn set_value(&mut self, i: usize, t: Poly_Triangle) {
        self.check_bounds(i);
        self.data[i - self.lower] = t;
    }

    /// Iterator over triangles in index order.
    pub fn iter(&self) -> impl Iterator<Item = &Poly_Triangle> {
        self.data.iter()
    }

    /// Iterator of `(1-based-index, Poly_Triangle)` pairs.
    pub fn iter_indexed(&self) -> impl Iterator<Item = (usize, Poly_Triangle)> + '_ {
        let lower = self.lower;
        self.data
            .iter()
            .enumerate()
            .map(move |(i, &t)| (i + lower, t))
    }

    /// Count how many triangles reference vertex index `node`.
    pub fn count_referencing(&self, node: usize) -> usize {
        self.data.iter().filter(|t| {
            let (a, b, c) = t.get();
            a == node || b == node || c == node
        }).count()
    }

    #[inline]
    fn check_bounds(&self, i: usize) {
        assert!(
            i >= self.lower && i <= self.upper(),
            "Poly_Array1OfTriangle: index {i} out of range [{}, {}]",
            self.lower,
            self.upper()
        );
    }
}

// ─── Poly_Triangulation ──────────────────────────────────────────────────────

/// A complete triangulated surface: nodes (3D vertices), optional 2D UV
/// parameters, optional per-node normals, and a triangle index array.
///
/// Mirrors `Poly_Triangulation` from OpenCascade's `Poly` package.
/// All vertex/triangle indices are **1-based**.
// occt: Poly_Triangulation
#[derive(Clone, Debug)]
pub struct Poly_Triangulation {
    /// 3D node positions (1-based: index `1..=nb_nodes`).
    nodes: Vec<Pnt>,
    /// Optional 2D UV parameters per node (same length as `nodes` when present).
    uvs: Option<Vec<Pnt2d>>,
    /// Optional per-node normals (same length as `nodes` when present).
    normals: Option<Vec<Vec3>>,
    /// Triangle connectivity (1-based: index `1..=nb_triangles`).
    triangles: Vec<Poly_Triangle>,
    /// Linear deflection used during meshing (cosmetic, not enforced here).
    deflection: f64,
}

impl Poly_Triangulation {
    /// Construct a triangulation with `nb_nodes` nodes and `nb_triangles`
    /// triangles, no UV or normals.
    /// Mirrors `Poly_Triangulation(nb_nodes, nb_triangles, has_uvs)`.
    ///
    /// # Panics
    /// Panics when `nb_nodes == 0` or `nb_triangles == 0`.
    pub fn new(nb_nodes: usize, nb_triangles: usize) -> Self {
        assert!(nb_nodes > 0,
            "Poly_Triangulation: nb_nodes must be >= 1");
        assert!(nb_triangles > 0,
            "Poly_Triangulation: nb_triangles must be >= 1");
        Self {
            nodes: vec![Pnt::origin(); nb_nodes],
            uvs: None,
            normals: None,
            triangles: vec![Poly_Triangle::new(); nb_triangles],
            deflection: 0.0,
        }
    }

    /// Construct and immediately populate nodes and triangles from slices.
    ///
    /// # Panics
    /// Panics when either slice is empty.
    pub fn from_slices(nodes: &[Pnt], triangles: &[Poly_Triangle]) -> Self {
        assert!(!nodes.is_empty(), "Poly_Triangulation::from_slices: no nodes");
        assert!(!triangles.is_empty(),
            "Poly_Triangulation::from_slices: no triangles");
        Self {
            nodes: nodes.to_vec(),
            uvs: None,
            normals: None,
            triangles: triangles.to_vec(),
            deflection: 0.0,
        }
    }

    // ── Sizing ────────────────────────────────────────────────────────────────

    /// Number of nodes (vertices). Mirrors `NbNodes()`.
    #[inline] pub fn nb_nodes(&self) -> usize { self.nodes.len() }

    /// Number of triangles. Mirrors `NbTriangles()`.
    #[inline] pub fn nb_triangles(&self) -> usize { self.triangles.len() }

    /// True when UV parameters are present.
    #[inline] pub fn has_uvs(&self) -> bool { self.uvs.is_some() }

    /// True when per-node normals are present.
    #[inline] pub fn has_normals(&self) -> bool { self.normals.is_some() }

    // ── Deflection ────────────────────────────────────────────────────────────

    /// Return the meshing deflection. Mirrors `Deflection()`.
    #[inline] pub fn deflection(&self) -> f64 { self.deflection }

    /// Set the meshing deflection. Mirrors `Deflection(d)`.
    #[inline] pub fn set_deflection(&mut self, d: f64) { self.deflection = d; }

    // ── Node access ───────────────────────────────────────────────────────────

    /// Return node at 1-based index `i`. Mirrors `Node(i)`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn node(&self, i: usize) -> Pnt {
        self.check_node(i);
        self.nodes[i - 1]
    }

    /// Set node at 1-based index `i`. Mirrors `ChangeNode(i)`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn set_node(&mut self, i: usize, p: Pnt) {
        self.check_node(i);
        self.nodes[i - 1] = p;
    }

    /// Mutable reference to node at 1-based index `i`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn node_mut(&mut self, i: usize) -> &mut Pnt {
        self.check_node(i);
        &mut self.nodes[i - 1]
    }

    /// Slice view of all nodes (0-indexed internally, but callers use 1-based
    /// access through `node()`).
    #[inline]
    pub fn nodes(&self) -> &[Pnt] {
        &self.nodes
    }

    // ── UV access ─────────────────────────────────────────────────────────────

    /// Allocate UV parameter storage for every node (all set to origin).
    /// Calling a second time is a no-op.
    pub fn add_uvs(&mut self) {
        if self.uvs.is_none() {
            self.uvs = Some(vec![Pnt2d::origin(); self.nodes.len()]);
        }
    }

    /// Return UV at 1-based index `i`. Mirrors `UVNode(i)`.
    ///
    /// # Panics
    /// Panics when UV storage is not allocated or `i` is out of range.
    #[inline]
    pub fn uv_node(&self, i: usize) -> Pnt2d {
        self.check_node(i);
        self.uvs.as_ref()
            .expect("Poly_Triangulation: UV storage not allocated; call add_uvs() first")[i - 1]
    }

    /// Set UV at 1-based index `i`. Mirrors `ChangeUVNode(i)`.
    ///
    /// # Panics
    /// Panics when UV storage is not allocated or `i` is out of range.
    #[inline]
    pub fn set_uv_node(&mut self, i: usize, uv: Pnt2d) {
        self.check_node(i);
        self.uvs.as_mut()
            .expect("Poly_Triangulation: UV storage not allocated; call add_uvs() first")[i - 1] = uv;
    }

    // ── Normal access ─────────────────────────────────────────────────────────

    /// Allocate per-node normal storage (all set to zero vector).
    /// Calling a second time is a no-op.
    pub fn add_normals(&mut self) {
        if self.normals.is_none() {
            self.normals = Some(vec![Vec3::origin(); self.nodes.len()]);
        }
    }

    /// Return per-node normal at 1-based index `i`. Mirrors `Normal(i)`.
    ///
    /// # Panics
    /// Panics when normal storage is not allocated or `i` is out of range.
    #[inline]
    pub fn normal(&self, i: usize) -> Vec3 {
        self.check_node(i);
        self.normals.as_ref()
            .expect("Poly_Triangulation: normal storage not allocated; call add_normals() first")[i - 1]
    }

    /// Set per-node normal at 1-based index `i`. Mirrors `SetNormal(i, n)`.
    ///
    /// # Panics
    /// Panics when normal storage is not allocated or `i` is out of range.
    #[inline]
    pub fn set_normal(&mut self, i: usize, n: Vec3) {
        self.check_node(i);
        self.normals.as_mut()
            .expect("Poly_Triangulation: normal storage not allocated; call add_normals() first")[i - 1] = n;
    }

    // ── Triangle access ───────────────────────────────────────────────────────

    /// Return triangle at 1-based index `i`. Mirrors `Triangle(i)`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn triangle(&self, i: usize) -> Poly_Triangle {
        self.check_tri(i);
        self.triangles[i - 1]
    }

    /// Set triangle at 1-based index `i`. Mirrors `ChangeTriangle(i)`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn set_triangle(&mut self, i: usize, t: Poly_Triangle) {
        self.check_tri(i);
        self.triangles[i - 1] = t;
    }

    /// Mutable reference to triangle at 1-based index `i`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn triangle_mut(&mut self, i: usize) -> &mut Poly_Triangle {
        self.check_tri(i);
        &mut self.triangles[i - 1]
    }

    /// Slice view of all triangles.
    #[inline]
    pub fn triangles(&self) -> &[Poly_Triangle] {
        &self.triangles
    }

    // ── Geometry helpers ──────────────────────────────────────────────────────

    /// Compute the area of triangle `i` (1-based) using the cross-product
    /// formula.  Returns 0 for a degenerate triangle.
    pub fn triangle_area(&self, i: usize) -> f64 {
        let (n1, n2, n3) = self.triangle(i).get();
        let a = self.node(n1);
        let b = self.node(n2);
        let c = self.node(n3);
        let ab = Pnt::new(b.x - a.x, b.y - a.y, b.z - a.z);
        let ac = Pnt::new(c.x - a.x, c.y - a.y, c.z - a.z);
        ab.cross(ac).norm() * 0.5
    }

    /// Total surface area (sum of all triangle areas).
    pub fn total_area(&self) -> f64 {
        (1..=self.nb_triangles()).map(|i| self.triangle_area(i)).sum()
    }

    /// Outward-facing normal of triangle `i` (unnormalised cross product,
    /// normalised to unit length).  Returns zero vector for degenerate triangles.
    pub fn triangle_normal(&self, i: usize) -> Vec3 {
        let (n1, n2, n3) = self.triangle(i).get();
        let a = self.node(n1);
        let b = self.node(n2);
        let c = self.node(n3);
        let ab = Pnt::new(b.x - a.x, b.y - a.y, b.z - a.z);
        let ac = Pnt::new(c.x - a.x, c.y - a.y, c.z - a.z);
        let n = ab.cross(ac);
        let len = n.norm();
        if len < 1.0e-14 {
            Vec3::origin()
        } else {
            Pnt::new(n.x / len, n.y / len, n.z / len)
        }
    }

    /// Centroid of triangle `i` (average of its three vertex positions).
    pub fn triangle_centroid(&self, i: usize) -> Pnt {
        let (n1, n2, n3) = self.triangle(i).get();
        let a = self.node(n1);
        let b = self.node(n2);
        let c = self.node(n3);
        Pnt::new(
            (a.x + b.x + c.x) / 3.0,
            (a.y + b.y + c.y) / 3.0,
            (a.z + b.z + c.z) / 3.0,
        )
    }

    /// Compute the axis-aligned bounding box of all nodes.
    /// Returns `(min, max)`.
    ///
    /// # Panics
    /// Panics on an empty node list (prevented by constructors).
    pub fn bounding_box(&self) -> (Pnt, Pnt) {
        let mut lo = self.nodes[0];
        let mut hi = self.nodes[0];
        for &p in &self.nodes[1..] {
            lo.x = lo.x.min(p.x);
            lo.y = lo.y.min(p.y);
            lo.z = lo.z.min(p.z);
            hi.x = hi.x.max(p.x);
            hi.y = hi.y.max(p.y);
            hi.z = hi.z.max(p.z);
        }
        (lo, hi)
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    #[inline]
    fn check_node(&self, i: usize) {
        assert!(
            i >= 1 && i <= self.nodes.len(),
            "Poly_Triangulation: node index {i} out of [1, {}]",
            self.nodes.len()
        );
    }

    #[inline]
    fn check_tri(&self, i: usize) {
        assert!(
            i >= 1 && i <= self.triangles.len(),
            "Poly_Triangulation: triangle index {i} out of [1, {}]",
            self.triangles.len()
        );
    }
}

// ─── Poly_Connect ────────────────────────────────────────────────────────────

/// Topological adjacency tool for a `Poly_Triangulation`.
///
/// Computes, for each vertex, the list of triangles that share it, and for
/// each triangle, its edge-adjacent neighbours. Mirrors `Poly_Connect` from
/// OpenCascade (which builds similar adjacency structures for smooth-normal
/// computation and edge traversal).
///
/// After construction the adjacency data is built once in O(T) time and then
/// available in O(degree) per query.
// occt: Poly_Connect
pub struct Poly_Connect {
    /// Number of nodes in the mesh.
    nb_nodes: usize,
    /// Number of triangles in the mesh.
    nb_triangles: usize,
    /// For each node (1-based, stored at index `node - 1`): list of triangle
    /// indices (1-based) that include that node.
    node_to_triangles: Vec<Vec<usize>>,
    /// For each triangle (1-based, stored at index `tri - 1`): its three
    /// edge-adjacent triangle indices (1-based), or 0 when the edge is on
    /// the boundary.  Stored as `[opp_edge1, opp_edge2, opp_edge3]` where
    /// edge `k` is opposite vertex `k` within the triangle.
    tri_neighbours: Vec<[usize; 3]>,
}

impl Poly_Connect {
    /// Build the adjacency structures from a `Poly_Triangulation`.
    /// Mirrors `Poly_Connect(T)`.
    pub fn new(mesh: &Poly_Triangulation) -> Self {
        let nb_nodes = mesh.nb_nodes();
        let nb_triangles = mesh.nb_triangles();

        // ── node → triangle fan ────────────────────────────────────────────
        let mut node_to_triangles: Vec<Vec<usize>> = vec![Vec::new(); nb_nodes];
        for ti in 1..=nb_triangles {
            let (n1, n2, n3) = mesh.triangle(ti).get();
            // Tolerate uninitialised triangles (node index 0) gracefully.
            if n1 > 0 && n1 <= nb_nodes { node_to_triangles[n1 - 1].push(ti); }
            if n2 > 0 && n2 <= nb_nodes { node_to_triangles[n2 - 1].push(ti); }
            if n3 > 0 && n3 <= nb_nodes { node_to_triangles[n3 - 1].push(ti); }
        }

        // ── triangle → edge-adjacent neighbours ───────────────────────────
        //
        // For triangle T with nodes (a, b, c):
        //   edge 0 is opposite node a → shared by a triangle containing both b and c
        //   edge 1 is opposite node b → shared by a triangle containing both a and c
        //   edge 2 is opposite node c → shared by a triangle containing both a and b
        let mut tri_neighbours: Vec<[usize; 3]> = vec![[0usize; 3]; nb_triangles];

        for ti in 1..=nb_triangles {
            let tri = mesh.triangle(ti);
            if !tri.is_valid() { continue; }
            let (a, b, c) = tri.get();
            // Each edge is identified by the two nodes it connects.
            let edges = [(b, c), (a, c), (a, b)]; // opposite to node a, b, c respectively
            for (edge_idx, &(p, q)) in edges.iter().enumerate() {
                if tri_neighbours[ti - 1][edge_idx] != 0 { continue; } // already found
                // Look for another triangle (≠ ti) that contains both p and q.
                'outer: for &other_ti in &node_to_triangles[p - 1] {
                    if other_ti == ti { continue; }
                    let (oa, ob, oc) = mesh.triangle(other_ti).get();
                    let other_nodes = [oa, ob, oc];
                    if other_nodes.contains(&q) {
                        // Found the adjacent triangle; record it for both sides.
                        tri_neighbours[ti - 1][edge_idx] = other_ti;
                        // Find which edge of the neighbour is shared (contains p and q).
                        let (xa, xb, xc) = (oa, ob, oc);
                        let neighbour_edges = [(xb, xc), (xa, xc), (xa, xb)];
                        for (nei, &(np, nq)) in neighbour_edges.iter().enumerate() {
                            if (np == p && nq == q) || (np == q && nq == p) {
                                tri_neighbours[other_ti - 1][nei] = ti;
                                break;
                            }
                        }
                        break 'outer;
                    }
                }
            }
        }

        Self {
            nb_nodes,
            nb_triangles,
            node_to_triangles,
            tri_neighbours,
        }
    }

    /// Number of nodes in the mesh this connector was built from.
    #[inline] pub fn nb_nodes(&self) -> usize { self.nb_nodes }

    /// Number of triangles in the mesh this connector was built from.
    #[inline] pub fn nb_triangles(&self) -> usize { self.nb_triangles }

    /// Return the list of triangle indices (1-based) that share vertex `node`
    /// (1-based). Mirrors `Triangles(node, tri_list)`.
    ///
    /// # Panics
    /// Panics when `node` is out of `[1, nb_nodes]`.
    pub fn triangles_of_node(&self, node: usize) -> &[usize] {
        assert!(
            node >= 1 && node <= self.nb_nodes,
            "Poly_Connect::triangles_of_node: node {node} out of [1, {}]",
            self.nb_nodes
        );
        &self.node_to_triangles[node - 1]
    }

    /// Number of triangles sharing vertex `node`.
    ///
    /// # Panics
    /// Panics when `node` is out of range.
    pub fn degree(&self, node: usize) -> usize {
        self.triangles_of_node(node).len()
    }

    /// Return the three edge-adjacent triangle indices (1-based) for triangle
    /// `tri` (1-based).  An index of `0` means the edge is a free boundary.
    /// Mirrors `Neighbours(tri, n1, n2, n3)`.
    ///
    /// # Panics
    /// Panics when `tri` is out of `[1, nb_triangles]`.
    pub fn neighbours(&self, tri: usize) -> [usize; 3] {
        assert!(
            tri >= 1 && tri <= self.nb_triangles,
            "Poly_Connect::neighbours: triangle {tri} out of [1, {}]",
            self.nb_triangles
        );
        self.tri_neighbours[tri - 1]
    }

    /// True when edge `edge` (0, 1, or 2) of triangle `tri` is a free boundary
    /// (no adjacent triangle exists).
    ///
    /// # Panics
    /// Panics when `tri` or `edge` is out of range.
    pub fn is_boundary_edge(&self, tri: usize, edge: usize) -> bool {
        assert!(edge < 3,
            "Poly_Connect::is_boundary_edge: edge index {edge} must be in [0, 2]");
        self.neighbours(tri)[edge] == 0
    }

    /// Count the number of boundary edges of triangle `tri` (0, 1, 2, or 3).
    ///
    /// # Panics
    /// Panics when `tri` is out of range.
    pub fn boundary_edge_count(&self, tri: usize) -> usize {
        self.neighbours(tri).iter().filter(|&&n| n == 0).count()
    }

    /// Compute the area-weighted average normal at node `node` using the normals
    /// of all triangles in its fan.  Returns a unit-length vector.
    /// If the node is isolated the zero vector is returned.
    pub fn averaged_normal(&self, node: usize, mesh: &Poly_Triangulation) -> Vec3 {
        let tris = self.triangles_of_node(node);
        if tris.is_empty() {
            return Vec3::origin();
        }
        let mut sum = Vec3::origin();
        for &ti in tris {
            let area = mesh.triangle_area(ti);
            let n = mesh.triangle_normal(ti);
            sum.x += n.x * area;
            sum.y += n.y * area;
            sum.z += n.z * area;
        }
        let len = sum.norm();
        if len < 1.0e-14 {
            Vec3::origin()
        } else {
            Pnt::new(sum.x / len, sum.y / len, sum.z / len)
        }
    }
}

// ─── Internal unit tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Pnt;
    use crate::gp2d::Pnt2d;

    const TOL: f64 = 1e-10;

    fn near(a: f64, b: f64) {
        assert!((a - b).abs() <= TOL, "expected {a} ≈ {b} (diff {})", (a - b).abs());
    }

    fn pnt_near(a: Pnt, b: Pnt) {
        near(a.x, b.x); near(a.y, b.y); near(a.z, b.z);
    }

    // ── Poly_Triangle ────────────────────────────────────────────────────────

    #[test]
    fn triangle_new_default() {
        let t = Poly_Triangle::new();
        assert_eq!(t.get(), (0, 0, 0));
        assert!(!t.is_valid());
    }

    #[test]
    fn triangle_from_nodes_and_value() {
        let t = Poly_Triangle::from_nodes(3, 7, 12);
        assert_eq!(t.get(), (3, 7, 12));
        assert_eq!(t.value(1), 3);
        assert_eq!(t.value(2), 7);
        assert_eq!(t.value(3), 12);
        assert!(t.is_valid());
    }

    #[test]
    fn triangle_set_and_set_value() {
        let mut t = Poly_Triangle::new();
        t.set(1, 2, 3);
        assert_eq!(t.get(), (1, 2, 3));
        t.set_value(2, 99);
        assert_eq!(t.value(2), 99);
    }

    #[test]
    #[should_panic]
    fn triangle_value_out_of_range() {
        let t = Poly_Triangle::from_nodes(1, 2, 3);
        let _ = t.value(4);
    }

    #[test]
    #[should_panic]
    fn triangle_from_nodes_zero_index_panics() {
        let _ = Poly_Triangle::from_nodes(0, 1, 2);
    }

    // ── Poly_Array1OfTriangle ────────────────────────────────────────────────

    #[test]
    fn array1_basic_bounds() {
        let arr = Poly_Array1OfTriangle::new(5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 5);
        assert!(!arr.is_empty());
    }

    #[test]
    fn array1_set_and_get() {
        let mut arr = Poly_Array1OfTriangle::new(3);
        arr.set_value(1, Poly_Triangle::from_nodes(1, 2, 3));
        arr.set_value(2, Poly_Triangle::from_nodes(2, 3, 4));
        arr.set_value(3, Poly_Triangle::from_nodes(3, 4, 5));
        assert_eq!(arr.value(2).get(), (2, 3, 4));
    }

    #[test]
    fn array1_with_bounds() {
        let arr = Poly_Array1OfTriangle::with_bounds(2, 6);
        assert_eq!(arr.lower(), 2);
        assert_eq!(arr.upper(), 6);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn array1_count_referencing() {
        let mut arr = Poly_Array1OfTriangle::new(4);
        arr.set_value(1, Poly_Triangle::from_nodes(1, 2, 3));
        arr.set_value(2, Poly_Triangle::from_nodes(2, 3, 4));
        arr.set_value(3, Poly_Triangle::from_nodes(3, 4, 5));
        arr.set_value(4, Poly_Triangle::from_nodes(1, 4, 5));
        // Node 3 appears in triangles 1, 2, 3
        assert_eq!(arr.count_referencing(3), 3);
        // Node 5 appears in triangles 3, 4
        assert_eq!(arr.count_referencing(5), 2);
    }

    // ── Poly_Triangulation ───────────────────────────────────────────────────

    fn unit_quad_mesh() -> Poly_Triangulation {
        // Two triangles forming a unit square in XY:
        //  nodes: 1=(0,0,0)  2=(1,0,0)  3=(1,1,0)  4=(0,1,0)
        //  tris:  1=[1,2,3]  2=[1,3,4]
        let mut mesh = Poly_Triangulation::new(4, 2);
        mesh.set_node(1, Pnt::new(0.0, 0.0, 0.0));
        mesh.set_node(2, Pnt::new(1.0, 0.0, 0.0));
        mesh.set_node(3, Pnt::new(1.0, 1.0, 0.0));
        mesh.set_node(4, Pnt::new(0.0, 1.0, 0.0));
        mesh.set_triangle(1, Poly_Triangle::from_nodes(1, 2, 3));
        mesh.set_triangle(2, Poly_Triangle::from_nodes(1, 3, 4));
        mesh
    }

    #[test]
    fn triangulation_construction_and_accessors() {
        let mesh = unit_quad_mesh();
        assert_eq!(mesh.nb_nodes(), 4);
        assert_eq!(mesh.nb_triangles(), 2);
        pnt_near(mesh.node(2), Pnt::new(1.0, 0.0, 0.0));
        assert_eq!(mesh.triangle(1).get(), (1, 2, 3));
    }

    #[test]
    fn triangulation_triangle_area() {
        let mesh = unit_quad_mesh();
        // Each right-isosceles triangle has area 0.5
        near(mesh.triangle_area(1), 0.5);
        near(mesh.triangle_area(2), 0.5);
        near(mesh.total_area(), 1.0);
    }

    #[test]
    fn triangulation_triangle_normal_z() {
        let mesh = unit_quad_mesh();
        // Both triangles lie in XY-plane → normal should be (0, 0, 1)
        let n = mesh.triangle_normal(1);
        near(n.x, 0.0); near(n.y, 0.0); near(n.z, 1.0);
    }

    #[test]
    fn triangulation_bounding_box() {
        let mesh = unit_quad_mesh();
        let (lo, hi) = mesh.bounding_box();
        near(lo.x, 0.0); near(lo.y, 0.0); near(lo.z, 0.0);
        near(hi.x, 1.0); near(hi.y, 1.0); near(hi.z, 0.0);
    }

    #[test]
    fn triangulation_uv_nodes() {
        let mut mesh = unit_quad_mesh();
        assert!(!mesh.has_uvs());
        mesh.add_uvs();
        assert!(mesh.has_uvs());
        mesh.set_uv_node(1, Pnt2d::new(0.0, 0.0));
        mesh.set_uv_node(3, Pnt2d::new(1.0, 1.0));
        assert!((mesh.uv_node(3).x - 1.0).abs() < TOL);
    }

    #[test]
    fn triangulation_normals() {
        let mut mesh = unit_quad_mesh();
        assert!(!mesh.has_normals());
        mesh.add_normals();
        assert!(mesh.has_normals());
        let z = Vec3::new(0.0, 0.0, 1.0);
        for i in 1..=4 { mesh.set_normal(i, z); }
        for i in 1..=4 {
            let n = mesh.normal(i);
            near(n.z, 1.0);
        }
    }

    // ── Poly_Connect ─────────────────────────────────────────────────────────

    #[test]
    fn connect_node_fans() {
        let mesh = unit_quad_mesh();
        let conn = Poly_Connect::new(&mesh);
        // Node 1 is shared by both triangles
        assert_eq!(conn.degree(1), 2);
        // Node 2 appears only in triangle 1
        assert_eq!(conn.degree(2), 1);
        // Node 3 is shared by both triangles
        assert_eq!(conn.degree(3), 2);
    }

    #[test]
    fn connect_edge_adjacency() {
        let mesh = unit_quad_mesh();
        let conn = Poly_Connect::new(&mesh);
        // Triangles 1 and 2 share edge (1,3) — they should be mutual neighbours
        let n1 = conn.neighbours(1);
        let n2 = conn.neighbours(2);
        assert!(n1.contains(&2), "tri 1 should see tri 2 as neighbour");
        assert!(n2.contains(&1), "tri 2 should see tri 1 as neighbour");
    }

    #[test]
    fn connect_boundary_edges() {
        let mesh = unit_quad_mesh();
        let conn = Poly_Connect::new(&mesh);
        // Each triangle has 2 boundary edges and 1 interior edge
        assert_eq!(conn.boundary_edge_count(1), 2);
        assert_eq!(conn.boundary_edge_count(2), 2);
    }

    #[test]
    fn connect_averaged_normal_flat_mesh() {
        let mesh = unit_quad_mesh();
        let conn = Poly_Connect::new(&mesh);
        // All normals point in +Z; averaged normal at any vertex should be (0,0,1)
        for v in 1..=4 {
            let n = conn.averaged_normal(v, &mesh);
            near(n.z, 1.0);
            near(n.x, 0.0);
            near(n.y, 0.0);
        }
    }
}
