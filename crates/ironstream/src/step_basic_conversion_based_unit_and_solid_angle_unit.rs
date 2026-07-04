// FILE: step_basic_conversion_based_unit_and_solid_angle_unit.rs
// occt: StepBasic_ConversionBasedUnitAndSolidAngleUnit

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_SolidAngleUnit {
    dummy: i32,
}

impl StepBasic_SolidAngleUnit {
    pub fn new() -> Self {
        StepBasic_SolidAngleUnit { dummy: 0 }
    }
}

pub struct StepBasic_ConversionBasedUnit {
    name: Option<Rc<RefCell<String>>>,
}

pub struct StepBasic_ConversionBasedUnitAndSolidAngleUnit {
    base: StepBasic_ConversionBasedUnit,
    solid_angle_unit: Option<Rc<RefCell<StepBasic_SolidAngleUnit>>>,
}

impl StepBasic_ConversionBasedUnitAndSolidAngleUnit {
    pub fn new() -> Self {
        StepBasic_ConversionBasedUnitAndSolidAngleUnit {
            base: StepBasic_ConversionBasedUnit { name: None },
            solid_angle_unit: None,
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

    pub fn set_solid_angle_unit(&mut self, solid_angle_unit: Option<Rc<RefCell<StepBasic_SolidAngleUnit>>>) {
        self.solid_angle_unit = solid_angle_unit;
    }

    pub fn solid_angle_unit(&self) -> Option<Rc<RefCell<StepBasic_SolidAngleUnit>>> {
        self.solid_angle_unit.clone()
    }
}

impl Default for StepBasic_ConversionBasedUnitAndSolidAngleUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let unit = StepBasic_ConversionBasedUnitAndSolidAngleUnit::new();
        assert!(unit.solid_angle_unit().is_none());
    }
}
