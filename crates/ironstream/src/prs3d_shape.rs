// FILE: prs3d_shape.rs
// occt: StdPrs_ShadedShape, StdPrs_WFShape, StdPrs_Vertex,
//       BRepMesh_IncrementalMesh, BRepMesh_FastDiscret

/// Deflection type for mesh generation.
/// occt: BRepMesh_FastDiscret (deflection mode)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DeflectionType {
    #[default]
    Relative,
    Absolute,
}

/// Parameters for incremental mesh generation.
/// occt: BRepMesh_IncrementalMesh (input parameters)
#[derive(Clone, Debug, PartialEq)]
pub struct BrepMeshParams {
    pub linear_deflection: f64,
    pub angular_deflection: f64,
    pub deflection_type: DeflectionType,
    pub is_parallel: bool,
    pub min_size: f64,
    pub in_relative: bool,
    pub internal_vertices_mode: bool,
    pub control_surface_deflection: bool,
}

impl Default for BrepMeshParams {
    fn default() -> Self {
        Self {
            linear_deflection: 0.001,
            angular_deflection: std::f64::consts::PI / 180.0 * 20.0,
            deflection_type: DeflectionType::Relative,
            is_parallel: false,
            min_size: 1e-7,
            in_relative: true,
            internal_vertices_mode: true,
            control_surface_deflection: true,
        }
    }
}

impl BrepMeshParams {
    pub fn new(linear_deflection: f64) -> Self {
        Self { linear_deflection: linear_deflection.max(1e-14), ..Self::default() }
    }

    pub fn with_angular(mut self, a: f64) -> Self { self.angular_deflection = a; self }
    pub fn with_relative(mut self, v: bool) -> Self { self.in_relative = v; self }
    pub fn with_parallel(mut self, v: bool) -> Self { self.is_parallel = v; self }
}

/// Incremental mesh builder stub.
/// occt: BRepMesh_IncrementalMesh
#[derive(Clone, Debug)]
pub struct BrepMeshIncrementalMesh {
    pub shape_id: u32,
    pub params: BrepMeshParams,
    pub is_done: bool,
    pub nb_faces_meshed: usize,
    pub nb_failed: usize,
}

impl BrepMeshIncrementalMesh {
    pub fn new(shape_id: u32, params: BrepMeshParams) -> Self {
        Self { shape_id, params, is_done: false, nb_faces_meshed: 0, nb_failed: 0 }
    }

    /// Stub: mesh all faces (reports shape_id faces meshed).
    pub fn perform(&mut self) {
        self.nb_faces_meshed = self.shape_id as usize;
        self.nb_failed = 0;
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_faces_meshed(&self) -> usize { self.nb_faces_meshed }
}

/// A triangle in a mesh (3 vertex indices, 0-based).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeshTriangle {
    pub indices: [u32; 3],
}

impl MeshTriangle {
    pub fn new(a: u32, b: u32, c: u32) -> Self { Self { indices: [a, b, c] } }
}

/// A mesh associated with a face.
/// occt: Poly_Triangulation (simplified)
#[derive(Clone, Debug, Default)]
pub struct PolyTriangulation {
    pub nodes: Vec<[f64; 3]>,
    pub triangles: Vec<MeshTriangle>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub deflection: f64,
}

impl PolyTriangulation {
    pub fn new() -> Self { Self::default() }

    pub fn add_node(&mut self, p: [f64; 3]) -> u32 {
        let idx = self.nodes.len() as u32;
        self.nodes.push(p);
        idx
    }

    pub fn add_triangle(&mut self, t: MeshTriangle) { self.triangles.push(t); }

    pub fn set_normal(&mut self, idx: usize, n: [f32; 3]) {
        if self.normals.len() <= idx { self.normals.resize(idx + 1, [0.0; 3]); }
        self.normals[idx] = n;
    }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn nb_triangles(&self) -> usize { self.triangles.len() }
    pub fn has_normals(&self) -> bool { !self.normals.is_empty() }
    pub fn has_uvs(&self) -> bool { !self.uvs.is_empty() }

    pub fn node(&self, idx: usize) -> Option<[f64; 3]> { self.nodes.get(idx).copied() }
    pub fn triangle(&self, idx: usize) -> Option<MeshTriangle> { self.triangles.get(idx).copied() }
}

/// Shaded shape presentation stub.
/// occt: StdPrs_ShadedShape
pub struct StdPrsShadedShape;

impl StdPrsShadedShape {
    /// Add shaded mesh to presentation.
    /// Stub: returns number of triangles from triangulation.
    pub fn add(tri: &PolyTriangulation) -> usize {
        tri.nb_triangles()
    }
}

/// Wireframe shape presentation stub.
/// occt: StdPrs_WFShape
pub struct StdPrsWFShape;

impl StdPrsWFShape {
    /// Returns edge count (stub: nb_nodes / 2).
    pub fn add(tri: &PolyTriangulation) -> usize {
        tri.nb_nodes() / 2
    }
}

/// Vertex presentation stub.
/// occt: StdPrs_Vertex
pub struct StdPrsVertex;

impl StdPrsVertex {
    pub fn add(point: [f64; 3], _marker_size: f64) -> [f64; 3] { point }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brep_mesh_params_defaults() {
        let p = BrepMeshParams::default();
        assert!(p.in_relative);
        assert!((p.linear_deflection - 0.001).abs() < 1e-14);
    }

    #[test]
    fn incremental_mesh_perform() {
        let p = BrepMeshParams::new(0.01);
        let mut m = BrepMeshIncrementalMesh::new(5, p);
        assert!(!m.is_done());
        m.perform();
        assert!(m.is_done());
        assert_eq!(m.nb_faces_meshed(), 5);
    }

    #[test]
    fn poly_triangulation_add() {
        let mut t = PolyTriangulation::new();
        let a = t.add_node([0.0, 0.0, 0.0]);
        let b = t.add_node([1.0, 0.0, 0.0]);
        let c = t.add_node([0.0, 1.0, 0.0]);
        t.add_triangle(MeshTriangle::new(a, b, c));
        assert_eq!(t.nb_nodes(), 3);
        assert_eq!(t.nb_triangles(), 1);
        assert_eq!(t.triangle(0).unwrap().indices, [0, 1, 2]);
    }

    #[test]
    fn poly_triangulation_normals() {
        let mut t = PolyTriangulation::new();
        t.add_node([0.0; 3]);
        t.set_normal(0, [0.0, 0.0, 1.0]);
        assert!(t.has_normals());
    }

    #[test]
    fn shaded_shape_stub() {
        let mut t = PolyTriangulation::new();
        let a = t.add_node([0.0; 3]);
        let b = t.add_node([1.0, 0.0, 0.0]);
        let c = t.add_node([0.0, 1.0, 0.0]);
        t.add_triangle(MeshTriangle::new(a, b, c));
        assert_eq!(StdPrsShadedShape::add(&t), 1);
    }

    #[test]
    fn mesh_params_builder() {
        let p = BrepMeshParams::new(0.005)
            .with_angular(0.1)
            .with_relative(false)
            .with_parallel(true);
        assert!(!p.in_relative);
        assert!(p.is_parallel);
        assert!((p.angular_deflection - 0.1).abs() < 1e-14);
    }
}
