// FILE: standard_dimension_mismatch.rs
// occt: Standard_DimensionMismatch

use crate::standard_domain_error::StandardDomainError;

/// Exception class for dimension mismatch errors
#[derive(Debug, Clone)]
pub struct StandardDimensionMismatch {
    error: StandardDomainError,
}

impl StandardDimensionMismatch {
    /// Creates a new StandardDimensionMismatch exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        StandardDimensionMismatch {
            error: StandardDomainError::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.error.message()
    }
}

impl std::fmt::Display for StandardDimensionMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Standard_DimensionMismatch: {}", self.message())
    }
}

impl std::error::Error for StandardDimensionMismatch {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_dimension_mismatch() {
        let err = StandardDimensionMismatch::new("array sizes don't match");
        assert_eq!(err.message(), "array sizes don't match");
    }
}
