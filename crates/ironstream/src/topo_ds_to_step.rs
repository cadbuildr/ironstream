// FILE: topo_ds_to_step.rs
// occt: TopoDSToStep

/// Static utility functions for TopoDSToStep conversion.
pub struct TopoDSToStep;

impl TopoDSToStep {
    /// Decodes a BuilderError to a string message.
    pub fn decode_builder_error(err: &BuilderError) -> String {
        match err {
            BuilderError::BuilderDone => "BuilderDone".to_string(),
            BuilderError::NoFaceMapped => "NoFaceMapped".to_string(),
            BuilderError::BuilderOther => "BuilderOther".to_string(),
        }
    }

    /// Decodes a FaceError to a string message.
    pub fn decode_face_error(err: &FaceError) -> String {
        match err {
            FaceError::FaceDone => "FaceDone".to_string(),
            FaceError::InfiniteFace => "InfiniteFace".to_string(),
            FaceError::NonManifoldFace => "NonManifoldFace".to_string(),
            FaceError::NoWireMapped => "NoWireMapped".to_string(),
            FaceError::FaceOther => "FaceOther".to_string(),
        }
    }

    /// Decodes a WireError to a string message.
    pub fn decode_wire_error(err: &WireError) -> String {
        match err {
            WireError::WireDone => "WireDone".to_string(),
            WireError::NonManifoldWire => "NonManifoldWire".to_string(),
            WireError::WireOther => "WireOther".to_string(),
        }
    }

    /// Decodes an EdgeError to a string message.
    pub fn decode_edge_error(err: &EdgeError) -> String {
        match err {
            EdgeError::EdgeDone => "EdgeDone".to_string(),
            EdgeError::NonManifoldEdge => "NonManifoldEdge".to_string(),
            EdgeError::EdgeOther => "EdgeOther".to_string(),
        }
    }

    /// Decodes a VertexError to a string message.
    pub fn decode_vertex_error(err: &VertexError) -> String {
        match err {
            VertexError::VertexDone => "VertexDone".to_string(),
            VertexError::VertexOther => "VertexOther".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BuilderError {
    BuilderDone,
    NoFaceMapped,
    BuilderOther,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FaceError {
    FaceDone,
    InfiniteFace,
    NonManifoldFace,
    NoWireMapped,
    FaceOther,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WireError {
    WireDone,
    NonManifoldWire,
    WireOther,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EdgeError {
    EdgeDone,
    NonManifoldEdge,
    EdgeOther,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VertexError {
    VertexDone,
    VertexOther,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_builder_error() {
        assert_eq!(
            TopoDSToStep::decode_builder_error(&BuilderError::BuilderDone),
            "BuilderDone"
        );
        assert_eq!(
            TopoDSToStep::decode_builder_error(&BuilderError::NoFaceMapped),
            "NoFaceMapped"
        );
    }

    #[test]
    fn test_decode_face_error() {
        assert_eq!(
            TopoDSToStep::decode_face_error(&FaceError::FaceDone),
            "FaceDone"
        );
        assert_eq!(
            TopoDSToStep::decode_face_error(&FaceError::InfiniteFace),
            "InfiniteFace"
        );
    }

    #[test]
    fn test_decode_edge_error() {
        assert_eq!(
            TopoDSToStep::decode_edge_error(&EdgeError::EdgeDone),
            "EdgeDone"
        );
    }
}
