// FILE: expr_not_evaluable.rs
// occt: Expr_NotEvaluable

use std::fmt;

#[derive(Debug, Clone)]
pub struct ExprNotEvaluable { message: String }

impl ExprNotEvaluable {
    pub fn new(message: impl Into<String>) -> Self { Self { message: message.into() } }
    pub fn message(&self) -> &str { &self.message }
}

impl fmt::Display for ExprNotEvaluable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expr_NotEvaluable: {}", self.message)
    }
}

impl std::error::Error for ExprNotEvaluable {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let e = ExprNotEvaluable::new("cannot eval");
        assert_eq!(e.message(), "cannot eval");
    }
}
