// FILE: rust/ironstream/crates/ironstream/src/poly_connect.rs
//! `poly_connect` — triangle adjacency helper, mirroring OpenCascade's
//! `Poly_Triangle` and `Poly_Connect` from the `Poly` package.
//!
//! This module is intentionally self-contained: it does not depend on
//! `crate::poly` so it can be used independently of the richer
//! `Poly_Triangulation` machinery.  Indexing is **0-based** throughout;
//! the OCCT originals use 1-based indices but the zero-dependency Rust port
//! uses idiomatic 0-based index conventions.

// ─── PolyTriangle ────────────────────────────────────────────────────────────

/// A single triangle defined by three vertex indices into a companion node
/// array.  Indices are 0-based.
///
// occt-ref: Poly_Triangle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PolyTriangle {
    /// The three vertex indices, stored as `[n0, n1, n2]`.
    n: [usize; 3],
}

impl PolyTriangle {
    /// Construct a triangle from three vertex indices.
    ///
    /// # Example
    /// ```
    /// use ironstream::poly_connect::PolyTriangle;
    /// let t = PolyTriangle::new(0, 1, 2);
    /// assert_eq!(t.nodes(), [0, 1, 2]);
    /// ```
    #[inline]
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self { n: [a, b, c] }
    }

    /// Return all three vertex indices as an array.
    #[inline]
    pub fn nodes(&self) -> [usize; 3] {
        self.n
    }

    /// Return the vertex index at local position `i` (0, 1, or 2).
    ///
    /// # Panics
    /// Panics if `i >= 3`.
    #[inline]
    pub fn node(&self, i: usize) -> usize {
        assert!(i < 3, "PolyTriangle::node: index {i} out of range [0, 3)");
        self.n[i]
    }

    /// Replace all three vertex indices in one call.
    #[inline]
    pub fn set_nodes(&mut self, a: usize, b: usize, c: usize) {
        self.n = [a, b, c];
    }
}

// ─── PolyConnect ─────────────────────────────────────────────────────────────

/// Triangle mesh adjacency structure.
///
/// Given a list of [`PolyTriangle`] values and the total number of distinct
/// vertices, `PolyConnect` precomputes:
///
/// * For each triangle, which (up to three) neighbouring triangles share one
///   of its edges (`adjacency`).
/// * For each vertex, which triangles contain it (built on demand via
///   [`triangles_around_node`](PolyConnect::triangles_around_node)).
///
/// Two triangles are **neighbours** if they share exactly two vertex indices
/// (i.e. they share an edge).  Each triangle has at most three neighbours,
/// one per edge.
///
// occt: Poly_Connect
#[derive(Clone, Debug)]
pub struct PolyConnect {
    /// The triangle list supplied at construction.
    triangles: Vec<PolyTriangle>,
    /// Total number of distinct vertex indices in the mesh.
    nb_nodes: usize,
    /// `adjacency[t][e]` is the index of the triangle that shares edge `e` of
    /// triangle `t`, or `None` if that edge is a boundary edge.
    ///
    /// Edge `e` connects `triangles[t].node(e)` and
    /// `triangles[t].node((e + 1) % 3)`.
    adjacency: Vec<[Option<usize>; 3]>,
}

impl PolyConnect {
    /// Build the adjacency table for the supplied triangle list.
    ///
    /// `nb_nodes` is the number of distinct vertices; it is stored and exposed
    /// via [`nb_nodes()`](PolyConnect::nb_nodes) but is not used internally
    /// beyond that (the adjacency build only relies on the triangle indices).
    ///
    /// **Complexity**: O(T²) in the naive worst case, but T is typically small
    /// for the mesh fragments this structure is used with.  A bucket-based
    /// O(T) build is used here via an edge-to-triangle map keyed on sorted
    /// vertex-index pairs.
    ///
    /// # Example
    /// ```
    /// use ironstream::poly_connect::{PolyTriangle, PolyConnect};
    /// // Two triangles sharing the edge (1, 2).
    /// let tris = vec![
    ///     PolyTriangle::new(0, 1, 2),
    ///     PolyTriangle::new(1, 3, 2),
    /// ];
    /// let pc = PolyConnect::new(tris, 4);
    /// let n0 = pc.neighbours(0);
    /// assert_eq!(n0[1], Some(1)); // edge (1,2) shared with triangle 1
    /// ```
    pub fn new(triangles: Vec<PolyTriangle>, nb_nodes: usize) -> Self {
        let nb_t = triangles.len();

        // Build a map: sorted edge (lo, hi) -> list of (triangle_index, local_edge_index)
        // We use a Vec of ((usize,usize), usize, usize) then sort, because std-only.
        // Each triangle contributes 3 edges.
        let mut edge_entries: Vec<((usize, usize), usize, usize)> =
            Vec::with_capacity(nb_t * 3);

        for (ti, tri) in triangles.iter().enumerate() {
            for e in 0..3usize {
                let a = tri.node(e);
                let b = tri.node((e + 1) % 3);
                let key = if a < b { (a, b) } else { (b, a) };
                edge_entries.push((key, ti, e));
            }
        }

        // Sort by edge key so equal edges are adjacent.
        edge_entries.sort_unstable_by_key(|&(key, _, _)| key);

        // Initialise adjacency with all None.
        let mut adjacency: Vec<[Option<usize>; 3]> = vec![[None, None, None]; nb_t];

        // Walk through sorted entries; consecutive equal keys are shared edges.
        let n = edge_entries.len();
        let mut i = 0;
        while i < n {
            let (key, ti, ei) = edge_entries[i];
            // Gather all entries with the same key (usually 1 or 2).
            let mut j = i + 1;
            while j < n && edge_entries[j].0 == key {
                j += 1;
            }
            // If exactly two triangles share this edge, link them.
            if j - i == 2 {
                let (_, ti2, ei2) = edge_entries[i + 1];
                adjacency[ti][ei] = Some(ti2);
                adjacency[ti2][ei2] = Some(ti);
            }
            // More than 2 — non-manifold; skip linking (leave None).
            i = j;
        }

        Self {
            triangles,
            nb_nodes,
            adjacency,
        }
    }

    /// Return the number of triangles.
    #[inline]
    pub fn nb_triangles(&self) -> usize {
        self.triangles.len()
    }

    /// Return the number of distinct vertex indices supplied at construction.
    #[inline]
    pub fn nb_nodes(&self) -> usize {
        self.nb_nodes
    }

    /// Return a reference to triangle `t`.
    ///
    /// # Panics
    /// Panics if `t >= nb_triangles()`.
    #[inline]
    pub fn triangle(&self, t: usize) -> &PolyTriangle {
        assert!(
            t < self.triangles.len(),
            "PolyConnect::triangle: index {t} out of range [0, {})",
            self.triangles.len()
        );
        &self.triangles[t]
    }

    /// Return the three (possibly absent) neighbours of triangle `t`.
    ///
    /// `neighbours(t)[e]` is the triangle that shares edge `e` of triangle
    /// `t`, where edge `e` connects `triangle(t).node(e)` and
    /// `triangle(t).node((e+1)%3)`.
    ///
    /// # Panics
    /// Panics if `t >= nb_triangles()`.
    #[inline]
    pub fn neighbours(&self, t: usize) -> [Option<usize>; 3] {
        assert!(
            t < self.adjacency.len(),
            "PolyConnect::neighbours: index {t} out of range [0, {})",
            self.adjacency.len()
        );
        self.adjacency[t]
    }

    /// Return the indices of all triangles that contain `node`.
    ///
    /// The result is sorted in ascending triangle-index order.
    ///
    /// # Panics
    /// Panics if `node >= nb_nodes()`.
    pub fn triangles_around_node(&self, node: usize) -> Vec<usize> {
        assert!(
            node < self.nb_nodes,
            "PolyConnect::triangles_around_node: node {node} out of range [0, {})",
            self.nb_nodes
        );
        let mut result: Vec<usize> = self
            .triangles
            .iter()
            .enumerate()
            .filter_map(|(ti, tri)| {
                if tri.nodes().contains(&node) {
                    Some(ti)
                } else {
                    None
                }
            })
            .collect();
        result.sort_unstable();
        result
    }
}
