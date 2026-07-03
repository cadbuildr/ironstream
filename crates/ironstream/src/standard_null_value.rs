// FILE: standard_null_value.rs
// occt: Standard_NullValue

//! Exception thrown when a null value is encountered unexpectedly.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StandardNullValue {
    message: String,
}

impl StandardNullValue {
    pub fn new(message: impl Into<String>) -> Self {
        StandardNullValue {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_NullValue"
    }
}

impl fmt::Display for StandardNullValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardNullValue {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_value_creation() {
        let err = StandardNullValue::new("Unexpected null value");
        assert_eq!(err.message(), "Unexpected null value");
    }

    #[test]
    fn test_null_value_display() {
        let err = StandardNullValue::new("Null pointer");
        assert_eq!(err.to_string(), "Standard_NullValue: Null pointer");
    }

    #[test]
    fn test_null_value_clone() {
        let err = StandardNullValue::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
