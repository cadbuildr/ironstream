// FILE: step_basic_si_unit_and_mass_unit.rs
// occt: StepBasic_SiUnitAndMassUnit

use std::rc::Rc;
use std::cell::RefCell;

use crate::step_basic_si_unit::StepBasicSiUnit;

pub struct StepBasicMassUnit;

pub struct StepBasicSiUnitAndMassUnit {
    base: StepBasicSiUnit,
    mass_unit: Option<Rc<RefCell<StepBasicMassUnit>>>,
}

impl StepBasicSiUnitAndMassUnit {
    pub fn new() -> Self {
        StepBasicSiUnitAndMassUnit {
            base: StepBasicSiUnit::new(),
            mass_unit: None,
        }
    }

    pub fn set_mass_unit(&mut self, mass_unit: Rc<RefCell<StepBasicMassUnit>>) {
        self.mass_unit = Some(mass_unit);
    }

    pub fn mass_unit(&self) -> Option<Rc<RefCell<StepBasicMassUnit>>> {
        self.mass_unit.clone()
    }
}

impl Default for StepBasicSiUnitAndMassUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let u = StepBasicSiUnitAndMassUnit::new();
        assert_eq!(u.mass_unit(), None);
    }
}
