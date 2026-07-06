// FILE: step_basic_ratio_unit.rs
// occt: StepBasic_RatioUnit

use std::rc::Rc;
use std::cell::RefCell;

/// Local placeholder for the referenced StepBasic_DimensionalExponents entity
/// (external plumbing; only referenced through shared handles here).
#[derive(Debug, PartialEq)]
pub struct StepBasicDimensionalExponents;

/// Local mirror of the StepBasic_NamedUnit base class:
/// holds the dimensional exponents of the unit.
pub struct StepBasicNamedUnit {
    dimensions: Option<Rc<RefCell<StepBasicDimensionalExponents>>>,
}

impl StepBasicNamedUnit {
    pub fn new() -> Self {
        StepBasicNamedUnit { dimensions: None }
    }

    pub fn init(&mut self, dimensions: Rc<RefCell<StepBasicDimensionalExponents>>) {
        self.dimensions = Some(dimensions);
    }

    pub fn set_dimensions(&mut self, dimensions: Rc<RefCell<StepBasicDimensionalExponents>>) {
        self.dimensions = Some(dimensions);
    }

    pub fn dimensions(&self) -> Option<Rc<RefCell<StepBasicDimensionalExponents>>> {
        self.dimensions.clone()
    }
}

impl Default for StepBasicNamedUnit {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a RatioUnit in the STEP AP standard.
///
/// In OCCT, StepBasic_RatioUnit inherits StepBasic_NamedUnit and adds no
/// fields of its own.
pub struct StepBasicRatioUnit {
    base: StepBasicNamedUnit,
}

impl StepBasicRatioUnit {
    /// Creates a new, uninitialized RatioUnit
    pub fn new() -> Self {
        StepBasicRatioUnit {
            base: StepBasicNamedUnit::new(),
        }
    }

    // Delegate to base class
    pub fn init(&mut self, dimensions: Rc<RefCell<StepBasicDimensionalExponents>>) {
        self.base.init(dimensions);
    }

    pub fn set_dimensions(&mut self, dimensions: Rc<RefCell<StepBasicDimensionalExponents>>) {
        self.base.set_dimensions(dimensions);
    }

    pub fn dimensions(&self) -> Option<Rc<RefCell<StepBasicDimensionalExponents>>> {
        self.base.dimensions()
    }
}

impl Default for StepBasicRatioUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let ru = StepBasicRatioUnit::new();
        assert_eq!(ru.dimensions(), None);
    }

    #[test]
    fn test_set_and_get_dimensions() {
        let mut ru = StepBasicRatioUnit::new();
        let dims = Rc::new(RefCell::new(StepBasicDimensionalExponents));
        ru.set_dimensions(dims.clone());
        assert!(ru.dimensions().is_some());
        assert!(Rc::ptr_eq(&ru.dimensions().unwrap(), &dims));
    }

    #[test]
    fn test_default() {
        let ru = StepBasicRatioUnit::default();
        assert_eq!(ru.dimensions(), None);
    }
}
