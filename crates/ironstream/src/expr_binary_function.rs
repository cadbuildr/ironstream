// FILE: expr_binary_function.rs
// occt: Expr_BinaryFunction

/// Defines the use of a binary function in an expression with given arguments.
#[derive(Debug, Clone)]
pub struct ExprBinaryFunction {
    function: String,
    first_expr: String,
    second_expr: String,
}

impl ExprBinaryFunction {
    /// Create a binary function with name and two expressions
    pub fn new(func: impl Into<String>, exp1: impl Into<String>, exp2: impl Into<String>) -> Self {
        Self {
            function: func.into(),
            first_expr: exp1.into(),
            second_expr: exp2.into(),
        }
    }

    /// Get the function name
    pub fn function(&self) -> &str {
        &self.function
    }

    /// Get the first expression
    pub fn first_expression(&self) -> &str {
        &self.first_expr
    }

    /// Get the second expression
    pub fn second_expression(&self) -> &str {
        &self.second_expr
    }

    /// Return a simplified version of the expression
    pub fn shallow_simplified(&self) -> Self {
        Self {
            function: self.function.clone(),
            first_expr: self.first_expr.clone(),
            second_expr: self.second_expr.clone(),
        }
    }

    /// Return a copy of this expression
    pub fn copy(&self) -> Self {
        Self {
            function: self.function.clone(),
            first_expr: self.first_expr.clone(),
            second_expr: self.second_expr.clone(),
        }
    }

    /// Test if this expression is identical to another
    pub fn is_identical(&self, other: &ExprBinaryFunction) -> bool {
        self.function == other.function
            && self.first_expr == other.first_expr
            && self.second_expr == other.second_expr
    }

    /// Check if the expression is linear
    pub fn is_linear(&self) -> bool {
        false // Binary functions are generally not linear
    }

    /// Evaluate the expression (simple numeric evaluation)
    pub fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String> {
        // Parse first argument
        let val1 = if let Ok(num) = self.first_expr.parse::<f64>() {
            num
        } else {
            // Look up variable
            let mut found = false;
            let mut result = 0.0;
            for (i, var) in vars.iter().enumerate() {
                if i < vals.len() && self.first_expr == *var {
                    result = vals[i];
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(format!("Unknown variable: {}", self.first_expr));
            }
            result
        };

        // Parse second argument
        let val2 = if let Ok(num) = self.second_expr.parse::<f64>() {
            num
        } else {
            // Look up variable
            let mut found = false;
            let mut result = 0.0;
            for (i, var) in vars.iter().enumerate() {
                if i < vals.len() && self.second_expr == *var {
                    result = vals[i];
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(format!("Unknown variable: {}", self.second_expr));
            }
            result
        };

        // Apply function
        match self.function.as_str() {
            "pow" => Ok(val1.powf(val2)),
            "atan2" => Ok(val1.atan2(val2)),
            "max" => Ok(val1.max(val2)),
            "min" => Ok(val1.min(val2)),
            _ => Err(format!("Unknown function: {}", self.function)),
        }
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!("{}({}, {})", self.function, self.first_expr, self.second_expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_binary_function() {
        let func = ExprBinaryFunction::new("pow", "2", "3");
        assert_eq!(func.function(), "pow");
        assert_eq!(func.first_expression(), "2");
        assert_eq!(func.second_expression(), "3");
    }

    #[test]
    fn test_copy() {
        let func1 = ExprBinaryFunction::new("max", "a", "b");
        let func2 = func1.copy();
        assert!(func1.is_identical(&func2));
    }

    #[test]
    fn test_is_identical() {
        let func1 = ExprBinaryFunction::new("min", "x", "y");
        let func2 = ExprBinaryFunction::new("min", "x", "y");
        let func3 = ExprBinaryFunction::new("max", "x", "y");
        assert!(func1.is_identical(&func2));
        assert!(!func1.is_identical(&func3));
    }

    #[test]
    fn test_is_linear() {
        let func = ExprBinaryFunction::new("pow", "x", "2");
        assert!(!func.is_linear());
    }

    #[test]
    fn test_evaluate_pow() {
        let func = ExprBinaryFunction::new("pow", "2", "3");
        let result = func.evaluate(&[], &[]).unwrap();
        assert!((result - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_atan2() {
        let func = ExprBinaryFunction::new("atan2", "1", "1");
        let result = func.evaluate(&[], &[]).unwrap();
        let expected = 1.0f64.atan2(1.0);
        assert!((result - expected).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_max() {
        let func = ExprBinaryFunction::new("max", "5", "3");
        let result = func.evaluate(&[], &[]).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_evaluate_min() {
        let func = ExprBinaryFunction::new("min", "5", "3");
        let result = func.evaluate(&[], &[]).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_evaluate_with_variables() {
        let func = ExprBinaryFunction::new("max", "x", "y");
        let vars = vec!["x", "y"];
        let vals = vec![2.5, 7.3];
        let result = func.evaluate(&vars, &vals).unwrap();
        assert_eq!(result, 7.3);
    }

    #[test]
    fn test_string_representation() {
        let func = ExprBinaryFunction::new("pow", "2", "3");
        assert_eq!(func.string(), "pow(2, 3)");
    }
}
