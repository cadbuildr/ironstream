// FILE: step_basic_role_association.rs
// occt: StepBasic_RoleAssociation

use std::cell::RefCell;
use std::rc::Rc;

/// Local model of StepBasic_ObjectRole (external plumbing).
/// Carries the role name / description as in the STEP entity.
#[derive(Debug, Clone, Default)]
pub struct StepBasicObjectRole {
    pub name: String,
    pub description: Option<String>,
}

impl StepBasicObjectRole {
    pub fn new(name: &str) -> Self {
        StepBasicObjectRole {
            name: name.to_string(),
            description: None,
        }
    }
}

/// Local model of the STEP SELECT type StepBasic_RoleSelect.
///
/// Mirrors StepBasic_RoleSelect::CaseNum numbering:
/// 1 -> ActionAssignment            2 -> ActionRequestAssignment
/// 3 -> ApprovalAssignment          4 -> ApprovalDateTime
/// 5 -> CertificationAssignment     6 -> ContractAssignment
/// 7 -> DocumentReference           8 -> EffectivityAssignment
/// 9 -> GroupAssignment            10 -> NameAssignment
/// 11 -> SecurityClassificationAssignment
/// 0 -> null / unrecognized
#[derive(Debug, Clone, Default, PartialEq)]
pub enum StepBasicRoleSelect {
    #[default]
    Null,
    ActionAssignment,
    ActionRequestAssignment,
    ApprovalAssignment,
    ApprovalDateTime,
    CertificationAssignment,
    ContractAssignment,
    DocumentReference,
    EffectivityAssignment,
    GroupAssignment,
    NameAssignment,
    SecurityClassificationAssignment,
}

impl StepBasicRoleSelect {
    /// Recognizes the kind of RoleSelect select type (OCCT CaseNum semantics).
    pub fn case_num(&self) -> i32 {
        match self {
            StepBasicRoleSelect::Null => 0,
            StepBasicRoleSelect::ActionAssignment => 1,
            StepBasicRoleSelect::ActionRequestAssignment => 2,
            StepBasicRoleSelect::ApprovalAssignment => 3,
            StepBasicRoleSelect::ApprovalDateTime => 4,
            StepBasicRoleSelect::CertificationAssignment => 5,
            StepBasicRoleSelect::ContractAssignment => 6,
            StepBasicRoleSelect::DocumentReference => 7,
            StepBasicRoleSelect::EffectivityAssignment => 8,
            StepBasicRoleSelect::GroupAssignment => 9,
            StepBasicRoleSelect::NameAssignment => 10,
            StepBasicRoleSelect::SecurityClassificationAssignment => 11,
        }
    }
}

/// Represents a RoleAssociation in the STEP standard (StepBasic_RoleAssociation).
///
/// Associates an ObjectRole with an item having that role.
pub struct StepBasicRoleAssociation {
    role: Option<Rc<RefCell<StepBasicObjectRole>>>,
    item_with_role: StepBasicRoleSelect,
}

impl StepBasicRoleAssociation {
    /// Creates a new, uninitialized RoleAssociation (OCCT default ctor).
    pub fn new() -> Self {
        StepBasicRoleAssociation {
            role: None,
            item_with_role: StepBasicRoleSelect::default(),
        }
    }

    /// Initializes the RoleAssociation with all required attributes (OCCT Init).
    pub fn init(
        &mut self,
        role: Rc<RefCell<StepBasicObjectRole>>,
        item_with_role: StepBasicRoleSelect,
    ) {
        self.role = Some(role);
        self.item_with_role = item_with_role;
    }

    /// Returns the role (OCCT Role).
    pub fn role(&self) -> Option<Rc<RefCell<StepBasicObjectRole>>> {
        self.role.clone()
    }

    /// Sets the role (OCCT SetRole).
    pub fn set_role(&mut self, role: Rc<RefCell<StepBasicObjectRole>>) {
        self.role = Some(role);
    }

    /// Returns the item with role (OCCT ItemWithRole).
    pub fn item_with_role(&self) -> StepBasicRoleSelect {
        self.item_with_role.clone()
    }

    /// Sets the item with role (OCCT SetItemWithRole).
    pub fn set_item_with_role(&mut self, item_with_role: StepBasicRoleSelect) {
        self.item_with_role = item_with_role;
    }
}

impl Default for StepBasicRoleAssociation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let ra = StepBasicRoleAssociation::new();
        assert!(ra.role().is_none());
        // A default (null) select has CaseNum 0 in OCCT.
        assert_eq!(ra.item_with_role().case_num(), 0);
    }

    #[test]
    fn test_init_sets_all_fields() {
        let mut ra = StepBasicRoleAssociation::new();
        let role = Rc::new(RefCell::new(StepBasicObjectRole::new("supplier")));
        ra.init(role.clone(), StepBasicRoleSelect::DocumentReference);
        assert!(ra.role().is_some());
        assert_eq!(ra.role().unwrap().borrow().name, "supplier");
        assert_eq!(ra.item_with_role(), StepBasicRoleSelect::DocumentReference);
        assert_eq!(ra.item_with_role().case_num(), 7);
    }

    #[test]
    fn test_set_and_get_role() {
        let mut ra = StepBasicRoleAssociation::new();
        let role = Rc::new(RefCell::new(StepBasicObjectRole::new("owner")));
        ra.set_role(role.clone());
        assert!(ra.role().is_some());
        // Handle semantics: same underlying object.
        assert!(Rc::ptr_eq(&ra.role().unwrap(), &role));
    }

    #[test]
    fn test_set_and_get_item_with_role() {
        let mut ra = StepBasicRoleAssociation::new();
        ra.set_item_with_role(StepBasicRoleSelect::ApprovalDateTime);
        assert_eq!(ra.item_with_role(), StepBasicRoleSelect::ApprovalDateTime);
        assert_eq!(ra.item_with_role().case_num(), 4);
    }

    #[test]
    fn test_case_num_numbering_matches_occt() {
        // Full CaseNum table from StepBasic_RoleSelect.cxx.
        let cases = [
            (StepBasicRoleSelect::Null, 0),
            (StepBasicRoleSelect::ActionAssignment, 1),
            (StepBasicRoleSelect::ActionRequestAssignment, 2),
            (StepBasicRoleSelect::ApprovalAssignment, 3),
            (StepBasicRoleSelect::ApprovalDateTime, 4),
            (StepBasicRoleSelect::CertificationAssignment, 5),
            (StepBasicRoleSelect::ContractAssignment, 6),
            (StepBasicRoleSelect::DocumentReference, 7),
            (StepBasicRoleSelect::EffectivityAssignment, 8),
            (StepBasicRoleSelect::GroupAssignment, 9),
            (StepBasicRoleSelect::NameAssignment, 10),
            (StepBasicRoleSelect::SecurityClassificationAssignment, 11),
        ];
        for (sel, num) in cases {
            assert_eq!(sel.case_num(), num);
        }
    }

    #[test]
    fn test_default() {
        let ra = StepBasicRoleAssociation::default();
        assert!(ra.role().is_none());
        assert_eq!(ra.item_with_role().case_num(), 0);
    }
}
