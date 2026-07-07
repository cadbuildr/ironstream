// FILE: step_basic_organization_role.rs
// occt: StepBasic_OrganizationRole

/// Represents a STEP OrganizationRole entity with a Name.
#[derive(Clone, Debug)]
pub struct StepBasicOrganizationRole {
    name: String,
}

impl StepBasicOrganizationRole {
    /// Create a new empty StepBasicOrganizationRole.
    pub fn new() -> Self {
        StepBasicOrganizationRole {
            name: String::new(),
        }
    }

    /// Initialize the Name field.
    pub fn init(&mut self, name: String) {
        self.name = name;
    }

    /// Returns the Name field.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the Name field.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for StepBasicOrganizationRole {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let role = StepBasicOrganizationRole::new();
        assert_eq!(role.name(), "");
    }

    #[test]
    fn test_init() {
        let mut role = StepBasicOrganizationRole::new();
        role.init("CEO".to_string());
        assert_eq!(role.name(), "CEO");
    }

    #[test]
    fn test_set_name() {
        let mut role = StepBasicOrganizationRole::new();
        role.set_name("CFO".to_string());
        assert_eq!(role.name(), "CFO");
    }
}
