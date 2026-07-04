// FILE: step_basic_action_method.rs
// occt: StepBasic_ActionMethod

use std::cell::RefCell;
use std::rc::Rc;

pub struct HString {
    value: String,
}

impl HString {
    pub fn new(value: String) -> Rc<RefCell<HString>> {
        Rc::new(RefCell::new(HString { value }))
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

pub struct StepBasic_ActionMethod {
    name: Option<Rc<RefCell<HString>>>,
    description: Option<Rc<RefCell<HString>>>,
    consequence: Option<Rc<RefCell<HString>>>,
    purpose: Option<Rc<RefCell<HString>>>,
    has_description: bool,
}

impl StepBasic_ActionMethod {
    pub fn new() -> Self {
        StepBasic_ActionMethod {
            name: None,
            description: None,
            consequence: None,
            purpose: None,
            has_description: false,
        }
    }

    pub fn init(
        &mut self,
        name: Option<Rc<RefCell<HString>>>,
        has_description: bool,
        description: Option<Rc<RefCell<HString>>>,
        consequence: Option<Rc<RefCell<HString>>>,
        purpose: Option<Rc<RefCell<HString>>>,
    ) {
        self.name = name;
        self.has_description = has_description;
        self.description = if has_description { description } else { None };
        self.consequence = consequence;
        self.purpose = purpose;
    }

    pub fn name(&self) -> Option<Rc<RefCell<HString>>> {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: Option<Rc<RefCell<HString>>>) {
        self.name = name;
    }

    pub fn description(&self) -> Option<Rc<RefCell<HString>>> {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: Option<Rc<RefCell<HString>>>) {
        self.description = description;
    }

    pub fn has_description(&self) -> bool {
        self.has_description
    }

    pub fn consequence(&self) -> Option<Rc<RefCell<HString>>> {
        self.consequence.clone()
    }

    pub fn set_consequence(&mut self, consequence: Option<Rc<RefCell<HString>>>) {
        self.consequence = consequence;
    }

    pub fn purpose(&self) -> Option<Rc<RefCell<HString>>> {
        self.purpose.clone()
    }

    pub fn set_purpose(&mut self, purpose: Option<Rc<RefCell<HString>>>) {
        self.purpose = purpose;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_method_creation() {
        let method = StepBasic_ActionMethod::new();
        assert!(method.name().is_none());
        assert!(!method.has_description());
    }

    #[test]
    fn test_action_method_init_with_description() {
        let mut method = StepBasic_ActionMethod::new();
        let name = HString::new("test_name".to_string());
        let desc = HString::new("test_desc".to_string());

        method.init(Some(name), true, Some(desc), None, None);

        assert!(method.name().is_some());
        assert!(method.has_description());
        assert!(method.description().is_some());
    }

    #[test]
    fn test_action_method_init_without_description() {
        let mut method = StepBasic_ActionMethod::new();
        let name = HString::new("test".to_string());

        method.init(Some(name), false, None, None, None);

        assert!(method.name().is_some());
        assert!(!method.has_description());
        assert!(method.description().is_none());
    }
}
