// FILE: step_repr_property_definition_representation.rs
// occt: StepRepr_PropertyDefinitionRepresentation

/// StepRepr_PropertyDefinitionRepresentation: Representation of STEP entity PropertyDefinitionRepresentation
#[derive(Clone, Debug)]
pub struct StepReprPropertyDefinitionRepresentation {
    definition: String,             // Simplified: storing identifier
    used_representation: String,    // Simplified: storing identifier
}

impl StepReprPropertyDefinitionRepresentation {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprPropertyDefinitionRepresentation {
            definition: String::new(),
            used_representation: String::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, definition: String, used_representation: String) {
        self.definition = definition;
        self.used_representation = used_representation;
    }

    /// Returns field Definition
    pub fn definition(&self) -> &str {
        &self.definition
    }

    /// Set field Definition
    pub fn set_definition(&mut self, definition: String) {
        self.definition = definition;
    }

    /// Returns field UsedRepresentation
    pub fn used_representation(&self) -> &str {
        &self.used_representation
    }

    /// Set field UsedRepresentation
    pub fn set_used_representation(&mut self, used_representation: String) {
        self.used_representation = used_representation;
    }
}

impl Default for StepReprPropertyDefinitionRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let pdr = StepReprPropertyDefinitionRepresentation::new();
        assert_eq!(pdr.definition(), "");
        assert_eq!(pdr.used_representation(), "");
    }

    #[test]
    fn test_init() {
        let mut pdr = StepReprPropertyDefinitionRepresentation::new();
        pdr.init("def1".to_string(), "repr1".to_string());
        assert_eq!(pdr.definition(), "def1");
        assert_eq!(pdr.used_representation(), "repr1");
    }

    #[test]
    fn test_set_definition() {
        let mut pdr = StepReprPropertyDefinitionRepresentation::new();
        pdr.set_definition("newdef".to_string());
        assert_eq!(pdr.definition(), "newdef");
    }

    #[test]
    fn test_set_used_representation() {
        let mut pdr = StepReprPropertyDefinitionRepresentation::new();
        pdr.set_used_representation("newrepr".to_string());
        assert_eq!(pdr.used_representation(), "newrepr");
    }
}
