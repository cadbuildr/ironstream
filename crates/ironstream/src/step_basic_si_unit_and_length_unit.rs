// FILE: step_basic_si_unit_and_length_unit.rs
// occt: StepBasic_SiUnitAndLengthUnit

use std::cell::RefCell;
use std::rc::Rc;

/// Local model of StepBasic_SiPrefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SiPrefix {
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

/// Local model of StepBasic_SiUnitName (order matches OCCT enum).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SiUnitName {
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

/// Local model of the StepBasic_SiUnit base class.
#[derive(Debug, Clone)]
pub struct StepBasicSiUnit {
    has_prefix: bool,
    prefix: Option<SiPrefix>,
    name: Option<SiUnitName>,
}

impl StepBasicSiUnit {
    pub fn new() -> Self {
        StepBasicSiUnit {
            has_prefix: false,
            prefix: None,
            name: None,
        }
    }

    /// OCCT StepBasic_SiUnit::Init.
    pub fn init(&mut self, has_prefix: bool, prefix: SiPrefix, name: SiUnitName) {
        self.has_prefix = has_prefix;
        self.prefix = Some(prefix);
        self.name = Some(name);
    }

    /// OCCT SetPrefix: sets the prefix and marks it present.
    pub fn set_prefix(&mut self, prefix: SiPrefix) {
        self.prefix = Some(prefix);
        self.has_prefix = true;
    }

    /// OCCT UnSetPrefix: only clears the flag.
    pub fn unset_prefix(&mut self) {
        self.has_prefix = false;
    }

    pub fn prefix(&self) -> Option<SiPrefix> {
        self.prefix
    }

    pub fn has_prefix(&self) -> bool {
        self.has_prefix
    }

    pub fn set_name(&mut self, name: SiUnitName) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<SiUnitName> {
        self.name
    }
}

impl Default for StepBasicSiUnit {
    fn default() -> Self {
        Self::new()
    }
}

/// Local model of StepBasic_LengthUnit (external plumbing: a NamedUnit
/// with dimensional exponents; only identity is relevant here).
#[derive(Debug, Clone, Default)]
pub struct StepBasicLengthUnit;

/// StepBasic_SiUnitAndLengthUnit: complex STEP entity combining
/// SI_UNIT and LENGTH_UNIT.
pub struct StepBasicSiUnitAndLengthUnit {
    base: StepBasicSiUnit,
    length_unit: Option<Rc<RefCell<StepBasicLengthUnit>>>,
}

impl StepBasicSiUnitAndLengthUnit {
    /// Returns a SiUnitAndLengthUnit (OCCT default ctor).
    pub fn new() -> Self {
        StepBasicSiUnitAndLengthUnit {
            base: StepBasicSiUnit::new(),
            length_unit: None,
        }
    }

    /// OCCT Init: creates the LengthUnit ANDOR component then
    /// initializes the inherited SiUnit fields.
    pub fn init(&mut self, has_prefix: bool, prefix: SiPrefix, name: SiUnitName) {
        self.length_unit = Some(Rc::new(RefCell::new(StepBasicLengthUnit)));
        self.base.init(has_prefix, prefix, name);
    }

    /// OCCT SetLengthUnit.
    pub fn set_length_unit(&mut self, length_unit: Rc<RefCell<StepBasicLengthUnit>>) {
        self.length_unit = Some(length_unit);
    }

    /// OCCT LengthUnit.
    pub fn length_unit(&self) -> Option<Rc<RefCell<StepBasicLengthUnit>>> {
        self.length_unit.clone()
    }

    /// Access to the inherited SiUnit part.
    pub fn si_unit(&self) -> &StepBasicSiUnit {
        &self.base
    }

    /// Mutable access to the inherited SiUnit part.
    pub fn si_unit_mut(&mut self) -> &mut StepBasicSiUnit {
        &mut self.base
    }
}

impl Default for StepBasicSiUnitAndLengthUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let u = StepBasicSiUnitAndLengthUnit::new();
        assert!(u.length_unit().is_none());
        assert!(!u.si_unit().has_prefix());
        assert!(u.si_unit().name().is_none());
    }

    #[test]
    fn test_init_creates_length_unit_component() {
        // OCCT Init instantiates the LengthUnit ANDOR component itself.
        let mut u = StepBasicSiUnitAndLengthUnit::new();
        u.init(true, SiPrefix::Milli, SiUnitName::Metre);
        assert!(u.length_unit().is_some());
        assert!(u.si_unit().has_prefix());
        assert_eq!(u.si_unit().prefix(), Some(SiPrefix::Milli));
        assert_eq!(u.si_unit().name(), Some(SiUnitName::Metre));
    }

    #[test]
    fn test_set_length_unit_handle() {
        let mut u = StepBasicSiUnitAndLengthUnit::new();
        let lu = Rc::new(RefCell::new(StepBasicLengthUnit));
        u.set_length_unit(lu.clone());
        assert!(u.length_unit().is_some());
        assert!(Rc::ptr_eq(&u.length_unit().unwrap(), &lu));
    }

    #[test]
    fn test_unset_prefix_keeps_prefix_value() {
        // OCCT UnSetPrefix only resets the flag, not the stored prefix.
        let mut u = StepBasicSiUnitAndLengthUnit::new();
        u.si_unit_mut().set_prefix(SiPrefix::Centi);
        u.si_unit_mut().unset_prefix();
        assert!(!u.si_unit().has_prefix());
        assert_eq!(u.si_unit().prefix(), Some(SiPrefix::Centi));
    }

    #[test]
    fn test_default() {
        let u = StepBasicSiUnitAndLengthUnit::default();
        assert!(u.length_unit().is_none());
    }
}
