// FILE: step_basic_characterized_object.rs
// occt: StepBasic_CharacterizedObject

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

pub struct StepBasic_CharacterizedObject {
    name: Option<Rc<RefCell<HString>>>,
    description: Option<Rc<RefCell<HString>>>,
    has_description: bool,
}

impl StepBasic_CharacterizedObject {
    pub fn new() -> Self {
        StepBasic_CharacterizedObject {
            name: None,
            description: None,
            has_description: false,
        }
    }

    pub fn init(
        &mut self,
        name: Option<Rc<RefCell<HString>>>,
        has_description: bool,
        description: Option<Rc<RefCell<HString>>>,
    ) {
        self.name = name;
        self.has_description = has_description;
        self.description = if has_description { description } else { None };
    }

    pub fn set_name(&mut self, name: Option<Rc<RefCell<HString>>>) {
        self.name = name;
    }

    pub fn name(&self) -> Option<Rc<RefCell<HString>>> {
        self.name.clone()
    }

    pub fn set_description(&mut self, description: Option<Rc<RefCell<HString>>>) {
        self.description = description.clone();
        self.has_description = description.is_some();
    }

    pub fn unset_description(&mut self) {
        self.description = None;
        self.has_description = false;
    }

    pub fn description(&self) -> Option<Rc<RefCell<HString>>> {
        self.description.clone()
    }

    pub fn has_description(&self) -> bool {
        self.has_description
    }
}

impl Default for StepBasic_CharacterizedObject {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let co = StepBasic_CharacterizedObject::new();
        assert!(co.name().is_none());
        assert!(!co.has_description());
    }

    #[test]
    fn test_set_description() {
        let mut co = StepBasic_CharacterizedObject::new();
        let desc = HString::new("A test object".to_string());
        co.set_description(Some(desc));
        assert!(co.has_description());
        assert!(co.description().is_some());
    }

    #[test]
    fn test_unset_description() {
        let mut co = StepBasic_CharacterizedObject::new();
        let desc = HString::new("Test".to_string());
        co.set_description(Some(desc));
        co.unset_description();
        assert!(!co.has_description());
    }
}
