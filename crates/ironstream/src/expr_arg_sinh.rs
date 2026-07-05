// FILE: expr_arg_sinh.rs
// occt: Expr_ArgSinh

/// Represents the inverse hyperbolic sine (ArgSinh) of an expression.
/// This is a unary expression that takes a single operand.
#[derive(Debug, Clone)]
pub struct ExprArgSinh {
    operand: String,
}

impl ExprArgSinh {
    /// Create the ArgSinh of the given expression
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
    pub fn is_identical(&self, other: &ExprArgSinh) -> bool {
        self.operand == other.operand
    }

    /// Check if the expression is linear (ArgSinh is not linear)
    pub fn is_linear(&self) -> bool {
        false
    }

    /// Evaluate the expression with given variables and values
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        // Simplified evaluation: treat operand as a variable name or number
        if let Ok(val) = self.operand.parse::<f64>() {
            return Ok(val.asinh());
        }
        // Look up variable in vars/vals
        for (i, var) in vars.iter().enumerate() {
            if i < vals.len() && self.operand == *var {
                return Ok(vals[i].asinh());
            }
        }
        Err(format!("Unknown variable: {}", self.operand))
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("ArgSinh({})", self.operand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_arg_sinh() {
        let asinh = ExprArgSinh::new("x");
        assert_eq!(asinh.operand(), "x");
    }

    #[test]
    fn test_copy() {
        let asinh1 = ExprArgSinh::new("x");
        let asinh2 = asinh1.copy();
        assert_eq!(asinh1.operand(), asinh2.operand());
    }

    #[test]
    fn test_is_identical() {
        let asinh1 = ExprArgSinh::new("expr1");
        let asinh2 = ExprArgSinh::new("expr1");
        let asinh3 = ExprArgSinh::new("expr2");
        assert!(asinh1.is_identical(&asinh2));
        assert!(!asinh1.is_identical(&asinh3));
    }

    #[test]
    fn test_is_linear() {
        let asinh = ExprArgSinh::new("x");
        assert!(!asinh.is_linear());
    }

    #[test]
    fn test_evaluate_zero() {
        let asinh = ExprArgSinh::new("0");
        let result = asinh.evaluate(&[], &[]).unwrap();
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_positive() {
        let asinh = ExprArgSinh::new("x");
        let vars = vec!["x"];
        let vals = vec![1.0];
        let result = asinh.evaluate(&vars, &vals).unwrap();
        assert!((result - 1.0f64.asinh()).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_negative() {
        let asinh = ExprArgSinh::new("x");
        let vars = vec!["x"];
        let vals = vec![-2.0];
        let result = asinh.evaluate(&vars, &vals).unwrap();
        assert!((result - (-2.0f64).asinh()).abs() < 1e-10);
    }

    #[test]
    fn test_string_representation() {
        let asinh = ExprArgSinh::new("x");
        assert_eq!(asinh.string(), "ArgSinh(x)");
    }
}
