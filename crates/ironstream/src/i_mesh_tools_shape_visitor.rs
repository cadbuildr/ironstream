// FILE: i_mesh_tools_shape_visitor.rs
// occt: IMeshTools_ShapeVisitor

/// Interface class for shape visitor.
pub trait IMeshToolsShapeVisitor {
    /// Handles face object.
    fn visit_face(&self, face: &str);

    /// Handles edge object.
    fn visit_edge(&self, edge: &str);
}

/// Default visitor implementation for testing.
pub struct DefaultIMeshToolsShapeVisitor;

impl IMeshToolsShapeVisitor for DefaultIMeshToolsShapeVisitor {
    fn visit_face(&self, _face: &str) {
        // No-op default implementation
    }

    fn visit_edge(&self, _edge: &str) {
        // No-op default implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visitor_trait() {
        let visitor = DefaultIMeshToolsShapeVisitor;
        visitor.visit_face("face1");
        visitor.visit_edge("edge1");
    }
}
