// FILE: step_repr_structural_response_property.rs
// occt: StepRepr_StructuralResponseProperty

/// Represents a structural response property in STEP, derived from PropertyDefinition.
pub struct StructuralResponseProperty {
    name: Option<String>,
    definition: Option<String>,
}

impl StructuralResponseProperty {
    /// Create a new StructuralResponseProperty
    pub fn new() -> Self {
        StructuralResponseProperty {
            name: None,
            definition: None,
        }
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the definition
    pub fn definition(&self) -> Option<&str> {
        self.definition.as_deref()
    }

    /// Set the definition
    pub fn set_definition(&mut self, definition: String) {
        self.definition = Some(definition);
    }
}

impl Default for StructuralResponseProperty {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let prop = StructuralResponseProperty::new();
        assert_eq!(prop.name(), None);
        assert_eq!(prop.definition(), None);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut prop = StructuralResponseProperty::new();
        prop.set_name("StructuralProp".to_string());
        assert_eq!(prop.name(), Some("StructuralProp"));
    }

    #[test]
    fn test_set_and_get_definition() {
        let mut prop = StructuralResponseProperty::new();
        prop.set_definition("Property definition".to_string());
        assert_eq!(prop.definition(), Some("Property definition"));
    }
}
