// FILE: step_basic_si_unit_and_area_unit.rs
// occt: StepBasic_SiUnitAndAreaUnit

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

/// Local model of StepBasic_SiUnitName (subset order matches OCCT enum).
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

/// Local model of the StepBasic_SiUnit base class:
/// optional SI prefix plus an SI unit name.
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

/// Local model of StepBasic_AreaUnit (external plumbing: a NamedUnit
/// with dimensional exponents; only the identity matters here).
#[derive(Debug, Clone, Default)]
pub struct StepBasicAreaUnit;

/// StepBasic_SiUnitAndAreaUnit: complex STEP entity combining
/// SI_UNIT and AREA_UNIT. Inherits StepBasic_SiUnit and carries a
/// handle to an AreaUnit component.
pub struct StepBasicSiUnitAndAreaUnit {
    base: StepBasicSiUnit,
    area_unit: Option<Rc<RefCell<StepBasicAreaUnit>>>,
}

impl StepBasicSiUnitAndAreaUnit {
    /// Returns a SiUnitAndAreaUnit (OCCT default ctor).
    pub fn new() -> Self {
        StepBasicSiUnitAndAreaUnit {
            base: StepBasicSiUnit::new(),
            area_unit: None,
        }
    }

    /// OCCT SetAreaUnit.
    pub fn set_area_unit(&mut self, area_unit: Rc<RefCell<StepBasicAreaUnit>>) {
        self.area_unit = Some(area_unit);
    }

    /// OCCT AreaUnit.
    pub fn area_unit(&self) -> Option<Rc<RefCell<StepBasicAreaUnit>>> {
        self.area_unit.clone()
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

impl Default for StepBasicSiUnitAndAreaUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let u = StepBasicSiUnitAndAreaUnit::new();
        assert!(u.area_unit().is_none());
        assert!(!u.si_unit().has_prefix());
        assert!(u.si_unit().name().is_none());
    }

    #[test]
    fn test_set_area_unit_handle() {
        let mut u = StepBasicSiUnitAndAreaUnit::new();
        let au = Rc::new(RefCell::new(StepBasicAreaUnit));
        u.set_area_unit(au.clone());
        assert!(u.area_unit().is_some());
        assert!(Rc::ptr_eq(&u.area_unit().unwrap(), &au));
    }

    #[test]
    fn test_inherited_si_unit_init() {
        let mut u = StepBasicSiUnitAndAreaUnit::new();
        u.si_unit_mut().init(true, SiPrefix::Milli, SiUnitName::Metre);
        assert!(u.si_unit().has_prefix());
        assert_eq!(u.si_unit().prefix(), Some(SiPrefix::Milli));
        assert_eq!(u.si_unit().name(), Some(SiUnitName::Metre));
    }

    #[test]
    fn test_unset_prefix_keeps_prefix_value() {
        // OCCT UnSetPrefix only resets the flag, not the stored prefix.
        let mut u = StepBasicSiUnitAndAreaUnit::new();
        u.si_unit_mut().set_prefix(SiPrefix::Kilo);
        assert!(u.si_unit().has_prefix());
        u.si_unit_mut().unset_prefix();
        assert!(!u.si_unit().has_prefix());
        assert_eq!(u.si_unit().prefix(), Some(SiPrefix::Kilo));
    }

    #[test]
    fn test_default() {
        let u = StepBasicSiUnitAndAreaUnit::default();
        assert!(u.area_unit().is_none());
    }
}
