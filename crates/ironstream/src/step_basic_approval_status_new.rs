// FILE: step_basic_approval_status.rs
// occt: StepBasic_ApprovalStatus

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

pub struct StepBasic_ApprovalStatus {
    name: Option<Rc<RefCell<HString>>>,
}

impl StepBasic_ApprovalStatus {
    pub fn new() -> Self {
        StepBasic_ApprovalStatus { name: None }
    }

    pub fn init(&mut self, name: Option<Rc<RefCell<HString>>>) {
        self.name = name;
    }

    pub fn set_name(&mut self, name: Option<Rc<RefCell<HString>>>) {
        self.name = name;
    }

    pub fn name(&self) -> Option<Rc<RefCell<HString>>> {
        self.name.clone()
    }
}

impl Default for StepBasic_ApprovalStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let status = StepBasic_ApprovalStatus::new();
        assert!(status.name().is_none());
    }

    #[test]
    fn test_set_name() {
        let mut status = StepBasic_ApprovalStatus::new();
        let name = HString::new("approved".to_string());
        status.set_name(Some(name));
        assert!(status.name().is_some());
    }

    #[test]
    fn test_init() {
        let mut status = StepBasic_ApprovalStatus::new();
        let name = HString::new("pending".to_string());
        status.init(Some(name));
        assert!(status.name().is_some());
    }
}
