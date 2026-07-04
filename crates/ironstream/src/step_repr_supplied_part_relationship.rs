// FILE: step_repr_supplied_part_relationship.rs
// occt: StepRepr_SuppliedPartRelationship

/// Represents a supplied part relationship in STEP.
/// Inherits from ProductDefinitionRelationship.
pub struct SuppliedPartRelationship {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
}

impl SuppliedPartRelationship {
    /// Create a new SuppliedPartRelationship
    pub fn new() -> Self {
        SuppliedPartRelationship {
            id: None,
            name: None,
            description: None,
        }
    }

    /// Initialize supplied part relationship
    pub fn init(&mut self, id: String, name: String, description: Option<String>) {
        self.id = Some(id);
        self.name = Some(name);
        self.description = description;
    }

    /// Get the id
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Set the id
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set the description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
}

impl Default for SuppliedPartRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let rel = SuppliedPartRelationship::new();
        assert_eq!(rel.id(), None);
        assert_eq!(rel.name(), None);
        assert_eq!(rel.description(), None);
    }

    #[test]
    fn test_init() {
        let mut rel = SuppliedPartRelationship::new();
        rel.init(
            "rel_id".to_string(),
            "rel_name".to_string(),
            Some("rel_description".to_string()),
        );
        assert_eq!(rel.id(), Some("rel_id"));
        assert_eq!(rel.name(), Some("rel_name"));
        assert_eq!(rel.description(), Some("rel_description"));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut rel = SuppliedPartRelationship::new();
        rel.set_name("TestName".to_string());
        assert_eq!(rel.name(), Some("TestName"));
    }

    #[test]
    fn test_set_and_get_description() {
        let mut rel = SuppliedPartRelationship::new();
        rel.set_description("Test Description".to_string());
        assert_eq!(rel.description(), Some("Test Description"));
    }
}
