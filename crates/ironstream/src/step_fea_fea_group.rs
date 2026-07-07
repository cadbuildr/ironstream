// FILE: step_fea_fea_group.rs
// occt: StepFEA_FeaGroup

/// Representation of STEP entity FeaGroup
#[derive(Debug, Clone)]
pub struct StepFeaFeaGroup {
    name: String,
    description: String,
    model_ref: Option<i32>,
}

impl StepFeaFeaGroup {
    /// Creates a new empty FeaGroup
    pub fn new() -> Self {
        StepFeaFeaGroup {
            name: String::new(),
            description: String::new(),
            model_ref: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, description: String, model_ref: Option<i32>) {
        self.name = name;
        self.description = description;
        self.model_ref = model_ref;
    }

    /// Returns field ModelRef
    pub fn model_ref(&self) -> Option<i32> {
        self.model_ref
    }

    /// Set field ModelRef
    pub fn set_model_ref(&mut self, model_ref: Option<i32>) {
        self.model_ref = model_ref;
    }

    /// Returns field name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set field name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns field description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set field description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}

impl Default for StepFeaFeaGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_group_creation() {
        let group = StepFeaFeaGroup::new();
        assert_eq!(group.name(), "");
        assert_eq!(group.description(), "");
        assert_eq!(group.model_ref(), None);
    }

    #[test]
    fn test_fea_group_init() {
        let mut group = StepFeaFeaGroup::new();
        group.init("Group".to_string(), "Description".to_string(), Some(1));

        assert_eq!(group.name(), "Group");
        assert_eq!(group.description(), "Description");
        assert_eq!(group.model_ref(), Some(1));
    }

    #[test]
    fn test_fea_group_setters() {
        let mut group = StepFeaFeaGroup::new();
        group.set_name("Test".to_string());
        group.set_description("Test Desc".to_string());
        group.set_model_ref(Some(2));

        assert_eq!(group.name(), "Test");
        assert_eq!(group.description(), "Test Desc");
        assert_eq!(group.model_ref(), Some(2));
    }
}
