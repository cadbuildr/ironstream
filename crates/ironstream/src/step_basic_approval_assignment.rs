// FILE: step_basic_approval_assignment.rs
// occt: StepBasic_ApprovalAssignment

use std::cell::RefCell;
use std::rc::Rc;

pub struct HString {
    value: String,
}

impl HString {
    pub fn new(value: String) -> Rc<RefCell<HString>> {
        Rc::new(RefCell::new(HString { value }))
    }
}

pub struct StepBasic_ApprovalStatus;

pub struct StepBasic_Approval {
    status: Option<Rc<RefCell<StepBasic_ApprovalStatus>>>,
    level: Option<Rc<RefCell<HString>>>,
}

impl StepBasic_Approval {
    pub fn new() -> Self {
        StepBasic_Approval {
            status: None,
            level: None,
        }
    }
}

pub struct StepBasic_ApprovalAssignment {
    assigned_approval: Option<Rc<RefCell<StepBasic_Approval>>>,
}

impl StepBasic_ApprovalAssignment {
    pub fn new() -> Self {
        StepBasic_ApprovalAssignment {
            assigned_approval: None,
        }
    }

    pub fn init(&mut self, assigned_approval: Option<Rc<RefCell<StepBasic_Approval>>>) {
        self.assigned_approval = assigned_approval;
    }

    pub fn set_assigned_approval(&mut self, assigned_approval: Option<Rc<RefCell<StepBasic_Approval>>>) {
        self.assigned_approval = assigned_approval;
    }

    pub fn assigned_approval(&self) -> Option<Rc<RefCell<StepBasic_Approval>>> {
        self.assigned_approval.clone()
    }
}

impl Default for StepBasic_ApprovalAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let assignment = StepBasic_ApprovalAssignment::new();
        assert!(assignment.assigned_approval().is_none());
    }

    #[test]
    fn test_set_assigned_approval() {
        let mut assignment = StepBasic_ApprovalAssignment::new();
        let approval = Rc::new(RefCell::new(StepBasic_Approval::new()));
        assignment.set_assigned_approval(Some(approval));
        assert!(assignment.assigned_approval().is_some());
    }

    #[test]
    fn test_init() {
        let mut assignment = StepBasic_ApprovalAssignment::new();
        let approval = Rc::new(RefCell::new(StepBasic_Approval::new()));
        assignment.init(Some(approval));
        assert!(assignment.assigned_approval().is_some());
    }
}
