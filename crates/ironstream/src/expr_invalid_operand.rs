// FILE: expr_invalid_operand.rs
// occt: Expr_InvalidOperand

use std::fmt;

/// Exception for invalid operand in expression.
#[derive(Debug, Clone)]
pub struct ExprInvalidOperand {
    message: String,
}

impl ExprInvalidOperand {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ExprInvalidOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expr_InvalidOperand: {}", self.message)
    }
}

impl std::error::Error for ExprInvalidOperand {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create() {
        let err = ExprInvalidOperand::new("bad operand");
        assert_eq!(err.message(), "bad operand");
    }
}
