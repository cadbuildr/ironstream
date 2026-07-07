// FILE: step_repr_externally_defined_representation.rs
// occt: StepRepr_ExternallyDefinedRepresentation

/// StepRepr_ExternallyDefinedRepresentation: An externally defined representation
/// Inherits from StepRepr_Representation
#[derive(Clone, Debug)]
pub struct StepReprExternallyDefinedRepresentation {
    name: String,
}

impl StepReprExternallyDefinedRepresentation {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprExternallyDefinedRepresentation {
            name: String::new(),
        }
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for StepReprExternallyDefinedRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let edr = StepReprExternallyDefinedRepresentation::new();
        assert_eq!(edr.name(), "");
    }

    #[test]
    fn test_set_name() {
        let mut edr = StepReprExternallyDefinedRepresentation::new();
        edr.set_name("external_repr".to_string());
        assert_eq!(edr.name(), "external_repr");
    }
}
