// FILE: expr_invalid_assignment.rs
// occt: Expr_InvalidAssignment

use std::fmt;

/// Exception raised for invalid variable assignment.
#[derive(Debug, Clone)]
pub struct ExprInvalidAssignment {
    message: String,
}

impl ExprInvalidAssignment {
    /// Create a new invalid assignment error
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ExprInvalidAssignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expr_InvalidAssignment: {}", self.message)
    }
}

impl std::error::Error for ExprInvalidAssignment {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let err = ExprInvalidAssignment::new("bad assignment");
        assert_eq!(err.message(), "bad assignment");
    }

    #[test]
    fn test_display() {
        let err = ExprInvalidAssignment::new("test");
        let s = format!("{}", err);
        assert!(s.contains("InvalidAssignment"));
    }

    #[test]
    fn test_clone() {
        let err1 = ExprInvalidAssignment::new("msg");
        let err2 = err1.clone();
        assert_eq!(err1.message(), err2.message());
    }
}
