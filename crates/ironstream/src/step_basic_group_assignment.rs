// FILE: step_basic_group_assignment.rs
// occt: StepBasic_GroupAssignment

/// Representation of STEP entity GroupAssignment
#[derive(Clone, Debug)]
pub struct GroupAssignment {
    assigned_group: Option<String>,
}

impl GroupAssignment {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            assigned_group: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, assigned_group: String) {
        self.assigned_group = Some(assigned_group);
    }

    /// Get assigned group
    pub fn assigned_group(&self) -> Option<&str> {
        self.assigned_group.as_deref()
    }

    /// Set assigned group
    pub fn set_assigned_group(&mut self, assigned_group: String) {
        self.assigned_group = Some(assigned_group);
    }
}

impl Default for GroupAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assign = GroupAssignment::new();
        assert!(assign.assigned_group().is_none());
    }

    #[test]
    fn test_init() {
        let mut assign = GroupAssignment::new();
        assign.init("group1".to_string());
        assert_eq!(assign.assigned_group(), Some("group1"));
    }

    #[test]
    fn test_set_assigned_group() {
        let mut assign = GroupAssignment::new();
        assign.set_assigned_group("group2".to_string());
        assert_eq!(assign.assigned_group(), Some("group2"));
    }

    #[test]
    fn test_default() {
        let assign = GroupAssignment::default();
        assert!(assign.assigned_group().is_none());
    }
}
