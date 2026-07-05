// FILE: expr_intrp_generator.rs
// occt: ExprIntrp_Generator

//! Base generator for expression interpreter components.

use std::collections::HashMap;

/// Expression generator base
pub struct ExprIntrpGenerator {
    variables: HashMap<String, f64>,
    functions: HashMap<String, String>,
}

impl ExprIntrpGenerator {
    /// Create a new generator
    pub fn new() -> Self {
        ExprIntrpGenerator {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    /// Define a variable
    pub fn define_variable(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
    }

    /// Get a variable value
    pub fn get_variable(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }

    /// Define a function
    pub fn define_function(&mut self, name: &str, definition: &str) {
        self.functions.insert(name.to_string(), definition.to_string());
    }
}

impl Default for ExprIntrpGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creation() {
        let gen = ExprIntrpGenerator::new();
        assert!(gen.variables.is_empty());
    }

    #[test]
    fn test_define_variable() {
        let mut gen = ExprIntrpGenerator::new();
        gen.define_variable("x", 10.0);
        assert_eq!(gen.get_variable("x"), Some(10.0));
    }
}
