// FILE: expr_arc_sine.rs
// occt: Expr_ArcSine

/// Represents the arc sine (Arcsin) of an expression.
/// This is a unary expression that takes a single operand.
#[derive(Debug, Clone)]
pub struct ExprArcSine {
    operand: String,
}

impl ExprArcSine {
    /// Create the Arcsin of the given expression
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
    pub fn is_identical(&self, other: &ExprArcSine) -> bool {
        self.operand == other.operand
    }

    /// Check if the expression is linear (Arcsin is not linear)
    pub fn is_linear(&self) -> bool {
        false
    }

    /// Evaluate the expression with given variables and values
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        // Simplified evaluation: treat operand as a variable name or number
        if let Ok(val) = self.operand.parse::<f64>() {
            if val < -1.0 || val > 1.0 {
                return Err("arcsin domain error: value out of [-1, 1]".to_string());
            }
            return Ok(val.asin());
        }
        // Look up variable in vars/vals
        for (i, var) in vars.iter().enumerate() {
            if i < vals.len() && self.operand == *var {
                if vals[i] < -1.0 || vals[i] > 1.0 {
                    return Err("arcsin domain error: value out of [-1, 1]".to_string());
                }
                return Ok(vals[i].asin());
            }
        }
        Err(format!("Unknown variable: {}", self.operand))
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("ArcSin({})", self.operand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_arc_sine() {
        let asin = ExprArcSine::new("x");
        assert_eq!(asin.operand(), "x");
    }

    #[test]
    fn test_copy() {
        let asin1 = ExprArcSine::new("x");
        let asin2 = asin1.copy();
        assert_eq!(asin1.operand(), asin2.operand());
    }

    #[test]
    fn test_is_identical() {
        let asin1 = ExprArcSine::new("expr1");
        let asin2 = ExprArcSine::new("expr1");
        let asin3 = ExprArcSine::new("expr2");
        assert!(asin1.is_identical(&asin2));
        assert!(!asin1.is_identical(&asin3));
    }

    #[test]
    fn test_is_linear() {
        let asin = ExprArcSine::new("x");
        assert!(!asin.is_linear());
    }

    #[test]
    fn test_evaluate_zero() {
        let asin = ExprArcSine::new("0");
        let result = asin.evaluate(&[], &[]).unwrap();
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_valid_range() {
        let asin = ExprArcSine::new("x");
        let vars = vec!["x"];
        let vals = vec![0.5];
        let result = asin.evaluate(&vars, &vals).unwrap();
        assert!((result - 0.5f64.asin()).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_out_of_range() {
        let asin = ExprArcSine::new("2.0");
        let result = asin.evaluate(&[], &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_string_representation() {
        let asin = ExprArcSine::new("x");
        assert_eq!(asin.string(), "ArcSin(x)");
    }
}
