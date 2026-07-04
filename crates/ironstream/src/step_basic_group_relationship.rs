// FILE: step_basic_group_relationship.rs
// occt: StepBasic_GroupRelationship

/// Representation of STEP entity GroupRelationship
#[derive(Clone, Debug)]
pub struct GroupRelationship {
    name: Option<String>,
    description: Option<String>,
    has_description: bool,
    relating_group: Option<String>,
    related_group: Option<String>,
}

impl GroupRelationship {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            has_description: false,
            relating_group: None,
            related_group: None,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        has_description: bool,
        description: Option<String>,
        relating_group: String,
        related_group: String,
    ) {
        self.name = Some(name);
        self.has_description = has_description;
        if has_description {
            self.description = description;
        }
        self.relating_group = Some(relating_group);
        self.related_group = Some(related_group);
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

    /// Get relating group
    pub fn relating_group(&self) -> Option<&str> {
        self.relating_group.as_deref()
    }

    /// Set relating group
    pub fn set_relating_group(&mut self, relating_group: String) {
        self.relating_group = Some(relating_group);
    }

    /// Get related group
    pub fn related_group(&self) -> Option<&str> {
        self.related_group.as_deref()
    }

    /// Set related group
    pub fn set_related_group(&mut self, related_group: String) {
        self.related_group = Some(related_group);
    }
}

impl Default for GroupRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rel = GroupRelationship::new();
        assert!(rel.name().is_none());
        assert!(rel.description().is_none());
        assert!(!rel.has_description());
        assert!(rel.relating_group().is_none());
        assert!(rel.related_group().is_none());
    }

    #[test]
    fn test_init() {
        let mut rel = GroupRelationship::new();
        rel.init(
            "rel1".to_string(),
            true,
            Some("desc1".to_string()),
            "grp1".to_string(),
            "grp2".to_string(),
        );
        assert_eq!(rel.name(), Some("rel1"));
        assert!(rel.has_description());
        assert_eq!(rel.description(), Some("desc1"));
        assert_eq!(rel.relating_group(), Some("grp1"));
        assert_eq!(rel.related_group(), Some("grp2"));
    }

    #[test]
    fn test_set_fields() {
        let mut rel = GroupRelationship::new();
        rel.set_name("rel2".to_string());
        rel.set_description("desc2".to_string());
        rel.set_relating_group("grp3".to_string());
        rel.set_related_group("grp4".to_string());

        assert_eq!(rel.name(), Some("rel2"));
        assert!(rel.has_description());
        assert_eq!(rel.relating_group(), Some("grp3"));
        assert_eq!(rel.related_group(), Some("grp4"));
    }
}
