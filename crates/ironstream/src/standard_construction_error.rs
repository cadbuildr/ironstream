// FILE: standard_construction_error.rs
// occt: Standard_ConstructionError

use crate::standard_domain_error::StandardDomainError;

/// Exception class for construction errors
#[derive(Debug, Clone)]
pub struct StandardConstructionError {
    error: StandardDomainError,
}

impl StandardConstructionError {
    /// Creates a new StandardConstructionError exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        StandardConstructionError {
            error: StandardDomainError::new(message),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        self.error.message()
    }
}

impl std::fmt::Display for StandardConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Standard_ConstructionError: {}", self.message())
    }
}

impl std::error::Error for StandardConstructionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_construction_error() {
        let err = StandardConstructionError::new("bad construction");
        assert_eq!(err.message(), "bad construction");
    }

    #[test]
    fn test_standard_construction_error_display() {
        let err = StandardConstructionError::new("invalid parameters");
        assert_eq!(err.to_string(), "Standard_ConstructionError: invalid parameters");
    }
}
