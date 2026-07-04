// FILE: step_ap214_applied_approval_assignment.rs
// occt: StepAP214_AppliedApprovalAssignment

/// Applied Approval Assignment for STEP AP214
pub struct StepAP214_AppliedApprovalAssignment {
    approval_id: i32,
}

impl StepAP214_AppliedApprovalAssignment {
    pub fn new() -> Self {
        StepAP214_AppliedApprovalAssignment { approval_id: 0 }
    }

    pub fn set_approval_id(&mut self, id: i32) {
        self.approval_id = id;
    }

    pub fn get_approval_id(&self) -> i32 {
        self.approval_id
    }
}

impl Default for StepAP214_AppliedApprovalAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let approval = StepAP214_AppliedApprovalAssignment::new();
        assert_eq!(approval.get_approval_id(), 0);
    }
}
