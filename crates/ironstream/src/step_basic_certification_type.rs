// FILE: step_basic_certification_type.rs
// occt: StepBasic_CertificationType

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

pub struct StepBasic_CertificationType {
    name: Option<Rc<RefCell<HString>>>,
}

impl StepBasic_CertificationType {
    pub fn new() -> Self {
        StepBasic_CertificationType { name: None }
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

impl Default for StepBasic_CertificationType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let ct = StepBasic_CertificationType::new();
        assert!(ct.name().is_none());
    }

    #[test]
    fn test_set_name() {
        let mut ct = StepBasic_CertificationType::new();
        let name = HString::new("quality".to_string());
        ct.set_name(Some(name));
        assert!(ct.name().is_some());
    }
}
