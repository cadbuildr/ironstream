// FILE: step_basic_si_unit.rs
// occt: StepBasic_SiUnit

use std::rc::Rc;
use std::cell::RefCell;

// Placeholder types
#[derive(Debug, PartialEq)]
pub struct StepBasicDimensionalExponents;

/// Local mirror of the StepBasic_SiPrefix enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepBasicSiPrefix {
    Exa,
    Peta,
    Tera,
    Giga,
    Mega,
    Kilo,
    Hecto,
    Deca,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
}

/// Local mirror of the StepBasic_SiUnitName enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepBasicSiUnitName {
    Metre,
    Gram,
    Second,
    Ampere,
    Kelvin,
    Mole,
    Candela,
    Radian,
    Steradian,
    Hertz,
    Newton,
    Pascal,
    Joule,
    Watt,
    Coulomb,
    Volt,
    Farad,
    Ohm,
    Siemens,
    Weber,
    Tesla,
    Henry,
    DegreeCelsius,
    Lumen,
    Lux,
    Becquerel,
    Gray,
    Sievert,
}

/// Local mirror of the StepBasic_NamedUnit base class:
/// holds the dimensional exponents of the unit.
pub struct StepBasicNamedUnit {
    dimensions: Option<Rc<RefCell<StepBasicDimensionalExponents>>>,
}

impl StepBasicNamedUnit {
    pub fn new() -> Self {
        StepBasicNamedUnit { dimensions: None }
    }
}

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

    /// In OCCT, StepBasic_SiUnit redefines SetDimensions as a forbidden
    /// no-op: the dimensions field is redefined and may not be set.
    pub fn set_dimensions(&mut self, _dimensions: Rc<RefCell<StepBasicDimensionalExponents>>) {
        // Field is redefined; set up forbidden (matches OCCT behavior).
    }

    /// In OCCT, StepBasic_SiUnit redefines Dimensions to return a null
    /// handle (the field is redefined).
    pub fn dimensions(&self) -> Option<Rc<RefCell<StepBasicDimensionalExponents>>> {
        None
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
