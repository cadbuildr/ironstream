// FILE: step_basic_date_role.rs
// occt: StepBasic_DateRole

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

pub struct StepBasic_DateRole {
    name: Option<Rc<RefCell<HString>>>,
}

impl StepBasic_DateRole {
    pub fn new() -> Self {
        StepBasic_DateRole { name: None }
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
        let role = StepBasic_DateRole::new();
        assert!(role.name().is_none());
    }
}
