// FILE: step_repr_material_property_representation.rs
// occt: StepRepr_MaterialPropertyRepresentation

/// StepRepr_MaterialPropertyRepresentation: Material property representation
/// Inherits from StepRepr_PropertyDefinitionRepresentation
#[derive(Clone, Debug)]
pub struct StepReprMaterialPropertyRepresentation {
    definition: String,
    dependent_environment: String, // Simplified: storing identifier
}

impl StepReprMaterialPropertyRepresentation {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprMaterialPropertyRepresentation {
            definition: String::new(),
            dependent_environment: String::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, definition: String, dependent_environment: String) {
        self.definition = definition;
        self.dependent_environment = dependent_environment;
    }

    /// Returns field DependentEnvironment
    pub fn dependent_environment(&self) -> &str {
        &self.dependent_environment
    }

    /// Set field DependentEnvironment
    pub fn set_dependent_environment(&mut self, environment: String) {
        self.dependent_environment = environment;
    }

    /// Get definition
    pub fn definition(&self) -> &str {
        &self.definition
    }

    /// Set definition
    pub fn set_definition(&mut self, definition: String) {
        self.definition = definition;
    }
}

impl Default for StepReprMaterialPropertyRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let mpr = StepReprMaterialPropertyRepresentation::new();
        assert_eq!(mpr.definition(), "");
        assert_eq!(mpr.dependent_environment(), "");
    }

    #[test]
    fn test_init() {
        let mut mpr = StepReprMaterialPropertyRepresentation::new();
        mpr.init("def1".to_string(), "env1".to_string());
        assert_eq!(mpr.definition(), "def1");
        assert_eq!(mpr.dependent_environment(), "env1");
    }

    #[test]
    fn test_set_dependent_environment() {
        let mut mpr = StepReprMaterialPropertyRepresentation::new();
        mpr.set_dependent_environment("new_env".to_string());
        assert_eq!(mpr.dependent_environment(), "new_env");
    }

    #[test]
    fn test_set_definition() {
        let mut mpr = StepReprMaterialPropertyRepresentation::new();
        mpr.set_definition("new_def".to_string());
        assert_eq!(mpr.definition(), "new_def");
    }
}
