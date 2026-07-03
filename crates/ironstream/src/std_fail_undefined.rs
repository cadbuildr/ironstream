// FILE: std_fail_undefined.rs
// occt: StdFail_Undefined

//! Exception thrown when a value is undefined.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StdFailUndefined {
    message: String,
}

impl StdFailUndefined {
    pub fn new(message: impl Into<String>) -> Self {
        StdFailUndefined {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "StdFail_Undefined"
    }
}

impl fmt::Display for StdFailUndefined {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StdFailUndefined {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undefined_creation() {
        let err = StdFailUndefined::new("Value is undefined");
        assert_eq!(err.message(), "Value is undefined");
    }

    #[test]
    fn test_undefined_display() {
        let err = StdFailUndefined::new("Undefined result");
        assert_eq!(err.to_string(), "StdFail_Undefined: Undefined result");
    }

    #[test]
    fn test_undefined_clone() {
        let err = StdFailUndefined::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
