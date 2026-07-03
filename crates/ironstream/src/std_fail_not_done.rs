// FILE: std_fail_not_done.rs
// occt: StdFail_NotDone

//! Exception thrown when an operation has not been computed.

use std::fmt;

#[derive(Debug, Clone)]
pub struct StdFailNotDone {
    message: String,
}

impl StdFailNotDone {
    pub fn new(message: impl Into<String>) -> Self {
        StdFailNotDone {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exception_type(&self) -> &str {
        "StdFail_NotDone"
    }
}

impl fmt::Display for StdFailNotDone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for StdFailNotDone {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_done_creation() {
        let err = StdFailNotDone::new("Operation not done");
        assert_eq!(err.message(), "Operation not done");
    }

    #[test]
    fn test_not_done_display() {
        let err = StdFailNotDone::new("Computation failed");
        assert_eq!(err.to_string(), "StdFail_NotDone: Computation failed");
    }

    #[test]
    fn test_not_done_clone() {
        let err = StdFailNotDone::new("Test");
        let err2 = err.clone();
        assert_eq!(err.message(), err2.message());
    }
}
