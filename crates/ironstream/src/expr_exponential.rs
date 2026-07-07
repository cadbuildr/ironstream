// FILE: expr_exponential.rs
// occt: Expr_Exponential

/// Represents the exponential function (e^x) of an expression.
#[derive(Debug, Clone)]
pub struct ExprExponential {
    operand: String,
}

impl ExprExponential {
    /// Create the exponential of the given expression
    pub fn new(expr: impl Into<String>) -> Self {
        Self {
            operand: expr.into(),
        }
    }

    /// Get the operand
    pub fn operand(&self) -> &str {
        &self.operand
    }

    /// Return a simplified version
    pub fn shallow_simplified(&self) -> Self {
        Self {
            operand: self.operand.clone(),
        }
    }

    /// Return a copy
    pub fn copy(&self) -> Self {
        Self {
            operand: self.operand.clone(),
        }
    }

    /// Test if identical
    pub fn is_identical(&self, other: &ExprExponential) -> bool {
        self.operand == other.operand
    }

    /// Check if linear
    pub fn is_linear(&self) -> bool {
        false
    }

    /// Evaluate: e^operand
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        if let Ok(val) = self.operand.parse::<f64>() {
            return Ok(val.exp());
        }
        for (i, var) in vars.iter().enumerate() {
            if i < vals.len() && self.operand == *var {
                return Ok(vals[i].exp());
            }
        }
        Err(format!("Unknown variable: {}", self.operand))
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("Exp({})", self.operand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_exponential() {
        let exp = ExprExponential::new("x");
        assert_eq!(exp.operand(), "x");
    }

    #[test]
    fn test_copy() {
        let exp1 = ExprExponential::new("y");
        let exp2 = exp1.copy();
        assert!(exp1.is_identical(&exp2));
    }

    #[test]
    fn test_is_identical() {
        let exp1 = ExprExponential::new("a");
        let exp2 = ExprExponential::new("a");
        let exp3 = ExprExponential::new("b");
        assert!(exp1.is_identical(&exp2));
        assert!(!exp1.is_identical(&exp3));
    }

    #[test]
    fn test_is_linear() {
        let exp = ExprExponential::new("x");
        assert!(!exp.is_linear());
    }

    #[test]
    fn test_evaluate_zero() {
        let exp = ExprExponential::new("0");
        let result = exp.evaluate(&[], &[]).unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_one() {
        let exp = ExprExponential::new("1");
        let result = exp.evaluate(&[], &[]).unwrap();
        assert!((result - std::f64::consts::E).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_with_variable() {
        let exp = ExprExponential::new("x");
        let vars = vec!["x"];
        let vals = vec![2.0];
        let result = exp.evaluate(&vars, &vals).unwrap();
        assert!((result - 2.0f64.exp()).abs() < 1e-10);
    }

    #[test]
    fn test_string_representation() {
        let exp = ExprExponential::new("x");
        assert_eq!(exp.string(), "Exp(x)");
    }
}
