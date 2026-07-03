// FILE: b_rep_offset_error.rs
// occt: BRepOffset_Error

/// Error codes for B-rep offset operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BRepOffsetError {
    /// No error
    NoError,
    /// Unknown error
    UnknownError,
    /// Bad normals on geometry
    BadNormalsOnGeometry,
    /// Geometry is C0 (not C1 continuous)
    C0Geometry,
    /// Null offset value
    NullOffset,
    /// Shell is not connected
    NotConnectedShell,
    /// Exception while trimming edges
    CannotTrimEdges,
    /// Exception while fusing vertices
    CannotFuseVertices,
    /// Exception while extending edges
    CannotExtentEdge,
    /// User break requested
    UserBreak,
    /// Mixed connectivity: partially C0 and tangent along edge
    MixedConnectivity,
}

impl BRepOffsetError {
    /// Returns true if this indicates success
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::NoError)
    }

    /// Returns true if this indicates an error
    pub fn is_error(&self) -> bool {
        !self.is_ok()
    }
}

impl Default for BRepOffsetError {
    fn default() -> Self {
        Self::NoError
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ok() {
        assert!(BRepOffsetError::NoError.is_ok());
        assert!(!BRepOffsetError::UnknownError.is_ok());
        assert!(!BRepOffsetError::C0Geometry.is_ok());
    }

    #[test]
    fn test_is_error() {
        assert!(BRepOffsetError::CannotTrimEdges.is_error());
        assert!(BRepOffsetError::UserBreak.is_error());
        assert!(!BRepOffsetError::NoError.is_error());
    }

    #[test]
    fn test_default() {
        assert_eq!(BRepOffsetError::default(), BRepOffsetError::NoError);
    }

    #[test]
    fn test_copy_semantics() {
        let err = BRepOffsetError::MixedConnectivity;
        let err2 = err;
        assert_eq!(err, err2);
    }
}
