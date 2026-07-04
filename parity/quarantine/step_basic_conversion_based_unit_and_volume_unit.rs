// FILE: step_basic_conversion_based_unit_and_volume_unit.rs
// occt: StepBasic_ConversionBasedUnitAndVolumeUnit

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_VolumeUnit {
    dummy: i32,
}

impl StepBasic_VolumeUnit {
    pub fn new() -> Self {
        StepBasic_VolumeUnit { dummy: 0 }
    }
}

pub struct StepBasic_ConversionBasedUnit {
    name: Option<Rc<RefCell<String>>>,
}

pub struct StepBasic_ConversionBasedUnitAndVolumeUnit {
    base: StepBasic_ConversionBasedUnit,
    volume_unit: Option<Rc<RefCell<StepBasic_VolumeUnit>>>,
}

impl StepBasic_ConversionBasedUnitAndVolumeUnit {
    pub fn new() -> Self {
        StepBasic_ConversionBasedUnitAndVolumeUnit {
            base: StepBasic_ConversionBasedUnit { name: None },
            volume_unit: None,
        }
    }

    pub fn set_volume_unit(&mut self, volume_unit: Option<Rc<RefCell<StepBasic_VolumeUnit>>>) {
        self.volume_unit = volume_unit;
    }

    pub fn volume_unit(&self) -> Option<Rc<RefCell<StepBasic_VolumeUnit>>> {
        self.volume_unit.clone()
    }
}

impl Default for StepBasic_ConversionBasedUnitAndVolumeUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let unit = StepBasic_ConversionBasedUnitAndVolumeUnit::new();
        assert!(unit.volume_unit().is_none());
    }
}
