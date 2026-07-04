// FILE: step_basic_conversion_based_unit_and_length_unit.rs
// occt: StepBasic_ConversionBasedUnitAndLengthUnit

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_LengthUnit {
    dummy: i32,
}

impl StepBasic_LengthUnit {
    pub fn new() -> Self {
        StepBasic_LengthUnit { dummy: 0 }
    }
}

pub struct StepBasic_ConversionBasedUnit {
    name: Option<Rc<RefCell<String>>>,
}

pub struct StepBasic_ConversionBasedUnitAndLengthUnit {
    base: StepBasic_ConversionBasedUnit,
    length_unit: Option<Rc<RefCell<StepBasic_LengthUnit>>>,
}

impl StepBasic_ConversionBasedUnitAndLengthUnit {
    pub fn new() -> Self {
        StepBasic_ConversionBasedUnitAndLengthUnit {
            base: StepBasic_ConversionBasedUnit { name: None },
            length_unit: None,
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

    pub fn set_length_unit(&mut self, length_unit: Option<Rc<RefCell<StepBasic_LengthUnit>>>) {
        self.length_unit = length_unit;
    }

    pub fn length_unit(&self) -> Option<Rc<RefCell<StepBasic_LengthUnit>>> {
        self.length_unit.clone()
    }
}

impl Default for StepBasic_ConversionBasedUnitAndLengthUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let unit = StepBasic_ConversionBasedUnitAndLengthUnit::new();
        assert!(unit.length_unit().is_none());
    }
}
