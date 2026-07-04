// FILE: step_basic_conversion_based_unit_and_area_unit.rs
// occt: StepBasic_ConversionBasedUnitAndAreaUnit

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_AreaUnit {
    dummy: i32,
}

impl StepBasic_AreaUnit {
    pub fn new() -> Self {
        StepBasic_AreaUnit { dummy: 0 }
    }
}

pub struct StepBasic_DimensionalExponents;
pub struct StepBasic_NamedUnit;

pub struct StepBasic_ConversionBasedUnit {
    name: Option<Rc<RefCell<String>>>,
}

pub struct StepBasic_ConversionBasedUnitAndAreaUnit {
    base: StepBasic_ConversionBasedUnit,
    area_unit: Option<Rc<RefCell<StepBasic_AreaUnit>>>,
}

impl StepBasic_ConversionBasedUnitAndAreaUnit {
    pub fn new() -> Self {
        StepBasic_ConversionBasedUnitAndAreaUnit {
            base: StepBasic_ConversionBasedUnit { name: None },
            area_unit: None,
        }
    }

    pub fn set_area_unit(&mut self, area_unit: Option<Rc<RefCell<StepBasic_AreaUnit>>>) {
        self.area_unit = area_unit;
    }

    pub fn area_unit(&self) -> Option<Rc<RefCell<StepBasic_AreaUnit>>> {
        self.area_unit.clone()
    }
}

impl Default for StepBasic_ConversionBasedUnitAndAreaUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let unit = StepBasic_ConversionBasedUnitAndAreaUnit::new();
        assert!(unit.area_unit().is_none());
    }
}
