// FILE: b_rep_mesh_data_face.rs
// occt: BRepMeshData_Face

/// Default implementation of face data model entity for mesh.
pub struct BRepMeshDataFace {
    face_id: usize,
    edges: Vec<usize>,
}

impl BRepMeshDataFace {
    /// Creates a new face data.
    pub fn new(_face_id: usize) -> Self {
        BRepMeshDataFace {
            face_id: _face_id,
            edges: Vec::new(),
        }
    }

    /// Adds an edge to the face.
    pub fn add_edge(&mut self, _edge_id: usize) {
        self.edges.push(_edge_id);
    }

    /// Returns the number of edges.
    pub fn edges_nb(&self) -> usize {
        self.edges.len()
    }

    /// Gets the edge at index.
    pub fn edge(&self, _index: usize) -> Option<usize> {
        if _index < self.edges.len() {
            Some(self.edges[_index])
        } else {
            None
        }
    }

    /// Clears the edges.
    pub fn clear(&mut self) {
        self.edges.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_creation() {
        let face = BRepMeshDataFace::new(0);
        assert_eq!(face.face_id, 0);
        assert_eq!(face.edges_nb(), 0);
    }

    #[test]
    fn test_face_add_edge() {
        let mut face = BRepMeshDataFace::new(0);
        face.add_edge(1);
        assert_eq!(face.edges_nb(), 1);
        assert_eq!(face.edge(0), Some(1));
    }

    #[test]
    fn test_face_clear() {
        let mut face = BRepMeshDataFace::new(0);
        face.add_edge(1);
        face.clear();
        assert_eq!(face.edges_nb(), 0);
    }
}
