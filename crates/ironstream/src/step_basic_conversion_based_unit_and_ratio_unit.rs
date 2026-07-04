// FILE: step_basic_conversion_based_unit_and_ratio_unit.rs
// occt: StepBasic_ConversionBasedUnitAndRatioUnit

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_RatioUnit {
    dummy: i32,
}

impl StepBasic_RatioUnit {
    pub fn new() -> Self {
        StepBasic_RatioUnit { dummy: 0 }
    }
}

pub struct StepBasic_ConversionBasedUnit {
    name: Option<Rc<RefCell<String>>>,
}

pub struct StepBasic_ConversionBasedUnitAndRatioUnit {
    base: StepBasic_ConversionBasedUnit,
    ratio_unit: Option<Rc<RefCell<StepBasic_RatioUnit>>>,
}

impl StepBasic_ConversionBasedUnitAndRatioUnit {
    pub fn new() -> Self {
        StepBasic_ConversionBasedUnitAndRatioUnit {
            base: StepBasic_ConversionBasedUnit { name: None },
            ratio_unit: None,
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

    pub fn set_ratio_unit(&mut self, ratio_unit: Option<Rc<RefCell<StepBasic_RatioUnit>>>) {
        self.ratio_unit = ratio_unit;
    }

    pub fn ratio_unit(&self) -> Option<Rc<RefCell<StepBasic_RatioUnit>>> {
        self.ratio_unit.clone()
    }
}

impl Default for StepBasic_ConversionBasedUnitAndRatioUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let unit = StepBasic_ConversionBasedUnitAndRatioUnit::new();
        assert!(unit.ratio_unit().is_none());
    }
}
