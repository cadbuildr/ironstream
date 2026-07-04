// FILE: step_basic_application_context_element.rs
// occt: StepBasic_ApplicationContextElement

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

pub struct StepBasic_ApplicationContextElement {
    name: Option<Rc<RefCell<HString>>>,
    frame_of_reference: Option<Rc<RefCell<dyn std::any::Any>>>,
}

impl StepBasic_ApplicationContextElement {
    pub fn new() -> Self {
        StepBasic_ApplicationContextElement {
            name: None,
            frame_of_reference: None,
        }
    }

    pub fn init(
        &mut self,
        name: Option<Rc<RefCell<HString>>>,
        frame_of_reference: Option<Rc<RefCell<dyn std::any::Any>>>,
    ) {
        self.name = name;
        self.frame_of_reference = frame_of_reference;
    }

    pub fn set_name(&mut self, name: Option<Rc<RefCell<HString>>>) {
        self.name = name;
    }

    pub fn name(&self) -> Option<Rc<RefCell<HString>>> {
        self.name.clone()
    }

    pub fn set_frame_of_reference(&mut self, frame_of_reference: Option<Rc<RefCell<dyn std::any::Any>>>) {
        self.frame_of_reference = frame_of_reference;
    }

    pub fn frame_of_reference(&self) -> Option<Rc<RefCell<dyn std::any::Any>>> {
        self.frame_of_reference.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let elem = StepBasic_ApplicationContextElement::new();
        assert!(elem.name().is_none());
        assert!(elem.frame_of_reference().is_none());
    }

    #[test]
    fn test_init() {
        let mut elem = StepBasic_ApplicationContextElement::new();
        let name = HString::new("test".to_string());
        let frame = Rc::new(RefCell::new(42));
        elem.init(Some(name), Some(frame));
        assert!(elem.name().is_some());
        assert!(elem.frame_of_reference().is_some());
    }
}
