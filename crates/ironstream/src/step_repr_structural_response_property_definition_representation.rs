// FILE: step_repr_structural_response_property_definition_representation.rs
// occt: StepRepr_StructuralResponsePropertyDefinitionRepresentation

/// Represents the structural response property definition representation in STEP.
pub struct StructuralResponsePropertyDefinitionRepresentation {
    property_definition: Option<String>,
    representation: Option<String>,
}

impl StructuralResponsePropertyDefinitionRepresentation {
    /// Create a new StructuralResponsePropertyDefinitionRepresentation
    pub fn new() -> Self {
        StructuralResponsePropertyDefinitionRepresentation {
            property_definition: None,
            representation: None,
        }
    }

    /// Get the property definition
    pub fn property_definition(&self) -> Option<&str> {
        self.property_definition.as_deref()
    }

    /// Set the property definition
    pub fn set_property_definition(&mut self, property_definition: String) {
        self.property_definition = Some(property_definition);
    }

    /// Get the representation
    pub fn representation(&self) -> Option<&str> {
        self.representation.as_deref()
    }

    /// Set the representation
    pub fn set_representation(&mut self, representation: String) {
        self.representation = Some(representation);
    }
}

impl Default for StructuralResponsePropertyDefinitionRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let srp = StructuralResponsePropertyDefinitionRepresentation::new();
        assert_eq!(srp.property_definition(), None);
        assert_eq!(srp.representation(), None);
    }

    #[test]
    fn test_set_and_get_property_definition() {
        let mut srp = StructuralResponsePropertyDefinitionRepresentation::new();
        srp.set_property_definition("PropertyDef".to_string());
        assert_eq!(srp.property_definition(), Some("PropertyDef"));
    }

    #[test]
    fn test_set_and_get_representation() {
        let mut srp = StructuralResponsePropertyDefinitionRepresentation::new();
        srp.set_representation("Representation".to_string());
        assert_eq!(srp.representation(), Some("Representation"));
    }
}
