// FILE: step_basic_si_unit_and_area_unit.rs
// occt: StepBasic_SiUnitAndAreaUnit

use std::rc::Rc;
use std::cell::RefCell;

use crate::step_basic_si_unit::StepBasicSiUnit;

// Placeholder types
pub struct StepBasicAreaUnit;

pub struct StepBasicSiUnitAndAreaUnit {
    base: StepBasicSiUnit,
    area_unit: Option<Rc<RefCell<StepBasicAreaUnit>>>,
}

impl StepBasicSiUnitAndAreaUnit {
    pub fn new() -> Self {
        StepBasicSiUnitAndAreaUnit {
            base: StepBasicSiUnit::new(),
            area_unit: None,
        }
    }

    pub fn set_area_unit(&mut self, area_unit: Rc<RefCell<StepBasicAreaUnit>>) {
        self.area_unit = Some(area_unit);
    }

    pub fn area_unit(&self) -> Option<Rc<RefCell<StepBasicAreaUnit>>> {
        self.area_unit.clone()
    }
}

impl Default for StepBasicSiUnitAndAreaUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let u = StepBasicSiUnitAndAreaUnit::new();
        assert_eq!(u.area_unit(), None);
    }

    #[test]
    fn test_default() {
        let u = StepBasicSiUnitAndAreaUnit::default();
        assert_eq!(u.area_unit(), None);
    }
}
