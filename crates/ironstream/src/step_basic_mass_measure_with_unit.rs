// FILE: step_basic_mass_measure_with_unit.rs
// occt: StepBasic_MassMeasureWithUnit

/// Representation of STEP entity MassMeasureWithUnit
/// Extends MeasureWithUnit with typed semantics for mass measurements
#[derive(Clone, Debug)]
pub struct MassMeasureWithUnit {
    value_component: Option<f64>,
    unit_component: Option<String>,
}

impl MassMeasureWithUnit {
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

impl Default for MassMeasureWithUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let measure = MassMeasureWithUnit::new();
        assert!(measure.value_component().is_none());
        assert!(measure.unit_component().is_none());
    }

    #[test]
    fn test_init() {
        let mut measure = MassMeasureWithUnit::new();
        measure.init(250.5, "kg".to_string());
        assert_eq!(measure.value_component(), Some(250.5));
        assert_eq!(measure.unit_component(), Some("kg"));
    }

    #[test]
    fn test_set_value_component() {
        let mut measure = MassMeasureWithUnit::new();
        measure.set_value_component(100.0);
        assert_eq!(measure.value_component(), Some(100.0));
    }

    #[test]
    fn test_set_unit_component() {
        let mut measure = MassMeasureWithUnit::new();
        measure.set_unit_component("g".to_string());
        assert_eq!(measure.unit_component(), Some("g"));
    }

    #[test]
    fn test_common_units() {
        let mut measure = MassMeasureWithUnit::new();
        measure.init(500.0, "gram".to_string());
        assert_eq!(measure.value_component(), Some(500.0));
        assert_eq!(measure.unit_component(), Some("gram"));
    }
}
