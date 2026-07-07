// FILE: step_basic_role_select.rs
// occt: StepBasic_RoleSelect

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
pub struct StepBasicActionAssignment;
pub struct StepBasicActionRequestAssignment;
pub struct StepBasicApprovalAssignment;
pub struct StepBasicApprovalDateTime;
pub struct StepBasicCertificationAssignment;
pub struct StepBasicContractAssignment;
pub struct StepBasicDocumentReference;
pub struct StepBasicEffectivityAssignment;
pub struct StepBasicGroupAssignment;
pub struct StepBasicNameAssignment;
pub struct StepBasicSecurityClassificationAssignment;

/// Represents a RoleSelect in the STEP AP standard.
///
/// A select type that can hold one of 11 different assignment/reference types.
#[derive(Clone)]
pub enum StepBasicRoleSelect {
    ActionAssignment(Rc<RefCell<StepBasicActionAssignment>>),
    ActionRequestAssignment(Rc<RefCell<StepBasicActionRequestAssignment>>),
    ApprovalAssignment(Rc<RefCell<StepBasicApprovalAssignment>>),
    ApprovalDateTime(Rc<RefCell<StepBasicApprovalDateTime>>),
    CertificationAssignment(Rc<RefCell<StepBasicCertificationAssignment>>),
    ContractAssignment(Rc<RefCell<StepBasicContractAssignment>>),
    DocumentReference(Rc<RefCell<StepBasicDocumentReference>>),
    EffectivityAssignment(Rc<RefCell<StepBasicEffectivityAssignment>>),
    GroupAssignment(Rc<RefCell<StepBasicGroupAssignment>>),
    NameAssignment(Rc<RefCell<StepBasicNameAssignment>>),
    SecurityClassificationAssignment(Rc<RefCell<StepBasicSecurityClassificationAssignment>>),
}

impl StepBasicRoleSelect {
    /// Creates a new RoleSelect
    pub fn new() -> Self {
        StepBasicRoleSelect::ActionAssignment(Rc::new(RefCell::new(
            StepBasicActionAssignment,
        )))
    }

    /// Returns the case number for the current type (1-11)
    pub fn case_num(&self) -> i32 {
        match self {
            StepBasicRoleSelect::ActionAssignment(_) => 1,
            StepBasicRoleSelect::ActionRequestAssignment(_) => 2,
            StepBasicRoleSelect::ApprovalAssignment(_) => 3,
            StepBasicRoleSelect::ApprovalDateTime(_) => 4,
            StepBasicRoleSelect::CertificationAssignment(_) => 5,
            StepBasicRoleSelect::ContractAssignment(_) => 6,
            StepBasicRoleSelect::DocumentReference(_) => 7,
            StepBasicRoleSelect::EffectivityAssignment(_) => 8,
            StepBasicRoleSelect::GroupAssignment(_) => 9,
            StepBasicRoleSelect::NameAssignment(_) => 10,
            StepBasicRoleSelect::SecurityClassificationAssignment(_) => 11,
        }
    }

    pub fn action_assignment(&self) -> Option<Rc<RefCell<StepBasicActionAssignment>>> {
        match self {
            StepBasicRoleSelect::ActionAssignment(a) => Some(a.clone()),
            _ => None,
        }
    }

    pub fn action_request_assignment(&self) -> Option<Rc<RefCell<StepBasicActionRequestAssignment>>> {
        match self {
            StepBasicRoleSelect::ActionRequestAssignment(a) => Some(a.clone()),
            _ => None,
        }
    }

    pub fn approval_assignment(&self) -> Option<Rc<RefCell<StepBasicApprovalAssignment>>> {
        match self {
            StepBasicRoleSelect::ApprovalAssignment(a) => Some(a.clone()),
            _ => None,
        }
    }

    pub fn approval_date_time(&self) -> Option<Rc<RefCell<StepBasicApprovalDateTime>>> {
        match self {
            StepBasicRoleSelect::ApprovalDateTime(a) => Some(a.clone()),
            _ => None,
        }
    }

    pub fn certification_assignment(&self) -> Option<Rc<RefCell<StepBasicCertificationAssignment>>> {
        match self {
            StepBasicRoleSelect::CertificationAssignment(c) => Some(c.clone()),
            _ => None,
        }
    }

    pub fn contract_assignment(&self) -> Option<Rc<RefCell<StepBasicContractAssignment>>> {
        match self {
            StepBasicRoleSelect::ContractAssignment(c) => Some(c.clone()),
            _ => None,
        }
    }

    pub fn document_reference(&self) -> Option<Rc<RefCell<StepBasicDocumentReference>>> {
        match self {
            StepBasicRoleSelect::DocumentReference(d) => Some(d.clone()),
            _ => None,
        }
    }

    pub fn effectivity_assignment(&self) -> Option<Rc<RefCell<StepBasicEffectivityAssignment>>> {
        match self {
            StepBasicRoleSelect::EffectivityAssignment(e) => Some(e.clone()),
            _ => None,
        }
    }

    pub fn group_assignment(&self) -> Option<Rc<RefCell<StepBasicGroupAssignment>>> {
        match self {
            StepBasicRoleSelect::GroupAssignment(g) => Some(g.clone()),
            _ => None,
        }
    }

    pub fn name_assignment(&self) -> Option<Rc<RefCell<StepBasicNameAssignment>>> {
        match self {
            StepBasicRoleSelect::NameAssignment(n) => Some(n.clone()),
            _ => None,
        }
    }

    pub fn security_classification_assignment(
        &self,
    ) -> Option<Rc<RefCell<StepBasicSecurityClassificationAssignment>>> {
        match self {
            StepBasicRoleSelect::SecurityClassificationAssignment(s) => Some(s.clone()),
            _ => None,
        }
    }
}

impl Default for StepBasicRoleSelect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let rs = StepBasicRoleSelect::new();
        assert_eq!(rs.case_num(), 1);
    }

    #[test]
    fn test_case_num_action_assignment() {
        let rs = StepBasicRoleSelect::ActionAssignment(Rc::new(RefCell::new(
            StepBasicActionAssignment,
        )));
        assert_eq!(rs.case_num(), 1);
        assert!(rs.action_assignment().is_some());
    }

    #[test]
    fn test_default() {
        let rs = StepBasicRoleSelect::default();
        assert_eq!(rs.case_num(), 1);
    }
}
