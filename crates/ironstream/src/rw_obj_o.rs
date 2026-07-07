// FILE: rw_obj_o.rs
// occt: RWObj

//! OBJ file format reader and writer.

#[derive(Debug, Clone)]
pub struct RWObj {
    vertices: Vec<(f32, f32, f32)>,
    normals: Vec<(f32, f32, f32)>,
    texcoords: Vec<(f32, f32)>,
    faces: Vec<Face>,
}

#[derive(Debug, Clone)]
pub struct Face {
    vertex_indices: [u32; 3],
    normal_indices: [Option<u32>; 3],
    texcoord_indices: [Option<u32>; 3],
}

impl Face {
    pub fn new(v1: u32, v2: u32, v3: u32) -> Self {
        Self {
            vertex_indices: [v1, v2, v3],
            normal_indices: [None, None, None],
            texcoord_indices: [None, None, None],
        }
    }
}

impl RWObj {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            texcoords: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.push((x, y, z));
    }

    pub fn add_normal(&mut self, x: f32, y: f32, z: f32) {
        self.normals.push((x, y, z));
    }

    pub fn add_texcoord(&mut self, u: f32, v: f32) {
        self.texcoords.push((u, v));
    }

    pub fn add_face(&mut self, face: Face) {
        self.faces.push(face);
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_normals(&self) -> usize {
        self.normals.len()
    }

    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn vertices(&self) -> &[(f32, f32, f32)] {
        &self.vertices
    }

    pub fn normals(&self) -> &[(f32, f32, f32)] {
        &self.normals
    }

    pub fn faces(&self) -> &[Face] {
        &self.faces
    }
}

impl Default for RWObj {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vertex() {
        let mut obj = RWObj::new();
        obj.add_vertex(0.0, 0.0, 0.0);
        obj.add_vertex(1.0, 0.0, 0.0);
        assert_eq!(obj.num_vertices(), 2);
    }

    #[test]
    fn test_add_face() {
        let mut obj = RWObj::new();
        obj.add_vertex(0.0, 0.0, 0.0);
        obj.add_vertex(1.0, 0.0, 0.0);
        obj.add_vertex(0.0, 1.0, 0.0);

        let face = Face::new(0, 1, 2);
        obj.add_face(face);
        assert_eq!(obj.num_faces(), 1);
    }

    #[test]
    fn test_add_normal() {
        let mut obj = RWObj::new();
        obj.add_normal(0.0, 0.0, 1.0);
        assert_eq!(obj.num_normals(), 1);
    }
}
