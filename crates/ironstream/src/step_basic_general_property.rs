// FILE: step_basic_general_property.rs
// occt: StepBasic_GeneralProperty

/// Representation of STEP entity GeneralProperty
#[derive(Clone, Debug)]
pub struct GeneralProperty {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    has_description: bool,
}

impl GeneralProperty {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            description: None,
            has_description: false,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        id: String,
        name: String,
        has_description: bool,
        description: Option<String>,
    ) {
        self.id = Some(id);
        self.name = Some(name);
        self.has_description = has_description;
        if has_description {
            self.description = description;
        }
    }

    /// Get id
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Set id
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    /// Get name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
        self.has_description = true;
    }

    /// Check if description is defined
    pub fn has_description(&self) -> bool {
        self.has_description
    }
}

impl Default for GeneralProperty {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let prop = GeneralProperty::new();
        assert!(prop.id().is_none());
        assert!(prop.name().is_none());
        assert!(prop.description().is_none());
        assert!(!prop.has_description());
    }

    #[test]
    fn test_init_without_description() {
        let mut prop = GeneralProperty::new();
        prop.init("id1".to_string(), "name1".to_string(), false, None);
        assert_eq!(prop.id(), Some("id1"));
        assert_eq!(prop.name(), Some("name1"));
        assert!(!prop.has_description());
    }

    #[test]
    fn test_init_with_description() {
        let mut prop = GeneralProperty::new();
        prop.init(
            "id2".to_string(),
            "name2".to_string(),
            true,
            Some("desc2".to_string()),
        );
        assert_eq!(prop.id(), Some("id2"));
        assert_eq!(prop.name(), Some("name2"));
        assert!(prop.has_description());
        assert_eq!(prop.description(), Some("desc2"));
    }

    #[test]
    fn test_set_description() {
        let mut prop = GeneralProperty::new();
        prop.set_description("desc1".to_string());
        assert!(prop.has_description());
        assert_eq!(prop.description(), Some("desc1"));
    }
}
