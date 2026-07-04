// FILE: t_data_std_expression.rs
// occt: TDataStd_Expression

use std::fmt;

/// Expression attribute for storing and managing mathematical expressions.
/// Stores a string expression and references to variables used by it.
pub struct TDataStd_Expression {
    expression: String,
    variables: Vec<String>,
    id: [u8; 16],
}

impl TDataStd_Expression {
    /// Create a new Expression attribute.
    pub fn new() -> Self {
        Self {
            expression: String::new(),
            variables: Vec::new(),
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for Expression attributes.
    pub fn get_id() -> [u8; 16] {
        // Standard OCCT GUID for TDataStd_Expression
        [
            0x13, 0x40, 0x1A, 0xD1, 0x2F, 0x6B, 0x48, 0x73, 0xB5, 0x2B, 0x3E, 0x80, 0x51, 0xC7,
            0x22, 0x22,
        ]
    }

    /// Set the expression string.
    pub fn set_expression(&mut self, expr: String) {
        self.expression = expr;
    }

    /// Get the expression string.
    pub fn get_expression(&self) -> &str {
        &self.expression
    }

    /// Build and return the expression name.
    pub fn name(&self) -> String {
        format!("Expression({})", self.expression)
    }

    /// Add a variable reference.
    pub fn add_variable(&mut self, var: String) {
        if !self.variables.contains(&var) {
            self.variables.push(var);
        }
    }

    /// Get the list of variable references.
    pub fn get_variables(&self) -> &[String] {
        &self.variables
    }

    /// Get mutable access to variables.
    pub fn get_variables_mut(&mut self) -> &mut Vec<String> {
        &mut self.variables
    }

    /// Get the ID of this attribute.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }
}

impl Default for TDataStd_Expression {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TDataStd_Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TDataStd_Expression")
            .field("expression", &self.expression)
            .field("variables", &self.variables)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_expression() {
        let expr = TDataStd_Expression::new();
        assert_eq!(expr.get_expression(), "");
        assert_eq!(expr.get_variables().len(), 0);
    }

    #[test]
    fn test_set_expression() {
        let mut expr = TDataStd_Expression::new();
        let expression_str = "x + y * 2".to_string();
        expr.set_expression(expression_str.clone());
        assert_eq!(expr.get_expression(), expression_str);
    }

    #[test]
    fn test_name() {
        let mut expr = TDataStd_Expression::new();
        expr.set_expression("x + 1".to_string());
        let name = expr.name();
        assert!(name.contains("Expression"));
        assert!(name.contains("x + 1"));
    }

    #[test]
    fn test_add_variable() {
        let mut expr = TDataStd_Expression::new();
        expr.add_variable("x".to_string());
        expr.add_variable("y".to_string());
        expr.add_variable("x".to_string()); // Duplicate
        assert_eq!(expr.get_variables().len(), 2);
        assert!(expr.get_variables().contains(&"x".to_string()));
        assert!(expr.get_variables().contains(&"y".to_string()));
    }

    #[test]
    fn test_get_id() {
        let id1 = TDataStd_Expression::get_id();
        let id2 = TDataStd_Expression::get_id();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_default() {
        let expr = TDataStd_Expression::default();
        assert_eq!(expr.get_expression(), "");
    }
}
