// FILE: step_repr_material_designation.rs
// occt: StepRepr_MaterialDesignation

/// StepRepr_MaterialDesignation: Material designation entity
#[derive(Clone, Debug)]
pub struct StepReprMaterialDesignation {
    name: String,
    of_definition: i32, // Simplified: storing definition case number
}

impl StepReprMaterialDesignation {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprMaterialDesignation {
            name: String::new(),
            of_definition: 0,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, of_definition: i32) {
        self.name = name;
        self.of_definition = of_definition;
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set definition
    pub fn set_of_definition(&mut self, definition: i32) {
        self.of_definition = definition;
    }

    /// Get definition
    pub fn of_definition(&self) -> i32 {
        self.of_definition
    }
}

impl Default for StepReprMaterialDesignation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let md = StepReprMaterialDesignation::new();
        assert_eq!(md.name(), "");
        assert_eq!(md.of_definition(), 0);
    }

    #[test]
    fn test_init() {
        let mut md = StepReprMaterialDesignation::new();
        md.init("material1".to_string(), 2);
        assert_eq!(md.name(), "material1");
        assert_eq!(md.of_definition(), 2);
    }

    #[test]
    fn test_set_name() {
        let mut md = StepReprMaterialDesignation::new();
        md.set_name("new_material".to_string());
        assert_eq!(md.name(), "new_material");
    }

    #[test]
    fn test_set_of_definition() {
        let mut md = StepReprMaterialDesignation::new();
        md.set_of_definition(3);
        assert_eq!(md.of_definition(), 3);
    }
}
