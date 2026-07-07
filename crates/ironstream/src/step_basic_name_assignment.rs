// FILE: step_basic_name_assignment.rs
// occt: StepBasic_NameAssignment

/// Representation of STEP entity NameAssignment
#[derive(Clone, Debug)]
pub struct NameAssignment {
    assigned_name: Option<String>,
}

impl NameAssignment {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            assigned_name: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, assigned_name: String) {
        self.assigned_name = Some(assigned_name);
    }

    /// Get assigned name
    pub fn assigned_name(&self) -> Option<&str> {
        self.assigned_name.as_deref()
    }

    /// Set assigned name
    pub fn set_assigned_name(&mut self, assigned_name: String) {
        self.assigned_name = Some(assigned_name);
    }
}

impl Default for NameAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assign = NameAssignment::new();
        assert!(assign.assigned_name().is_none());
    }

    #[test]
    fn test_init() {
        let mut assign = NameAssignment::new();
        assign.init("name1".to_string());
        assert_eq!(assign.assigned_name(), Some("name1"));
    }

    #[test]
    fn test_set_assigned_name() {
        let mut assign = NameAssignment::new();
        assign.set_assigned_name("name2".to_string());
        assert_eq!(assign.assigned_name(), Some("name2"));
    }

    #[test]
    fn test_default() {
        let assign = NameAssignment::default();
        assert!(assign.assigned_name().is_none());
    }
}
