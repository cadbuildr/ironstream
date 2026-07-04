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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let stat = StepBasic_ApprovalStatus::new();
        assert!(stat.name().is_none());
    }

    #[test]
    fn test_init() {
        let mut stat = StepBasic_ApprovalStatus::new();
        let name = HString::new("APPROVED".to_string());
        stat.init(Some(name));
        assert!(stat.name().is_some());
    }
}
