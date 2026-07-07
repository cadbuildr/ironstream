// FILE: step_repr_material_property.rs
// occt: StepRepr_MaterialProperty

/// StepRepr_MaterialProperty: Material property entity
/// Inherits from StepRepr_PropertyDefinition
#[derive(Clone, Debug)]
pub struct StepReprMaterialProperty {
    definition: String, // Simplified: storing definition identifier
}

impl StepReprMaterialProperty {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprMaterialProperty {
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

impl Default for StepReprMaterialProperty {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let mp = StepReprMaterialProperty::new();
        assert_eq!(mp.definition(), "");
    }

    #[test]
    fn test_set_definition() {
        let mut mp = StepReprMaterialProperty::new();
        mp.set_definition("def1".to_string());
        assert_eq!(mp.definition(), "def1");
    }
}
