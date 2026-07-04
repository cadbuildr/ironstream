// FILE: step_basic_derived_unit.rs
// occt: StepBasic_DerivedUnit

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_DerivedUnitElement {
    exponent: f64,
}

impl StepBasic_DerivedUnitElement {
    pub fn new() -> Self {
        StepBasic_DerivedUnitElement { exponent: 0.0 }
    }
}

pub struct StepBasic_DerivedUnit {
    elements: Vec<Rc<RefCell<StepBasic_DerivedUnitElement>>>,
}

impl StepBasic_DerivedUnit {
    pub fn new() -> Self {
        StepBasic_DerivedUnit {
            elements: Vec::new(),
        }
    }

    pub fn init(&mut self, elements: Vec<Rc<RefCell<StepBasic_DerivedUnitElement>>>) {
        self.elements = elements;
    }

    pub fn set_elements(&mut self, elements: Vec<Rc<RefCell<StepBasic_DerivedUnitElement>>>) {
        self.elements = elements;
    }

    pub fn elements(&self) -> Vec<Rc<RefCell<StepBasic_DerivedUnitElement>>> {
        self.elements.clone()
    }

    pub fn nb_elements(&self) -> usize {
        self.elements.len()
    }

    pub fn elements_value(&self, num: usize) -> Option<Rc<RefCell<StepBasic_DerivedUnitElement>>> {
        if num > 0 && num <= self.elements.len() {
            Some(self.elements[num - 1].clone())
        } else {
            None
        }
    }
}

impl Default for StepBasic_DerivedUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let du = StepBasic_DerivedUnit::new();
        assert_eq!(du.nb_elements(), 0);
    }
}
