// FILE: std_l_persistent_variable.rs
// occt: StdLPersistent_Variable

/// Persistent variable attribute
pub struct StdLPersistentVariable {
    is_constant: bool,
    unit: String,
}

impl StdLPersistentVariable {
    /// Create empty variable
    pub fn new() -> Self {
        StdLPersistentVariable {
            is_constant: false,
            unit: String::new(),
        }
    }

    /// Check if variable is constant
    pub fn is_constant(&self) -> bool {
        self.is_constant
    }

    /// Set constant flag
    pub fn set_constant(&mut self, constant: bool) {
        self.is_constant = constant;
    }

    /// Get the unit
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Set the unit
    pub fn set_unit(&mut self, unit: &str) {
        self.unit = unit.to_string();
    }
}

impl Default for StdLPersistentVariable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let var = StdLPersistentVariable::new();
        assert!(!var.is_constant());
        assert_eq!(var.unit(), "");
    }

    #[test]
    fn test_constant() {
        let mut var = StdLPersistentVariable::new();
        var.set_constant(true);
        assert!(var.is_constant());
    }

    #[test]
    fn test_unit() {
        let mut var = StdLPersistentVariable::new();
        var.set_unit("mm");
        assert_eq!(var.unit(), "mm");
    }
}
