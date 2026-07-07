// FILE: step_basic_ratio_measure_with_unit.rs
// occt: StepBasic_RatioMeasureWithUnit

/// Local placeholder for the StepBasic_Unit select type (external plumbing).
#[derive(Debug, Clone, PartialEq)]
pub struct StepBasicUnit;

/// Local mirror of the StepBasic_MeasureWithUnit base class:
/// holds a value component and a unit component.
pub struct StepBasicMeasureWithUnit {
    value_component: f64,
    unit_component: Option<StepBasicUnit>,
}

impl StepBasicMeasureWithUnit {
    pub fn new() -> Self {
        StepBasicMeasureWithUnit {
            value_component: 0.0,
            unit_component: None,
        }
    }

    pub fn init(&mut self, value_component: f64, unit_component: StepBasicUnit) {
        self.value_component = value_component;
        self.unit_component = Some(unit_component);
    }

    pub fn set_value_component(&mut self, value: f64) {
        self.value_component = value;
    }

    pub fn value_component(&self) -> f64 {
        self.value_component
    }

    pub fn set_unit_component(&mut self, unit: StepBasicUnit) {
        self.unit_component = Some(unit);
    }

    pub fn unit_component(&self) -> Option<StepBasicUnit> {
        self.unit_component.clone()
    }
}

impl Default for StepBasicMeasureWithUnit {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a RatioMeasureWithUnit in the STEP AP standard.
///
/// In OCCT, StepBasic_RatioMeasureWithUnit inherits StepBasic_MeasureWithUnit
/// and adds no fields of its own.
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
    pub fn init(&mut self, value_component: f64, unit_component: StepBasicUnit) {
        self.base.init(value_component, unit_component);
    }

    pub fn set_value_component(&mut self, value: f64) {
        self.base.set_value_component(value);
    }

    pub fn value_component(&self) -> f64 {
        self.base.value_component()
    }

    pub fn set_unit_component(&mut self, unit: StepBasicUnit) {
        self.base.set_unit_component(unit);
    }

    pub fn unit_component(&self) -> Option<StepBasicUnit> {
        self.base.unit_component()
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
        assert_eq!(r.unit_component(), None);
    }

    #[test]
    fn test_set_and_get_value_component() {
        let mut r = StepBasicRatioMeasureWithUnit::new();
        r.set_value_component(2.5);
        assert!((r.value_component() - 2.5).abs() < 1e-6);
    }

    #[test]
    fn test_init_sets_value_and_unit() {
        let mut r = StepBasicRatioMeasureWithUnit::new();
        r.init(0.75, StepBasicUnit);
        assert!((r.value_component() - 0.75).abs() < 1e-12);
        assert_eq!(r.unit_component(), Some(StepBasicUnit));
    }

    #[test]
    fn test_default() {
        let r = StepBasicRatioMeasureWithUnit::default();
        assert_eq!(r.value_component(), 0.0);
    }
}
