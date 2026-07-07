// FILE: expr_invalid_function.rs
// occt: Expr_InvalidFunction

use std::fmt;

/// Exception raised for invalid function.
#[derive(Debug, Clone)]
pub struct ExprInvalidFunction {
    message: String,
}

impl ExprInvalidFunction {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ExprInvalidFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expr_InvalidFunction: {}", self.message)
    }
}

impl std::error::Error for ExprInvalidFunction {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create() {
        let err = ExprInvalidFunction::new("bad func");
        assert_eq!(err.message(), "bad func");
    }
}
