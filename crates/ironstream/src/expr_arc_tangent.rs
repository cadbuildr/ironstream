// FILE: expr_arc_tangent.rs
// occt: Expr_ArcTangent

/// Represents the arc tangent (Arctan) of an expression.
/// This is a unary expression that takes a single operand.
#[derive(Debug, Clone)]
pub struct ExprArcTangent {
    operand: String,
}

impl ExprArcTangent {
    /// Create the Arctan of the given expression
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
    pub fn is_identical(&self, other: &ExprArcTangent) -> bool {
        self.operand == other.operand
    }

    /// Check if the expression is linear (Arctan is not linear)
    pub fn is_linear(&self) -> bool {
        false
    }

    /// Evaluate the expression with given variables and values
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        // Simplified evaluation: treat operand as a variable name or number
        if let Ok(val) = self.operand.parse::<f64>() {
            return Ok(val.atan());
        }
        // Look up variable in vars/vals
        for (i, var) in vars.iter().enumerate() {
            if i < vals.len() && self.operand == *var {
                return Ok(vals[i].atan());
            }
        }
        Err(format!("Unknown variable: {}", self.operand))
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("ArcTan({})", self.operand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_arc_tangent() {
        let atan = ExprArcTangent::new("x");
        assert_eq!(atan.operand(), "x");
    }

    #[test]
    fn test_copy() {
        let atan1 = ExprArcTangent::new("x");
        let atan2 = atan1.copy();
        assert_eq!(atan1.operand(), atan2.operand());
    }

    #[test]
    fn test_is_identical() {
        let atan1 = ExprArcTangent::new("expr1");
        let atan2 = ExprArcTangent::new("expr1");
        let atan3 = ExprArcTangent::new("expr2");
        assert!(atan1.is_identical(&atan2));
        assert!(!atan1.is_identical(&atan3));
    }

    #[test]
    fn test_is_linear() {
        let atan = ExprArcTangent::new("x");
        assert!(!atan.is_linear());
    }

    #[test]
    fn test_evaluate_zero() {
        let atan = ExprArcTangent::new("0");
        let result = atan.evaluate(&[], &[]).unwrap();
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_one() {
        let atan = ExprArcTangent::new("1");
        let result = atan.evaluate(&[], &[]).unwrap();
        assert!((result - std::f64::consts::PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_variable() {
        let atan = ExprArcTangent::new("x");
        let vars = vec!["x"];
        let vals = vec![2.0];
        let result = atan.evaluate(&vars, &vals).unwrap();
        assert!((result - 2.0f64.atan()).abs() < 1e-10);
    }

    #[test]
    fn test_string_representation() {
        let atan = ExprArcTangent::new("x");
        assert_eq!(atan.string(), "ArcTan(x)");
    }
}
