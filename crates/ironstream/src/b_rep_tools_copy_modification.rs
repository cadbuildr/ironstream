// FILE: b_rep_tools_copy_modification.rs
// occt: BRepTools_CopyModification

use std::collections::HashMap;

/// Copy modification for creating a modified copy of a shape.
pub struct BRepToolsCopyModification {
    face_map: HashMap<u32, (String, String, f64)>,
    edge_map: HashMap<u32, (String, String, f64)>,
    vertex_map: HashMap<u32, (f64, f64, f64, f64)>,
}

impl BRepToolsCopyModification {
    /// Create a new copy modification.
    pub fn new() -> Self {
        Self {
            face_map: HashMap::new(),
            edge_map: HashMap::new(),
            vertex_map: HashMap::new(),
        }
    }

    /// Register a face modification.
    pub fn add_face_modification(
        &mut self,
        face_id: u32,
        surface: String,
        location: String,
        tol: f64,
    ) {
        self.face_map.insert(face_id, (surface, location, tol));
    }

    /// Register an edge modification.
    pub fn add_edge_modification(
        &mut self,
        edge_id: u32,
        curve: String,
        location: String,
        tol: f64,
    ) {
        self.edge_map.insert(edge_id, (curve, location, tol));
    }

    /// Register a vertex modification.
    pub fn add_vertex_modification(&mut self, vertex_id: u32, x: f64, y: f64, z: f64, tol: f64) {
        self.vertex_map.insert(vertex_id, (x, y, z, tol));
    }

    /// Get face modification.
    pub fn get_face(&self, face_id: u32) -> Option<(&str, &str, f64)> {
        self.face_map
            .get(&face_id)
            .map(|(s, l, t)| (s.as_str(), l.as_str(), *t))
    }

    /// Get edge modification.
    pub fn get_edge(&self, edge_id: u32) -> Option<(&str, &str, f64)> {
        self.edge_map
            .get(&edge_id)
            .map(|(s, l, t)| (s.as_str(), l.as_str(), *t))
    }

    /// Get vertex modification.
    pub fn get_vertex(&self, vertex_id: u32) -> Option<(f64, f64, f64, f64)> {
        self.vertex_map.get(&vertex_id).copied()
    }

    /// Clear all modifications.
    pub fn clear(&mut self) {
        self.face_map.clear();
        self.edge_map.clear();
        self.vertex_map.clear();
    }
}

impl Default for BRepToolsCopyModification {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = BRepToolsCopyModification::new();
        assert_eq!(m.face_map.len(), 0);
    }

    #[test]
    fn test_add_face() {
        let mut m = BRepToolsCopyModification::new();
        m.add_face_modification(1, "surface".to_string(), "loc".to_string(), 0.01);

        let result = m.get_face(1);
        assert_eq!(result, Some(("surface", "loc", 0.01)));
    }

    #[test]
    fn test_add_vertex() {
        let mut m = BRepToolsCopyModification::new();
        m.add_vertex_modification(1, 1.0, 2.0, 3.0, 0.01);

        let result = m.get_vertex(1);
        assert_eq!(result, Some((1.0, 2.0, 3.0, 0.01)));
    }

    #[test]
    fn test_clear() {
        let mut m = BRepToolsCopyModification::new();
        m.add_face_modification(1, "s".to_string(), "l".to_string(), 0.01);
        m.clear();

        assert_eq!(m.face_map.len(), 0);
    }
}
