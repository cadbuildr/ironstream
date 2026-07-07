// FILE: step_fea_curve3d_element_property.rs
// occt: StepFEA_Curve3dElementProperty

/// Representation of STEP entity Curve3dElementProperty.
#[derive(Clone)]
pub struct Curve3dElementProperty {
    property_id: Option<String>,
    description: Option<String>,
    material_direction: Option<String>,
}

impl Curve3dElementProperty {
    pub fn new() -> Self {
        Self {
            property_id: None,
            description: None,
            material_direction: None,
        }
    }

    pub fn init(
        &mut self,
        property_id: Option<String>,
        description: Option<String>,
        material_direction: Option<String>,
    ) {
        self.property_id = property_id;
        self.description = description;
        self.material_direction = material_direction;
    }

    pub fn property_id(&self) -> Option<&str> {
        self.property_id.as_deref()
    }

    pub fn set_property_id(&mut self, id: Option<String>) {
        self.property_id = id;
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description(&mut self, desc: Option<String>) {
        self.description = desc;
    }

    pub fn material_direction(&self) -> Option<&str> {
        self.material_direction.as_deref()
    }

    pub fn set_material_direction(&mut self, dir: Option<String>) {
        self.material_direction = dir;
    }
}

impl Default for Curve3dElementProperty {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let prop = Curve3dElementProperty::new();
        assert!(prop.property_id().is_none());
    }

    #[test]
    fn test_init() {
        let mut prop = Curve3dElementProperty::new();
        prop.init(
            Some("PROP1".to_string()),
            Some("Curve Property".to_string()),
            Some("XYZ".to_string()),
        );

        assert_eq!(prop.property_id(), Some("PROP1"));
        assert_eq!(prop.material_direction(), Some("XYZ"));
    }
}
