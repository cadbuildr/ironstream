// FILE: step_basic_time_measure_with_unit.rs
// occt: StepBasic_TimeMeasureWithUnit

//! A measure of time with an associated unit.
//! This is a simple typed wrapper around MeasureWithUnit for time quantities.

/// A TimeMeasureWithUnit represents a time measurement paired with a unit.
/// It extends the concept of MeasureWithUnit to specifically work with time values.
#[derive(Debug, Clone)]
pub struct StepBasicTimeMeasureWithUnit {
    /// The numeric value of the measure
    value: Option<f64>,
    /// The unit (as a string identifier for the unit type)
    unit: Option<String>,
}

impl StepBasicTimeMeasureWithUnit {
    /// Create a new TimeMeasureWithUnit instance
    pub fn new() -> Self {
        Self {
            value: None,
            unit: None,
        }
    }

    /// Set the numeric value component
    pub fn set_value_component(&mut self, value: f64) {
        self.value = Some(value);
    }

    /// Get the numeric value component
    pub fn value_component(&self) -> f64 {
        self.value.unwrap_or(0.0)
    }

    /// Set the unit component
    pub fn set_unit_component(&mut self, unit: String) {
        self.unit = Some(unit);
    }

    /// Get the unit component
    pub fn unit_component(&self) -> Option<&str> {
        self.unit.as_deref()
    }
}

impl Default for StepBasicTimeMeasureWithUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tmwu = StepBasicTimeMeasureWithUnit::new();
        assert_eq!(tmwu.value_component(), 0.0);
        assert_eq!(tmwu.unit_component(), None);
    }

    #[test]
    fn test_set_value_component() {
        let mut tmwu = StepBasicTimeMeasureWithUnit::new();
        tmwu.set_value_component(42.5);
        assert_eq!(tmwu.value_component(), 42.5);
    }

    #[test]
    fn test_set_unit_component() {
        let mut tmwu = StepBasicTimeMeasureWithUnit::new();
        tmwu.set_unit_component("second".to_string());
        assert_eq!(tmwu.unit_component(), Some("second"));
    }

    #[test]
    fn test_default() {
        let tmwu = StepBasicTimeMeasureWithUnit::default();
        assert_eq!(tmwu.value_component(), 0.0);
        assert_eq!(tmwu.unit_component(), None);
    }
}
