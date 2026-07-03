// FILE: std_fail_undefined_derivative.rs
// occt: StdFail_UndefinedDerivative

//! Exception thrown when a derivative is undefined.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StdFailUndefinedDerivative {
    message: String,
}

impl StdFailUndefinedDerivative {
    pub fn new(message: impl Into<String>) -> Self {
        StdFailUndefinedDerivative {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "StdFail_UndefinedDerivative"
    }
}

impl fmt::Display for StdFailUndefinedDerivative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StdFailUndefinedDerivative {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undefined_derivative_creation() {
        let err = StdFailUndefinedDerivative::new("Derivative undefined");
        assert_eq!(err.message(), "Derivative undefined");
    }

    #[test]
    fn test_undefined_derivative_display() {
        let err = StdFailUndefinedDerivative::new("Cannot compute derivative");
        assert_eq!(
            err.to_string(),
            "StdFail_UndefinedDerivative: Cannot compute derivative"
        );
    }

    #[test]
    fn test_undefined_derivative_clone() {
        let err = StdFailUndefinedDerivative::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
