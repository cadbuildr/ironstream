// FILE: step_element_surface_element_property.rs
// occt: StepElement_SurfaceElementProperty

/// Representation of STEP entity SurfaceElementProperty.
#[derive(Clone)]
pub struct SurfaceElementProperty {
    property_id: Option<String>,
    description: Option<String>,
    section: Option<String>,
}

impl SurfaceElementProperty {
    /// Creates a new SurfaceElementProperty.
    pub fn new() -> Self {
        Self {
            property_id: None,
            description: None,
            section: None,
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        property_id: Option<String>,
        description: Option<String>,
        section: Option<String>,
    ) {
        self.property_id = property_id;
        self.description = description;
        self.section = section;
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

    pub fn section(&self) -> Option<&str> {
        self.section.as_deref()
    }

    pub fn set_section(&mut self, sec: Option<String>) {
        self.section = sec;
    }
}

impl Default for SurfaceElementProperty {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let prop = SurfaceElementProperty::new();
        assert!(prop.property_id().is_none());
        assert!(prop.description().is_none());
        assert!(prop.section().is_none());
    }

    #[test]
    fn test_init() {
        let mut prop = SurfaceElementProperty::new();
        prop.init(
            Some("PROP1".to_string()),
            Some("Surface property".to_string()),
            Some("Section1".to_string()),
        );

        assert_eq!(prop.property_id(), Some("PROP1"));
        assert_eq!(prop.description(), Some("Surface property"));
        assert_eq!(prop.section(), Some("Section1"));
    }

    #[test]
    fn test_setters() {
        let mut prop = SurfaceElementProperty::new();
        prop.set_property_id(Some("ID".to_string()));
        prop.set_description(Some("Desc".to_string()));
        prop.set_section(Some("Sec".to_string()));

        assert_eq!(prop.property_id(), Some("ID"));
        assert_eq!(prop.description(), Some("Desc"));
        assert_eq!(prop.section(), Some("Sec"));
    }
}
