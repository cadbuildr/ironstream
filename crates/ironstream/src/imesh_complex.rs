// FILE: imesh_complex.rs
// occt: IMeshData_Face, IMeshData_Edge, IMeshData_Curve, IMeshData_Model,
//       BRepMesh_Triangle, BRepMesh_Vertex, BRepMesh_Edge

/// A mesh vertex with 2D UV and 3D position.
#[derive(Clone, Debug)]
pub struct MeshVertex {
    pub id: u32,
    pub uv: [f64; 2],
    pub xyz: [f64; 3],
    pub is_on_boundary: bool,
}

impl MeshVertex {
    pub fn new(id: u32, uv: [f64; 2], xyz: [f64; 3]) -> Self {
        Self { id, uv, xyz, is_on_boundary: false }
    }

    pub fn on_boundary(mut self) -> Self { self.is_on_boundary = true; self }
}

/// A directed mesh edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeshEdgeDef {
    pub v0: u32,
    pub v1: u32,
    pub is_boundary: bool,
}

impl MeshEdgeDef {
    pub fn new(v0: u32, v1: u32) -> Self { Self { v0, v1, is_boundary: false } }
    pub fn boundary(mut self) -> Self { self.is_boundary = true; self }
    pub fn reversed(&self) -> Self { Self { v0: self.v1, v1: self.v0, is_boundary: self.is_boundary } }
}

// occt: BRepMesh_Triangle
#[derive(Clone, Copy, Debug)]
pub struct MeshTriangle {
    pub vertices: [u32; 3],
    pub edges: [u32; 3],
    pub is_deleted: bool,
}

impl MeshTriangle {
    pub fn new(v: [u32; 3], e: [u32; 3]) -> Self { Self { vertices: v, edges: e, is_deleted: false } }
    pub fn delete(&mut self) { self.is_deleted = true; }
    pub fn contains_vertex(&self, v: u32) -> bool { self.vertices.iter().any(|&x| x == v) }
    pub fn opposite_edge(&self, v: u32) -> Option<u32> {
        for i in 0..3 {
            if self.vertices[i] == v { return Some(self.edges[(i + 1) % 3]); }
        }
        None
    }
}

/// IMeshData face data structure.
#[derive(Clone, Debug, Default)]
pub struct IMeshFace {
    pub face_id: u64,
    pub vertices: Vec<MeshVertex>,
    pub edges: Vec<MeshEdgeDef>,
    pub triangles: Vec<MeshTriangle>,
    pub is_done: bool,
    pub deflection: f64,
}

impl IMeshFace {
    pub fn new(face_id: u64) -> Self { Self { face_id, deflection: 1e-3, ..Default::default() } }

    pub fn add_vertex(&mut self, v: MeshVertex) -> u32 {
        let id = self.vertices.len() as u32;
        self.vertices.push(v);
        id
    }

    pub fn add_edge(&mut self, e: MeshEdgeDef) -> u32 {
        let id = self.edges.len() as u32;
        self.edges.push(e);
        id
    }

    pub fn add_triangle(&mut self, t: MeshTriangle) { self.triangles.push(t); }

    pub fn nb_vertices(&self) -> usize { self.vertices.len() }
    pub fn nb_edges(&self) -> usize { self.edges.len() }
    pub fn nb_triangles(&self) -> usize { self.triangles.iter().filter(|t| !t.is_deleted).count() }

    pub fn boundary_vertices(&self) -> Vec<&MeshVertex> {
        self.vertices.iter().filter(|v| v.is_on_boundary).collect()
    }

    pub fn boundary_edges(&self) -> Vec<&MeshEdgeDef> {
        self.edges.iter().filter(|e| e.is_boundary).collect()
    }

    pub fn edge_length(&self, e: &MeshEdgeDef) -> Option<f64> {
        let v0 = self.vertices.get(e.v0 as usize)?;
        let v1 = self.vertices.get(e.v1 as usize)?;
        let dx = v1.xyz[0]-v0.xyz[0]; let dy = v1.xyz[1]-v0.xyz[1]; let dz = v1.xyz[2]-v0.xyz[2];
        Some((dx*dx+dy*dy+dz*dz).sqrt())
    }
}

/// Full mesh model across all faces.
#[derive(Clone, Debug, Default)]
pub struct IMeshModel {
    pub faces: Vec<IMeshFace>,
    pub max_deflection: f64,
    pub angle_deflection: f64,
}

impl IMeshModel {
    pub fn new() -> Self { Self { max_deflection: 1e-3, angle_deflection: 0.5, ..Default::default() } }

    pub fn add_face(&mut self, face: IMeshFace) { self.faces.push(face); }
    pub fn nb_faces(&self) -> usize { self.faces.len() }
    pub fn total_triangles(&self) -> usize { self.faces.iter().map(|f| f.nb_triangles()).sum() }
    pub fn total_vertices(&self) -> usize { self.faces.iter().map(|f| f.nb_vertices()).sum() }
    pub fn is_done(&self) -> bool { self.faces.iter().all(|f| f.is_done) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_face_basics() {
        let mut f = IMeshFace::new(1);
        let v0 = f.add_vertex(MeshVertex::new(0, [0.0,0.0], [0.0,0.0,0.0]).on_boundary());
        let v1 = f.add_vertex(MeshVertex::new(1, [1.0,0.0], [1.0,0.0,0.0]).on_boundary());
        let v2 = f.add_vertex(MeshVertex::new(2, [0.0,1.0], [0.0,1.0,0.0]));
        let e0 = f.add_edge(MeshEdgeDef::new(v0, v1).boundary());
        let e1 = f.add_edge(MeshEdgeDef::new(v1, v2));
        let e2 = f.add_edge(MeshEdgeDef::new(v2, v0));
        f.add_triangle(MeshTriangle::new([v0, v1, v2], [e0, e1, e2]));
        f.is_done = true;
        assert_eq!(f.nb_vertices(), 3);
        assert_eq!(f.nb_triangles(), 1);
        assert_eq!(f.boundary_vertices().len(), 2);
        assert_eq!(f.boundary_edges().len(), 1);
    }

    #[test]
    fn mesh_triangle_ops() {
        let t = MeshTriangle::new([0, 1, 2], [10, 11, 12]);
        assert!(t.contains_vertex(1));
        assert!(!t.contains_vertex(5));
    }

    #[test]
    fn mesh_model_totals() {
        let mut model = IMeshModel::new();
        let mut f = IMeshFace::new(0);
        f.add_vertex(MeshVertex::new(0, [0.0,0.0], [0.0;3]));
        f.add_vertex(MeshVertex::new(1, [1.0,0.0], [1.0,0.0,0.0]));
        f.add_vertex(MeshVertex::new(2, [0.0,1.0], [0.0,1.0,0.0]));
        f.add_triangle(MeshTriangle::new([0,1,2], [0,1,2]));
        f.is_done = true;
        model.add_face(f);
        assert_eq!(model.total_triangles(), 1);
        assert_eq!(model.total_vertices(), 3);
        assert!(model.is_done());
    }
}
