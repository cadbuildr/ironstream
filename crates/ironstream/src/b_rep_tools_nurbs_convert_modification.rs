// FILE: b_rep_tools_nurbs_convert_modification.rs
// occt: BRepTools_NurbsConvertModification

use std::collections::HashMap;

/// NURBS conversion modification for shapes.
pub struct BRepToolsNurbsConvertModification {
    converted_faces: HashMap<u32, Vec<f64>>,
    converted_edges: HashMap<u32, Vec<f64>>,
    conversion_tolerance: f64,
}

impl BRepToolsNurbsConvertModification {
    /// Create a new NURBS conversion modification.
    pub fn new() -> Self {
        Self {
            converted_faces: HashMap::new(),
            converted_edges: HashMap::new(),
            conversion_tolerance: 1e-7,
        }
    }

    /// Set the conversion tolerance.
    pub fn set_tolerance(&mut self, tol: f64) {
        self.conversion_tolerance = tol;
    }

    /// Get the conversion tolerance.
    pub fn tolerance(&self) -> f64 {
        self.conversion_tolerance
    }

    /// Add a converted face as NURBS control points.
    pub fn add_converted_face(&mut self, face_id: u32, knots: Vec<f64>) {
        self.converted_faces.insert(face_id, knots);
    }

    /// Add a converted edge as NURBS control points.
    pub fn add_converted_edge(&mut self, edge_id: u32, knots: Vec<f64>) {
        self.converted_edges.insert(edge_id, knots);
    }

    /// Get converted face data.
    pub fn get_converted_face(&self, face_id: u32) -> Option<&[f64]> {
        self.converted_faces.get(&face_id).map(|v| v.as_slice())
    }

    /// Get converted edge data.
    pub fn get_converted_edge(&self, edge_id: u32) -> Option<&[f64]> {
        self.converted_edges.get(&edge_id).map(|v| v.as_slice())
    }

    /// Clear all conversions.
    pub fn clear(&mut self) {
        self.converted_faces.clear();
        self.converted_edges.clear();
    }

    /// Check if a face was converted.
    pub fn is_face_converted(&self, face_id: u32) -> bool {
        self.converted_faces.contains_key(&face_id)
    }

    /// Check if an edge was converted.
    pub fn is_edge_converted(&self, edge_id: u32) -> bool {
        self.converted_edges.contains_key(&edge_id)
    }
}

impl Default for BRepToolsNurbsConvertModification {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = BRepToolsNurbsConvertModification::new();
        assert_eq!(m.tolerance(), 1e-7);
    }

    #[test]
    fn test_set_tolerance() {
        let mut m = BRepToolsNurbsConvertModification::new();
        m.set_tolerance(1e-6);
        assert_eq!(m.tolerance(), 1e-6);
    }

    #[test]
    fn test_add_converted_face() {
        let mut m = BRepToolsNurbsConvertModification::new();
        m.add_converted_face(1, vec![0.0, 1.0, 2.0]);

        assert!(m.is_face_converted(1));
        assert_eq!(m.get_converted_face(1), Some(&[0.0, 1.0, 2.0][..]));
    }

    #[test]
    fn test_add_converted_edge() {
        let mut m = BRepToolsNurbsConvertModification::new();
        m.add_converted_edge(1, vec![1.0, 2.0]);

        assert!(m.is_edge_converted(1));
        assert_eq!(m.get_converted_edge(1), Some(&[1.0, 2.0][..]));
    }

    #[test]
    fn test_clear() {
        let mut m = BRepToolsNurbsConvertModification::new();
        m.add_converted_face(1, vec![0.0]);
        m.add_converted_edge(1, vec![1.0]);

        m.clear();
        assert!(!m.is_face_converted(1));
        assert!(!m.is_edge_converted(1));
    }
}
