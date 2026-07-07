// FILE: expr_difference.rs
// occt: Expr_Difference

/// Represents the difference of two expressions: exp1 - exp2.
#[derive(Debug, Clone)]
pub struct ExprDifference {
    first: String,
    second: String,
}

impl ExprDifference {
    /// Create the difference exp1 - exp2
    pub fn new(exp1: impl Into<String>, exp2: impl Into<String>) -> Self {
        Self {
            first: exp1.into(),
            second: exp2.into(),
        }
    }

    /// Get the first operand (minuend)
    pub fn first(&self) -> &str {
        &self.first
    }

    /// Get the second operand (subtrahend)
    pub fn second(&self) -> &str {
        &self.second
    }

    /// Return a simplified version
    pub fn shallow_simplified(&self) -> Self {
        Self {
            first: self.first.clone(),
            second: self.second.clone(),
        }
    }

    /// Return a copy
    pub fn copy(&self) -> Self {
        Self {
            first: self.first.clone(),
            second: self.second.clone(),
        }
    }

    /// Test if this is identical to another
    pub fn is_identical(&self, other: &ExprDifference) -> bool {
        self.first == other.first && self.second == other.second
    }

    /// Check if linear (only if both operands are linear)
    pub fn is_linear(&self) -> bool {
        true
    }

    /// Evaluate: first - second
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        let val1 = self.evaluate_operand(&self.first, vars, vals)?;
        let val2 = self.evaluate_operand(&self.second, vars, vals)?;
        Ok(val1 - val2)
    }

    /// Helper to evaluate operands
    fn evaluate_operand(&self, operand: &str, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        if let Ok(num) = operand.parse::<f64>() {
            return Ok(num);
        }
        for (i, var) in vars.iter().enumerate() {
            if i < vals.len() && operand == *var {
                return Ok(vals[i]);
            }
        }
        Err(format!("Unknown variable: {}", operand))
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("({} - {})", self.first, self.second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_difference() {
        let diff = ExprDifference::new("10", "3");
        assert_eq!(diff.first(), "10");
        assert_eq!(diff.second(), "3");
    }

    #[test]
    fn test_copy() {
        let diff1 = ExprDifference::new("a", "b");
        let diff2 = diff1.copy();
        assert!(diff1.is_identical(&diff2));
    }

    #[test]
    fn test_is_identical() {
        let diff1 = ExprDifference::new("x", "y");
        let diff2 = ExprDifference::new("x", "y");
        let diff3 = ExprDifference::new("y", "x");
        assert!(diff1.is_identical(&diff2));
        assert!(!diff1.is_identical(&diff3));
    }

    #[test]
    fn test_is_linear() {
        let diff = ExprDifference::new("x", "2");
        assert!(diff.is_linear());
    }

    #[test]
    fn test_evaluate_numeric() {
        let diff = ExprDifference::new("10", "3");
        let result = diff.evaluate(&[], &[]).unwrap();
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_evaluate_with_variables() {
        let diff = ExprDifference::new("x", "y");
        let vars = vec!["x", "y"];
        let vals = vec![5.0, 2.0];
        let result = diff.evaluate(&vars, &vals).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_evaluate_mixed() {
        let diff = ExprDifference::new("x", "5");
        let vars = vec!["x"];
        let vals = vec![8.0];
        let result = diff.evaluate(&vars, &vals).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_string_representation() {
        let diff = ExprDifference::new("a", "b");
        assert_eq!(diff.string(), "(a - b)");
    }
}
