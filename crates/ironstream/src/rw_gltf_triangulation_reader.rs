// FILE: rw_gltf_triangulation_reader.rs
// occt: RWGltf_TriangulationReader

//! Triangulation reader for glTF.

#[derive(Debug, Clone)]
pub struct TriangulationReader {
    vertices: Vec<(f32, f32, f32)>,
    faces: Vec<(u32, u32, u32)>,
}

impl TriangulationReader {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.push((x, y, z));
    }

    pub fn add_face(&mut self, v1: u32, v2: u32, v3: u32) {
        self.faces.push((v1, v2, v3));
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn vertices(&self) -> &[(f32, f32, f32)] {
        &self.vertices
    }

    pub fn faces(&self) -> &[(u32, u32, u32)] {
        &self.faces
    }
}

impl Default for TriangulationReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vertex() {
        let mut reader = TriangulationReader::new();
        reader.add_vertex(0.0, 0.0, 0.0);
        assert_eq!(reader.num_vertices(), 1);
    }

    #[test]
    fn test_add_face() {
        let mut reader = TriangulationReader::new();
        reader.add_face(0, 1, 2);
        assert_eq!(reader.num_faces(), 1);
    }
}
