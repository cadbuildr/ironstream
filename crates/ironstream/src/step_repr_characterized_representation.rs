// FILE: step_repr_characterized_representation.rs
// occt: StepRepr_CharacterizedRepresentation

/// StepRepr_CharacterizedRepresentation: A representation with a description field.
/// Inherits from StepRepr_Representation.
#[derive(Clone, Debug)]
pub struct StepReprCharacterizedRepresentation {
    name: String,
    description: String,
}

impl StepReprCharacterizedRepresentation {
    /// Create a new StepReprCharacterizedRepresentation
    pub fn new() -> Self {
        StepReprCharacterizedRepresentation {
            name: String::new(),
            description: String::new(),
        }
    }

    /// Initialize with name, description, items, and context
    pub fn init(&mut self, name: String, description: String) {
        self.name = name;
        self.description = description;
    }

    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Get description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for StepReprCharacterizedRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let repr = StepReprCharacterizedRepresentation::new();
        assert_eq!(repr.name(), "");
        assert_eq!(repr.description(), "");
    }

    #[test]
    fn test_init() {
        let mut repr = StepReprCharacterizedRepresentation::new();
        repr.init("test_name".to_string(), "test_description".to_string());
        assert_eq!(repr.name(), "test_name");
        assert_eq!(repr.description(), "test_description");
    }

    #[test]
    fn test_set_description() {
        let mut repr = StepReprCharacterizedRepresentation::new();
        repr.set_description("new_desc".to_string());
        assert_eq!(repr.description(), "new_desc");
    }

    #[test]
    fn test_set_name() {
        let mut repr = StepReprCharacterizedRepresentation::new();
        repr.set_name("new_name".to_string());
        assert_eq!(repr.name(), "new_name");
    }
}
