// FILE: poly_algo.rs
// occt: Poly_MergeNodesTool
// occt-ref: Poly_CoherencePatch, Poly_SelfIntersectionNodeFilter

/// Merges coincident nodes in a polygon mesh within a tolerance.
/// occt: Poly_MergeNodesTool
#[derive(Clone, Debug)]
pub struct PolyMergeNodesTool {
    pub tolerance: f64,
    pub nodes: Vec<[f64; 3]>,
    pub remap: Vec<usize>,    // remap[old_idx] = new_idx
}

impl PolyMergeNodesTool {
    pub fn new(tolerance: f64) -> Self {
        Self {
            tolerance: tolerance.max(0.0),
            nodes: Vec::new(),
            remap: Vec::new(),
        }
    }

    /// Add a node; returns the canonical index (possibly merged with existing).
    pub fn add_node(&mut self, p: [f64; 3]) -> usize {
        let old = self.nodes.len();
        for (i, n) in self.nodes.iter().enumerate() {
            let d2: f64 = (0..3).map(|k| (n[k] - p[k]).powi(2)).sum();
            if d2.sqrt() <= self.tolerance {
                self.remap.push(i);
                return i;
            }
        }
        self.nodes.push(p);
        self.remap.push(old);
        old
    }

    pub fn canonical_index(&self, old: usize) -> Option<usize> { self.remap.get(old).copied() }
    pub fn nb_original(&self) -> usize { self.remap.len() }
    pub fn nb_merged(&self) -> usize { self.nodes.len() }

    pub fn nb_duplicates_removed(&self) -> usize {
        self.remap.len().saturating_sub(self.nodes.len())
    }
}

/// Coherence patch: set of triangles sharing a common edge (for manifold checks).
/// occt-note: Poly_CoherencePatch (simplified)
#[derive(Clone, Debug, Default)]
pub struct PolyCoherencePatch {
    pub triangles: Vec<[usize; 3]>,    // each triangle: 3 node indices
    pub nb_free_edges: usize,
    pub nb_shared_edges: usize,
}

impl PolyCoherencePatch {
    pub fn new() -> Self { Self::default() }

    pub fn add_triangle(&mut self, a: usize, b: usize, c: usize) {
        self.triangles.push([a, b, c]);
    }

    /// Counts free and shared edges (stub: every triangle contributes 3 half-edges).
    pub fn compute_edge_stats(&mut self) {
        let mut half_edges: Vec<(usize, usize)> = Vec::new();
        for &[a, b, c] in &self.triangles {
            half_edges.push((a.min(b), a.max(b)));
            half_edges.push((b.min(c), b.max(c)));
            half_edges.push((a.min(c), a.max(c)));
        }
        half_edges.sort();
        let mut free = 0usize;
        let mut shared = 0usize;
        let mut i = 0;
        while i < half_edges.len() {
            let e = half_edges[i];
            let mut cnt = 1;
            while i + cnt < half_edges.len() && half_edges[i + cnt] == e { cnt += 1; }
            if cnt == 1 { free += 1; } else { shared += 1; }
            i += cnt;
        }
        self.nb_free_edges = free;
        self.nb_shared_edges = shared;
    }

    pub fn nb_triangles(&self) -> usize { self.triangles.len() }
    pub fn is_closed(&self) -> bool { self.nb_free_edges == 0 }
}

/// Removes nodes that are inside self-intersecting regions.
/// occt-note: Poly_SelfIntersectionNodeFilter (stub)
#[derive(Clone, Debug, Default)]
pub struct PolySelfIntersectionNodeFilter {
    pub removed_ids: Vec<usize>,
}

impl PolySelfIntersectionNodeFilter {
    pub fn new() -> Self { Self::default() }

    /// Stub: flags nodes with the same coordinates as duplicates.
    pub fn filter(&mut self, nodes: &[[f64; 3]], tolerance: f64) {
        self.removed_ids.clear();
        for i in 0..nodes.len() {
            for j in (i+1)..nodes.len() {
                let d2: f64 = (0..3).map(|k| (nodes[i][k] - nodes[j][k]).powi(2)).sum();
                if d2.sqrt() <= tolerance && !self.removed_ids.contains(&j) {
                    self.removed_ids.push(j);
                }
            }
        }
    }

    pub fn nb_removed(&self) -> usize { self.removed_ids.len() }
    pub fn is_removed(&self, id: usize) -> bool { self.removed_ids.contains(&id) }
}

/// Computes polygon mesh quality metrics.
// occt-ref: Poly_Triangulation // quality accessors
#[derive(Clone, Debug, Default)]
pub struct PolyMeshQuality {
    pub min_edge_length: f64,
    pub max_edge_length: f64,
    pub min_aspect_ratio: f64,
    pub max_aspect_ratio: f64,
    pub nb_degenerate: usize,
}

impl PolyMeshQuality {
    pub fn new() -> Self {
        Self {
            min_edge_length: f64::INFINITY,
            max_edge_length: 0.0,
            min_aspect_ratio: f64::INFINITY,
            max_aspect_ratio: 0.0,
            nb_degenerate: 0,
        }
    }

    /// Analyse triangles defined by flat node/index arrays.
    pub fn analyze(&mut self, nodes: &[[f64; 3]], tris: &[[usize; 3]]) {
        *self = Self::new();
        for &[a, b, c] in tris {
            if a >= nodes.len() || b >= nodes.len() || c >= nodes.len() { continue; }
            let pa = nodes[a]; let pb = nodes[b]; let pc = nodes[c];
            let e0 = edge_len(pa, pb);
            let e1 = edge_len(pb, pc);
            let e2 = edge_len(pa, pc);
            let max_e = e0.max(e1).max(e2);
            let min_e = e0.min(e1).min(e2);
            if max_e < 1e-14 { self.nb_degenerate += 1; continue; }
            let ar = max_e / min_e.max(1e-14);
            self.min_edge_length = self.min_edge_length.min(min_e);
            self.max_edge_length = self.max_edge_length.max(max_e);
            self.min_aspect_ratio = self.min_aspect_ratio.min(ar);
            self.max_aspect_ratio = self.max_aspect_ratio.max(ar);
        }
    }
}

fn edge_len(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((0..3).map(|i| (a[i]-b[i]).powi(2)).sum::<f64>()).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_nodes_within_tolerance() {
        let mut m = PolyMergeNodesTool::new(0.01);
        let i0 = m.add_node([0.0, 0.0, 0.0]);
        let i1 = m.add_node([0.005, 0.0, 0.0]); // within tol
        let i2 = m.add_node([1.0, 0.0, 0.0]);
        assert_eq!(i0, 0);
        assert_eq!(i1, 0); // merged
        assert_ne!(i2, 0);
        assert_eq!(m.nb_merged(), 2);
        assert_eq!(m.nb_duplicates_removed(), 1);
    }

    #[test]
    fn coherence_patch_edge_stats() {
        let mut p = PolyCoherencePatch::new();
        // Two triangles sharing edge (1,2)
        p.add_triangle(0, 1, 2);
        p.add_triangle(1, 2, 3);
        p.compute_edge_stats();
        assert_eq!(p.nb_triangles(), 2);
        assert!(!p.is_closed()); // has free edges
    }

    #[test]
    fn self_intersection_filter() {
        let nodes = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 0.005]];
        let mut f = PolySelfIntersectionNodeFilter::new();
        f.filter(&nodes, 0.01);
        assert_eq!(f.nb_removed(), 1);
        assert!(f.is_removed(2));
    }

    #[test]
    fn mesh_quality_equilateral() {
        let nodes = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 0.866, 0.0]];
        let tris = [[0, 1, 2]];
        let mut q = PolyMeshQuality::new();
        q.analyze(&nodes, &tris);
        // Equilateral: aspect ratio ~1
        assert!(q.min_aspect_ratio > 0.9 && q.min_aspect_ratio < 1.1);
        assert_eq!(q.nb_degenerate, 0);
    }
}
