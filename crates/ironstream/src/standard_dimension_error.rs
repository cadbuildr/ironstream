// FILE: standard_dimension_error.rs
// occt: Standard_DimensionError

use crate::standard_domain_error::StandardDomainError;

/// Exception class for dimension errors
#[derive(Debug, Clone)]
pub struct StandardDimensionError {
    error: StandardDomainError,
}

impl StandardDimensionError {
    /// Creates a new StandardDimensionError exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        StandardDimensionError {
            error: StandardDomainError::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.error.message()
    }
}

impl std::fmt::Display for StandardDimensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Standard_DimensionError: {}", self.message())
    }
}

impl std::error::Error for StandardDimensionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_dimension_error() {
        let err = StandardDimensionError::new("dimension mismatch");
        assert_eq!(err.message(), "dimension mismatch");
    }
}
