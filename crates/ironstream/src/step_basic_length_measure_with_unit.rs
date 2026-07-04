// FILE: step_basic_length_measure_with_unit.rs
// occt: StepBasic_LengthMeasureWithUnit

/// Representation of STEP entity LengthMeasureWithUnit
/// Extends MeasureWithUnit with typed semantics for length measurements
#[derive(Clone, Debug)]
pub struct LengthMeasureWithUnit {
    // Inherited from MeasureWithUnit
    value_component: Option<f64>,
    unit_component: Option<String>,
}

impl LengthMeasureWithUnit {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            value_component: None,
            unit_component: None,
        }
    }

    /// Initialize fields
    pub fn init(&mut self, value_component: f64, unit_component: String) {
        self.value_component = Some(value_component);
        self.unit_component = Some(unit_component);
    }

    /// Set value component (numeric value)
    pub fn set_value_component(&mut self, value: f64) {
        self.value_component = Some(value);
    }

    /// Get value component
    pub fn value_component(&self) -> Option<f64> {
        self.value_component
    }

    /// Set unit component
    pub fn set_unit_component(&mut self, unit: String) {
        self.unit_component = Some(unit);
    }

    /// Get unit component
    pub fn unit_component(&self) -> Option<&str> {
        self.unit_component.as_deref()
    }
}

impl Default for LengthMeasureWithUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let measure = LengthMeasureWithUnit::new();
        assert!(measure.value_component().is_none());
        assert!(measure.unit_component().is_none());
    }

    #[test]
    fn test_init() {
        let mut measure = LengthMeasureWithUnit::new();
        measure.init(100.5, "mm".to_string());
        assert_eq!(measure.value_component(), Some(100.5));
        assert_eq!(measure.unit_component(), Some("mm"));
    }

    #[test]
    fn test_set_value_component() {
        let mut measure = LengthMeasureWithUnit::new();
        measure.set_value_component(50.0);
        assert_eq!(measure.value_component(), Some(50.0));
    }

    #[test]
    fn test_set_unit_component() {
        let mut measure = LengthMeasureWithUnit::new();
        measure.set_unit_component("cm".to_string());
        assert_eq!(measure.unit_component(), Some("cm"));
    }

    #[test]
    fn test_precision() {
        let mut measure = LengthMeasureWithUnit::new();
        measure.set_value_component(123.456789);
        assert_eq!(measure.value_component(), Some(123.456789));
    }
}
