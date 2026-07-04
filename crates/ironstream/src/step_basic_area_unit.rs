// FILE: step_basic_area_unit.rs
// occt: StepBasic_AreaUnit

use std::cell::RefCell;
use std::rc::Rc;

pub struct StepBasic_DimensionalExponents {
    length: f64,
    mass: f64,
    time: f64,
}

pub struct StepBasic_NamedUnit {
    dimensions: Option<Rc<RefCell<StepBasic_DimensionalExponents>>>,
}

pub struct StepBasic_AreaUnit {
    base: StepBasic_NamedUnit,
}

impl StepBasic_AreaUnit {
    pub fn new() -> Self {
        StepBasic_AreaUnit {
            base: StepBasic_NamedUnit {
                dimensions: None,
            },
        }
    }

    pub fn set_dimensions(&mut self, dimensions: Option<Rc<RefCell<StepBasic_DimensionalExponents>>>) {
        self.base.dimensions = dimensions;
    }

    pub fn dimensions(&self) -> Option<Rc<RefCell<StepBasic_DimensionalExponents>>> {
        self.base.dimensions.clone()
    }
}

impl Default for StepBasic_AreaUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let unit = StepBasic_AreaUnit::new();
        assert!(unit.dimensions().is_none());
    }
}
