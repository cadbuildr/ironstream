// FILE: geom_lib_denominator_multiplier_ptr.rs
// occt: GeomLib_DenominatorMultiplierPtr

/// Pointer for denominator multiplier in geometric operations.
pub struct GeomLibDenominatorMultiplierPtr {
    value: f64,
}

impl GeomLibDenominatorMultiplierPtr {
    /// Create a new denominator multiplier pointer.
    pub fn new(val: f64) -> Self {
        GeomLibDenominatorMultiplierPtr { value: val }
    }

    /// Get the value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Set the value.
    pub fn set_value(&mut self, val: f64) {
        self.value = val;
    }
}

impl Default for GeomLibDenominatorMultiplierPtr {
    fn default() -> Self {
        Self::new(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_multiplier() {
        let mult = GeomLibDenominatorMultiplierPtr::new(2.5);
        assert_eq!(mult.value(), 2.5);
    }

    #[test]
    fn test_set_value() {
        let mut mult = GeomLibDenominatorMultiplierPtr::new(1.0);
        mult.set_value(3.0);
        assert_eq!(mult.value(), 3.0);
    }

    #[test]
    fn test_default() {
        let mult = GeomLibDenominatorMultiplierPtr::default();
        assert_eq!(mult.value(), 1.0);
    }
}
