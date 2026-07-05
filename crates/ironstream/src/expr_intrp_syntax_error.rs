// FILE: expr_intrp_syntax_error.rs
// occt: ExprIntrp_SyntaxError

use std::fmt;

/// Exception raised when a syntax error occurs in expression parsing.
#[derive(Debug, Clone)]
pub struct ExprIntrpSyntaxError {
    message: String,
}

impl ExprIntrpSyntaxError {
    /// Create a new ExprIntrp_SyntaxError with the given message
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

impl fmt::Display for ExprIntrpSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExprIntrp_SyntaxError: {}", self.message)
    }
}

impl std::error::Error for ExprIntrpSyntaxError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_syntax_error() {
        let err = ExprIntrpSyntaxError::new("invalid syntax");
        assert_eq!(err.message(), "invalid syntax");
    }

    #[test]
    fn test_display_error() {
        let err = ExprIntrpSyntaxError::new("parse failed");
        let display_str = format!("{}", err);
        assert!(display_str.contains("ExprIntrp_SyntaxError"));
        assert!(display_str.contains("parse failed"));
    }

    #[test]
    fn test_error_trait() {
        let err = ExprIntrpSyntaxError::new("test error");
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_clone_syntax_error() {
        let err1 = ExprIntrpSyntaxError::new("test");
        let err2 = err1.clone();
        assert_eq!(err1.message(), err2.message());
    }
}
