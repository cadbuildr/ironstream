// FILE: step_basic_person_and_organization_assignment.rs
// occt: StepBasic_PersonAndOrganizationAssignment

#[derive(Clone, Debug)]
pub struct StepBasicPersonAndOrganizationAssignment {
    assigned_po_id: String,
    role_id: String,
}

impl StepBasicPersonAndOrganizationAssignment {
    pub fn new() -> Self {
        Self {
            assigned_po_id: String::new(),
            role_id: String::new(),
        }
    }

    pub fn init(&mut self, assigned_po_id: String, role_id: String) {
        self.assigned_po_id = assigned_po_id;
        self.role_id = role_id;
    }

    pub fn assigned_person_and_organization(&self) -> &str {
        &self.assigned_po_id
    }

    pub fn set_assigned_person_and_organization(&mut self, id: String) {
        self.assigned_po_id = id;
    }

    pub fn role(&self) -> &str {
        &self.role_id
    }

    pub fn set_role(&mut self, id: String) {
        self.role_id = id;
    }
}

impl Default for StepBasicPersonAndOrganizationAssignment {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut a = StepBasicPersonAndOrganizationAssignment::new();
        a.init("PO-001".into(), "R-001".into());
        assert_eq!(a.assigned_person_and_organization(), "PO-001");
        assert_eq!(a.role(), "R-001");
    }
}
