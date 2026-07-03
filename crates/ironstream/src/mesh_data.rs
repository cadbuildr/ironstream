// FILE: mesh_data.rs
// occt: BRep_Triangulation, Poly_Connect, MeshDS_DataSource,
//       MeshVS_DataSource, MeshVS_Mesh, IMeshData_Edge, IMeshData_Face

/// A single triangular element.
/// occt: Poly_Triangle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PolyTriangle {
    pub n1: usize,
    pub n2: usize,
    pub n3: usize,
}

impl PolyTriangle {
    pub fn new(n1: usize, n2: usize, n3: usize) -> Self { Self { n1, n2, n3 } }

    pub fn get(&self, i: usize) -> Option<usize> {
        match i {
            1 => Some(self.n1),
            2 => Some(self.n2),
            3 => Some(self.n3),
            _ => None,
        }
    }

    pub fn set(&mut self, i: usize, v: usize) {
        match i { 1 => self.n1 = v, 2 => self.n2 = v, 3 => self.n3 = v, _ => {} }
    }
}

/// Quality metrics for a mesh.
#[derive(Clone, Debug, Default)]
pub struct MeshQuality {
    pub min_edge_length: f64,
    pub max_edge_length: f64,
    pub avg_edge_length: f64,
    pub min_aspect_ratio: f64,
    pub max_aspect_ratio: f64,
    pub nb_degenerate_triangles: usize,
}

/// A 3D mesh: nodes + triangles (1-based, like OCCT Poly_Triangulation).
/// occt: Poly_Triangulation / MeshDS_DataSource / BRep_Triangulation
#[derive(Clone, Debug, Default)]
pub struct MeshDataSource {
    pub mesh_id: u32,
    pub nodes: Vec<[f64; 3]>,
    pub uv_nodes: Vec<[f64; 2]>,   // optional 2D UV coordinates
    pub normals: Vec<[f64; 3]>,    // optional per-node normals
    pub triangles: Vec<PolyTriangle>,
    pub deflection: f64,
}

impl MeshDataSource {
    pub fn new(mesh_id: u32) -> Self {
        Self { mesh_id, deflection: 0.001, ..Self::default() }
    }

    /// Add a node (1-based indexing externally: returned index is 1-based).
    pub fn add_node(&mut self, p: [f64; 3]) -> usize {
        self.nodes.push(p);
        self.nodes.len()
    }

    pub fn set_uv_node(&mut self, i: usize, uv: [f64; 2]) {
        if i >= 1 && i <= self.uv_nodes.len() + 1 {
            if i == self.uv_nodes.len() + 1 { self.uv_nodes.push(uv); }
            else { self.uv_nodes[i - 1] = uv; }
        }
    }

    pub fn set_normal(&mut self, i: usize, n: [f64; 3]) {
        if i >= 1 && i <= self.normals.len() + 1 {
            if i == self.normals.len() + 1 { self.normals.push(n); }
            else { self.normals[i - 1] = n; }
        }
    }

    pub fn add_triangle(&mut self, t: PolyTriangle) -> usize {
        self.triangles.push(t);
        self.triangles.len()
    }

    pub fn node(&self, i: usize) -> Option<[f64; 3]> {
        if i == 0 { None } else { self.nodes.get(i - 1).copied() }
    }

    pub fn uv_node(&self, i: usize) -> Option<[f64; 2]> {
        if i == 0 { None } else { self.uv_nodes.get(i - 1).copied() }
    }

    pub fn normal(&self, i: usize) -> Option<[f64; 3]> {
        if i == 0 { None } else { self.normals.get(i - 1).copied() }
    }

    pub fn triangle(&self, i: usize) -> Option<PolyTriangle> {
        if i == 0 { None } else { self.triangles.get(i - 1).copied() }
    }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn nb_triangles(&self) -> usize { self.triangles.len() }
    pub fn has_uv_nodes(&self) -> bool { self.uv_nodes.len() == self.nodes.len() }
    pub fn has_normals(&self) -> bool { self.normals.len() == self.nodes.len() }
    pub fn deflection(&self) -> f64 { self.deflection }
    pub fn set_deflection(&mut self, d: f64) { self.deflection = d; }

    /// Bounding box of all nodes.
    pub fn bounding_box(&self) -> Option<([f64; 3], [f64; 3])> {
        if self.nodes.is_empty() { return None; }
        let mut mn = [f64::INFINITY; 3];
        let mut mx = [f64::NEG_INFINITY; 3];
        for n in &self.nodes {
            for k in 0..3 { mn[k] = mn[k].min(n[k]); mx[k] = mx[k].max(n[k]); }
        }
        Some((mn, mx))
    }

    /// Volume of the mesh (sum of signed tetrahedra volumes, origin-based).
    pub fn volume(&self) -> f64 {
        let mut vol = 0.0f64;
        for t in &self.triangles {
            if t.n1 == 0 || t.n2 == 0 || t.n3 == 0 { continue; }
            if t.n1 > self.nodes.len() || t.n2 > self.nodes.len() || t.n3 > self.nodes.len() { continue; }
            let a = self.nodes[t.n1 - 1];
            let b = self.nodes[t.n2 - 1];
            let c = self.nodes[t.n3 - 1];
            // signed volume of tetrahedron (a, b, c, origin)
            vol += a[0]*(b[1]*c[2]-b[2]*c[1])
                 + a[1]*(b[2]*c[0]-b[0]*c[2])
                 + a[2]*(b[0]*c[1]-b[1]*c[0]);
        }
        vol.abs() / 6.0
    }

    /// Compute quality metrics.
    pub fn quality(&self) -> MeshQuality {
        let mut q = MeshQuality::default();
        if self.triangles.is_empty() { return q; }
        q.min_edge_length = f64::INFINITY;
        let mut total_edge = 0.0f64;
        let mut n_edges = 0usize;
        for t in &self.triangles {
            let nodes: Vec<[f64; 3]> = [t.n1, t.n2, t.n3].iter()
                .filter_map(|&i| if i >= 1 && i <= self.nodes.len() { Some(self.nodes[i-1]) } else { None })
                .collect();
            if nodes.len() < 3 { q.nb_degenerate_triangles += 1; continue; }
            let edges = [
                (0usize, 1usize), (1, 2), (2, 0)
            ];
            let mut lens = [0.0f64; 3];
            for (k, &(a, b)) in edges.iter().enumerate() {
                let d = [nodes[a][0]-nodes[b][0], nodes[a][1]-nodes[b][1], nodes[a][2]-nodes[b][2]];
                lens[k] = (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt();
                q.min_edge_length = q.min_edge_length.min(lens[k]);
                q.max_edge_length = q.max_edge_length.max(lens[k]);
                total_edge += lens[k];
                n_edges += 1;
            }
        }
        if n_edges > 0 { q.avg_edge_length = total_edge / n_edges as f64; }
        q.min_aspect_ratio = 1.0;
        q.max_aspect_ratio = 1.0;
        q
    }
}

/// Connectivity of triangles around nodes.
/// occt: Poly_Connect
#[derive(Clone, Debug, Default)]
pub struct PolyConnect {
    pub mesh: Option<Box<MeshDataSource>>,
    node_triangles: Vec<Vec<usize>>, // node_triangles[node_idx] = list of tri indices (0-based)
}

impl PolyConnect {
    pub fn new(mesh: MeshDataSource) -> Self {
        let nb_nodes = mesh.nb_nodes();
        let mut node_triangles = vec![Vec::new(); nb_nodes];
        for (ti, t) in mesh.triangles.iter().enumerate() {
            for &ni in &[t.n1, t.n2, t.n3] {
                if ni >= 1 && ni <= nb_nodes {
                    node_triangles[ni - 1].push(ti);
                }
            }
        }
        Self { mesh: Some(Box::new(mesh)), node_triangles }
    }

    /// Number of triangles around node i (1-based).
    pub fn nb_triangles_around_node(&self, i: usize) -> usize {
        if i == 0 { return 0; }
        self.node_triangles.get(i - 1).map(|v| v.len()).unwrap_or(0)
    }

    /// Triangle indices (1-based) around node i (1-based).
    pub fn triangles_around_node(&self, i: usize) -> Vec<usize> {
        if i == 0 { return vec![]; }
        self.node_triangles.get(i - 1)
            .map(|v| v.iter().map(|&ti| ti + 1).collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tetrahedron() -> MeshDataSource {
        let mut m = MeshDataSource::new(1);
        m.add_node([0.0, 0.0, 0.0]);
        m.add_node([1.0, 0.0, 0.0]);
        m.add_node([0.5, 1.0, 0.0]);
        m.add_node([0.5, 0.5, 1.0]);
        m.add_triangle(PolyTriangle::new(1, 2, 3));
        m.add_triangle(PolyTriangle::new(1, 2, 4));
        m.add_triangle(PolyTriangle::new(2, 3, 4));
        m.add_triangle(PolyTriangle::new(1, 3, 4));
        m
    }

    #[test]
    fn poly_triangle_access() {
        let mut t = PolyTriangle::new(1, 2, 3);
        assert_eq!(t.get(1), Some(1));
        assert_eq!(t.get(2), Some(2));
        assert_eq!(t.get(0), None);
        t.set(1, 10);
        assert_eq!(t.n1, 10);
    }

    #[test]
    fn mesh_data_add_retrieve() {
        let m = make_tetrahedron();
        assert_eq!(m.nb_nodes(), 4);
        assert_eq!(m.nb_triangles(), 4);
        assert_eq!(m.node(1), Some([0.0, 0.0, 0.0]));
        assert!(m.node(0).is_none());
        assert_eq!(m.triangle(1), Some(PolyTriangle::new(1, 2, 3)));
    }

    #[test]
    fn mesh_bounding_box() {
        let m = make_tetrahedron();
        let (mn, mx) = m.bounding_box().unwrap();
        assert!(mn[0].abs() < 1e-10);
        assert!((mx[0] - 1.0).abs() < 1e-10);
        assert!((mx[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn poly_connect_around_node() {
        let m = make_tetrahedron();
        let conn = PolyConnect::new(m);
        let tris = conn.triangles_around_node(1);
        assert_eq!(tris.len(), 3); // node 1 is in triangles 1, 2, 4
        assert_eq!(conn.nb_triangles_around_node(0), 0);
    }

    #[test]
    fn mesh_quality_basic() {
        let m = make_tetrahedron();
        let q = m.quality();
        assert!(q.min_edge_length > 0.0);
        assert!(q.max_edge_length >= q.min_edge_length);
        assert_eq!(q.nb_degenerate_triangles, 0);
    }
}
