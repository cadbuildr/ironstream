// FILE: step_basic_time_unit.rs
// occt: StepBasic_TimeUnit

//! A NamedUnit representing a time unit.

use std::rc::Rc;
use std::cell::RefCell;

/// Represents dimensional exponents (e.g., length, mass, time, etc.)
/// This is a simplified representation for demonstration purposes.
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
        Self {
            length: 0,
            mass: 0,
            time: 1,
            current: 0,
            temperature: 0,
            amount: 0,
            luminosity: 0,
        }
    }
}

/// A TimeUnit is a NamedUnit specifically for time measurements.
/// It carries dimensional exponents to indicate that it represents a time dimension.
#[derive(Debug, Clone)]
pub struct StepBasicTimeUnit {
    /// The dimensional exponents for this unit
    dimensions: Option<Rc<RefCell<DimensionalExponents>>>,
}

impl StepBasicTimeUnit {
    /// Create a new TimeUnit instance
    pub fn new() -> Self {
        Self {
            dimensions: Some(Rc::new(RefCell::new(DimensionalExponents::default()))),
        }
    }

    /// Initialize the TimeUnit with dimensional exponents
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

impl Default for StepBasicTimeUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tu = StepBasicTimeUnit::new();
        assert!(tu.dimensions.is_some());
    }

    #[test]
    fn test_default_dimensions() {
        let tu = StepBasicTimeUnit::new();
        let dims = tu.dimensions().unwrap();
        let dims_ref = dims.borrow();
        assert_eq!(dims_ref.time, 1);
        assert_eq!(dims_ref.length, 0);
        assert_eq!(dims_ref.mass, 0);
    }

    #[test]
    fn test_init() {
        let mut tu = StepBasicTimeUnit::new();
        let new_dims = Rc::new(RefCell::new(DimensionalExponents {
            length: 0,
            mass: 0,
            time: 1,
            current: 0,
            temperature: 0,
            amount: 0,
            luminosity: 0,
        }));
        tu.init(new_dims.clone());
        assert!(tu.dimensions().is_some());
    }

    #[test]
    fn test_set_dimensions() {
        let mut tu = StepBasicTimeUnit::new();
        let custom_dims = Rc::new(RefCell::new(DimensionalExponents {
            length: 1,
            mass: 0,
            time: 0,
            current: 0,
            temperature: 0,
            amount: 0,
            luminosity: 0,
        }));
        tu.set_dimensions(custom_dims);
        let dims = tu.dimensions().unwrap();
        let dims_ref = dims.borrow();
        assert_eq!(dims_ref.length, 1);
    }

    #[test]
    fn test_default() {
        let tu = StepBasicTimeUnit::default();
        assert!(tu.dimensions.is_some());
    }
}
