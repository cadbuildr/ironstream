// FILE: step_basic_identification_assignment.rs
// occt: StepBasic_IdentificationAssignment

/// Representation of STEP entity IdentificationAssignment
#[derive(Clone, Debug)]
pub struct IdentificationAssignment {
    assigned_id: Option<String>,
    role: Option<String>,
}

impl IdentificationAssignment {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            assigned_id: None,
            role: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, assigned_id: String, role: String) {
        self.assigned_id = Some(assigned_id);
        self.role = Some(role);
    }

    /// Get assigned id
    pub fn assigned_id(&self) -> Option<&str> {
        self.assigned_id.as_deref()
    }

    /// Set assigned id
    pub fn set_assigned_id(&mut self, assigned_id: String) {
        self.assigned_id = Some(assigned_id);
    }

    /// Get role
    pub fn role(&self) -> Option<&str> {
        self.role.as_deref()
    }

    /// Set role
    pub fn set_role(&mut self, role: String) {
        self.role = Some(role);
    }
}

impl Default for IdentificationAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id_assign = IdentificationAssignment::new();
        assert!(id_assign.assigned_id().is_none());
        assert!(id_assign.role().is_none());
    }

    #[test]
    fn test_init() {
        let mut id_assign = IdentificationAssignment::new();
        id_assign.init("id123".to_string(), "role1".to_string());
        assert_eq!(id_assign.assigned_id(), Some("id123"));
        assert_eq!(id_assign.role(), Some("role1"));
    }

    #[test]
    fn test_set_fields() {
        let mut id_assign = IdentificationAssignment::new();
        id_assign.set_assigned_id("id456".to_string());
        id_assign.set_role("role2".to_string());
        assert_eq!(id_assign.assigned_id(), Some("id456"));
        assert_eq!(id_assign.role(), Some("role2"));
    }

    #[test]
    fn test_default() {
        let id_assign = IdentificationAssignment::default();
        assert!(id_assign.assigned_id().is_none());
    }
}
