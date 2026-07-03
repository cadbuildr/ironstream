// FILE: standard_range_error.rs
// occt: Standard_RangeError

//! Exception thrown when a value is out of range.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StandardRangeError {
    message: String,
}

impl StandardRangeError {
    pub fn new(message: impl Into<String>) -> Self {
        StandardRangeError {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_RangeError"
    }
}

impl fmt::Display for StandardRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardRangeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_error_creation() {
        let err = StandardRangeError::new("Value out of range");
        assert_eq!(err.message(), "Value out of range");
    }

    #[test]
    fn test_range_error_display() {
        let err = StandardRangeError::new("Index 10 beyond bounds");
        assert_eq!(err.to_string(), "Standard_RangeError: Index 10 beyond bounds");
    }

    #[test]
    fn test_range_error_clone() {
        let err = StandardRangeError::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
