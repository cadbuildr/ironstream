// FILE: standard_multiply_defined.rs
// occt: Standard_MultiplyDefined

//! Exception thrown when an object is defined multiple times.
//! Inherits from Standard_DomainError.

use std::fmt;

/// Exception thrown when an object is defined multiple times.
/// Inherits from Standard_DomainError.
#[derive(Debug, Clone)]
pub struct StandardMultiplyDefined {
    message: String,
}

impl StandardMultiplyDefined {
    pub fn new(message: impl Into<String>) -> Self {
        StandardMultiplyDefined {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_MultiplyDefined"
    }
}

impl fmt::Display for StandardMultiplyDefined {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardMultiplyDefined {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_defined_creation() {
        let err = StandardMultiplyDefined::new("Object defined twice");
        assert_eq!(err.message(), "Object defined twice");
        assert_eq!(err.exception_type(), "Standard_MultiplyDefined");
    }

    #[test]
    fn test_multiply_defined_display() {
        let err = StandardMultiplyDefined::new("Duplicate key");
        assert_eq!(err.to_string(), "Standard_MultiplyDefined: Duplicate key");
    }

    #[test]
    fn test_multiply_defined_error_trait() {
        let err = StandardMultiplyDefined::new("Defined");
        let _e: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_multiply_defined_clone() {
        let err1 = StandardMultiplyDefined::new("Original");
        let err2 = err1.clone();
        assert_eq!(err1.message(), err2.message());
    }
}
