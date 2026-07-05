// FILE: expr_arg_cosh.rs
// occt: Expr_ArgCosh

/// Represents the inverse hyperbolic cosine (ArgCosh) of an expression.
/// This is a unary expression that takes a single operand.
#[derive(Debug, Clone)]
pub struct ExprArgCosh {
    operand: String,
}

impl ExprArgCosh {
    /// Create the ArgCosh of the given expression
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
    pub fn is_identical(&self, other: &ExprArgCosh) -> bool {
        self.operand == other.operand
    }

    /// Check if the expression is linear (ArgCosh is not linear)
    pub fn is_linear(&self) -> bool {
        false
    }

    /// Evaluate the expression with given variables and values
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        // Simplified evaluation: treat operand as a variable name or number
        if let Ok(val) = self.operand.parse::<f64>() {
            if val < 1.0 {
                return Err("acosh domain error: value must be >= 1".to_string());
            }
            return Ok(val.acosh());
        }
        // Look up variable in vars/vals
        for (i, var) in vars.iter().enumerate() {
            if i < vals.len() && self.operand == *var {
                if vals[i] < 1.0 {
                    return Err("acosh domain error: value must be >= 1".to_string());
                }
                return Ok(vals[i].acosh());
            }
        }
        Err(format!("Unknown variable: {}", self.operand))
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("ArgCosh({})", self.operand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_arg_cosh() {
        let acosh = ExprArgCosh::new("x");
        assert_eq!(acosh.operand(), "x");
    }

    #[test]
    fn test_copy() {
        let acosh1 = ExprArgCosh::new("x");
        let acosh2 = acosh1.copy();
        assert_eq!(acosh1.operand(), acosh2.operand());
    }

    #[test]
    fn test_is_identical() {
        let acosh1 = ExprArgCosh::new("expr1");
        let acosh2 = ExprArgCosh::new("expr1");
        let acosh3 = ExprArgCosh::new("expr2");
        assert!(acosh1.is_identical(&acosh2));
        assert!(!acosh1.is_identical(&acosh3));
    }

    #[test]
    fn test_is_linear() {
        let acosh = ExprArgCosh::new("x");
        assert!(!acosh.is_linear());
    }

    #[test]
    fn test_evaluate_one() {
        let acosh = ExprArgCosh::new("1");
        let result = acosh.evaluate(&[], &[]).unwrap();
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_valid_range() {
        let acosh = ExprArgCosh::new("x");
        let vars = vec!["x"];
        let vals = vec![2.0];
        let result = acosh.evaluate(&vars, &vals).unwrap();
        assert!((result - 2.0f64.acosh()).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_out_of_range() {
        let acosh = ExprArgCosh::new("0.5");
        let result = acosh.evaluate(&[], &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_string_representation() {
        let acosh = ExprArgCosh::new("x");
        assert_eq!(acosh.string(), "ArgCosh(x)");
    }
}
