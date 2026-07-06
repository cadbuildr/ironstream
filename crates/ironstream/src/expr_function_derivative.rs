// FILE: expr_function_derivative.rs
// occt: Expr_FunctionDerivative

/// Represents the derivative of a function with respect to a variable.
#[derive(Debug, Clone)]
pub struct ExprFunctionDerivative {
    function_name: String,
    variable: String,
    degree: i32,
}

impl ExprFunctionDerivative {
    /// Create a function derivative with the given degree
    pub fn new(func_name: impl Into<String>, var: impl Into<String>, deg: i32) -> Result<Self, String> {
        if deg <= 0 {
            return Err("Degree must be > 0".to_string());
        }
        Ok(Self {
            function_name: func_name.into(),
            variable: var.into(),
            degree: deg,
        })
    }

    /// Get the function name
    pub fn function_name(&self) -> &str {
        &self.function_name
    }

    /// Get the derivation variable
    pub fn variable(&self) -> &str {
        &self.variable
    }

    /// Get the degree of derivation
    pub fn degree(&self) -> i32 {
        self.degree
    }

    /// Return a copy
    pub fn copy(&self) -> Self {
        Self {
            function_name: self.function_name.clone(),
            variable: self.variable.clone(),
            degree: self.degree,
        }
    }

    /// Check if identical to another
    pub fn is_identical(&self, other: &ExprFunctionDerivative) -> bool {
        self.function_name == other.function_name
            && self.variable == other.variable
            && self.degree == other.degree
    }

    /// Check if linear on variable
    pub fn is_linear_on_variable(&self) -> bool {
        false
    }

    /// Get the string name
    pub fn get_string_name(&self) -> String {
        format!(
            "D{}({})/D{}{}",
            if self.degree > 1 {
                self.degree.to_string()
            } else {
                String::new()
            },
            self.function_name,
            self.variable,
            if self.degree > 1 {
                format!("^{}", self.degree)
            } else {
                String::new()
            }
        )
    }

    /// Return a string representation
    pub fn string(&self) -> String {
        format!(
            "Derivative({}, {}, {})",
            self.function_name, self.variable, self.degree
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_derivative() {
        let deriv = ExprFunctionDerivative::new("sin", "x", 1).unwrap();
        assert_eq!(deriv.function_name(), "sin");
        assert_eq!(deriv.variable(), "x");
        assert_eq!(deriv.degree(), 1);
    }

    #[test]
    fn test_create_invalid_degree() {
        assert!(ExprFunctionDerivative::new("sin", "x", 0).is_err());
        assert!(ExprFunctionDerivative::new("sin", "x", -1).is_err());
    }

    #[test]
    fn test_second_derivative() {
        let deriv = ExprFunctionDerivative::new("f", "x", 2).unwrap();
        assert_eq!(deriv.degree(), 2);
    }

    #[test]
    fn test_copy() {
        let deriv1 = ExprFunctionDerivative::new("cos", "y", 1).unwrap();
        let deriv2 = deriv1.copy();
        assert!(deriv1.is_identical(&deriv2));
    }

    #[test]
    fn test_is_identical() {
        let deriv1 = ExprFunctionDerivative::new("f", "x", 1).unwrap();
        let deriv2 = ExprFunctionDerivative::new("f", "x", 1).unwrap();
        let deriv3 = ExprFunctionDerivative::new("f", "x", 2).unwrap();
        assert!(deriv1.is_identical(&deriv2));
        assert!(!deriv1.is_identical(&deriv3));
    }

    #[test]
    fn test_is_linear() {
        let deriv = ExprFunctionDerivative::new("f", "x", 1).unwrap();
        assert!(!deriv.is_linear_on_variable());
    }

    #[test]
    fn test_get_string_name_first_derivative() {
        let deriv = ExprFunctionDerivative::new("sin", "x", 1).unwrap();
        let name = deriv.get_string_name();
        assert!(name.contains("sin"));
        assert!(name.contains("x"));
    }

    #[test]
    fn test_get_string_name_second_derivative() {
        let deriv = ExprFunctionDerivative::new("f", "y", 2).unwrap();
        let name = deriv.get_string_name();
        assert!(name.contains("f"));
        assert!(name.contains("y"));
        assert!(name.contains("2"));
    }

    #[test]
    fn test_string_representation() {
        let deriv = ExprFunctionDerivative::new("g", "z", 3).unwrap();
        let s = deriv.string();
        assert!(s.contains("g"));
        assert!(s.contains("z"));
        assert!(s.contains("3"));
    }
}
