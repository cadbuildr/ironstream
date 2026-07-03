// FILE: standard_type_mismatch.rs
// occt: Standard_TypeMismatch

//! Exception thrown when incompatible types are used together.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StandardTypeMismatch {
    message: String,
}

impl StandardTypeMismatch {
    pub fn new(message: impl Into<String>) -> Self {
        StandardTypeMismatch {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_TypeMismatch"
    }
}

impl fmt::Display for StandardTypeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardTypeMismatch {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_mismatch_creation() {
        let err = StandardTypeMismatch::new("Type mismatch");
        assert_eq!(err.message(), "Type mismatch");
    }

    #[test]
    fn test_type_mismatch_display() {
        let err = StandardTypeMismatch::new("Expected int, got string");
        assert_eq!(
            err.to_string(),
            "Standard_TypeMismatch: Expected int, got string"
        );
    }

    #[test]
    fn test_type_mismatch_clone() {
        let err = StandardTypeMismatch::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
