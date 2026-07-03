// FILE: b_rep_mesh_data_model.rs
// occt: BRepMeshData_Model

/// Data model container for mesh generation.
pub struct BRepMeshDataModel {
    vertices: Vec<f64>,
    faces: Vec<usize>,
}

impl BRepMeshDataModel {
    /// Creates a new mesh data model.
    pub fn new() -> Self {
        BRepMeshDataModel {
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }

    /// Adds a vertex to the model.
    pub fn add_vertex(&mut self, _vertex_id: usize) {
        self.vertices.push(0.0);
    }

    /// Returns the number of vertices.
    pub fn vertices_nb(&self) -> usize {
        self.vertices.len()
    }

    /// Adds a face to the model.
    pub fn add_face(&mut self, _face_id: usize) {
        self.faces.push(_face_id);
    }

    /// Returns the number of faces.
    pub fn faces_nb(&self) -> usize {
        self.faces.len()
    }

    /// Clears the model.
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.faces.clear();
    }
}

impl Default for BRepMeshDataModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_creation() {
        let model = BRepMeshDataModel::new();
        assert_eq!(model.vertices_nb(), 0);
        assert_eq!(model.faces_nb(), 0);
    }

    #[test]
    fn test_model_add_vertex() {
        let mut model = BRepMeshDataModel::new();
        model.add_vertex(0);
        assert_eq!(model.vertices_nb(), 1);
    }

    #[test]
    fn test_model_add_face() {
        let mut model = BRepMeshDataModel::new();
        model.add_face(0);
        assert_eq!(model.faces_nb(), 1);
    }

    #[test]
    fn test_model_clear() {
        let mut model = BRepMeshDataModel::new();
        model.add_vertex(0);
        model.add_face(0);
        model.clear();
        assert_eq!(model.vertices_nb(), 0);
        assert_eq!(model.faces_nb(), 0);
    }
}
