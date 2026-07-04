// FILE: step_basic_certification.rs
// occt: StepBasic_Certification

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

pub struct StepBasic_CertificationType;

pub struct StepBasic_Certification {
    name: Option<Rc<RefCell<HString>>>,
    kind: Option<Rc<RefCell<StepBasic_CertificationType>>>,
}

impl StepBasic_Certification {
    pub fn new() -> Self {
        StepBasic_Certification {
            name: None,
            kind: None,
        }
    }

    pub fn init(
        &mut self,
        name: Option<Rc<RefCell<HString>>>,
        kind: Option<Rc<RefCell<StepBasic_CertificationType>>>,
    ) {
        self.name = name;
        self.kind = kind;
    }

    pub fn set_name(&mut self, name: Option<Rc<RefCell<HString>>>) {
        self.name = name;
    }

    pub fn name(&self) -> Option<Rc<RefCell<HString>>> {
        self.name.clone()
    }

    pub fn set_kind(&mut self, kind: Option<Rc<RefCell<StepBasic_CertificationType>>>) {
        self.kind = kind;
    }

    pub fn kind(&self) -> Option<Rc<RefCell<StepBasic_CertificationType>>> {
        self.kind.clone()
    }
}

impl Default for StepBasic_Certification {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let cert = StepBasic_Certification::new();
        assert!(cert.name().is_none());
        assert!(cert.kind().is_none());
    }

    #[test]
    fn test_init() {
        let mut cert = StepBasic_Certification::new();
        let name = HString::new("ISO9001".to_string());
        cert.init(Some(name), None);
        assert!(cert.name().is_some());
    }
}
