// FILE: step_basic_approval_role.rs
// occt: StepBasic_ApprovalRole

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

pub struct StepBasic_ApprovalRole {
    role: Option<Rc<RefCell<HString>>>,
}

impl StepBasic_ApprovalRole {
    pub fn new() -> Self {
        StepBasic_ApprovalRole { role: None }
    }

    pub fn init(&mut self, role: Option<Rc<RefCell<HString>>>) {
        self.role = role;
    }

    pub fn set_role(&mut self, role: Option<Rc<RefCell<HString>>>) {
        self.role = role;
    }

    pub fn role(&self) -> Option<Rc<RefCell<HString>>> {
        self.role.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let ar = StepBasic_ApprovalRole::new();
        assert!(ar.role().is_none());
    }

    #[test]
    fn test_init() {
        let mut ar = StepBasic_ApprovalRole::new();
        let role = HString::new("Reviewer".to_string());
        ar.init(Some(role));
        assert!(ar.role().is_some());
    }
}
