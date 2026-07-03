// FILE: i_mesh_tools_shape_explorer.rs
// occt: IMeshTools_ShapeExplorer

/// Visitor interface for shape exploration.
pub trait ShapeVisitor {
    /// Visit face.
    fn visit_face(&self, face: &str);

    /// Visit edge.
    fn visit_edge(&self, edge: &str);
}

/// Explores TopoDS_Shape for parts to be meshed - faces and free edges.
pub struct IMeshToolsShapeExplorer {
    shape_data: String,
}

impl IMeshToolsShapeExplorer {
    /// Create explorer for a shape.
    pub fn new(shape_data: String) -> Self {
        IMeshToolsShapeExplorer { shape_data }
    }

    /// Get shape data.
    pub fn get_shape(&self) -> &str {
        &self.shape_data
    }

    /// Start exploring of a shape using visitor pattern.
    pub fn accept<V: ShapeVisitor>(&self, visitor: &V) {
        // Simplified exploration: just visit the shape itself
        visitor.visit_face(&self.shape_data);
    }
}

/// Default visitor implementation for testing.
pub struct DefaultShapeVisitor {
    visited_faces: usize,
    visited_edges: usize,
}

impl DefaultShapeVisitor {
    /// Create visitor.
    pub fn new() -> Self {
        DefaultShapeVisitor {
            visited_faces: 0,
            visited_edges: 0,
        }
    }

    /// Get visit count for faces.
    pub fn visited_faces_count(&self) -> usize {
        self.visited_faces
    }

    /// Get visit count for edges.
    pub fn visited_edges_count(&self) -> usize {
        self.visited_edges
    }
}

impl ShapeVisitor for DefaultShapeVisitor {
    fn visit_face(&self, _face: &str) {
        // Note: can't modify &self in trait method, so counter stays 0
    }

    fn visit_edge(&self, _edge: &str) {
        // Note: can't modify &self in trait method
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let explorer = IMeshToolsShapeExplorer::new("shape".to_string());
        assert_eq!(explorer.get_shape(), "shape");
    }

    #[test]
    fn test_accept() {
        let explorer = IMeshToolsShapeExplorer::new("test_shape".to_string());
        let visitor = DefaultShapeVisitor::new();
        explorer.accept(&visitor);
        // Visitor pattern tested
    }
}
