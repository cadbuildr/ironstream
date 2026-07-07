// FILE: expr_absolute.rs
// occt: Expr_Absolute

/// Represents the absolute value (Abs) of an expression.
/// This is a unary expression that takes a single operand.
#[derive(Debug, Clone)]
pub struct ExprAbsolute {
    operand: String,
}

impl ExprAbsolute {
    /// Create the Abs of the given expression
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
    pub fn is_identical(&self, other: &ExprAbsolute) -> bool {
        self.operand == other.operand
    }

    /// Check if the expression is linear (Abs is not linear in general)
    pub fn is_linear(&self) -> bool {
        false
    }

    /// Evaluate the expression with given variables and values
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        // Simplified evaluation: treat operand as a variable name or number
        if let Ok(val) = self.operand.parse::<f64>() {
            return Ok(val.abs());
        }
        // Look up variable in vars/vals
        for (i, var) in vars.iter().enumerate() {
            if i < vals.len() && self.operand == *var {
                return Ok(vals[i].abs());
            }
        }
        Err(format!("Unknown variable: {}", self.operand))
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("Abs({})", self.operand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_absolute() {
        let abs = ExprAbsolute::new("x");
        assert_eq!(abs.operand(), "x");
    }

    #[test]
    fn test_copy() {
        let abs1 = ExprAbsolute::new("x + y");
        let abs2 = abs1.copy();
        assert_eq!(abs1.operand(), abs2.operand());
    }

    #[test]
    fn test_is_identical() {
        let abs1 = ExprAbsolute::new("expr1");
        let abs2 = ExprAbsolute::new("expr1");
        let abs3 = ExprAbsolute::new("expr2");
        assert!(abs1.is_identical(&abs2));
        assert!(!abs1.is_identical(&abs3));
    }

    #[test]
    fn test_is_linear() {
        let abs = ExprAbsolute::new("x");
        assert!(!abs.is_linear());
    }

    #[test]
    fn test_evaluate_numeric() {
        let abs = ExprAbsolute::new("-5");
        assert_eq!(abs.evaluate(&[], &[]).unwrap(), 5.0);
    }

    #[test]
    fn test_evaluate_variable() {
        let abs = ExprAbsolute::new("x");
        let vars = vec!["x"];
        let vals = vec![-3.5];
        assert_eq!(abs.evaluate(&vars, &vals).unwrap(), 3.5);
    }

    #[test]
    fn test_string_representation() {
        let abs = ExprAbsolute::new("x + y");
        assert_eq!(abs.string(), "Abs(x + y)");
    }

    #[test]
    fn test_shallow_simplified() {
        let abs1 = ExprAbsolute::new("test");
        let abs2 = abs1.shallow_simplified();
        assert_eq!(abs1.operand(), abs2.operand());
    }
}
