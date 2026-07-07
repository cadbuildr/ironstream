// FILE: step_element_surface_section_field_varying.rs
// occt: StepElement_SurfaceSectionFieldVarying

/// Representation of STEP entity SurfaceSectionFieldVarying.
/// Inherits from SurfaceSectionField.
#[derive(Clone)]
pub struct SurfaceSectionFieldVarying {
    definitions: Option<Vec<String>>,
    additional_node_values: bool,
}

impl SurfaceSectionFieldVarying {
    /// Creates a new SurfaceSectionFieldVarying.
    pub fn new() -> Self {
        Self {
            definitions: None,
            additional_node_values: false,
        }
    }

    /// Initializes all fields.
    pub fn init(&mut self, definitions: Option<Vec<String>>, additional_node_values: bool) {
        self.definitions = definitions;
        self.additional_node_values = additional_node_values;
    }

    pub fn definitions(&self) -> Option<&Vec<String>> {
        self.definitions.as_ref()
    }

    pub fn set_definitions(&mut self, defs: Option<Vec<String>>) {
        self.definitions = defs;
    }

    pub fn additional_node_values(&self) -> bool {
        self.additional_node_values
    }

    pub fn set_additional_node_values(&mut self, val: bool) {
        self.additional_node_values = val;
    }
}

impl Default for SurfaceSectionFieldVarying {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let field = SurfaceSectionFieldVarying::new();
        assert!(field.definitions().is_none());
        assert!(!field.additional_node_values());
    }

    #[test]
    fn test_init() {
        let mut field = SurfaceSectionFieldVarying::new();
        let defs = vec!["def1".to_string(), "def2".to_string()];

        field.init(Some(defs.clone()), true);

        assert_eq!(field.definitions().unwrap().len(), 2);
        assert!(field.additional_node_values());
    }

    #[test]
    fn test_setters() {
        let mut field = SurfaceSectionFieldVarying::new();
        let defs = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        field.set_definitions(Some(defs.clone()));
        field.set_additional_node_values(true);

        assert_eq!(field.definitions().unwrap().len(), 3);
        assert!(field.additional_node_values());
    }
}
