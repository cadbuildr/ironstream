// FILE: std_fail_undefined_value.rs
// occt: StdFail_UndefinedValue

//! Exception thrown when a value is undefined.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StdFailUndefinedValue {
    message: String,
}

impl StdFailUndefinedValue {
    pub fn new(message: impl Into<String>) -> Self {
        StdFailUndefinedValue {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "StdFail_UndefinedValue"
    }
}

impl fmt::Display for StdFailUndefinedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StdFailUndefinedValue {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undefined_value_creation() {
        let err = StdFailUndefinedValue::new("Value undefined");
        assert_eq!(err.message(), "Value undefined");
    }

    #[test]
    fn test_undefined_value_display() {
        let err = StdFailUndefinedValue::new("Cannot compute value");
        assert_eq!(
            err.to_string(),
            "StdFail_UndefinedValue: Cannot compute value"
        );
    }

    #[test]
    fn test_undefined_value_clone() {
        let err = StdFailUndefinedValue::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
