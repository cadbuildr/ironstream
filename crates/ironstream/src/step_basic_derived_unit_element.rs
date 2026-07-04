// FILE: step_basic_derived_unit_element.rs
// occt: StepBasic_DerivedUnitElement

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_NamedUnit {
    dummy: i32,
}

impl StepBasic_NamedUnit {
    pub fn new() -> Self {
        StepBasic_NamedUnit { dummy: 0 }
    }
}

pub struct StepBasic_DerivedUnitElement {
    unit: Option<Rc<RefCell<StepBasic_NamedUnit>>>,
    exponent: f64,
}

impl StepBasic_DerivedUnitElement {
    pub fn new() -> Self {
        StepBasic_DerivedUnitElement {
            unit: None,
            exponent: 0.0,
        }
    }

    pub fn init(&mut self, unit: Option<Rc<RefCell<StepBasic_NamedUnit>>>, exponent: f64) {
        self.unit = unit;
        self.exponent = exponent;
    }

    pub fn set_unit(&mut self, unit: Option<Rc<RefCell<StepBasic_NamedUnit>>>) {
        self.unit = unit;
    }

    pub fn unit(&self) -> Option<Rc<RefCell<StepBasic_NamedUnit>>> {
        self.unit.clone()
    }

    pub fn set_exponent(&mut self, exponent: f64) {
        self.exponent = exponent;
    }

    pub fn exponent(&self) -> f64 {
        self.exponent
    }
}

impl Default for StepBasic_DerivedUnitElement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let due = StepBasic_DerivedUnitElement::new();
        assert!(due.unit().is_none());
        assert_eq!(due.exponent(), 0.0);
    }
}
