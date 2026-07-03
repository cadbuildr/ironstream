// FILE: b_rep_tools_g_trsf_modification.rs
// occt: BRepTools_GTrsfModification

use std::collections::HashMap;

/// Global transformation modification for shapes.
pub struct BRepToolsGTrsfModification {
    transform_matrix: [[f64; 4]; 4],
    affected_faces: HashMap<u32, bool>,
    affected_edges: HashMap<u32, bool>,
    affected_vertices: HashMap<u32, bool>,
}

impl BRepToolsGTrsfModification {
    /// Create a new global transformation modification.
    pub fn new() -> Self {
        let mut identity = [[0.0; 4]; 4];
        identity[0][0] = 1.0;
        identity[1][1] = 1.0;
        identity[2][2] = 1.0;
        identity[3][3] = 1.0;

        Self {
            transform_matrix: identity,
            affected_faces: HashMap::new(),
            affected_edges: HashMap::new(),
            affected_vertices: HashMap::new(),
        }
    }

    /// Set the transformation matrix (4x4).
    pub fn set_matrix(&mut self, matrix: [[f64; 4]; 4]) {
        self.transform_matrix = matrix;
    }

    /// Mark a face as affected.
    pub fn mark_face(&mut self, face_id: u32, affected: bool) {
        self.affected_faces.insert(face_id, affected);
    }

    /// Mark an edge as affected.
    pub fn mark_edge(&mut self, edge_id: u32, affected: bool) {
        self.affected_edges.insert(edge_id, affected);
    }

    /// Mark a vertex as affected.
    pub fn mark_vertex(&mut self, vertex_id: u32, affected: bool) {
        self.affected_vertices.insert(vertex_id, affected);
    }

    /// Check if a face is affected.
    pub fn is_face_affected(&self, face_id: u32) -> bool {
        *self.affected_faces.get(&face_id).unwrap_or(&false)
    }

    /// Check if an edge is affected.
    pub fn is_edge_affected(&self, edge_id: u32) -> bool {
        *self.affected_edges.get(&edge_id).unwrap_or(&false)
    }

    /// Check if a vertex is affected.
    pub fn is_vertex_affected(&self, vertex_id: u32) -> bool {
        *self.affected_vertices.get(&vertex_id).unwrap_or(&false)
    }

    /// Get the transformation matrix.
    pub fn get_matrix(&self) -> &[[f64; 4]; 4] {
        &self.transform_matrix
    }

    /// Clear all markings.
    pub fn clear(&mut self) {
        self.affected_faces.clear();
        self.affected_edges.clear();
        self.affected_vertices.clear();
    }
}

impl Default for BRepToolsGTrsfModification {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = BRepToolsGTrsfModification::new();
        let matrix = m.get_matrix();
        assert_eq!(matrix[0][0], 1.0);
        assert_eq!(matrix[1][1], 1.0);
        assert_eq!(matrix[2][2], 1.0);
        assert_eq!(matrix[3][3], 1.0);
    }

    #[test]
    fn test_mark_and_check() {
        let mut m = BRepToolsGTrsfModification::new();
        m.mark_face(1, true);
        m.mark_edge(2, true);

        assert!(m.is_face_affected(1));
        assert!(!m.is_face_affected(2));
        assert!(m.is_edge_affected(2));
        assert!(!m.is_edge_affected(1));
    }

    #[test]
    fn test_clear() {
        let mut m = BRepToolsGTrsfModification::new();
        m.mark_face(1, true);
        m.mark_vertex(1, true);

        m.clear();
        assert!(!m.is_face_affected(1));
        assert!(!m.is_vertex_affected(1));
    }
}
