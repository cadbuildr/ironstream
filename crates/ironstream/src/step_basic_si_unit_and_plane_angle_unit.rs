// FILE: step_basic_si_unit_and_plane_angle_unit.rs
// occt: StepBasic_SiUnitAndPlaneAngleUnit

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

// Local helper mirroring StepBasic_PlaneAngleUnit (external plumbing).
// StepBasic_PlaneAngleUnit::Init takes a DimensionalExponents handle.
#[derive(Debug)]
pub struct StepBasicPlaneAngleUnit {
    dimensions: Option<StepBasicDimensionalExponents>,
}

impl StepBasicPlaneAngleUnit {
    pub fn new() -> Self {
        StepBasicPlaneAngleUnit { dimensions: None }
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

// Complex (ANDOR) entity: a SiUnit that is also a PlaneAngleUnit
pub struct StepBasicSiUnitAndPlaneAngleUnit {
    base: StepBasicSiUnit,
    plane_angle_unit: Option<Rc<RefCell<StepBasicPlaneAngleUnit>>>,
}

impl StepBasicSiUnitAndPlaneAngleUnit {
    // Returns a SiUnitAndMassUnit (fields left default, as in OCCT ctor)
    pub fn new() -> Self {
        StepBasicSiUnitAndPlaneAngleUnit {
            base: StepBasicSiUnit::new(),
            plane_angle_unit: None,
        }
    }

    // Mirrors StepBasic_SiUnitAndPlaneAngleUnit::Init: builds the MassUnit
    // component (with null dimensions) and inits the SiUnit component.
    pub fn init(
        &mut self,
        has_a_prefix: bool,
        a_prefix: StepBasicSiPrefix,
        a_name: StepBasicSiUnitName,
    ) {
        let mut pau = StepBasicPlaneAngleUnit::new();
        pau.init(None);
        self.plane_angle_unit = Some(Rc::new(RefCell::new(pau)));
        self.base.init(has_a_prefix, a_prefix, a_name);
    }

    pub fn set_plane_angle_unit(&mut self, a_plane_angle_unit: Rc<RefCell<StepBasicPlaneAngleUnit>>) {
        self.plane_angle_unit = Some(a_plane_angle_unit);
    }

    pub fn plane_angle_unit(&self) -> Option<Rc<RefCell<StepBasicPlaneAngleUnit>>> {
        self.plane_angle_unit.clone()
    }

    // Access to the SiUnit base component
    pub fn si_unit(&self) -> &StepBasicSiUnit {
        &self.base
    }

    pub fn si_unit_mut(&mut self) -> &mut StepBasicSiUnit {
        &mut self.base
    }
}

impl Default for StepBasicSiUnitAndPlaneAngleUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let u = StepBasicSiUnitAndPlaneAngleUnit::new();
        assert!(u.plane_angle_unit().is_none());
        assert!(!u.si_unit().has_prefix());
    }

    #[test]
    fn test_init_builds_plane_angle_unit_and_si_unit() {
        let mut u = StepBasicSiUnitAndPlaneAngleUnit::new();
        u.init(true, StepBasicSiPrefix::Kilo, StepBasicSiUnitName::Radian);

        let pau = u.plane_angle_unit().expect("init must create a plane angle unit");
        assert!(pau.borrow().dimensions().is_none());

        assert!(u.si_unit().has_prefix());
        assert_eq!(u.si_unit().prefix(), StepBasicSiPrefix::Kilo);
        assert_eq!(u.si_unit().name(), StepBasicSiUnitName::Radian);
    }

    #[test]
    fn test_set_plane_angle_unit() {
        let mut u = StepBasicSiUnitAndPlaneAngleUnit::new();
        let pau = Rc::new(RefCell::new(StepBasicPlaneAngleUnit::new()));
        u.set_plane_angle_unit(Rc::clone(&pau));
        assert!(Rc::ptr_eq(&u.plane_angle_unit().unwrap(), &pau));
    }

    #[test]
    fn test_si_unit_prefix_handling() {
        let mut u = StepBasicSiUnitAndPlaneAngleUnit::new();
        u.init(false, StepBasicSiPrefix::Exa, StepBasicSiUnitName::Radian);
        assert!(!u.si_unit().has_prefix());

        u.si_unit_mut().set_prefix(StepBasicSiPrefix::Milli);
        assert!(u.si_unit().has_prefix());
        assert_eq!(u.si_unit().prefix(), StepBasicSiPrefix::Milli);

        u.si_unit_mut().unset_prefix();
        assert!(!u.si_unit().has_prefix());
    }
}
