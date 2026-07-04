// FILE: step_shape_tolerance_method_definition.rs
// occt: StepShape_ToleranceMethodDefinition

use std::sync::Arc;

/// Placeholder for StepShape_ToleranceValue
pub struct ToleranceValue {
    value: f64,
}

impl ToleranceValue {
    pub fn new(value: f64) -> Self {
        ToleranceValue { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

/// Placeholder for StepShape_LimitsAndFits
pub struct LimitsAndFits {
    lower: f64,
    upper: f64,
}

impl LimitsAndFits {
    pub fn new(lower: f64, upper: f64) -> Self {
        LimitsAndFits { lower, upper }
    }

    pub fn lower(&self) -> f64 {
        self.lower
    }

    pub fn upper(&self) -> f64 {
        self.upper
    }
}

/// A discriminated union type for tolerance method definitions.
/// Can be either a ToleranceValue or LimitsAndFits.
pub enum ToleranceMethodDefinition {
    /// Case 1: ToleranceValue
    ToleranceValue(Arc<ToleranceValue>),
    /// Case 2: LimitsAndFits
    LimitsAndFits(Arc<LimitsAndFits>),
}

impl ToleranceMethodDefinition {
    /// Create from a ToleranceValue
    pub fn from_tolerance_value(value: Arc<ToleranceValue>) -> Self {
        ToleranceMethodDefinition::ToleranceValue(value)
    }

    /// Create from LimitsAndFits
    pub fn from_limits_and_fits(fits: Arc<LimitsAndFits>) -> Self {
        ToleranceMethodDefinition::LimitsAndFits(fits)
    }

    /// Get the case number (kind) of this definition
    /// 1 -> ToleranceValue
    /// 2 -> LimitsAndFits
    pub fn case_num(&self) -> usize {
        match self {
            ToleranceMethodDefinition::ToleranceValue(_) => 1,
            ToleranceMethodDefinition::LimitsAndFits(_) => 2,
        }
    }

    /// Try to get as ToleranceValue
    pub fn as_tolerance_value(&self) -> Option<&Arc<ToleranceValue>> {
        match self {
            ToleranceMethodDefinition::ToleranceValue(val) => Some(val),
            _ => None,
        }
    }

    /// Try to get as LimitsAndFits
    pub fn as_limits_and_fits(&self) -> Option<&Arc<LimitsAndFits>> {
        match self {
            ToleranceMethodDefinition::LimitsAndFits(fits) => Some(fits),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num_tolerance_value() {
        let val = Arc::new(ToleranceValue::new(0.5));
        let tmd = ToleranceMethodDefinition::from_tolerance_value(val);
        assert_eq!(tmd.case_num(), 1);
    }

    #[test]
    fn test_case_num_limits_and_fits() {
        let fits = Arc::new(LimitsAndFits::new(0.0, 1.0));
        let tmd = ToleranceMethodDefinition::from_limits_and_fits(fits);
        assert_eq!(tmd.case_num(), 2);
    }

    #[test]
    fn test_as_tolerance_value() {
        let val = Arc::new(ToleranceValue::new(0.25));
        let tmd = ToleranceMethodDefinition::from_tolerance_value(val.clone());
        assert!(tmd.as_tolerance_value().is_some());
        assert_eq!(tmd.as_tolerance_value().unwrap().value(), 0.25);
        assert!(tmd.as_limits_and_fits().is_none());
    }

    #[test]
    fn test_as_limits_and_fits() {
        let fits = Arc::new(LimitsAndFits::new(0.1, 0.9));
        let tmd = ToleranceMethodDefinition::from_limits_and_fits(fits.clone());
        assert!(tmd.as_limits_and_fits().is_some());
        assert_eq!(tmd.as_limits_and_fits().unwrap().lower(), 0.1);
        assert_eq!(tmd.as_limits_and_fits().unwrap().upper(), 0.9);
        assert!(tmd.as_tolerance_value().is_none());
    }

    #[test]
    fn test_multiple_definitions() {
        let val = ToleranceMethodDefinition::from_tolerance_value(Arc::new(ToleranceValue::new(1.0)));
        let fits = ToleranceMethodDefinition::from_limits_and_fits(Arc::new(LimitsAndFits::new(0.5, 1.5)));

        assert_eq!(val.case_num(), 1);
        assert_eq!(fits.case_num(), 2);
    }
}
