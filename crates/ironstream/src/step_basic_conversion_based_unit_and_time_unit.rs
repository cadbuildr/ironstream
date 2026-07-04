// FILE: step_basic_conversion_based_unit_and_time_unit.rs
// occt: StepBasic_ConversionBasedUnitAndTimeUnit

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_TimeUnit {
    dummy: i32,
}

impl StepBasic_TimeUnit {
    pub fn new() -> Self {
        StepBasic_TimeUnit { dummy: 0 }
    }
}

pub struct StepBasic_ConversionBasedUnit {
    name: Option<Rc<RefCell<String>>>,
}

pub struct StepBasic_ConversionBasedUnitAndTimeUnit {
    base: StepBasic_ConversionBasedUnit,
    time_unit: Option<Rc<RefCell<StepBasic_TimeUnit>>>,
}

impl StepBasic_ConversionBasedUnitAndTimeUnit {
    pub fn new() -> Self {
        StepBasic_ConversionBasedUnitAndTimeUnit {
            base: StepBasic_ConversionBasedUnit { name: None },
            time_unit: None,
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

    pub fn set_time_unit(&mut self, time_unit: Option<Rc<RefCell<StepBasic_TimeUnit>>>) {
        self.time_unit = time_unit;
    }

    pub fn time_unit(&self) -> Option<Rc<RefCell<StepBasic_TimeUnit>>> {
        self.time_unit.clone()
    }
}

impl Default for StepBasic_ConversionBasedUnitAndTimeUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let unit = StepBasic_ConversionBasedUnitAndTimeUnit::new();
        assert!(unit.time_unit().is_none());
    }
}
