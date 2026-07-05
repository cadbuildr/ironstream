// FILE: expr_poly_expression.rs
// occt: Expr_PolyExpression

#[derive(Debug, Clone)]
pub struct ExprPolyExpression {
    operands: Vec<String>,
}

impl ExprPolyExpression {
    pub fn new() -> Self { Self { operands: Vec::new() } }
    pub fn add_operand(&mut self, op: impl Into<String>) { self.operands.push(op.into()); }
    pub fn operands(&self) -> &[String] { &self.operands }
    pub fn copy(&self) -> Self { Self { operands: self.operands.clone() } }
    pub fn string(&self) -> String { format!("({})", self.operands.join(" op ")) }
}

impl Default for ExprPolyExpression {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut p = ExprPolyExpression::new();
        p.add_operand("a");
        p.add_operand("b");
        assert_eq!(p.operands().len(), 2);
    }
}
