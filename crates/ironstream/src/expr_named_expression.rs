// FILE: expr_named_expression.rs
// occt: Expr_NamedExpression

#[derive(Debug, Clone)]
pub struct ExprNamedExpression {
    name: String,
}

impl ExprNamedExpression {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into() } }
    pub fn name(&self) -> &str { &self.name }
    pub fn copy(&self) -> Self { Self { name: self.name.clone() } }
    pub fn is_identical(&self, other: &ExprNamedExpression) -> bool { self.name == other.name }
    pub fn string(&self) -> String { self.name.clone() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let e = ExprNamedExpression::new("expr1");
        assert_eq!(e.name(), "expr1");
    }
}
