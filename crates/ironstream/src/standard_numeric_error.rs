// FILE: standard_numeric_error.rs
// occt: Standard_NumericError

//! Exception thrown for numeric computation errors.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StandardNumericError {
    message: String,
}

impl StandardNumericError {
    pub fn new(message: impl Into<String>) -> Self {
        StandardNumericError {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_NumericError"
    }
}

impl fmt::Display for StandardNumericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardNumericError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_error_creation() {
        let err = StandardNumericError::new("Numeric computation failed");
        assert_eq!(err.message(), "Numeric computation failed");
    }

    #[test]
    fn test_numeric_error_display() {
        let err = StandardNumericError::new("Division by zero");
        assert_eq!(err.to_string(), "Standard_NumericError: Division by zero");
    }

    #[test]
    fn test_numeric_error_clone() {
        let err = StandardNumericError::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
