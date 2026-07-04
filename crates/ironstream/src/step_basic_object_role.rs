// FILE: step_basic_object_role.rs
// occt: StepBasic_ObjectRole

/// Represents a STEP ObjectRole entity with Name and optional Description.
#[derive(Clone, Debug)]
pub struct StepBasicObjectRole {
    name: String,
    description: Option<String>,
    has_description: bool,
}

impl StepBasicObjectRole {
    /// Create a new empty StepBasicObjectRole.
    pub fn new() -> Self {
        StepBasicObjectRole {
            name: String::new(),
            description: None,
            has_description: false,
        }
    }

    /// Initialize all fields.
    pub fn init(&mut self, name: String, has_description: bool, description: Option<String>) {
        self.name = name;
        self.has_description = has_description;
        if has_description {
            self.description = description;
        } else {
            self.description = None;
        }
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
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set the Description field.
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Returns whether Description is defined.
    pub fn has_description(&self) -> bool {
        self.has_description
    }
}

impl Default for StepBasicObjectRole {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let role = StepBasicObjectRole::new();
        assert_eq!(role.name(), "");
        assert_eq!(role.description(), None);
        assert!(!role.has_description());
    }

    #[test]
    fn test_init_with_description() {
        let mut role = StepBasicObjectRole::new();
        role.init(
            "Manager".to_string(),
            true,
            Some("Manages the project".to_string()),
        );

        assert_eq!(role.name(), "Manager");
        assert_eq!(role.description(), Some("Manages the project"));
        assert!(role.has_description());
    }

    #[test]
    fn test_init_without_description() {
        let mut role = StepBasicObjectRole::new();
        role.init(
            "Worker".to_string(),
            false,
            Some("ignored".to_string()),
        );

        assert_eq!(role.name(), "Worker");
        assert_eq!(role.description(), None);
        assert!(!role.has_description());
    }

    #[test]
    fn test_setters() {
        let mut role = StepBasicObjectRole::new();
        role.set_name("Reviewer".to_string());
        role.set_description("Reviews work".to_string());

        assert_eq!(role.name(), "Reviewer");
        assert_eq!(role.description(), Some("Reviews work"));
    }
}
