// FILE: step_basic_group.rs
// occt: StepBasic_Group

/// Representation of STEP entity Group
#[derive(Clone, Debug)]
pub struct Group {
    name: Option<String>,
    description: Option<String>,
    has_description: bool,
}

impl Group {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            has_description: false,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, has_description: bool, description: Option<String>) {
        self.name = Some(name);
        self.has_description = has_description;
        if has_description {
            self.description = description;
        }
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

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let group = Group::new();
        assert!(group.name().is_none());
        assert!(group.description().is_none());
        assert!(!group.has_description());
    }

    #[test]
    fn test_init_without_description() {
        let mut group = Group::new();
        group.init("group1".to_string(), false, None);
        assert_eq!(group.name(), Some("group1"));
        assert!(!group.has_description());
    }

    #[test]
    fn test_init_with_description() {
        let mut group = Group::new();
        group.init("group2".to_string(), true, Some("desc2".to_string()));
        assert_eq!(group.name(), Some("group2"));
        assert!(group.has_description());
        assert_eq!(group.description(), Some("desc2"));
    }

    #[test]
    fn test_set_description() {
        let mut group = Group::new();
        group.set_description("desc1".to_string());
        assert!(group.has_description());
        assert_eq!(group.description(), Some("desc1"));
    }
}
