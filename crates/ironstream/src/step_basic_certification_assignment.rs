// FILE: step_basic_certification_assignment.rs
// occt: StepBasic_CertificationAssignment

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

pub struct StepBasic_Certification;

pub struct StepBasic_CertificationAssignment {
    assigned_certification: Option<Rc<RefCell<StepBasic_Certification>>>,
    role: Option<Rc<RefCell<HString>>>,
}

impl StepBasic_CertificationAssignment {
    pub fn new() -> Self {
        StepBasic_CertificationAssignment {
            assigned_certification: None,
            role: None,
        }
    }

    pub fn init(
        &mut self,
        assigned_certification: Option<Rc<RefCell<StepBasic_Certification>>>,
        role: Option<Rc<RefCell<HString>>>,
    ) {
        self.assigned_certification = assigned_certification;
        self.role = role;
    }

    pub fn set_assigned_certification(&mut self, cert: Option<Rc<RefCell<StepBasic_Certification>>>) {
        self.assigned_certification = cert;
    }

    pub fn assigned_certification(&self) -> Option<Rc<RefCell<StepBasic_Certification>>> {
        self.assigned_certification.clone()
    }

    pub fn set_role(&mut self, role: Option<Rc<RefCell<HString>>>) {
        self.role = role;
    }

    pub fn role(&self) -> Option<Rc<RefCell<HString>>> {
        self.role.clone()
    }
}

impl Default for StepBasic_CertificationAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let ca = StepBasic_CertificationAssignment::new();
        assert!(ca.assigned_certification().is_none());
        assert!(ca.role().is_none());
    }

    #[test]
    fn test_init() {
        let mut ca = StepBasic_CertificationAssignment::new();
        let role = HString::new("authority".to_string());
        ca.init(None, Some(role));
        assert!(ca.role().is_some());
    }
}
