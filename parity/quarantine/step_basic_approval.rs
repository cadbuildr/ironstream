// FILE: step_basic_approval.rs
// occt: StepBasic_Approval

use std::cell::RefCell;
use std::rc::Rc;

pub struct HString {
    value: String,
}

impl HString {
    pub fn new(value: String) -> Rc<RefCell<HString>> {
        Rc::new(RefCell::new(HString { value }))
    }

    pub fn as_str(&self) -> &str {
        &self.value
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

    pub fn init(
        &mut self,
        status: Option<Rc<RefCell<StepBasic_ApprovalStatus>>>,
        level: Option<Rc<RefCell<HString>>>,
    ) {
        self.status = status;
        self.level = level;
    }

    pub fn set_status(&mut self, status: Option<Rc<RefCell<StepBasic_ApprovalStatus>>>) {
        self.status = status;
    }

    pub fn status(&self) -> Option<Rc<RefCell<StepBasic_ApprovalStatus>>> {
        self.status.clone()
    }

    pub fn set_level(&mut self, level: Option<Rc<RefCell<HString>>>) {
        self.level = level;
    }

    pub fn level(&self) -> Option<Rc<RefCell<HString>>> {
        self.level.clone()
    }
}

impl Default for StepBasic_Approval {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let approval = StepBasic_Approval::new();
        assert!(approval.status().is_none());
        assert!(approval.level().is_none());
    }

    #[test]
    fn test_set_level() {
        let mut approval = StepBasic_Approval::new();
        let level = HString::new("approved".to_string());
        approval.set_level(Some(level.clone()));
        assert!(approval.level().is_some());
    }

    #[test]
    fn test_init() {
        let mut approval = StepBasic_Approval::new();
        let level = HString::new("approved".to_string());
        approval.init(None, Some(level));
        assert!(approval.level().is_some());
    }
}
