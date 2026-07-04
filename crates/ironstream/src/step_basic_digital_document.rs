// FILE: step_basic_digital_document.rs
// occt: StepBasic_DigitalDocument

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

pub struct StepBasic_DocumentType {
    description: Option<Rc<RefCell<HString>>>,
}

pub struct StepBasic_Document {
    id: Option<Rc<RefCell<HString>>>,
    name: Option<Rc<RefCell<HString>>>,
    description: Option<Rc<RefCell<HString>>>,
    kind: Option<Rc<RefCell<StepBasic_DocumentType>>>,
    has_description: bool,
}

impl StepBasic_Document {
    pub fn new() -> Self {
        StepBasic_Document {
            id: None,
            name: None,
            description: None,
            kind: None,
            has_description: false,
        }
    }
}

pub struct StepBasic_DigitalDocument {
    base: StepBasic_Document,
}

impl StepBasic_DigitalDocument {
    pub fn new() -> Self {
        StepBasic_DigitalDocument {
            base: StepBasic_Document::new(),
        }
    }
}

impl Default for StepBasic_DigitalDocument {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let dd = StepBasic_DigitalDocument::new();
        assert!(dd.base.id.is_none());
    }
}
