// FILE: step_element_surface_section_field_constant.rs
// occt: StepElement_SurfaceSectionFieldConstant

/// Representation of STEP entity SurfaceSectionFieldConstant.
/// Inherits from SurfaceSectionField.
#[derive(Clone)]
pub struct SurfaceSectionFieldConstant {
    definition: Option<Box<String>>,
}

impl SurfaceSectionFieldConstant {
    /// Creates a new SurfaceSectionFieldConstant.
    pub fn new() -> Self {
        Self {
            definition: None,
        }
    }

    /// Initializes the definition field.
    pub fn init(&mut self, definition: Option<String>) {
        self.definition = definition.map(Box::new);
    }

    pub fn definition(&self) -> Option<&str> {
        self.definition.as_ref().map(|d| d.as_str())
    }

    pub fn set_definition(&mut self, def: Option<String>) {
        self.definition = def.map(Box::new);
    }
}

impl Default for SurfaceSectionFieldConstant {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let field = SurfaceSectionFieldConstant::new();
        assert!(field.definition().is_none());
    }

    #[test]
    fn test_init() {
        let mut field = SurfaceSectionFieldConstant::new();
        field.init(Some("ConstantSection".to_string()));
        assert_eq!(field.definition(), Some("ConstantSection"));
    }

    #[test]
    fn test_set_definition() {
        let mut field = SurfaceSectionFieldConstant::new();
        field.set_definition(Some("NewDef".to_string()));
        assert_eq!(field.definition(), Some("NewDef"));

        field.set_definition(None);
        assert!(field.definition().is_none());
    }
}
