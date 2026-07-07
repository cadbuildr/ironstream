// FILE: t_data_std_variable.rs
// occt: TDataStd_Variable

/// A Variable attribute representing a named variable with a value and optional expression.
/// Variables can be constant or assigned to expressions.
#[derive(Clone, Debug)]
pub struct TDataStd_Variable {
    name: String,
    value: Option<f64>,
    unit: String,
    is_constant: bool,
    is_assigned: bool,
    expression: Option<String>,
    id: [u8; 16],
}

impl TDataStd_Variable {
    /// Create a new Variable attribute.
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value: None,
            unit: String::new(),
            is_constant: false,
            is_assigned: false,
            expression: None,
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for Variable attributes.
    pub fn get_id() -> [u8; 16] {
        // Standard OCCT GUID for TDataStd_Variable
        [
            0x1E, 0x7C, 0xB5, 0xA2, 0x2F, 0xD1, 0x43, 0x6C, 0x95, 0x4A, 0x7D, 0x61, 0x88, 0x22,
            0x22, 0x22,
        ]
    }

    /// Set the name of the variable.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get the name of the variable.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the value of the variable.
    pub fn set_value(&mut self, value: f64) {
        self.value = Some(value);
    }

    /// Check if the variable has a value.
    pub fn is_valued(&self) -> bool {
        self.value.is_some()
    }

    /// Get the value of the variable.
    pub fn get_value(&self) -> Option<f64> {
        self.value
    }

    /// Set the unit of the variable.
    pub fn set_unit(&mut self, unit: String) {
        self.unit = unit;
    }

    /// Get the unit of the variable.
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Set whether the variable is constant.
    pub fn set_constant(&mut self, is_constant: bool) {
        self.is_constant = is_constant;
    }

    /// Check if the variable is constant.
    pub fn is_constant(&self) -> bool {
        self.is_constant
    }

    /// Assign an expression to the variable.
    pub fn assign(&mut self, expression: String) {
        self.expression = Some(expression);
        self.is_assigned = true;
    }

    /// Check if the variable has an assigned expression.
    pub fn is_assigned(&self) -> bool {
        self.is_assigned && self.expression.is_some()
    }

    /// Get the assigned expression.
    pub fn expression(&self) -> Option<&str> {
        self.expression.as_deref()
    }

    /// Remove the assigned expression.
    pub fn desassign(&mut self) {
        self.expression = None;
        self.is_assigned = false;
    }

    /// Get the ID of this attribute.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }
}

impl Default for TDataStd_Variable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_variable() {
        let var = TDataStd_Variable::new();
        assert_eq!(var.name(), "");
        assert!(!var.is_valued());
        assert!(!var.is_assigned());
    }

    #[test]
    fn test_set_name() {
        let mut var = TDataStd_Variable::new();
        var.set_name("x".to_string());
        assert_eq!(var.name(), "x");
    }

    #[test]
    fn test_set_value() {
        let mut var = TDataStd_Variable::new();
        var.set_value(3.14);
        assert!(var.is_valued());
        assert_eq!(var.get_value(), Some(3.14));
    }

    #[test]
    fn test_set_unit() {
        let mut var = TDataStd_Variable::new();
        var.set_unit("mm".to_string());
        assert_eq!(var.unit(), "mm");
    }

    #[test]
    fn test_constant() {
        let mut var = TDataStd_Variable::new();
        assert!(!var.is_constant());
        var.set_constant(true);
        assert!(var.is_constant());
    }

    #[test]
    fn test_assign_expression() {
        let mut var = TDataStd_Variable::new();
        var.assign("x + y".to_string());
        assert!(var.is_assigned());
        assert_eq!(var.expression(), Some("x + y"));
    }

    #[test]
    fn test_desassign() {
        let mut var = TDataStd_Variable::new();
        var.assign("x + y".to_string());
        assert!(var.is_assigned());
        var.desassign();
        assert!(!var.is_assigned());
    }

    #[test]
    fn test_default() {
        let var = TDataStd_Variable::default();
        assert_eq!(var.name(), "");
    }
}
