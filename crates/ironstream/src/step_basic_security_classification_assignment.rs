// FILE: step_basic_security_classification_assignment.rs
// occt: StepBasic_SecurityClassificationAssignment

use std::rc::Rc;
use std::cell::RefCell;

// Local placeholder for the referenced StepBasic_SecurityClassification entity
// (external plumbing; only referenced through shared handles here).
#[derive(Debug, PartialEq)]
pub struct StepBasicSecurityClassification;

/// Represents a SecurityClassificationAssignment in the STEP AP standard.
pub struct StepBasicSecurityClassificationAssignment {
    assigned_security_classification: Option<Rc<RefCell<StepBasicSecurityClassification>>>,
}

impl StepBasicSecurityClassificationAssignment {
    pub fn new() -> Self {
        StepBasicSecurityClassificationAssignment {
            assigned_security_classification: None,
        }
    }

    pub fn init(
        &mut self,
        assigned_security_classification: Rc<RefCell<StepBasicSecurityClassification>>,
    ) {
        self.assigned_security_classification = Some(assigned_security_classification);
    }

    pub fn set_assigned_security_classification(
        &mut self,
        assigned_security_classification: Rc<RefCell<StepBasicSecurityClassification>>,
    ) {
        self.assigned_security_classification = Some(assigned_security_classification);
    }

    pub fn assigned_security_classification(&self) -> Option<Rc<RefCell<StepBasicSecurityClassification>>> {
        self.assigned_security_classification.clone()
    }
}

impl Default for StepBasicSecurityClassificationAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let sca = StepBasicSecurityClassificationAssignment::new();
        assert_eq!(sca.assigned_security_classification(), None);
    }

    #[test]
    fn test_default() {
        let sca = StepBasicSecurityClassificationAssignment::default();
        assert_eq!(sca.assigned_security_classification(), None);
    }
}
