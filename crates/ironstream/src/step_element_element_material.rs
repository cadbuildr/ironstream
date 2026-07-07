// FILE: step_element_element_material.rs
// occt: StepElement_ElementMaterial

/// Representation of STEP entity ElementMaterial.
#[derive(Clone)]
pub struct ElementMaterial {
    material_id: Option<String>,
    description: Option<String>,
    properties: Option<Vec<String>>,
}

impl ElementMaterial {
    /// Creates a new ElementMaterial.
    pub fn new() -> Self {
        Self {
            material_id: None,
            description: None,
            properties: None,
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        material_id: Option<String>,
        description: Option<String>,
        properties: Option<Vec<String>>,
    ) {
        self.material_id = material_id;
        self.description = description;
        self.properties = properties;
    }

    pub fn material_id(&self) -> Option<&str> {
        self.material_id.as_deref()
    }

    pub fn set_material_id(&mut self, id: Option<String>) {
        self.material_id = id;
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description(&mut self, desc: Option<String>) {
        self.description = desc;
    }

    pub fn properties(&self) -> Option<&Vec<String>> {
        self.properties.as_ref()
    }

    pub fn set_properties(&mut self, props: Option<Vec<String>>) {
        self.properties = props;
    }
}

impl Default for ElementMaterial {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let mat = ElementMaterial::new();
        assert!(mat.material_id().is_none());
        assert!(mat.description().is_none());
        assert!(mat.properties().is_none());
    }

    #[test]
    fn test_init() {
        let mut mat = ElementMaterial::new();
        let props = vec!["prop1".to_string(), "prop2".to_string()];
        mat.init(
            Some("Steel".to_string()),
            Some("Steel material".to_string()),
            Some(props.clone()),
        );

        assert_eq!(mat.material_id(), Some("Steel"));
        assert_eq!(mat.description(), Some("Steel material"));
        assert_eq!(mat.properties().unwrap().len(), 2);
    }

    #[test]
    fn test_setters() {
        let mut mat = ElementMaterial::new();
        mat.set_material_id(Some("Aluminum".to_string()));
        mat.set_description(Some("Aluminum alloy".to_string()));

        assert_eq!(mat.material_id(), Some("Aluminum"));
        assert_eq!(mat.description(), Some("Aluminum alloy"));
    }
}
