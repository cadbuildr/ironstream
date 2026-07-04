// FILE: step_shape_plus_minus_tolerance.rs
// occt: StepShape_PlusMinusTolerance

/// Placeholder for StepShape_ToleranceMethodDefinition
#[derive(Clone, Debug)]
pub struct ToleranceMethodDefinition {
    value: f64,
}

impl ToleranceMethodDefinition {
    pub fn new(value: f64) -> Self {
        ToleranceMethodDefinition { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

/// Placeholder for StepShape_DimensionalCharacteristic
#[derive(Clone, Debug)]
pub struct DimensionalCharacteristic {
    value: f64,
}

impl DimensionalCharacteristic {
    pub fn new(value: f64) -> Self {
        DimensionalCharacteristic { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

/// Represents a plus/minus tolerance definition in STEP format.
/// Used for dimensional tolerances.
pub struct PlusMinusTolerance {
    range: ToleranceMethodDefinition,
    toleranced_dimension: DimensionalCharacteristic,
}

impl PlusMinusTolerance {
    /// Create a new PlusMinusTolerance
    pub fn new() -> Self {
        PlusMinusTolerance {
            range: ToleranceMethodDefinition::new(0.0),
            toleranced_dimension: DimensionalCharacteristic::new(0.0),
        }
    }

    /// Initialize with range and toleranced dimension
    pub fn init(
        &mut self,
        range: ToleranceMethodDefinition,
        toleranced_dimension: DimensionalCharacteristic,
    ) {
        self.range = range;
        self.toleranced_dimension = toleranced_dimension;
    }

    /// Get the tolerance range
    pub fn range(&self) -> &ToleranceMethodDefinition {
        &self.range
    }

    /// Set the tolerance range
    pub fn set_range(&mut self, range: ToleranceMethodDefinition) {
        self.range = range;
    }

    /// Get the toleranced dimension
    pub fn toleranced_dimension(&self) -> &DimensionalCharacteristic {
        &self.toleranced_dimension
    }

    /// Set the toleranced dimension
    pub fn set_toleranced_dimension(&mut self, toleranced_dimension: DimensionalCharacteristic) {
        self.toleranced_dimension = toleranced_dimension;
    }
}

impl Default for PlusMinusTolerance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_minus_tolerance_creation() {
        let pmt = PlusMinusTolerance::new();
        assert_eq!(pmt.range().value(), 0.0);
        assert_eq!(pmt.toleranced_dimension().value(), 0.0);
    }

    #[test]
    fn test_init_method() {
        let mut pmt = PlusMinusTolerance::new();
        let range = ToleranceMethodDefinition::new(0.5);
        let dimension = DimensionalCharacteristic::new(10.0);

        pmt.init(range.clone(), dimension.clone());

        assert_eq!(pmt.range().value(), 0.5);
        assert_eq!(pmt.toleranced_dimension().value(), 10.0);
    }

    #[test]
    fn test_set_range() {
        let mut pmt = PlusMinusTolerance::new();
        let range = ToleranceMethodDefinition::new(0.25);

        pmt.set_range(range);

        assert_eq!(pmt.range().value(), 0.25);
    }

    #[test]
    fn test_set_toleranced_dimension() {
        let mut pmt = PlusMinusTolerance::new();
        let dimension = DimensionalCharacteristic::new(5.5);

        pmt.set_toleranced_dimension(dimension);

        assert_eq!(pmt.toleranced_dimension().value(), 5.5);
    }
}
