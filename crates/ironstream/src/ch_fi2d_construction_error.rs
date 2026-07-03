// FILE: ch_fi2d_construction_error.rs
// occt: ChFi2d_ConstructionError

/// Error codes that can occur during fillet construction on planar wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChFi2dConstructionError {
    /// the face is not planar
    NotPlanar,
    /// the face is null
    NoFace,
    /// the two faces used for the initialisation are uncompatible
    InitialisationError,
    /// the parameters as distances or angle for chamfer are less or equal to zero
    ParametersError,
    /// the initialization has been successful
    Ready,
    /// the construction is done
    IsDone,
    /// the algorithm could not find a solution
    ComputationError,
    /// the vertex given to locate the fillet or the chamfer is not connected to 2 edges
    ConnexionError,
    /// the two edges connected to the vertex are tangent
    TangencyError,
    /// the first edge is degenerated
    FirstEdgeDegenerated,
    /// the last edge is degenerated
    LastEdgeDegenerated,
    /// the two edges are degenerated
    BothEdgesDegenerated,
    /// One or the two edges connected to the vertex is a fillet or a chamfer;
    /// One or the two edges connected to the vertex is not a line or a circle
    NotAuthorized,
}

impl ChFi2dConstructionError {
    /// Returns true if the status indicates success
    pub fn is_done(&self) -> bool {
        matches!(self, Self::IsDone | Self::Ready)
    }

    /// Returns true if the status indicates an error
    pub fn is_error(&self) -> bool {
        matches!(
            self,
            Self::NotPlanar
                | Self::NoFace
                | Self::InitialisationError
                | Self::ParametersError
                | Self::ComputationError
                | Self::ConnexionError
                | Self::TangencyError
                | Self::NotAuthorized
        )
    }
}

impl Default for ChFi2dConstructionError {
    fn default() -> Self {
        Self::NotPlanar
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_done() {
        assert!(ChFi2dConstructionError::IsDone.is_done());
        assert!(ChFi2dConstructionError::Ready.is_done());
        assert!(!ChFi2dConstructionError::NotPlanar.is_done());
    }

    #[test]
    fn test_is_error() {
        assert!(ChFi2dConstructionError::NotPlanar.is_error());
        assert!(ChFi2dConstructionError::ComputationError.is_error());
        assert!(!ChFi2dConstructionError::Ready.is_error());
    }

    #[test]
    fn test_default() {
        let err = ChFi2dConstructionError::default();
        assert_eq!(err, ChFi2dConstructionError::NotPlanar);
    }

    #[test]
    fn test_copy_trait() {
        let err = ChFi2dConstructionError::IsDone;
        let err2 = err;
        assert_eq!(err, err2);
    }
}
