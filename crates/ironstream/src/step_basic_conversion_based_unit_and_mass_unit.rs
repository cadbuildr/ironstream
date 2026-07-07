// FILE: step_basic_conversion_based_unit_and_mass_unit.rs
// occt: StepBasic_ConversionBasedUnitAndMassUnit

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_MassUnit {
    dummy: i32,
}

impl StepBasic_MassUnit {
    pub fn new() -> Self {
        StepBasic_MassUnit { dummy: 0 }
    }
}

pub struct StepBasic_ConversionBasedUnit {
    name: Option<Rc<RefCell<String>>>,
}

pub struct StepBasic_ConversionBasedUnitAndMassUnit {
    base: StepBasic_ConversionBasedUnit,
    mass_unit: Option<Rc<RefCell<StepBasic_MassUnit>>>,
}

impl StepBasic_ConversionBasedUnitAndMassUnit {
    pub fn new() -> Self {
        StepBasic_ConversionBasedUnitAndMassUnit {
            base: StepBasic_ConversionBasedUnit { name: None },
            mass_unit: None,
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

    pub fn set_mass_unit(&mut self, mass_unit: Option<Rc<RefCell<StepBasic_MassUnit>>>) {
        self.mass_unit = mass_unit;
    }

    pub fn mass_unit(&self) -> Option<Rc<RefCell<StepBasic_MassUnit>>> {
        self.mass_unit.clone()
    }
}

impl Default for StepBasic_ConversionBasedUnitAndMassUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let unit = StepBasic_ConversionBasedUnitAndMassUnit::new();
        assert!(unit.mass_unit().is_none());
    }
}
