// FILE: step_basic_measure_with_unit.rs
// occt: StepBasic_MeasureWithUnit

/// Representation of STEP entity MeasureWithUnit
#[derive(Clone, Debug)]
pub struct MeasureWithUnit {
    value_component: Option<f64>,
    unit_component: Option<String>,
}

impl MeasureWithUnit {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            value_component: None,
            unit_component: None,
        }
    }

    /// Initialize all fields
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

    /// Get value component member
    pub fn value_component_member(&self) -> Option<f64> {
        self.value_component
    }

    /// Set value component member
    pub fn set_value_component_member(&mut self, val: f64) {
        self.value_component = Some(val);
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

impl Default for MeasureWithUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let measure = MeasureWithUnit::new();
        assert!(measure.value_component().is_none());
        assert!(measure.unit_component().is_none());
    }

    #[test]
    fn test_init() {
        let mut measure = MeasureWithUnit::new();
        measure.init(50.0, "mm".to_string());
        assert_eq!(measure.value_component(), Some(50.0));
        assert_eq!(measure.unit_component(), Some("mm"));
    }

    #[test]
    fn test_set_value_component() {
        let mut measure = MeasureWithUnit::new();
        measure.set_value_component(123.456);
        assert_eq!(measure.value_component(), Some(123.456));
    }

    #[test]
    fn test_set_unit_component() {
        let mut measure = MeasureWithUnit::new();
        measure.set_unit_component("kg".to_string());
        assert_eq!(measure.unit_component(), Some("kg"));
    }

    #[test]
    fn test_value_component_member() {
        let mut measure = MeasureWithUnit::new();
        measure.set_value_component_member(75.5);
        assert_eq!(measure.value_component_member(), Some(75.5));
    }

    #[test]
    fn test_precision() {
        let mut measure = MeasureWithUnit::new();
        measure.init(1.23456789, "unit".to_string());
        assert_eq!(measure.value_component(), Some(1.23456789));
    }

    #[test]
    fn test_default() {
        let measure = MeasureWithUnit::default();
        assert!(measure.value_component().is_none());
        assert!(measure.unit_component().is_none());
    }
}
