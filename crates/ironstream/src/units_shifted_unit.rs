// FILE: units_shifted_unit.rs
// occt: Units_ShiftedUnit

use crate::units_unit::UnitsUnit;

/// A unit with an offset (shift)
#[derive(Debug, Clone)]
pub struct UnitsShiftedUnit {
    unit: UnitsUnit,
    shift: f64,
}

impl UnitsShiftedUnit {
    pub fn new(name: &str, symbol: &str, factor: f64, shift: f64, quantity: &str) -> Self {
        UnitsShiftedUnit {
            unit: UnitsUnit::new(name, symbol, factor, quantity),
            shift,
        }
    }

    pub fn unit(&self) -> &UnitsUnit {
        &self.unit
    }

    pub fn shift(&self) -> f64 {
        self.shift
    }

    pub fn set_shift(&mut self, shift: f64) {
        self.shift = shift;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shifted_unit_new() {
        let su = UnitsShiftedUnit::new("Celsius", "C", 1.0, 273.15, "Temperature");
        assert_eq!(su.shift(), 273.15);
    }

    #[test]
    fn test_shifted_unit_set_shift() {
        let mut su = UnitsShiftedUnit::new("Kelvin", "K", 1.0, 0.0, "Temperature");
        su.set_shift(273.15);
        assert_eq!(su.shift(), 273.15);
    }
}
