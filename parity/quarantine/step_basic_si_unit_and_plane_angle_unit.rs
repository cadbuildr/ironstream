// FILE: step_basic_si_unit_and_plane_angle_unit.rs
// occt: StepBasic_SiUnitAndPlaneAngleUnit

use std::rc::Rc;
use std::cell::RefCell;

use crate::step_basic_si_unit::StepBasicSiUnit;

pub struct StepBasicPlaneAngleUnit;

pub struct StepBasicSiUnitAndPlaneAngleUnit {
    base: StepBasicSiUnit,
    plane_angle_unit: Option<Rc<RefCell<StepBasicPlaneAngleUnit>>>,
}

impl StepBasicSiUnitAndPlaneAngleUnit {
    pub fn new() -> Self {
        StepBasicSiUnitAndPlaneAngleUnit {
            base: StepBasicSiUnit::new(),
            plane_angle_unit: None,
        }
    }

    pub fn set_plane_angle_unit(&mut self, plane_angle_unit: Rc<RefCell<StepBasicPlaneAngleUnit>>) {
        self.plane_angle_unit = Some(plane_angle_unit);
    }

    pub fn plane_angle_unit(&self) -> Option<Rc<RefCell<StepBasicPlaneAngleUnit>>> {
        self.plane_angle_unit.clone()
    }
}

impl Default for StepBasicSiUnitAndPlaneAngleUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let u = StepBasicSiUnitAndPlaneAngleUnit::new();
        assert_eq!(u.plane_angle_unit(), None);
    }
}
