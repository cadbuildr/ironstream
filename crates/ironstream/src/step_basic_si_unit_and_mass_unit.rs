// FILE: step_basic_si_unit_and_mass_unit.rs
// occt: StepBasic_SiUnitAndMassUnit

use std::cell::RefCell;
use std::rc::Rc;

// Local helper mirroring StepBasic_SiPrefix (external plumbing)
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

// Local helper mirroring StepBasic_SiUnitName (external plumbing, subset)
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
    Newton,
}

// Local helper mirroring StepBasic_DimensionalExponents (external plumbing)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StepBasicDimensionalExponents;

// Local helper mirroring StepBasic_MassUnit (external plumbing).
// StepBasic_MassUnit::Init takes a DimensionalExponents handle.
#[derive(Debug)]
pub struct StepBasicMassUnit {
    dimensions: Option<StepBasicDimensionalExponents>,
}

impl StepBasicMassUnit {
    pub fn new() -> Self {
        StepBasicMassUnit { dimensions: None }
    }

    pub fn init(&mut self, dimensions: Option<StepBasicDimensionalExponents>) {
        self.dimensions = dimensions;
    }

    pub fn dimensions(&self) -> Option<StepBasicDimensionalExponents> {
        self.dimensions
    }
}

// Local helper mirroring the StepBasic_SiUnit base class (external plumbing)
#[derive(Debug)]
pub struct StepBasicSiUnit {
    has_prefix: bool,
    prefix: StepBasicSiPrefix,
    name: StepBasicSiUnitName,
}

impl StepBasicSiUnit {
    pub fn new() -> Self {
        StepBasicSiUnit {
            has_prefix: false,
            prefix: StepBasicSiPrefix::Exa,
            name: StepBasicSiUnitName::Metre,
        }
    }

    pub fn init(
        &mut self,
        has_a_prefix: bool,
        a_prefix: StepBasicSiPrefix,
        a_name: StepBasicSiUnitName,
    ) {
        self.has_prefix = has_a_prefix;
        self.prefix = a_prefix;
        self.name = a_name;
    }

    pub fn set_prefix(&mut self, a_prefix: StepBasicSiPrefix) {
        self.prefix = a_prefix;
        self.has_prefix = true;
    }

    pub fn unset_prefix(&mut self) {
        self.has_prefix = false;
    }

    pub fn prefix(&self) -> StepBasicSiPrefix {
        self.prefix
    }

    pub fn has_prefix(&self) -> bool {
        self.has_prefix
    }

    pub fn set_name(&mut self, a_name: StepBasicSiUnitName) {
        self.name = a_name;
    }

    pub fn name(&self) -> StepBasicSiUnitName {
        self.name
    }
}

// Complex (ANDOR) entity: a SiUnit that is also a MassUnit
pub struct StepBasicSiUnitAndMassUnit {
    base: StepBasicSiUnit,
    mass_unit: Option<Rc<RefCell<StepBasicMassUnit>>>,
}

impl StepBasicSiUnitAndMassUnit {
    // Returns a SiUnitAndMassUnit (fields left default, as in OCCT ctor)
    pub fn new() -> Self {
        StepBasicSiUnitAndMassUnit {
            base: StepBasicSiUnit::new(),
            mass_unit: None,
        }
    }

    // Mirrors StepBasic_SiUnitAndMassUnit::Init: builds the MassUnit
    // component (with null dimensions) and inits the SiUnit component.
    pub fn init(
        &mut self,
        has_a_prefix: bool,
        a_prefix: StepBasicSiPrefix,
        a_name: StepBasicSiUnitName,
    ) {
        let mut mass = StepBasicMassUnit::new();
        mass.init(None);
        self.mass_unit = Some(Rc::new(RefCell::new(mass)));
        self.base.init(has_a_prefix, a_prefix, a_name);
    }

    pub fn set_mass_unit(&mut self, a_mass_unit: Rc<RefCell<StepBasicMassUnit>>) {
        self.mass_unit = Some(a_mass_unit);
    }

    pub fn mass_unit(&self) -> Option<Rc<RefCell<StepBasicMassUnit>>> {
        self.mass_unit.clone()
    }

    // Access to the SiUnit base component
    pub fn si_unit(&self) -> &StepBasicSiUnit {
        &self.base
    }

    pub fn si_unit_mut(&mut self) -> &mut StepBasicSiUnit {
        &mut self.base
    }
}

impl Default for StepBasicSiUnitAndMassUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let u = StepBasicSiUnitAndMassUnit::new();
        assert!(u.mass_unit().is_none());
        assert!(!u.si_unit().has_prefix());
    }

    #[test]
    fn test_init_builds_mass_unit_and_si_unit() {
        let mut u = StepBasicSiUnitAndMassUnit::new();
        u.init(true, StepBasicSiPrefix::Kilo, StepBasicSiUnitName::Gram);

        let mass = u.mass_unit().expect("init must create a mass unit");
        assert!(mass.borrow().dimensions().is_none());

        assert!(u.si_unit().has_prefix());
        assert_eq!(u.si_unit().prefix(), StepBasicSiPrefix::Kilo);
        assert_eq!(u.si_unit().name(), StepBasicSiUnitName::Gram);
    }

    #[test]
    fn test_set_mass_unit() {
        let mut u = StepBasicSiUnitAndMassUnit::new();
        let mass = Rc::new(RefCell::new(StepBasicMassUnit::new()));
        u.set_mass_unit(Rc::clone(&mass));
        assert!(Rc::ptr_eq(&u.mass_unit().unwrap(), &mass));
    }

    #[test]
    fn test_si_unit_prefix_handling() {
        let mut u = StepBasicSiUnitAndMassUnit::new();
        u.init(false, StepBasicSiPrefix::Exa, StepBasicSiUnitName::Gram);
        assert!(!u.si_unit().has_prefix());

        u.si_unit_mut().set_prefix(StepBasicSiPrefix::Milli);
        assert!(u.si_unit().has_prefix());
        assert_eq!(u.si_unit().prefix(), StepBasicSiPrefix::Milli);

        u.si_unit_mut().unset_prefix();
        assert!(!u.si_unit().has_prefix());
    }
}
