// FILE: step_basic_measure_value_member.rs
// occt: StepBasic_MeasureValueMember

/// Representation of STEP entity MeasureValueMember
/// A SELECT type for various measure values
#[derive(Clone, Debug)]
pub struct MeasureValueMember {
    value: Option<f64>,
    name: Option<String>,
}

impl MeasureValueMember {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            value: None,
            name: None,
        }
    }

    /// Set the numeric value
    pub fn set_value(&mut self, value: f64) {
        self.value = Some(value);
    }

    /// Get the numeric value
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Check if value is defined
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Set the name (type identifier)
    pub fn set_name(&mut self, name: String) -> bool {
        self.name = Some(name);
        true
    }

    /// Get the name (type identifier)
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Check if name is defined
    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }
}

impl Default for MeasureValueMember {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let member = MeasureValueMember::new();
        assert!(member.value().is_none());
        assert!(member.name().is_none());
    }

    #[test]
    fn test_set_value() {
        let mut member = MeasureValueMember::new();
        member.set_value(42.5);
        assert!(member.has_value());
        assert_eq!(member.value(), Some(42.5));
    }

    #[test]
    fn test_set_name() {
        let mut member = MeasureValueMember::new();
        assert!(member.set_name("length_measure".to_string()));
        assert!(member.has_name());
        assert_eq!(member.name(), Some("length_measure"));
    }

    #[test]
    fn test_both_value_and_name() {
        let mut member = MeasureValueMember::new();
        member.set_value(100.0);
        member.set_name("plane_angle_measure".to_string());
        assert_eq!(member.value(), Some(100.0));
        assert_eq!(member.name(), Some("plane_angle_measure"));
    }

    #[test]
    fn test_different_measure_types() {
        let measure_types = vec![
            "length_measure",
            "time_measure",
            "plane_angle_measure",
            "area_measure",
            "volume_measure",
        ];
        for measure_type in measure_types {
            let mut member = MeasureValueMember::new();
            member.set_name(measure_type.to_string());
            assert_eq!(member.name(), Some(measure_type));
        }
    }

    #[test]
    fn test_default() {
        let member = MeasureValueMember::default();
        assert!(!member.has_value());
        assert!(!member.has_name());
    }
}
