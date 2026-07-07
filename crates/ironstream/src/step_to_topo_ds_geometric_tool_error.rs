// FILE: step_to_topo_ds_geometric_tool_error.rs
// occt: StepToTopoDS_GeometricToolError

/// Enumeration of geometric tool error states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometricToolError {
    /// Operation completed successfully
    Done = 0,
    /// Curve is degenerated
    IsDegenerated = 1,
    /// No PCurve found
    HasNoPCurve = 2,
    /// Wrong 3D parameters
    Wrong3dParameters = 3,
    /// No projection on curve
    NoProjectionOnCurve = 4,
    /// Other/unspecified error
    Other = 5,
}

impl GeometricToolError {
    /// Check if the error indicates success
    pub fn is_done(&self) -> bool {
        *self == GeometricToolError::Done
    }

    /// Get a human-readable description of the error
    pub fn description(&self) -> &'static str {
        match self {
            GeometricToolError::Done => "Geometric tool operation completed successfully",
            GeometricToolError::IsDegenerated => "Curve is degenerated",
            GeometricToolError::HasNoPCurve => "No PCurve found",
            GeometricToolError::Wrong3dParameters => "Wrong 3D parameters",
            GeometricToolError::NoProjectionOnCurve => "No projection on curve",
            GeometricToolError::Other => "Geometric tool operation failed with other error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometric_tool_error_done() {
        let error = GeometricToolError::Done;
        assert!(error.is_done());
        assert_eq!(error as i32, 0);
    }

    #[test]
    fn test_geometric_tool_error_is_degenerated() {
        let error = GeometricToolError::IsDegenerated;
        assert!(!error.is_done());
        assert_eq!(error as i32, 1);
    }

    #[test]
    fn test_geometric_tool_error_has_no_pcurve() {
        let error = GeometricToolError::HasNoPCurve;
        assert!(!error.is_done());
        assert_eq!(error as i32, 2);
    }

    #[test]
    fn test_geometric_tool_error_wrong_3d_parameters() {
        let error = GeometricToolError::Wrong3dParameters;
        assert!(!error.is_done());
        assert_eq!(error as i32, 3);
    }

    #[test]
    fn test_geometric_tool_error_no_projection() {
        let error = GeometricToolError::NoProjectionOnCurve;
        assert!(!error.is_done());
        assert_eq!(error as i32, 4);
    }

    #[test]
    fn test_geometric_tool_error_other() {
        let error = GeometricToolError::Other;
        assert!(!error.is_done());
        assert_eq!(error as i32, 5);
    }

    #[test]
    fn test_geometric_tool_error_description() {
        assert_eq!(
            GeometricToolError::Done.description(),
            "Geometric tool operation completed successfully"
        );
        assert_eq!(
            GeometricToolError::IsDegenerated.description(),
            "Curve is degenerated"
        );
        assert_eq!(
            GeometricToolError::HasNoPCurve.description(),
            "No PCurve found"
        );
        assert_eq!(
            GeometricToolError::Wrong3dParameters.description(),
            "Wrong 3D parameters"
        );
        assert_eq!(
            GeometricToolError::NoProjectionOnCurve.description(),
            "No projection on curve"
        );
        assert_eq!(
            GeometricToolError::Other.description(),
            "Geometric tool operation failed with other error"
        );
    }

    #[test]
    fn test_geometric_tool_error_equality() {
        let error1 = GeometricToolError::Done;
        let error2 = GeometricToolError::Done;
        assert_eq!(error1, error2);

        let error3 = GeometricToolError::Other;
        assert_ne!(error1, error3);
    }
}
