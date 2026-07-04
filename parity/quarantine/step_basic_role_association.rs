// FILE: step_basic_role_association.rs
// occt: StepBasic_RoleAssociation

use std::rc::Rc;
use std::cell::RefCell;

use crate::step_basic_role_select::StepBasicRoleSelect;

// Placeholder types
pub struct StepBasicObjectRole;

/// Represents a RoleAssociation in the STEP AP standard.
///
/// Associates a role with an item that has that role.
pub struct StepBasicRoleAssociation {
    role: Option<Rc<RefCell<StepBasicObjectRole>>>,
    item_with_role: StepBasicRoleSelect,
}

impl StepBasicRoleAssociation {
    /// Creates a new, uninitialized RoleAssociation
    pub fn new() -> Self {
        StepBasicRoleAssociation {
            role: None,
            item_with_role: StepBasicRoleSelect::default(),
        }
    }

    /// Initializes the RoleAssociation with all required attributes
    pub fn init(&mut self, role: Rc<RefCell<StepBasicObjectRole>>, item_with_role: StepBasicRoleSelect) {
        self.role = Some(role);
        self.item_with_role = item_with_role;
    }

    /// Returns the role
    pub fn role(&self) -> Option<Rc<RefCell<StepBasicObjectRole>>> {
        self.role.clone()
    }

    /// Sets the role
    pub fn set_role(&mut self, role: Rc<RefCell<StepBasicObjectRole>>) {
        self.role = Some(role);
    }

    /// Returns the item with role
    pub fn item_with_role(&self) -> StepBasicRoleSelect {
        self.item_with_role.clone()
    }

    /// Sets the item with role
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
        assert_eq!(ra.role(), None);
        assert_eq!(ra.item_with_role().case_num(), 1);
    }

    #[test]
    fn test_set_and_get_role() {
        let mut ra = StepBasicRoleAssociation::new();
        let role = Rc::new(RefCell::new(StepBasicObjectRole));
        ra.set_role(role.clone());
        assert!(ra.role().is_some());
    }

    #[test]
    fn test_default() {
        let ra = StepBasicRoleAssociation::default();
        assert_eq!(ra.role(), None);
    }
}
