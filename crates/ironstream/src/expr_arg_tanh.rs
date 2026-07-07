// FILE: expr_arg_tanh.rs
// occt: Expr_ArgTanh

/// Represents the inverse hyperbolic tangent (ArgTanh) of an expression.
/// This is a unary expression that takes a single operand.
#[derive(Debug, Clone)]
pub struct ExprArgTanh {
    operand: String,
}

impl ExprArgTanh {
    /// Create the ArgTanh of the given expression
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
    pub fn is_identical(&self, other: &ExprArgTanh) -> bool {
        self.operand == other.operand
    }

    /// Check if the expression is linear (ArgTanh is not linear)
    pub fn is_linear(&self) -> bool {
        false
    }

    /// Evaluate the expression with given variables and values
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        // Simplified evaluation: treat operand as a variable name or number
        if let Ok(val) = self.operand.parse::<f64>() {
            if val <= -1.0 || val >= 1.0 {
                return Err("atanh domain error: value must be in (-1, 1)".to_string());
            }
            return Ok(val.atanh());
        }
        // Look up variable in vars/vals
        for (i, var) in vars.iter().enumerate() {
            if i < vals.len() && self.operand == *var {
                if vals[i] <= -1.0 || vals[i] >= 1.0 {
                    return Err("atanh domain error: value must be in (-1, 1)".to_string());
                }
                return Ok(vals[i].atanh());
            }
        }
        Err(format!("Unknown variable: {}", self.operand))
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("ArgTanh({})", self.operand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_arg_tanh() {
        let atanh = ExprArgTanh::new("x");
        assert_eq!(atanh.operand(), "x");
    }

    #[test]
    fn test_copy() {
        let atanh1 = ExprArgTanh::new("x");
        let atanh2 = atanh1.copy();
        assert_eq!(atanh1.operand(), atanh2.operand());
    }

    #[test]
    fn test_is_identical() {
        let atanh1 = ExprArgTanh::new("expr1");
        let atanh2 = ExprArgTanh::new("expr1");
        let atanh3 = ExprArgTanh::new("expr2");
        assert!(atanh1.is_identical(&atanh2));
        assert!(!atanh1.is_identical(&atanh3));
    }

    #[test]
    fn test_is_linear() {
        let atanh = ExprArgTanh::new("x");
        assert!(!atanh.is_linear());
    }

    #[test]
    fn test_evaluate_zero() {
        let atanh = ExprArgTanh::new("0");
        let result = atanh.evaluate(&[], &[]).unwrap();
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_valid_range() {
        let atanh = ExprArgTanh::new("x");
        let vars = vec!["x"];
        let vals = vec![0.5];
        let result = atanh.evaluate(&vars, &vals).unwrap();
        assert!((result - 0.5f64.atanh()).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_out_of_range_positive() {
        let atanh = ExprArgTanh::new("1.0");
        let result = atanh.evaluate(&[], &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_out_of_range_negative() {
        let atanh = ExprArgTanh::new("-1.5");
        let result = atanh.evaluate(&[], &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_string_representation() {
        let atanh = ExprArgTanh::new("x");
        assert_eq!(atanh.string(), "ArgTanh(x)");
    }
}
