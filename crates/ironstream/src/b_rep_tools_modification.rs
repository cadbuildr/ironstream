// FILE: b_rep_tools_modification.rs
// occt: BRepTools_Modification

/// Defines geometric modifications to a shape.
pub trait BRepToolsModification {
    /// Return true if the face has been modified.
    fn new_surface(&self, face_id: u32) -> Option<(String, String, f64, bool, bool)>;

    /// Return true if the edge has been modified.
    fn new_curve(&self, edge_id: u32) -> Option<(String, String, f64)>;

    /// Return true if the vertex has been modified.
    fn new_point(&self, vertex_id: u32) -> Option<(f64, f64, f64, f64)>;

    /// Return true if the face has been modified according to changed triangulation.
    fn new_triangulation(&self, face_id: u32) -> Option<Vec<f64>> {
        None
    }

    /// Return true if the edge has been modified according to changed polygon.
    fn new_polygon(&self, edge_id: u32) -> Option<Vec<f64>> {
        None
    }

    /// Return true if the edge has been modified according to changed polygon on triangulation.
    fn new_polygon_on_triangulation(&self, edge_id: u32, face_id: u32) -> Option<Vec<u32>> {
        None
    }
}

/// Default implementation of BRepTools_Modification.
pub struct BRepToolsModificationImpl;

impl BRepToolsModification for BRepToolsModificationImpl {
    fn new_surface(&self, _face_id: u32) -> Option<(String, String, f64, bool, bool)> {
        None
    }

    fn new_curve(&self, _edge_id: u32) -> Option<(String, String, f64)> {
        None
    }

    fn new_point(&self, _vertex_id: u32) -> Option<(f64, f64, f64, f64)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modification_impl() {
        let m = BRepToolsModificationImpl;
        assert_eq!(m.new_surface(1), None);
        assert_eq!(m.new_curve(1), None);
        assert_eq!(m.new_point(1), None);
        assert_eq!(m.new_triangulation(1), None);
        assert_eq!(m.new_polygon(1), None);
        assert_eq!(m.new_polygon_on_triangulation(1, 1), None);
    }

    #[test]
    fn test_modification_trait() {
        let m: &dyn BRepToolsModification = &BRepToolsModificationImpl;
        assert_eq!(m.new_surface(1), None);
    }
}
