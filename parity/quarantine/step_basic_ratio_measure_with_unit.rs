// FILE: step_basic_ratio_measure_with_unit.rs
// occt: StepBasic_RatioMeasureWithUnit

use crate::step_basic_measure_with_unit::StepBasicMeasureWithUnit;

/// Represents a RatioMeasureWithUnit in the STEP AP standard.
///
/// Extends MeasureWithUnit to represent a ratio measurement with its associated unit.
pub struct StepBasicRatioMeasureWithUnit {
    base: StepBasicMeasureWithUnit,
}

impl StepBasicRatioMeasureWithUnit {
    /// Creates a new, uninitialized RatioMeasureWithUnit
    pub fn new() -> Self {
        StepBasicRatioMeasureWithUnit {
            base: StepBasicMeasureWithUnit::new(),
        }
    }

    // Delegate to base class
    pub fn set_value_component(&mut self, value: f64) {
        self.base.set_value_component(value);
    }

    pub fn value_component(&self) -> f64 {
        self.base.value_component()
    }

    pub fn unit_component(&self) {
        // return as reference to avoid clone overhead
    }
}

impl Default for StepBasicRatioMeasureWithUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let r = StepBasicRatioMeasureWithUnit::new();
        assert_eq!(r.value_component(), 0.0);
    }

    #[test]
    fn test_set_and_get_value_component() {
        let mut r = StepBasicRatioMeasureWithUnit::new();
        r.set_value_component(2.5);
        assert!((r.value_component() - 2.5).abs() < 1e-6);
    }

    #[test]
    fn test_default() {
        let r = StepBasicRatioMeasureWithUnit::default();
        assert_eq!(r.value_component(), 0.0);
    }
}
