// FILE: expr_binary_expression.rs
// occt: Expr_BinaryExpression

/// Represents all binary expressions. The order of the two operands is significant.
#[derive(Debug, Clone)]
pub struct ExprBinaryExpression {
    first_operand: String,
    second_operand: String,
}

impl ExprBinaryExpression {
    /// Create a new binary expression
    pub fn new(first: impl Into<String>, second: impl Into<String>) -> Self {
        Self {
            first_operand: first.into(),
            second_operand: second.into(),
        }
    }

    /// Get the first operand
    pub fn first_operand(&self) -> &str {
        &self.first_operand
    }

    /// Get the second operand
    pub fn second_operand(&self) -> &str {
        &self.second_operand
    }

    /// Set the first operand
    pub fn set_first_operand(&mut self, exp: impl Into<String>) -> Result<(), String> {
        let new_exp = exp.into();
        if new_exp == self.first_operand && new_exp == self.second_operand {
            return Err("InvalidOperand: cannot set same expression as both operands".to_string());
        }
        self.first_operand = new_exp;
        Ok(())
    }

    /// Set the second operand
    pub fn set_second_operand(&mut self, exp: impl Into<String>) -> Result<(), String> {
        let new_exp = exp.into();
        if new_exp == self.first_operand && new_exp == self.second_operand {
            return Err("InvalidOperand: cannot set same expression as both operands".to_string());
        }
        self.second_operand = new_exp;
        Ok(())
    }

    /// Returns the number of sub-expressions (always 2 for binary)
    pub fn nb_sub_expressions(&self) -> usize {
        2
    }

    /// Returns the i-th sub-expression (1-based indexing)
    pub fn sub_expression(&self, i: usize) -> Option<&str> {
        match i {
            1 => Some(&self.first_operand),
            2 => Some(&self.second_operand),
            _ => None,
        }
    }

    /// Does this expression contain unknowns?
    pub fn contains_unknowns(&self) -> bool {
        self.first_operand.contains('x')
            || self.first_operand.contains('y')
            || self.first_operand.contains('z')
            || self.second_operand.contains('x')
            || self.second_operand.contains('y')
            || self.second_operand.contains('z')
    }

    /// Tests if this expression contains the given expression
    pub fn contains(&self, exp: &str) -> bool {
        self.first_operand.contains(exp) || self.second_operand.contains(exp)
    }

    /// Replace all occurrences of var with the given expression
    pub fn replace(&mut self, var: &str, with: &str) {
        self.first_operand = self.first_operand.replace(var, with);
        self.second_operand = self.second_operand.replace(var, with);
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("({} op {})", self.first_operand, self.second_operand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_binary_expression() {
        let expr = ExprBinaryExpression::new("x", "y");
        assert_eq!(expr.first_operand(), "x");
        assert_eq!(expr.second_operand(), "y");
    }

    #[test]
    fn test_set_operands() {
        let mut expr = ExprBinaryExpression::new("a", "b");
        assert!(expr.set_first_operand("c").is_ok());
        assert_eq!(expr.first_operand(), "c");
        assert!(expr.set_second_operand("d").is_ok());
        assert_eq!(expr.second_operand(), "d");
    }

    #[test]
    fn test_nb_sub_expressions() {
        let expr = ExprBinaryExpression::new("x", "y");
        assert_eq!(expr.nb_sub_expressions(), 2);
    }

    #[test]
    fn test_sub_expression() {
        let expr = ExprBinaryExpression::new("first", "second");
        assert_eq!(expr.sub_expression(1), Some("first"));
        assert_eq!(expr.sub_expression(2), Some("second"));
        assert_eq!(expr.sub_expression(3), None);
        assert_eq!(expr.sub_expression(0), None);
    }

    #[test]
    fn test_contains_unknowns() {
        let expr_with_unknown = ExprBinaryExpression::new("x", "5");
        assert!(expr_with_unknown.contains_unknowns());

        let expr_without_unknown = ExprBinaryExpression::new("3", "5");
        assert!(!expr_without_unknown.contains_unknowns());

        let expr_y = ExprBinaryExpression::new("2", "y");
        assert!(expr_y.contains_unknowns());
    }

    #[test]
    fn test_contains_expression() {
        let expr = ExprBinaryExpression::new("x + y", "z * 2");
        assert!(expr.contains("x"));
        assert!(expr.contains("z"));
        assert!(!expr.contains("w"));
    }

    #[test]
    fn test_replace() {
        let mut expr = ExprBinaryExpression::new("x + 1", "x * 2");
        expr.replace("x", "y");
        assert_eq!(expr.first_operand(), "y + 1");
        assert_eq!(expr.second_operand(), "y * 2");
    }

    #[test]
    fn test_string_representation() {
        let expr = ExprBinaryExpression::new("a", "b");
        let s = expr.string();
        assert!(s.contains("a"));
        assert!(s.contains("b"));
    }
}
