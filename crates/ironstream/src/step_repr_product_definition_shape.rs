// FILE: step_repr_product_definition_shape.rs
// occt: StepRepr_ProductDefinitionShape

/// StepRepr_ProductDefinitionShape: Representation of STEP entity ProductDefinitionShape
/// Inherits from StepRepr_PropertyDefinition
#[derive(Clone, Debug)]
pub struct StepReprProductDefinitionShape {
    definition: String,
}

impl StepReprProductDefinitionShape {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprProductDefinitionShape {
            definition: String::new(),
        }
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

impl Default for StepReprProductDefinitionShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let pds = StepReprProductDefinitionShape::new();
        assert_eq!(pds.definition(), "");
    }

    #[test]
    fn test_set_definition() {
        let mut pds = StepReprProductDefinitionShape::new();
        pds.set_definition("shape1".to_string());
        assert_eq!(pds.definition(), "shape1");
    }
}
