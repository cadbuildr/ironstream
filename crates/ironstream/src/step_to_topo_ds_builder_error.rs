// FILE: step_to_topo_ds_builder_error.rs
// occt: StepToTopoDS_BuilderError

/// Enumeration of builder error states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuilderError {
    /// Operation completed successfully
    Done = 0,
    /// Other/unspecified error occurred
    Other = 1,
}

impl BuilderError {
    /// Check if the error indicates success
    pub fn is_done(&self) -> bool {
        *self == BuilderError::Done
    }

    /// Get a human-readable description of the error
    pub fn description(&self) -> &'static str {
        match self {
            BuilderError::Done => "Builder operation completed successfully",
            BuilderError::Other => "Builder operation failed with other error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_error_done() {
        let error = BuilderError::Done;
        assert!(error.is_done());
        assert_eq!(error as i32, 0);
    }

    #[test]
    fn test_builder_error_other() {
        let error = BuilderError::Other;
        assert!(!error.is_done());
        assert_eq!(error as i32, 1);
    }

    #[test]
    fn test_builder_error_description() {
        let done_error = BuilderError::Done;
        assert_eq!(
            done_error.description(),
            "Builder operation completed successfully"
        );

        let other_error = BuilderError::Other;
        assert_eq!(
            other_error.description(),
            "Builder operation failed with other error"
        );
    }

    #[test]
    fn test_builder_error_equality() {
        let error1 = BuilderError::Done;
        let error2 = BuilderError::Done;
        assert_eq!(error1, error2);

        let error3 = BuilderError::Other;
        assert_ne!(error1, error3);
    }
}
