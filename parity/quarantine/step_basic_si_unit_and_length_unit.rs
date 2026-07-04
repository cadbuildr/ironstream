// FILE: step_basic_si_unit_and_length_unit.rs
// occt: StepBasic_SiUnitAndLengthUnit

use std::rc::Rc;
use std::cell::RefCell;

use crate::step_basic_si_unit::StepBasicSiUnit;

pub struct StepBasicLengthUnit;

pub struct StepBasicSiUnitAndLengthUnit {
    base: StepBasicSiUnit,
    length_unit: Option<Rc<RefCell<StepBasicLengthUnit>>>,
}

impl StepBasicSiUnitAndLengthUnit {
    pub fn new() -> Self {
        StepBasicSiUnitAndLengthUnit {
            base: StepBasicSiUnit::new(),
            length_unit: None,
        }
    }

    pub fn set_length_unit(&mut self, length_unit: Rc<RefCell<StepBasicLengthUnit>>) {
        self.length_unit = Some(length_unit);
    }

    pub fn length_unit(&self) -> Option<Rc<RefCell<StepBasicLengthUnit>>> {
        self.length_unit.clone()
    }
}

impl Default for StepBasicSiUnitAndLengthUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let u = StepBasicSiUnitAndLengthUnit::new();
        assert_eq!(u.length_unit(), None);
    }
}
