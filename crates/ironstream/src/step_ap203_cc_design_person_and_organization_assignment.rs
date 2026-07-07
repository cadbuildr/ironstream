// FILE: step_ap203_cc_design_person_and_organization_assignment.rs
// occt: StepAP203_CcDesignPersonAndOrganizationAssignment

/// CC Design Person and Organization Assignment for STEP AP203
pub struct StepAP203_CcDesignPersonAndOrganizationAssignment {
    person_id: i32,
    org_id: i32,
}

impl StepAP203_CcDesignPersonAndOrganizationAssignment {
    pub fn new() -> Self {
        StepAP203_CcDesignPersonAndOrganizationAssignment {
            person_id: 0,
            org_id: 0,
        }
    }

    pub fn set_person_id(&mut self, id: i32) {
        self.person_id = id;
    }

    pub fn get_person_id(&self) -> i32 {
        self.person_id
    }

    pub fn set_org_id(&mut self, id: i32) {
        self.org_id = id;
    }

    pub fn get_org_id(&self) -> i32 {
        self.org_id
    }
}

impl Default for StepAP203_CcDesignPersonAndOrganizationAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let assign = StepAP203_CcDesignPersonAndOrganizationAssignment::new();
        assert_eq!(assign.get_person_id(), 0);
        assert_eq!(assign.get_org_id(), 0);
    }
}
