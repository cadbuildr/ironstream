// FILE: step_basic_conversion_based_unit.rs
// occt: StepBasic_ConversionBasedUnit

use std::cell::RefCell;
use std::rc::Rc;

pub struct HString {
    value: String,
}

impl HString {
    pub fn new(value: String) -> Rc<RefCell<HString>> {
        Rc::new(RefCell::new(HString { value }))
    }
}

pub struct StepBasic_DimensionalExponents {
    values: Vec<f64>,
}

pub struct StepBasic_NamedUnit {
    dimensions: Option<Rc<RefCell<StepBasic_DimensionalExponents>>>,
}

pub struct StepBasic_MeasureWithUnit;

pub struct StepBasic_ConversionBasedUnit {
    base: StepBasic_NamedUnit,
    name: Option<Rc<RefCell<HString>>>,
    conversion_factor: Option<Rc<RefCell<StepBasic_MeasureWithUnit>>>,
}

impl StepBasic_ConversionBasedUnit {
    pub fn new() -> Self {
        StepBasic_ConversionBasedUnit {
            base: StepBasic_NamedUnit {
                dimensions: None,
            },
            name: None,
            conversion_factor: None,
        }
    }

    pub fn init(
        &mut self,
        name: Option<Rc<RefCell<HString>>>,
        conversion_factor: Option<Rc<RefCell<StepBasic_MeasureWithUnit>>>,
    ) {
        self.name = name;
        self.conversion_factor = conversion_factor;
    }

    pub fn set_name(&mut self, name: Option<Rc<RefCell<HString>>>) {
        self.name = name;
    }

    pub fn name(&self) -> Option<Rc<RefCell<HString>>> {
        self.name.clone()
    }

    pub fn set_conversion_factor(&mut self, cf: Option<Rc<RefCell<StepBasic_MeasureWithUnit>>>) {
        self.conversion_factor = cf;
    }

    pub fn conversion_factor(&self) -> Option<Rc<RefCell<StepBasic_MeasureWithUnit>>> {
        self.conversion_factor.clone()
    }

    pub fn set_dimensions(&mut self, dimensions: Option<Rc<RefCell<StepBasic_DimensionalExponents>>>) {
        self.base.dimensions = dimensions;
    }

    pub fn dimensions(&self) -> Option<Rc<RefCell<StepBasic_DimensionalExponents>>> {
        self.base.dimensions.clone()
    }
}

impl Default for StepBasic_ConversionBasedUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let unit = StepBasic_ConversionBasedUnit::new();
        assert!(unit.name().is_none());
        assert!(unit.conversion_factor().is_none());
    }

    #[test]
    fn test_set_name() {
        let mut unit = StepBasic_ConversionBasedUnit::new();
        let name = HString::new("inch".to_string());
        unit.set_name(Some(name));
        assert!(unit.name().is_some());
    }
}
