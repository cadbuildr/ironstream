// FILE: standard_negative_value.rs
// occt: Standard_NegativeValue

//! Exception thrown when a negative value is encountered unexpectedly.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StandardNegativeValue {
    message: String,
}

impl StandardNegativeValue {
    pub fn new(message: impl Into<String>) -> Self {
        StandardNegativeValue {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "Standard_NegativeValue"
    }
}

impl fmt::Display for StandardNegativeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StandardNegativeValue {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_value_creation() {
        let err = StandardNegativeValue::new("Negative value not allowed");
        assert_eq!(err.message(), "Negative value not allowed");
    }

    #[test]
    fn test_negative_value_display() {
        let err = StandardNegativeValue::new("-5");
        assert_eq!(err.to_string(), "Standard_NegativeValue: -5");
    }

    #[test]
    fn test_negative_value_clone() {
        let err = StandardNegativeValue::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
