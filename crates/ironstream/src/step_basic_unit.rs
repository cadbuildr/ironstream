// FILE: step_basic_unit.rs
// occt: StepBasic_Unit

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
pub struct StepBasicNamedUnit;
pub struct StepBasicDerivedUnit;

/// Represents a Unit in the STEP AP standard.
///
/// A select type that can hold either a NamedUnit or a DerivedUnit.
#[derive(Clone)]
pub enum StepBasicUnit {
    NamedUnit(Rc<RefCell<StepBasicNamedUnit>>),
    DerivedUnit(Rc<RefCell<StepBasicDerivedUnit>>),
}

impl StepBasicUnit {
    /// Creates a new Unit
    pub fn new() -> Self {
        StepBasicUnit::NamedUnit(Rc::new(RefCell::new(StepBasicNamedUnit)))
    }

    /// Returns the case number for the current type:
    /// 1 -> NamedUnit
    /// 2 -> DerivedUnit
    pub fn case_num(&self) -> i32 {
        match self {
            StepBasicUnit::NamedUnit(_) => 1,
            StepBasicUnit::DerivedUnit(_) => 2,
        }
    }

    /// Returns the value as a NamedUnit (None if another type)
    pub fn named_unit(&self) -> Option<Rc<RefCell<StepBasicNamedUnit>>> {
        match self {
            StepBasicUnit::NamedUnit(nu) => Some(nu.clone()),
            _ => None,
        }
    }

    /// Returns the value as a DerivedUnit (None if another type)
    pub fn derived_unit(&self) -> Option<Rc<RefCell<StepBasicDerivedUnit>>> {
        match self {
            StepBasicUnit::DerivedUnit(du) => Some(du.clone()),
            _ => None,
        }
    }
}

impl Default for StepBasicUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let unit = StepBasicUnit::new();
        assert_eq!(unit.case_num(), 1);
    }

    #[test]
    fn test_case_num_named_unit() {
        let unit = StepBasicUnit::NamedUnit(Rc::new(RefCell::new(StepBasicNamedUnit)));
        assert_eq!(unit.case_num(), 1);
        assert!(unit.named_unit().is_some());
    }

    #[test]
    fn test_case_num_derived_unit() {
        let unit = StepBasicUnit::DerivedUnit(Rc::new(RefCell::new(StepBasicDerivedUnit)));
        assert_eq!(unit.case_num(), 2);
        assert!(unit.derived_unit().is_some());
    }

    #[test]
    fn test_default() {
        let unit = StepBasicUnit::default();
        assert_eq!(unit.case_num(), 1);
    }
}
