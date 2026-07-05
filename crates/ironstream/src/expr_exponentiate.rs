// FILE: expr_exponentiate.rs
// occt: Expr_Exponentiate

/// Represents exponentiation: exp1 ^ exp2.
#[derive(Debug, Clone)]
pub struct ExprExponentiate {
    base: String,
    exponent: String,
}

impl ExprExponentiate {
    /// Create exponentiation exp1 ^ exp2
    pub fn new(exp1: impl Into<String>, exp2: impl Into<String>) -> Self {
        Self {
            base: exp1.into(),
            exponent: exp2.into(),
        }
    }

    /// Get the base
    pub fn base(&self) -> &str {
        &self.base
    }

    /// Get the exponent
    pub fn exponent(&self) -> &str {
        &self.exponent
    }

    /// Return a simplified version
    pub fn shallow_simplified(&self) -> Self {
        Self {
            base: self.base.clone(),
            exponent: self.exponent.clone(),
        }
    }

    /// Return a copy
    pub fn copy(&self) -> Self {
        Self {
            base: self.base.clone(),
            exponent: self.exponent.clone(),
        }
    }

    /// Test if identical
    pub fn is_identical(&self, other: &ExprExponentiate) -> bool {
        self.base == other.base && self.exponent == other.exponent
    }

    /// Check if linear
    pub fn is_linear(&self) -> bool {
        false
    }

    /// Evaluate: base ^ exponent
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        let base_val = self.evaluate_operand(&self.base, vars, vals)?;
        let exp_val = self.evaluate_operand(&self.exponent, vars, vals)?;
        Ok(base_val.powf(exp_val))
    }

    /// Helper to evaluate operands
    fn evaluate_operand(&self, operand: &str, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        if let Ok(num) = operand.parse::<f64>() {
            return Ok(num);
        }
        for (i, var) in vars.iter().enumerate() {
            if i < vals.len() && operand == *var {
                return Ok(vals[i]);
            }
        }
        Err(format!("Unknown variable: {}", operand))
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("({} ^ {})", self.base, self.exponent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_exponentiate() {
        let exp = ExprExponentiate::new("2", "3");
        assert_eq!(exp.base(), "2");
        assert_eq!(exp.exponent(), "3");
    }

    #[test]
    fn test_copy() {
        let exp1 = ExprExponentiate::new("a", "b");
        let exp2 = exp1.copy();
        assert!(exp1.is_identical(&exp2));
    }

    #[test]
    fn test_is_identical() {
        let exp1 = ExprExponentiate::new("x", "y");
        let exp2 = ExprExponentiate::new("x", "y");
        let exp3 = ExprExponentiate::new("y", "x");
        assert!(exp1.is_identical(&exp2));
        assert!(!exp1.is_identical(&exp3));
    }

    #[test]
    fn test_is_linear() {
        let exp = ExprExponentiate::new("x", "2");
        assert!(!exp.is_linear());
    }

    #[test]
    fn test_evaluate_simple() {
        let exp = ExprExponentiate::new("2", "3");
        let result = exp.evaluate(&[], &[]).unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_evaluate_square() {
        let exp = ExprExponentiate::new("5", "2");
        let result = exp.evaluate(&[], &[]).unwrap();
        assert_eq!(result, 25.0);
    }

    #[test]
    fn test_evaluate_with_variables() {
        let exp = ExprExponentiate::new("x", "y");
        let vars = vec!["x", "y"];
        let vals = vec![2.0, 4.0];
        let result = exp.evaluate(&vars, &vals).unwrap();
        assert_eq!(result, 16.0);
    }

    #[test]
    fn test_evaluate_fractional_exponent() {
        let exp = ExprExponentiate::new("4", "0.5");
        let result = exp.evaluate(&[], &[]).unwrap();
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_string_representation() {
        let exp = ExprExponentiate::new("a", "b");
        assert_eq!(exp.string(), "(a ^ b)");
    }
}
