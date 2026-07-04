// FILE: step_basic_organization_assignment.rs
// occt: StepBasic_OrganizationAssignment

/// Represents a STEP OrganizationAssignment entity with an Organization and Role.
#[derive(Clone, Debug)]
pub struct StepBasicOrganizationAssignment {
    assigned_organization_id: String, // Simplified: using ID string
    role_id: String,                  // Simplified: using ID string
}

impl StepBasicOrganizationAssignment {
    /// Create a new empty StepBasicOrganizationAssignment.
    pub fn new() -> Self {
        StepBasicOrganizationAssignment {
            assigned_organization_id: String::new(),
            role_id: String::new(),
        }
    }

    /// Initialize all fields.
    pub fn init(&mut self, assigned_organization_id: String, role_id: String) {
        self.assigned_organization_id = assigned_organization_id;
        self.role_id = role_id;
    }

    /// Returns the AssignedOrganization ID.
    pub fn assigned_organization(&self) -> &str {
        &self.assigned_organization_id
    }

    /// Set the AssignedOrganization ID.
    pub fn set_assigned_organization(&mut self, id: String) {
        self.assigned_organization_id = id;
    }

    /// Returns the Role ID.
    pub fn role(&self) -> &str {
        &self.role_id
    }

    /// Set the Role ID.
    pub fn set_role(&mut self, id: String) {
        self.role_id = id;
    }
}

impl Default for StepBasicOrganizationAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assign = StepBasicOrganizationAssignment::new();
        assert_eq!(assign.assigned_organization(), "");
        assert_eq!(assign.role(), "");
    }

    #[test]
    fn test_init() {
        let mut assign = StepBasicOrganizationAssignment::new();
        assign.init("ORG-001".to_string(), "ROLE-001".to_string());

        assert_eq!(assign.assigned_organization(), "ORG-001");
        assert_eq!(assign.role(), "ROLE-001");
    }

    #[test]
    fn test_setters() {
        let mut assign = StepBasicOrganizationAssignment::new();
        assign.set_assigned_organization("ORG-002".to_string());
        assign.set_role("ROLE-002".to_string());

        assert_eq!(assign.assigned_organization(), "ORG-002");
        assert_eq!(assign.role(), "ROLE-002");
    }
}
