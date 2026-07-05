// FILE: expr_cosh.rs
// occt: Expr_Cosh

/// Represents the hyperbolic cosine (Cosh) of an expression.
/// This is a unary expression that takes a single operand.
#[derive(Debug, Clone)]
pub struct ExprCosh {
    operand: String,
}

impl ExprCosh {
    /// Create the Cosh of the given expression
    pub fn new(expr: impl Into<String>) -> Self {
        Self {
            operand: expr.into(),
        }
    }

    /// Get the operand expression
    pub fn operand(&self) -> &str {
        &self.operand
    }

    /// Return a simplified version of the expression
    pub fn shallow_simplified(&self) -> Self {
        Self {
            operand: self.operand.clone(),
        }
    }

    /// Return a copy of this expression
    pub fn copy(&self) -> Self {
        Self {
            operand: self.operand.clone(),
        }
    }

    /// Test if this expression is identical to another
    pub fn is_identical(&self, other: &ExprCosh) -> bool {
        self.operand == other.operand
    }

    /// Check if the expression is linear (Cosh is not linear)
    pub fn is_linear(&self) -> bool {
        false
    }

    /// Evaluate the expression with given variables and values
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        // Simplified evaluation: treat operand as a variable name or number
        if let Ok(val) = self.operand.parse::<f64>() {
            return Ok(val.cosh());
        }
        // Look up variable in vars/vals
        for (i, var) in vars.iter().enumerate() {
            if i < vals.len() && self.operand == *var {
                return Ok(vals[i].cosh());
            }
        }
        Err(format!("Unknown variable: {}", self.operand))
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("Cosh({})", self.operand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cosh() {
        let cosh = ExprCosh::new("x");
        assert_eq!(cosh.operand(), "x");
    }

    #[test]
    fn test_copy() {
        let cosh1 = ExprCosh::new("x");
        let cosh2 = cosh1.copy();
        assert_eq!(cosh1.operand(), cosh2.operand());
    }

    #[test]
    fn test_is_identical() {
        let cosh1 = ExprCosh::new("expr1");
        let cosh2 = ExprCosh::new("expr1");
        let cosh3 = ExprCosh::new("expr2");
        assert!(cosh1.is_identical(&cosh2));
        assert!(!cosh1.is_identical(&cosh3));
    }

    #[test]
    fn test_is_linear() {
        let cosh = ExprCosh::new("x");
        assert!(!cosh.is_linear());
    }

    #[test]
    fn test_evaluate_zero() {
        let cosh = ExprCosh::new("0");
        let result = cosh.evaluate(&[], &[]).unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_positive() {
        let cosh = ExprCosh::new("x");
        let vars = vec!["x"];
        let vals = vec![1.0];
        let result = cosh.evaluate(&vars, &vals).unwrap();
        assert!((result - 1.0f64.cosh()).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_negative() {
        let cosh = ExprCosh::new("x");
        let vars = vec!["x"];
        let vals = vec![-2.0];
        let result = cosh.evaluate(&vars, &vals).unwrap();
        assert!((result - (-2.0f64).cosh()).abs() < 1e-10);
    }

    #[test]
    fn test_string_representation() {
        let cosh = ExprCosh::new("x");
        assert_eq!(cosh.string(), "Cosh(x)");
    }
}
