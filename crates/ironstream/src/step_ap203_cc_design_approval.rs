// FILE: step_ap203_cc_design_approval.rs
// occt: StepAP203_CcDesignApproval

/// CC Design Approval for STEP AP203
pub struct StepAP203_CcDesignApproval {
    approval_id: i32,
}

impl StepAP203_CcDesignApproval {
    pub fn new() -> Self {
        StepAP203_CcDesignApproval { approval_id: 0 }
    }

    pub fn set_approval_id(&mut self, id: i32) {
        self.approval_id = id;
    }

    pub fn get_approval_id(&self) -> i32 {
        self.approval_id
    }
}

impl Default for StepAP203_CcDesignApproval {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let approval = StepAP203_CcDesignApproval::new();
        assert_eq!(approval.get_approval_id(), 0);
    }

    #[test]
    fn test_set_approval_id() {
        let mut approval = StepAP203_CcDesignApproval::new();
        approval.set_approval_id(42);
        assert_eq!(approval.get_approval_id(), 42);
    }
}
