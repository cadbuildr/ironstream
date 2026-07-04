// FILE: step_basic_general_property_relationship.rs
// occt: StepBasic_GeneralPropertyRelationship

/// Representation of STEP entity GeneralPropertyRelationship
#[derive(Clone, Debug)]
pub struct GeneralPropertyRelationship {
    name: Option<String>,
    description: Option<String>,
    has_description: bool,
    relating_general_property: Option<String>,
    related_general_property: Option<String>,
}

impl GeneralPropertyRelationship {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            has_description: false,
            relating_general_property: None,
            related_general_property: None,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        has_description: bool,
        description: Option<String>,
        relating_general_property: String,
        related_general_property: String,
    ) {
        self.name = Some(name);
        self.has_description = has_description;
        if has_description {
            self.description = description;
        }
        self.relating_general_property = Some(relating_general_property);
        self.related_general_property = Some(related_general_property);
    }

    /// Get name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Check if description is defined
    pub fn has_description(&self) -> bool {
        self.has_description
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

    /// Get relating general property
    pub fn relating_general_property(&self) -> Option<&str> {
        self.relating_general_property.as_deref()
    }

    /// Set relating general property
    pub fn set_relating_general_property(&mut self, relating_general_property: String) {
        self.relating_general_property = Some(relating_general_property);
    }

    /// Get related general property
    pub fn related_general_property(&self) -> Option<&str> {
        self.related_general_property.as_deref()
    }

    /// Set related general property
    pub fn set_related_general_property(&mut self, related_general_property: String) {
        self.related_general_property = Some(related_general_property);
    }
}

impl Default for GeneralPropertyRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rel = GeneralPropertyRelationship::new();
        assert!(rel.name().is_none());
        assert!(!rel.has_description());
        assert!(rel.description().is_none());
        assert!(rel.relating_general_property().is_none());
        assert!(rel.related_general_property().is_none());
    }

    #[test]
    fn test_init() {
        let mut rel = GeneralPropertyRelationship::new();
        rel.init(
            "rel1".to_string(),
            true,
            Some("desc1".to_string()),
            "prop1".to_string(),
            "prop2".to_string(),
        );
        assert_eq!(rel.name(), Some("rel1"));
        assert!(rel.has_description());
        assert_eq!(rel.description(), Some("desc1"));
        assert_eq!(rel.relating_general_property(), Some("prop1"));
        assert_eq!(rel.related_general_property(), Some("prop2"));
    }

    #[test]
    fn test_set_description() {
        let mut rel = GeneralPropertyRelationship::new();
        rel.set_description("desc2".to_string());
        assert!(rel.has_description());
        assert_eq!(rel.description(), Some("desc2"));
    }
}
