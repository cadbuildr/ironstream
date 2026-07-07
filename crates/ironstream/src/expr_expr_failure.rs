// FILE: expr_expr_failure.rs
// occt: Expr_ExprFailure

use std::fmt;

/// Exception raised when an expression evaluation fails.
#[derive(Debug, Clone)]
pub struct ExprExprFailure {
    message: String,
}

impl ExprExprFailure {
    /// Create a new failure with the given message
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

impl fmt::Display for ExprExprFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expr_ExprFailure: {}", self.message)
    }
}

impl std::error::Error for ExprExprFailure {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_failure() {
        let err = ExprExprFailure::new("evaluation error");
        assert_eq!(err.message(), "evaluation error");
    }

    #[test]
    fn test_display() {
        let err = ExprExprFailure::new("test failure");
        let s = format!("{}", err);
        assert!(s.contains("Expr_ExprFailure"));
        assert!(s.contains("test failure"));
    }

    #[test]
    fn test_error_trait() {
        let err = ExprExprFailure::new("fail");
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_clone() {
        let err1 = ExprExprFailure::new("msg");
        let err2 = err1.clone();
        assert_eq!(err1.message(), err2.message());
    }
}
