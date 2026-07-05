// FILE: expr_cosine.rs
// occt: Expr_Cosine

/// Represents the cosine of an expression.
/// This is a unary expression that takes a single operand.
#[derive(Debug, Clone)]
pub struct ExprCosine {
    operand: String,
}

impl ExprCosine {
    /// Create the cosine of the given expression
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
    pub fn is_identical(&self, other: &ExprCosine) -> bool {
        self.operand == other.operand
    }

    /// Check if the expression is linear (Cosine is not linear)
    pub fn is_linear(&self) -> bool {
        false
    }

    /// Evaluate the expression with given variables and values
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        // Simplified evaluation: treat operand as a variable name or number
        if let Ok(val) = self.operand.parse::<f64>() {
            return Ok(val.cos());
        }
        // Look up variable in vars/vals
        for (i, var) in vars.iter().enumerate() {
            if i < vals.len() && self.operand == *var {
                return Ok(vals[i].cos());
            }
        }
        Err(format!("Unknown variable: {}", self.operand))
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("Cos({})", self.operand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cosine() {
        let cos = ExprCosine::new("x");
        assert_eq!(cos.operand(), "x");
    }

    #[test]
    fn test_copy() {
        let cos1 = ExprCosine::new("x");
        let cos2 = cos1.copy();
        assert_eq!(cos1.operand(), cos2.operand());
    }

    #[test]
    fn test_is_identical() {
        let cos1 = ExprCosine::new("expr1");
        let cos2 = ExprCosine::new("expr1");
        let cos3 = ExprCosine::new("expr2");
        assert!(cos1.is_identical(&cos2));
        assert!(!cos1.is_identical(&cos3));
    }

    #[test]
    fn test_is_linear() {
        let cos = ExprCosine::new("x");
        assert!(!cos.is_linear());
    }

    #[test]
    fn test_evaluate_zero() {
        let cos = ExprCosine::new("0");
        let result = cos.evaluate(&[], &[]).unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_pi_half() {
        let cos = ExprCosine::new("x");
        let vars = vec!["x"];
        let vals = vec![std::f64::consts::PI / 2.0];
        let result = cos.evaluate(&vars, &vals).unwrap();
        assert!(result.abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_pi() {
        let cos = ExprCosine::new("x");
        let vars = vec!["x"];
        let vals = vec![std::f64::consts::PI];
        let result = cos.evaluate(&vars, &vals).unwrap();
        assert!((result - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_string_representation() {
        let cos = ExprCosine::new("x");
        assert_eq!(cos.string(), "Cos(x)");
    }
}
