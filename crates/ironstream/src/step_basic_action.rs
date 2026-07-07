// FILE: step_basic_action.rs
// occt: StepBasic_Action

/// Representation of STEP Basic Action entity.
#[derive(Clone, Debug)]
pub struct Action {
    name: String,
    description: String,
    has_description: bool,
}

impl Action {
    pub fn new() -> Self {
        Action {
            name: String::new(),
            description: String::new(),
            has_description: false,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.has_description = true;
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn has_description(&self) -> bool {
        self.has_description
    }
}

impl Default for Action {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let action = Action::new();
        assert_eq!(action.name(), "");
        assert_eq!(action.description(), "");
        assert!(!action.has_description());
    }

    #[test]
    fn test_set_description() {
        let mut action = Action::new();
        action.set_description("test description".to_string());
        assert!(action.has_description());
        assert_eq!(action.description(), "test description");
    }
}
