// FILE: geom_triangle_mesh.rs
//! `TriangulationMesh` — a zero-dependency pure-Rust stub for a triangulated
//! surface mesh, modelled after `Poly_Triangulation` from OpenCascade.
//!
//! Stores node positions, triangle connectivity, optional per-node normals and
//! optional per-node UV parameters.  The `compute_normals` method derives
//! area-weighted per-node normals from the triangle fan without any external
//! dependencies.

// ─── Triangle ────────────────────────────────────────────────────────────────

/// A single triangle defined by three **0-based** node indices.
///
/// Mirrors `Poly_Triangle` from OpenCascade's `Poly` package.
// occt-ref: Poly_Triangulation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Triangle {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

impl Triangle {
    /// Construct a triangle from three node indices.
    #[inline]
    pub fn new(a: u32, b: u32, c: u32) -> Self {
        Self { a, b, c }
    }

    /// Returns `true` if node index `i` is one of the three vertices.
    #[inline]
    pub fn contains(&self, i: u32) -> bool {
        self.a == i || self.b == i || self.c == i
    }
}

// ─── TriangulationMesh ───────────────────────────────────────────────────────

/// A triangulated surface mesh carrying node positions, triangle connectivity,
/// optional per-node normals and optional per-node UV coordinates.
///
/// Mirrors `Poly_Triangulation` from OpenCascade's `Poly` package.
// occt-ref: Poly_Triangulation
#[derive(Clone, Debug, Default)]
pub struct TriangulationMesh {
    /// 3-D node positions.
    pub nodes: Vec<[f64; 3]>,
    /// Triangle connectivity (0-based node indices).
    pub triangles: Vec<Triangle>,
    /// Per-node normals; empty when not yet computed / not provided.
    pub normals: Vec<[f64; 3]>,
    /// Per-node UV parameters; empty when not provided.
    pub uvs: Vec<[f64; 2]>,
}

impl TriangulationMesh {
    /// Create an empty mesh.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            triangles: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
        }
    }

    /// Append a node and return its 0-based index.
    pub fn add_node(&mut self, p: [f64; 3]) -> u32 {
        let idx = self.nodes.len() as u32;
        self.nodes.push(p);
        idx
    }

    /// Append a triangle by its three 0-based node indices.
    pub fn add_triangle(&mut self, a: u32, b: u32, c: u32) {
        self.triangles.push(Triangle::new(a, b, c));
    }

    /// Returns `true` when the normals array is non-empty.
    #[inline]
    pub fn has_normals(&self) -> bool {
        !self.normals.is_empty()
    }

    /// Returns `true` when the UV array is non-empty.
    #[inline]
    pub fn has_uvs(&self) -> bool {
        !self.uvs.is_empty()
    }

    /// Number of nodes in the mesh.
    #[inline]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of triangles in the mesh.
    #[inline]
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    /// Compute area-weighted per-node normals from the triangle fan and store
    /// them in `self.normals`, replacing any previously stored normals.
    ///
    /// Each triangle contributes its (unnormalized) face normal — whose
    /// magnitude equals twice the triangle area — to each of its three nodes.
    /// Accumulated vectors are then unit-normalized.  Nodes unreferenced by any
    /// triangle receive the zero normal `[0.0, 0.0, 0.0]`.
    pub fn compute_normals(&mut self) {
        let n = self.nodes.len();
        let mut acc: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]; n];

        for tri in &self.triangles {
            let pa = self.nodes[tri.a as usize];
            let pb = self.nodes[tri.b as usize];
            let pc = self.nodes[tri.c as usize];

            // edge vectors
            let ab = [pb[0] - pa[0], pb[1] - pa[1], pb[2] - pa[2]];
            let ac = [pc[0] - pa[0], pc[1] - pa[1], pc[2] - pa[2]];

            // cross product (area-weighted face normal)
            let face_n = cross(ab, ac);

            for idx in [tri.a, tri.b, tri.c] {
                let v = &mut acc[idx as usize];
                v[0] += face_n[0];
                v[1] += face_n[1];
                v[2] += face_n[2];
            }
        }

        self.normals = acc
            .into_iter()
            .map(|v| {
                let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                if len > 0.0 {
                    [v[0] / len, v[1] / len, v[2] / len]
                } else {
                    [0.0, 0.0, 0.0]
                }
            })
            .collect();
    }
}

// ─── helpers ─────────────────────────────────────────────────────────────────

#[inline]
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

// ─── tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_contains() {
        let t = Triangle::new(1, 5, 9);
        assert!(t.contains(1));
        assert!(t.contains(5));
        assert!(t.contains(9));
        assert!(!t.contains(0));
        assert!(!t.contains(2));
    }

    #[test]
    fn add_node_returns_index() {
        let mut m = TriangulationMesh::new();
        assert_eq!(m.add_node([0.0, 0.0, 0.0]), 0);
        assert_eq!(m.add_node([1.0, 0.0, 0.0]), 1);
        assert_eq!(m.add_node([0.0, 1.0, 0.0]), 2);
        assert_eq!(m.node_count(), 3);
    }

    #[test]
    fn triangle_count() {
        let mut m = TriangulationMesh::new();
        m.add_node([0.0, 0.0, 0.0]);
        m.add_node([1.0, 0.0, 0.0]);
        m.add_node([0.0, 1.0, 0.0]);
        m.add_triangle(0, 1, 2);
        assert_eq!(m.triangle_count(), 1);
    }

    #[test]
    fn has_normals_and_uvs_start_false() {
        let m = TriangulationMesh::new();
        assert!(!m.has_normals());
        assert!(!m.has_uvs());
    }

    #[test]
    fn compute_normals_flat_triangle() {
        // A single triangle lying in the XY plane; all normals must point along +Z.
        let mut m = TriangulationMesh::new();
        m.add_node([0.0, 0.0, 0.0]);
        m.add_node([1.0, 0.0, 0.0]);
        m.add_node([0.0, 1.0, 0.0]);
        m.add_triangle(0, 1, 2);
        m.compute_normals();

        assert!(m.has_normals());
        assert_eq!(m.normals.len(), 3);

        for n in &m.normals {
            assert!((n[0]).abs() < 1e-12, "nx should be ~0");
            assert!((n[1]).abs() < 1e-12, "ny should be ~0");
            assert!((n[2] - 1.0).abs() < 1e-12, "nz should be ~1");
        }
    }

    #[test]
    fn compute_normals_replaces_previous() {
        let mut m = TriangulationMesh::new();
        m.add_node([0.0, 0.0, 0.0]);
        m.add_node([1.0, 0.0, 0.0]);
        m.add_node([0.0, 1.0, 0.0]);
        m.add_triangle(0, 1, 2);
        m.normals = vec![[9.9, 9.9, 9.9]; 3]; // stale data
        m.compute_normals();
        assert!((m.normals[0][2] - 1.0).abs() < 1e-12);
    }
}
