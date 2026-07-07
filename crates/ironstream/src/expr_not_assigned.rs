// FILE: expr_not_assigned.rs
// occt: Expr_NotAssigned

use std::fmt;

#[derive(Debug, Clone)]
pub struct ExprNotAssigned { message: String }

impl ExprNotAssigned {
    pub fn new(message: impl Into<String>) -> Self { Self { message: message.into() } }
    pub fn message(&self) -> &str { &self.message }
}

impl fmt::Display for ExprNotAssigned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expr_NotAssigned: {}", self.message)
    }
}

impl std::error::Error for ExprNotAssigned {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let e = ExprNotAssigned::new("no value");
        assert_eq!(e.message(), "no value");
    }
}
