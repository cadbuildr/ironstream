// FILE: step_basic_uncertainty_measure_with_unit.rs
// occt: StepBasic_UncertaintyMeasureWithUnit

//! A measure with unit that includes uncertainty information (name and description).

/// An UncertaintyMeasureWithUnit extends MeasureWithUnit by adding
/// a name and description to characterize the uncertainty.
#[derive(Debug, Clone)]
pub struct StepBasicUncertaintyMeasureWithUnit {
    /// The numeric value of the measure
    value: Option<f64>,
    /// The unit (as a string identifier)
    unit: Option<String>,
    /// Name of the uncertainty
    name: Option<String>,
    /// Description of the uncertainty
    description: Option<String>,
}

impl StepBasicUncertaintyMeasureWithUnit {
    /// Create a new UncertaintyMeasureWithUnit instance
    pub fn new() -> Self {
        Self {
            value: None,
            unit: None,
            name: None,
            description: None,
        }
    }

    /// Initialize with all components
    pub fn init(
        &mut self,
        value: Option<f64>,
        unit: Option<String>,
        name: Option<String>,
        description: Option<String>,
    ) {
        self.value = value;
        self.unit = unit;
        self.name = name;
        self.description = description;
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

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl Default for StepBasicUncertaintyMeasureWithUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let u = StepBasicUncertaintyMeasureWithUnit::new();
        assert_eq!(u.value_component(), 0.0);
        assert_eq!(u.unit_component(), None);
        assert_eq!(u.name(), None);
        assert_eq!(u.description(), None);
    }

    #[test]
    fn test_init() {
        let mut u = StepBasicUncertaintyMeasureWithUnit::new();
        u.init(
            Some(1.5),
            Some("meter".to_string()),
            Some("tolerance".to_string()),
            Some("±0.1mm".to_string()),
        );
        assert_eq!(u.value_component(), 1.5);
        assert_eq!(u.unit_component(), Some("meter"));
        assert_eq!(u.name(), Some("tolerance"));
        assert_eq!(u.description(), Some("±0.1mm"));
    }

    #[test]
    fn test_set_value_component() {
        let mut u = StepBasicUncertaintyMeasureWithUnit::new();
        u.set_value_component(2.5);
        assert_eq!(u.value_component(), 2.5);
    }

    #[test]
    fn test_set_unit_component() {
        let mut u = StepBasicUncertaintyMeasureWithUnit::new();
        u.set_unit_component("millimeter".to_string());
        assert_eq!(u.unit_component(), Some("millimeter"));
    }

    #[test]
    fn test_set_name() {
        let mut u = StepBasicUncertaintyMeasureWithUnit::new();
        u.set_name("positional_tolerance".to_string());
        assert_eq!(u.name(), Some("positional_tolerance"));
    }

    #[test]
    fn test_set_description() {
        let mut u = StepBasicUncertaintyMeasureWithUnit::new();
        u.set_description("Maximum variation in position".to_string());
        assert_eq!(u.description(), Some("Maximum variation in position"));
    }

    #[test]
    fn test_default() {
        let u = StepBasicUncertaintyMeasureWithUnit::default();
        assert_eq!(u.value_component(), 0.0);
    }
}
