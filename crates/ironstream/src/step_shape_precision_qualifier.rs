// FILE: step_shape_precision_qualifier.rs
// occt: StepShape_PrecisionQualifier

/// Represents a precision qualifier for dimensional tolerances in STEP format.
pub struct PrecisionQualifier {
    precision_value: i32,
}

impl PrecisionQualifier {
    /// Create a new PrecisionQualifier
    pub fn new() -> Self {
        PrecisionQualifier {
            precision_value: 0,
        }
    }

    /// Initialize with precision value
    pub fn init(&mut self, precision_value: i32) {
        self.precision_value = precision_value;
    }

    /// Get the precision value
    pub fn precision_value(&self) -> i32 {
        self.precision_value
    }

    /// Set the precision value
    pub fn set_precision_value(&mut self, precision_value: i32) {
        self.precision_value = precision_value;
    }
}

impl Default for PrecisionQualifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision_qualifier_creation() {
        let pq = PrecisionQualifier::new();
        assert_eq!(pq.precision_value(), 0);
    }

    #[test]
    fn test_init_method() {
        let mut pq = PrecisionQualifier::new();
        pq.init(5);

        assert_eq!(pq.precision_value(), 5);
    }

    #[test]
    fn test_set_precision_value() {
        let mut pq = PrecisionQualifier::new();
        pq.set_precision_value(10);

        assert_eq!(pq.precision_value(), 10);
    }

    #[test]
    fn test_precision_value_negative() {
        let mut pq = PrecisionQualifier::new();
        pq.init(-3);

        assert_eq!(pq.precision_value(), -3);
    }
}
