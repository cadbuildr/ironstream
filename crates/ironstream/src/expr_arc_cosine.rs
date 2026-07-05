// FILE: expr_arc_cosine.rs
// occt: Expr_ArcCosine

/// Represents the arc cosine (Arccos) of an expression.
/// This is a unary expression that takes a single operand.
#[derive(Debug, Clone)]
pub struct ExprArcCosine {
    operand: String,
}

impl ExprArcCosine {
    /// Create the Arccos of the given expression
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
    pub fn is_identical(&self, other: &ExprArcCosine) -> bool {
        self.operand == other.operand
    }

    /// Check if the expression is linear (Arccos is not linear)
    pub fn is_linear(&self) -> bool {
        false
    }

    /// Evaluate the expression with given variables and values
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        // Simplified evaluation: treat operand as a variable name or number
        if let Ok(val) = self.operand.parse::<f64>() {
            if val < -1.0 || val > 1.0 {
                return Err("arccos domain error: value out of [-1, 1]".to_string());
            }
            return Ok(val.acos());
        }
        // Look up variable in vars/vals
        for (i, var) in vars.iter().enumerate() {
            if i < vals.len() && self.operand == *var {
                if vals[i] < -1.0 || vals[i] > 1.0 {
                    return Err("arccos domain error: value out of [-1, 1]".to_string());
                }
                return Ok(vals[i].acos());
            }
        }
        Err(format!("Unknown variable: {}", self.operand))
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("ArcCos({})", self.operand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_arc_cosine() {
        let acos = ExprArcCosine::new("x");
        assert_eq!(acos.operand(), "x");
    }

    #[test]
    fn test_copy() {
        let acos1 = ExprArcCosine::new("x");
        let acos2 = acos1.copy();
        assert_eq!(acos1.operand(), acos2.operand());
    }

    #[test]
    fn test_is_identical() {
        let acos1 = ExprArcCosine::new("expr1");
        let acos2 = ExprArcCosine::new("expr1");
        let acos3 = ExprArcCosine::new("expr2");
        assert!(acos1.is_identical(&acos2));
        assert!(!acos1.is_identical(&acos3));
    }

    #[test]
    fn test_is_linear() {
        let acos = ExprArcCosine::new("x");
        assert!(!acos.is_linear());
    }

    #[test]
    fn test_evaluate_zero() {
        let acos = ExprArcCosine::new("0");
        let result = acos.evaluate(&[], &[]).unwrap();
        assert!((result - std::f64::consts::PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_valid_range() {
        let acos = ExprArcCosine::new("x");
        let vars = vec!["x"];
        let vals = vec![0.5];
        let result = acos.evaluate(&vars, &vals).unwrap();
        assert!((result - 0.5f64.acos()).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_out_of_range() {
        let acos = ExprArcCosine::new("2.0");
        let result = acos.evaluate(&[], &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_string_representation() {
        let acos = ExprArcCosine::new("x");
        assert_eq!(acos.string(), "ArcCos(x)");
    }
}
