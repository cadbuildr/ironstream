// FILE: step_basic_contract.rs
// occt: StepBasic_Contract

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

pub struct StepBasic_ContractType;

pub struct StepBasic_Contract {
    name: Option<Rc<RefCell<HString>>>,
    description: Option<Rc<RefCell<HString>>>,
    kind: Option<Rc<RefCell<StepBasic_ContractType>>>,
}

impl StepBasic_Contract {
    pub fn new() -> Self {
        StepBasic_Contract {
            name: None,
            description: None,
            kind: None,
        }
    }

    pub fn init(
        &mut self,
        name: Option<Rc<RefCell<HString>>>,
        description: Option<Rc<RefCell<HString>>>,
        kind: Option<Rc<RefCell<StepBasic_ContractType>>>,
    ) {
        self.name = name;
        self.description = description;
        self.kind = kind;
    }

    pub fn set_name(&mut self, name: Option<Rc<RefCell<HString>>>) {
        self.name = name;
    }

    pub fn name(&self) -> Option<Rc<RefCell<HString>>> {
        self.name.clone()
    }

    pub fn set_description(&mut self, description: Option<Rc<RefCell<HString>>>) {
        self.description = description;
    }

    pub fn description(&self) -> Option<Rc<RefCell<HString>>> {
        self.description.clone()
    }

    pub fn set_kind(&mut self, kind: Option<Rc<RefCell<StepBasic_ContractType>>>) {
        self.kind = kind;
    }

    pub fn kind(&self) -> Option<Rc<RefCell<StepBasic_ContractType>>> {
        self.kind.clone()
    }
}

impl Default for StepBasic_Contract {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let contract = StepBasic_Contract::new();
        assert!(contract.name().is_none());
        assert!(contract.description().is_none());
        assert!(contract.kind().is_none());
    }

    #[test]
    fn test_init() {
        let mut contract = StepBasic_Contract::new();
        let name = HString::new("Service Agreement".to_string());
        let desc = HString::new("A contract".to_string());
        contract.init(Some(name), Some(desc), None);
        assert!(contract.name().is_some());
        assert!(contract.description().is_some());
    }
}
