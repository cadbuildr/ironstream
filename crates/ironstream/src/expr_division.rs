// FILE: expr_division.rs
// occt: Expr_Division

/// Represents the division of two expressions: exp1 / exp2.
#[derive(Debug, Clone)]
pub struct ExprDivision {
    first: String,
    second: String,
}

impl ExprDivision {
    /// Create the division exp1 / exp2
    pub fn new(exp1: impl Into<String>, exp2: impl Into<String>) -> Self {
        Self {
            first: exp1.into(),
            second: exp2.into(),
        }
    }

    /// Get the dividend (numerator)
    pub fn first(&self) -> &str {
        &self.first
    }

    /// Get the divisor (denominator)
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

    /// Test if identical
    pub fn is_identical(&self, other: &ExprDivision) -> bool {
        self.first == other.first && self.second == other.second
    }

    /// Check if linear
    pub fn is_linear(&self) -> bool {
        false
    }

    /// Evaluate: first / second
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        let val1 = self.evaluate_operand(&self.first, vars, vals)?;
        let val2 = self.evaluate_operand(&self.second, vars, vals)?;
        if val2 == 0.0 {
            return Err("Division by zero".to_string());
        }
        Ok(val1 / val2)
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
        format!("({} / {})", self.first, self.second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_division() {
        let div = ExprDivision::new("10", "2");
        assert_eq!(div.first(), "10");
        assert_eq!(div.second(), "2");
    }

    #[test]
    fn test_copy() {
        let div1 = ExprDivision::new("a", "b");
        let div2 = div1.copy();
        assert!(div1.is_identical(&div2));
    }

    #[test]
    fn test_is_identical() {
        let div1 = ExprDivision::new("x", "y");
        let div2 = ExprDivision::new("x", "y");
        let div3 = ExprDivision::new("y", "x");
        assert!(div1.is_identical(&div2));
        assert!(!div1.is_identical(&div3));
    }

    #[test]
    fn test_is_linear() {
        let div = ExprDivision::new("x", "2");
        assert!(!div.is_linear());
    }

    #[test]
    fn test_evaluate_numeric() {
        let div = ExprDivision::new("10", "2");
        let result = div.evaluate(&[], &[]).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_evaluate_with_variables() {
        let div = ExprDivision::new("x", "y");
        let vars = vec!["x", "y"];
        let vals = vec![12.0, 3.0];
        let result = div.evaluate(&vars, &vals).unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_evaluate_division_by_zero() {
        let div = ExprDivision::new("5", "0");
        let result = div.evaluate(&[], &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_string_representation() {
        let div = ExprDivision::new("a", "b");
        assert_eq!(div.string(), "(a / b)");
    }
}
