// FILE: step_basic_si_unit.rs
// occt: StepBasic_SiUnit

use std::rc::Rc;
use std::cell::RefCell;

use crate::step_basic_named_unit::StepBasicNamedUnit;
use crate::step_basic_si_prefix::StepBasicSiPrefix;
use crate::step_basic_si_unit_name::StepBasicSiUnitName;

// Placeholder types
pub struct StepBasicDimensionalExponents;

/// Represents a SiUnit in the STEP AP standard.
///
/// Extends NamedUnit to represent SI units with optional prefix.
pub struct StepBasicSiUnit {
    base: StepBasicNamedUnit,
    prefix: Option<StepBasicSiPrefix>,
    name: StepBasicSiUnitName,
}

impl StepBasicSiUnit {
    pub fn new() -> Self {
        StepBasicSiUnit {
            base: StepBasicNamedUnit::new(),
            prefix: None,
            name: StepBasicSiUnitName::Metre,
        }
    }

    pub fn init(
        &mut self,
        has_prefix: bool,
        prefix: StepBasicSiPrefix,
        name: StepBasicSiUnitName,
    ) {
        self.prefix = if has_prefix { Some(prefix) } else { None };
        self.name = name;
    }

    pub fn set_prefix(&mut self, prefix: StepBasicSiPrefix) {
        self.prefix = Some(prefix);
    }

    pub fn unset_prefix(&mut self) {
        self.prefix = None;
    }

    pub fn prefix(&self) -> Option<StepBasicSiPrefix> {
        self.prefix
    }

    pub fn has_prefix(&self) -> bool {
        self.prefix.is_some()
    }

    pub fn set_name(&mut self, name: StepBasicSiUnitName) {
        self.name = name;
    }

    pub fn name(&self) -> StepBasicSiUnitName {
        self.name
    }

    pub fn set_dimensions(&mut self, dimensions: Rc<RefCell<StepBasicDimensionalExponents>>) {
        self.base.set_dimensions(dimensions);
    }

    pub fn dimensions(&self) -> Option<Rc<RefCell<StepBasicDimensionalExponents>>> {
        self.base.dimensions()
    }
}

impl Default for StepBasicSiUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let su = StepBasicSiUnit::new();
        assert!(!su.has_prefix());
        assert_eq!(su.name(), StepBasicSiUnitName::Metre);
    }

    #[test]
    fn test_set_and_get_prefix() {
        let mut su = StepBasicSiUnit::new();
        su.set_prefix(StepBasicSiPrefix::Kilo);
        assert!(su.has_prefix());
        assert_eq!(su.prefix(), Some(StepBasicSiPrefix::Kilo));
    }

    #[test]
    fn test_unset_prefix() {
        let mut su = StepBasicSiUnit::new();
        su.set_prefix(StepBasicSiPrefix::Milli);
        su.unset_prefix();
        assert!(!su.has_prefix());
    }

    #[test]
    fn test_set_and_get_name() {
        let mut su = StepBasicSiUnit::new();
        su.set_name(StepBasicSiUnitName::Joule);
        assert_eq!(su.name(), StepBasicSiUnitName::Joule);
    }

    #[test]
    fn test_default() {
        let su = StepBasicSiUnit::default();
        assert!(!su.has_prefix());
    }
}
