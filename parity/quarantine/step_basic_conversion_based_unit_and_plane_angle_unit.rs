// FILE: step_basic_conversion_based_unit_and_plane_angle_unit.rs
// occt: StepBasic_ConversionBasedUnitAndPlaneAngleUnit

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_PlaneAngleUnit {
    dummy: i32,
}

impl StepBasic_PlaneAngleUnit {
    pub fn new() -> Self {
        StepBasic_PlaneAngleUnit { dummy: 0 }
    }
}

pub struct StepBasic_ConversionBasedUnit {
    name: Option<Rc<RefCell<String>>>,
}

pub struct StepBasic_ConversionBasedUnitAndPlaneAngleUnit {
    base: StepBasic_ConversionBasedUnit,
    plane_angle_unit: Option<Rc<RefCell<StepBasic_PlaneAngleUnit>>>,
}

impl StepBasic_ConversionBasedUnitAndPlaneAngleUnit {
    pub fn new() -> Self {
        StepBasic_ConversionBasedUnitAndPlaneAngleUnit {
            base: StepBasic_ConversionBasedUnit { name: None },
            plane_angle_unit: None,
        }
    }

    pub fn init(
        &mut self,
        dimensions: Option<Rc<RefCell<String>>>,
        name: Option<Rc<RefCell<String>>>,
        conversion_factor: Option<Rc<RefCell<String>>>,
    ) {
        self.base.name = name;
    }

    pub fn set_plane_angle_unit(&mut self, plane_angle_unit: Option<Rc<RefCell<StepBasic_PlaneAngleUnit>>>) {
        self.plane_angle_unit = plane_angle_unit;
    }

    pub fn plane_angle_unit(&self) -> Option<Rc<RefCell<StepBasic_PlaneAngleUnit>>> {
        self.plane_angle_unit.clone()
    }
}

impl Default for StepBasic_ConversionBasedUnitAndPlaneAngleUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let unit = StepBasic_ConversionBasedUnitAndPlaneAngleUnit::new();
        assert!(unit.plane_angle_unit().is_none());
    }
}
