// FILE: topo_ds_to_step_builder_error.rs
// occt: TopoDSToStep_BuilderError

/// Error enumeration for TopoDSToStep builder operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuilderError {
    BuilderDone,
    NoFaceMapped,
    BuilderOther,
}

impl BuilderError {
    /// Returns the error code as an integer.
    pub fn code(&self) -> i32 {
        match self {
            BuilderError::BuilderDone => 0,
            BuilderError::NoFaceMapped => 1,
            BuilderError::BuilderOther => 2,
        }
    }

    /// Returns a description of the error.
    pub fn description(&self) -> &'static str {
        match self {
            BuilderError::BuilderDone => "Builder operation succeeded",
            BuilderError::NoFaceMapped => "No face was mapped",
            BuilderError::BuilderOther => "Other builder error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code() {
        assert_eq!(BuilderError::BuilderDone.code(), 0);
        assert_eq!(BuilderError::NoFaceMapped.code(), 1);
        assert_eq!(BuilderError::BuilderOther.code(), 2);
    }

    #[test]
    fn test_error_description() {
        assert_eq!(BuilderError::BuilderDone.description(), "Builder operation succeeded");
        assert_eq!(BuilderError::NoFaceMapped.description(), "No face was mapped");
    }

    #[test]
    fn test_equality() {
        assert_eq!(BuilderError::BuilderDone, BuilderError::BuilderDone);
        assert_ne!(BuilderError::BuilderDone, BuilderError::NoFaceMapped);
    }
}
