// FILE: std_fail_infinite_solutions.rs
// occt: StdFail_InfiniteSolutions

//! Exception thrown when a problem has infinite solutions.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StdFailInfiniteSolutions {
    message: String,
}

impl StdFailInfiniteSolutions {
    pub fn new(message: impl Into<String>) -> Self {
        StdFailInfiniteSolutions {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "StdFail_InfiniteSolutions"
    }
}

impl fmt::Display for StdFailInfiniteSolutions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StdFailInfiniteSolutions {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infinite_solutions_creation() {
        let err = StdFailInfiniteSolutions::new("Infinite solutions");
        assert_eq!(err.message(), "Infinite solutions");
    }

    #[test]
    fn test_infinite_solutions_display() {
        let err = StdFailInfiniteSolutions::new("Problem has many solutions");
        assert_eq!(
            err.to_string(),
            "StdFail_InfiniteSolutions: Problem has many solutions"
        );
    }

    #[test]
    fn test_infinite_solutions_clone() {
        let err = StdFailInfiniteSolutions::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
