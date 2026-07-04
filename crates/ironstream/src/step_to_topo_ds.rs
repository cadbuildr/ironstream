// FILE: step_to_topo_ds.rs
// occt: StepToTopoDS

use std::sync::Arc;

/// Error types for topology conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuilderError {
    NotDone = 0,
    VertexError = 1,
    EdgeError = 2,
    WireError = 3,
    FaceError = 4,
    ShellError = 5,
    SolidError = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellError {
    NotDone = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaceError {
    NotDone = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeError {
    NotDone = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexError {
    NotDone = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexLoopError {
    NotDone = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolyLoopError {
    NotDone = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometricToolError {
    NotDone = 0,
}

/// This module implements the mapping between AP214
/// Shape representation and OpenCascade Shape Representation.
pub struct StepToTopoDS;

impl StepToTopoDS {
    /// Decode a BuilderError to a human-readable error message
    pub fn decode_builder_error(error: BuilderError) -> Arc<str> {
        match error {
            BuilderError::NotDone => Arc::from("Builder error: Not done"),
            BuilderError::VertexError => Arc::from("Builder error: Vertex"),
            BuilderError::EdgeError => Arc::from("Builder error: Edge"),
            BuilderError::WireError => Arc::from("Builder error: Wire"),
            BuilderError::FaceError => Arc::from("Builder error: Face"),
            BuilderError::ShellError => Arc::from("Builder error: Shell"),
            BuilderError::SolidError => Arc::from("Builder error: Solid"),
        }
    }

    /// Decode a ShellError to a human-readable error message
    pub fn decode_shell_error(error: ShellError) -> Arc<str> {
        match error {
            ShellError::NotDone => Arc::from("Shell error: Not done"),
        }
    }

    /// Decode a FaceError to a human-readable error message
    pub fn decode_face_error(error: FaceError) -> Arc<str> {
        match error {
            FaceError::NotDone => Arc::from("Face error: Not done"),
        }
    }

    /// Decode an EdgeError to a human-readable error message
    pub fn decode_edge_error(error: EdgeError) -> Arc<str> {
        match error {
            EdgeError::NotDone => Arc::from("Edge error: Not done"),
        }
    }

    /// Decode a VertexError to a human-readable error message
    pub fn decode_vertex_error(error: VertexError) -> Arc<str> {
        match error {
            VertexError::NotDone => Arc::from("Vertex error: Not done"),
        }
    }

    /// Decode a VertexLoopError to a human-readable error message
    pub fn decode_vertex_loop_error(error: VertexLoopError) -> Arc<str> {
        match error {
            VertexLoopError::NotDone => Arc::from("VertexLoop error: Not done"),
        }
    }

    /// Decode a PolyLoopError to a human-readable error message
    pub fn decode_poly_loop_error(error: PolyLoopError) -> Arc<str> {
        match error {
            PolyLoopError::NotDone => Arc::from("PolyLoop error: Not done"),
        }
    }

    /// Decode a GeometricToolError to a human-readable error message
    pub fn decode_geometric_tool_error(error: GeometricToolError) -> &'static str {
        match error {
            GeometricToolError::NotDone => "GeometricTool error: Not done",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_builder_error_not_done() {
        let msg = StepToTopoDS::decode_builder_error(BuilderError::NotDone);
        assert_eq!(msg.as_ref(), "Builder error: Not done");
    }

    #[test]
    fn test_decode_builder_error_vertex() {
        let msg = StepToTopoDS::decode_builder_error(BuilderError::VertexError);
        assert_eq!(msg.as_ref(), "Builder error: Vertex");
    }

    #[test]
    fn test_decode_builder_error_edge() {
        let msg = StepToTopoDS::decode_builder_error(BuilderError::EdgeError);
        assert_eq!(msg.as_ref(), "Builder error: Edge");
    }

    #[test]
    fn test_decode_shell_error() {
        let msg = StepToTopoDS::decode_shell_error(ShellError::NotDone);
        assert_eq!(msg.as_ref(), "Shell error: Not done");
    }

    #[test]
    fn test_decode_face_error() {
        let msg = StepToTopoDS::decode_face_error(FaceError::NotDone);
        assert_eq!(msg.as_ref(), "Face error: Not done");
    }

    #[test]
    fn test_decode_edge_error() {
        let msg = StepToTopoDS::decode_edge_error(EdgeError::NotDone);
        assert_eq!(msg.as_ref(), "Edge error: Not done");
    }

    #[test]
    fn test_decode_vertex_error() {
        let msg = StepToTopoDS::decode_vertex_error(VertexError::NotDone);
        assert_eq!(msg.as_ref(), "Vertex error: Not done");
    }

    #[test]
    fn test_decode_vertex_loop_error() {
        let msg = StepToTopoDS::decode_vertex_loop_error(VertexLoopError::NotDone);
        assert_eq!(msg.as_ref(), "VertexLoop error: Not done");
    }

    #[test]
    fn test_decode_poly_loop_error() {
        let msg = StepToTopoDS::decode_poly_loop_error(PolyLoopError::NotDone);
        assert_eq!(msg.as_ref(), "PolyLoop error: Not done");
    }

    #[test]
    fn test_decode_geometric_tool_error() {
        let msg = StepToTopoDS::decode_geometric_tool_error(GeometricToolError::NotDone);
        assert_eq!(msg, "GeometricTool error: Not done");
    }
}
