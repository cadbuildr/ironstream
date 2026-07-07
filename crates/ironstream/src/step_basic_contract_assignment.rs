// FILE: step_basic_contract_assignment.rs
// occt: StepBasic_ContractAssignment

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

pub struct StepBasic_Contract;

pub struct StepBasic_ContractAssignment {
    assigned_contract: Option<Rc<RefCell<StepBasic_Contract>>>,
    role: Option<Rc<RefCell<HString>>>,
}

impl StepBasic_ContractAssignment {
    pub fn new() -> Self {
        StepBasic_ContractAssignment {
            assigned_contract: None,
            role: None,
        }
    }

    pub fn init(
        &mut self,
        assigned_contract: Option<Rc<RefCell<StepBasic_Contract>>>,
        role: Option<Rc<RefCell<HString>>>,
    ) {
        self.assigned_contract = assigned_contract;
        self.role = role;
    }

    pub fn set_assigned_contract(&mut self, contract: Option<Rc<RefCell<StepBasic_Contract>>>) {
        self.assigned_contract = contract;
    }

    pub fn assigned_contract(&self) -> Option<Rc<RefCell<StepBasic_Contract>>> {
        self.assigned_contract.clone()
    }

    pub fn set_role(&mut self, role: Option<Rc<RefCell<HString>>>) {
        self.role = role;
    }

    pub fn role(&self) -> Option<Rc<RefCell<HString>>> {
        self.role.clone()
    }
}

impl Default for StepBasic_ContractAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let ca = StepBasic_ContractAssignment::new();
        assert!(ca.assigned_contract().is_none());
        assert!(ca.role().is_none());
    }

    #[test]
    fn test_init() {
        let mut ca = StepBasic_ContractAssignment::new();
        let role = HString::new("vendor".to_string());
        ca.init(None, Some(role));
        assert!(ca.role().is_some());
    }
}
