// FILE: expr_general_function.rs
// occt: Expr_GeneralFunction

/// Abstract base class defining the general purposes of any function.
pub trait ExprGeneralFunction {
    /// Returns the number of variables
    fn nb_of_variables(&self) -> usize;

    /// Returns the variable denoted by the given index (1-based)
    fn variable(&self, index: usize) -> Option<String>;

    /// Returns a copy with the same form
    fn copy(&self) -> Box<dyn ExprGeneralFunction>;

    /// Returns the derivative for the given variable
    fn derivative(&self, var: &str) -> Option<Box<dyn ExprGeneralFunction>>;

    /// Returns the nth derivative for the given variable
    fn derivative_n(&self, var: &str, degree: usize) -> Option<Box<dyn ExprGeneralFunction>>;

    /// Evaluates the function with given variables and values
    fn evaluate(&self, vars: &[&str], vals: &[f64]) -> Result<f64, String>;

    /// Tests if identical to another function
    fn is_identical(&self, other: &dyn ExprGeneralFunction) -> bool;

    /// Tests if linear on the variable at the given index
    fn is_linear_on_variable(&self, index: usize) -> bool;

    /// Returns the string name
    fn get_string_name(&self) -> String;
}

/// A simple concrete implementation
#[derive(Debug, Clone)]
pub struct SimpleFunction {
    name: String,
    variables: Vec<String>,
}

impl SimpleFunction {
    /// Create a new simple function
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            variables: Vec::new(),
        }
    }

    /// Add a variable
    pub fn add_variable(&mut self, var: impl Into<String>) {
        self.variables.push(var.into());
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl ExprGeneralFunction for SimpleFunction {
    fn nb_of_variables(&self) -> usize {
        self.variables.len()
    }

    fn variable(&self, index: usize) -> Option<String> {
        if index == 0 || index > self.variables.len() {
            None
        } else {
            self.variables.get(index - 1).cloned()
        }
    }

    fn copy(&self) -> Box<dyn ExprGeneralFunction> {
        Box::new(self.clone())
    }

    fn derivative(&self, _var: &str) -> Option<Box<dyn ExprGeneralFunction>> {
        None // Simplified: no derivative implemented
    }

    fn derivative_n(&self, _var: &str, _degree: usize) -> Option<Box<dyn ExprGeneralFunction>> {
        None // Simplified: no derivative implemented
    }

    fn evaluate(&self, _vars: &[&str], _vals: &[f64]) -> Result<f64, String> {
        Err("evaluate not implemented".to_string())
    }

    fn is_identical(&self, _other: &dyn ExprGeneralFunction) -> bool {
        false
    }

    fn is_linear_on_variable(&self, _index: usize) -> bool {
        false
    }

    fn get_string_name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_simple_function() {
        let func = SimpleFunction::new("sin");
        assert_eq!(func.name(), "sin");
        assert_eq!(func.nb_of_variables(), 0);
    }

    #[test]
    fn test_add_variables() {
        let mut func = SimpleFunction::new("f");
        func.add_variable("x");
        func.add_variable("y");
        assert_eq!(func.nb_of_variables(), 2);
        assert_eq!(func.variable(1), Some("x".to_string()));
        assert_eq!(func.variable(2), Some("y".to_string()));
    }

    #[test]
    fn test_variable_indexing() {
        let mut func = SimpleFunction::new("g");
        func.add_variable("a");
        assert_eq!(func.variable(0), None);
        assert_eq!(func.variable(1), Some("a".to_string()));
        assert_eq!(func.variable(2), None);
    }

    #[test]
    fn test_copy() {
        let mut func1 = SimpleFunction::new("cos");
        func1.add_variable("z");
        let func2 = func1.copy();
        assert_eq!(func2.get_string_name(), "cos");
        assert_eq!(func2.nb_of_variables(), 1);
    }

    #[test]
    fn test_get_string_name() {
        let func = SimpleFunction::new("tan");
        assert_eq!(func.get_string_name(), "tan");
    }

    #[test]
    fn test_is_linear() {
        let func = SimpleFunction::new("exp");
        assert!(!func.is_linear_on_variable(1));
    }
}
