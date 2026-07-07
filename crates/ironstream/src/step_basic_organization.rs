// FILE: step_basic_organization.rs
// occt: StepBasic_Organization

/// Represents a STEP Organization entity with optional ID, Name, and Description.
#[derive(Clone, Debug)]
pub struct StepBasicOrganization {
    id: Option<String>,
    has_id: bool,
    name: String,
    description: String,
}

impl StepBasicOrganization {
    /// Create a new empty StepBasicOrganization.
    pub fn new() -> Self {
        StepBasicOrganization {
            id: None,
            has_id: false,
            name: String::new(),
            description: String::new(),
        }
    }

    /// Initialize all fields.
    pub fn init(
        &mut self,
        has_id: bool,
        id: Option<String>,
        name: String,
        description: String,
    ) {
        self.has_id = has_id;
        if has_id {
            self.id = id;
        } else {
            self.id = None;
        }
        self.name = name;
        self.description = description;
    }

    /// Returns the ID field.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Set the ID field.
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
        self.has_id = true;
    }

    /// Unset the ID field.
    pub fn unset_id(&mut self) {
        self.id = None;
        self.has_id = false;
    }

    /// Returns whether ID is defined.
    pub fn has_id(&self) -> bool {
        self.has_id
    }

    /// Returns the Name field.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the Name field.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns the Description field.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set the Description field.
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}

impl Default for StepBasicOrganization {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let org = StepBasicOrganization::new();
        assert_eq!(org.id(), None);
        assert!(!org.has_id());
        assert_eq!(org.name(), "");
        assert_eq!(org.description(), "");
    }

    #[test]
    fn test_init_with_id() {
        let mut org = StepBasicOrganization::new();
        org.init(
            true,
            Some("ORG-001".to_string()),
            "Acme Corp".to_string(),
            "A company".to_string(),
        );

        assert_eq!(org.id(), Some("ORG-001"));
        assert!(org.has_id());
        assert_eq!(org.name(), "Acme Corp");
        assert_eq!(org.description(), "A company");
    }

    #[test]
    fn test_init_without_id() {
        let mut org = StepBasicOrganization::new();
        org.init(
            false,
            Some("ignored".to_string()),
            "Beta Inc".to_string(),
            "Another company".to_string(),
        );

        assert_eq!(org.id(), None);
        assert!(!org.has_id());
        assert_eq!(org.name(), "Beta Inc");
    }

    #[test]
    fn test_set_id() {
        let mut org = StepBasicOrganization::new();
        org.set_id("ID123".to_string());

        assert_eq!(org.id(), Some("ID123"));
        assert!(org.has_id());
    }

    #[test]
    fn test_unset_id() {
        let mut org = StepBasicOrganization::new();
        org.set_id("ID456".to_string());
        assert!(org.has_id());

        org.unset_id();
        assert_eq!(org.id(), None);
        assert!(!org.has_id());
    }

    #[test]
    fn test_setters() {
        let mut org = StepBasicOrganization::new();
        org.set_name("NewName".to_string());
        org.set_description("NewDesc".to_string());

        assert_eq!(org.name(), "NewName");
        assert_eq!(org.description(), "NewDesc");
    }
}
