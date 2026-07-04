// FILE: step_basic_security_classification.rs
// occt: StepBasic_SecurityClassification

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
pub struct StepBasicSecurityClassificationLevel;

/// Represents a SecurityClassification in the STEP AP standard.
pub struct StepBasicSecurityClassification {
    name: Option<String>,
    purpose: Option<String>,
    security_level: Option<Rc<RefCell<StepBasicSecurityClassificationLevel>>>,
}

impl StepBasicSecurityClassification {
    pub fn new() -> Self {
        StepBasicSecurityClassification {
            name: None,
            purpose: None,
            security_level: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        purpose: String,
        security_level: Rc<RefCell<StepBasicSecurityClassificationLevel>>,
    ) {
        self.name = Some(name);
        self.purpose = Some(purpose);
        self.security_level = Some(security_level);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_purpose(&mut self, purpose: String) {
        self.purpose = Some(purpose);
    }

    pub fn purpose(&self) -> Option<&str> {
        self.purpose.as_deref()
    }

    pub fn set_security_level(
        &mut self,
        security_level: Rc<RefCell<StepBasicSecurityClassificationLevel>>,
    ) {
        self.security_level = Some(security_level);
    }

    pub fn security_level(&self) -> Option<Rc<RefCell<StepBasicSecurityClassificationLevel>>> {
        self.security_level.clone()
    }
}

impl Default for StepBasicSecurityClassification {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let sc = StepBasicSecurityClassification::new();
        assert_eq!(sc.name(), None);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut sc = StepBasicSecurityClassification::new();
        sc.set_name("classified".to_string());
        assert_eq!(sc.name(), Some("classified"));
    }

    #[test]
    fn test_default() {
        let sc = StepBasicSecurityClassification::default();
        assert_eq!(sc.name(), None);
    }
}
