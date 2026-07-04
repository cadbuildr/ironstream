// FILE: step_basic_volume_unit.rs
// occt: StepBasic_VolumeUnit

//! A NamedUnit representing a volume unit.

use std::rc::Rc;
use std::cell::RefCell;

/// Represents dimensional exponents for units.
#[derive(Debug, Clone)]
pub struct DimensionalExponents {
    /// Length exponent
    pub length: i32,
    /// Mass exponent
    pub mass: i32,
    /// Time exponent
    pub time: i32,
    /// Electric current exponent
    pub current: i32,
    /// Temperature exponent
    pub temperature: i32,
    /// Amount of substance exponent
    pub amount: i32,
    /// Luminous intensity exponent
    pub luminosity: i32,
}

impl Default for DimensionalExponents {
    fn default() -> Self {
        // Volume has exponent 3 for length
        Self {
            length: 3,
            mass: 0,
            time: 0,
            current: 0,
            temperature: 0,
            amount: 0,
            luminosity: 0,
        }
    }
}

/// A VolumeUnit is a NamedUnit specifically for volume measurements.
/// It carries dimensional exponents to indicate that it represents a volume dimension (L^3).
#[derive(Debug, Clone)]
pub struct StepBasicVolumeUnit {
    /// The dimensional exponents for this unit
    dimensions: Option<Rc<RefCell<DimensionalExponents>>>,
}

impl StepBasicVolumeUnit {
    /// Create a new VolumeUnit instance
    pub fn new() -> Self {
        Self {
            dimensions: Some(Rc::new(RefCell::new(DimensionalExponents::default()))),
        }
    }

    /// Initialize the VolumeUnit with dimensional exponents
    pub fn init(&mut self, dimensions: Rc<RefCell<DimensionalExponents>>) {
        self.dimensions = Some(dimensions);
    }

    /// Set the dimensional exponents
    pub fn set_dimensions(&mut self, dimensions: Rc<RefCell<DimensionalExponents>>) {
        self.dimensions = Some(dimensions);
    }

    /// Get the dimensional exponents
    pub fn dimensions(&self) -> Option<Rc<RefCell<DimensionalExponents>>> {
        self.dimensions.clone()
    }
}

impl Default for StepBasicVolumeUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vu = StepBasicVolumeUnit::new();
        assert!(vu.dimensions.is_some());
    }

    #[test]
    fn test_default_dimensions() {
        let vu = StepBasicVolumeUnit::new();
        let dims = vu.dimensions().unwrap();
        let dims_ref = dims.borrow();
        assert_eq!(dims_ref.length, 3);
        assert_eq!(dims_ref.mass, 0);
        assert_eq!(dims_ref.time, 0);
    }

    #[test]
    fn test_init() {
        let mut vu = StepBasicVolumeUnit::new();
        let new_dims = Rc::new(RefCell::new(DimensionalExponents {
            length: 3,
            mass: 0,
            time: 0,
            current: 0,
            temperature: 0,
            amount: 0,
            luminosity: 0,
        }));
        vu.init(new_dims.clone());
        assert!(vu.dimensions().is_some());
    }

    #[test]
    fn test_set_dimensions() {
        let mut vu = StepBasicVolumeUnit::new();
        let custom_dims = Rc::new(RefCell::new(DimensionalExponents {
            length: 3,
            mass: 0,
            time: 0,
            current: 0,
            temperature: 0,
            amount: 0,
            luminosity: 0,
        }));
        vu.set_dimensions(custom_dims);
        let dims = vu.dimensions().unwrap();
        let dims_ref = dims.borrow();
        assert_eq!(dims_ref.length, 3);
    }

    #[test]
    fn test_default() {
        let vu = StepBasicVolumeUnit::default();
        assert!(vu.dimensions.is_some());
    }
}
