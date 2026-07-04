// FILE: step_repr_value_range.rs
// occt: StepRepr_ValueRange

/// Represents a value range for dimensional tolerances in STEP.
/// This is derived from CompoundRepresentationItem.
pub struct ValueRange {
    name: Option<String>,
    lower_bound: Option<f64>,
    upper_bound: Option<f64>,
}

impl ValueRange {
    /// Create a new ValueRange
    pub fn new() -> Self {
        ValueRange {
            name: None,
            lower_bound: None,
            upper_bound: None,
        }
    }

    /// Initialize value range with name and bounds
    pub fn init(&mut self, name: String, lower_bound: f64, upper_bound: f64) {
        self.name = Some(name);
        self.lower_bound = Some(lower_bound);
        self.upper_bound = Some(upper_bound);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the lower bound
    pub fn lower_bound(&self) -> Option<f64> {
        self.lower_bound
    }

    /// Set the lower bound
    pub fn set_lower_bound(&mut self, bound: f64) {
        self.lower_bound = Some(bound);
    }

    /// Get the upper bound
    pub fn upper_bound(&self) -> Option<f64> {
        self.upper_bound
    }

    /// Set the upper bound
    pub fn set_upper_bound(&mut self, bound: f64) {
        self.upper_bound = Some(bound);
    }
}

impl Default for ValueRange {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let range = ValueRange::new();
        assert_eq!(range.name(), None);
        assert_eq!(range.lower_bound(), None);
        assert_eq!(range.upper_bound(), None);
    }

    #[test]
    fn test_init() {
        let mut range = ValueRange::new();
        range.init("Range1".to_string(), 1.0, 2.0);
        assert_eq!(range.name(), Some("Range1"));
        assert_eq!(range.lower_bound(), Some(1.0));
        assert_eq!(range.upper_bound(), Some(2.0));
    }

    #[test]
    fn test_set_and_get_bounds() {
        let mut range = ValueRange::new();
        range.set_lower_bound(0.5);
        range.set_upper_bound(1.5);
        assert_eq!(range.lower_bound(), Some(0.5));
        assert_eq!(range.upper_bound(), Some(1.5));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut range = ValueRange::new();
        range.set_name("TestRange".to_string());
        assert_eq!(range.name(), Some("TestRange"));
    }
}
