// FILE: step_element_measure_or_unspecified_value.rs
// occt: StepElement_MeasureOrUnspecifiedValue

/// Representation of STEP SELECT type MeasureOrUnspecifiedValue
#[derive(Clone, Debug, PartialEq)]
pub enum MeasureOrUnspecifiedValue {
    ContextDependentMeasure(f64),
    UnspecifiedValue,
}

impl MeasureOrUnspecifiedValue {
    pub fn case_num(&self) -> i32 {
        0
    }

    pub fn case_mem(&self) -> i32 {
        match self {
            MeasureOrUnspecifiedValue::ContextDependentMeasure(_) => 1,
            MeasureOrUnspecifiedValue::UnspecifiedValue => 2,
        }
    }

    pub fn context_dependent_measure(&self) -> Option<f64> {
        match self {
            MeasureOrUnspecifiedValue::ContextDependentMeasure(v) => Some(*v),
            _ => None,
        }
    }

    pub fn set_context_dependent_measure(&mut self, val: f64) {
        *self = MeasureOrUnspecifiedValue::ContextDependentMeasure(val);
    }

    pub fn unspecified_value(&self) -> Option<()> {
        match self {
            MeasureOrUnspecifiedValue::UnspecifiedValue => Some(()),
            _ => None,
        }
    }

    pub fn set_unspecified_value(&mut self) {
        *self = MeasureOrUnspecifiedValue::UnspecifiedValue;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_mem() {
        let measure = MeasureOrUnspecifiedValue::ContextDependentMeasure(1.5);
        assert_eq!(measure.case_mem(), 1);

        let unspec = MeasureOrUnspecifiedValue::UnspecifiedValue;
        assert_eq!(unspec.case_mem(), 2);
    }

    #[test]
    fn test_context_dependent_measure() {
        let measure = MeasureOrUnspecifiedValue::ContextDependentMeasure(3.14);
        assert!(measure.context_dependent_measure().is_some());
        assert!((measure.context_dependent_measure().unwrap() - 3.14).abs() < 1e-6);
    }

    #[test]
    fn test_set_context_dependent_measure() {
        let mut measure = MeasureOrUnspecifiedValue::UnspecifiedValue;
        measure.set_context_dependent_measure(2.71);
        assert_eq!(measure.case_mem(), 1);
        assert_eq!(measure.context_dependent_measure(), Some(2.71));
    }

    #[test]
    fn test_unspecified_value() {
        let unspec = MeasureOrUnspecifiedValue::UnspecifiedValue;
        assert!(unspec.unspecified_value().is_some());
    }

    #[test]
    fn test_set_unspecified_value() {
        let mut measure = MeasureOrUnspecifiedValue::ContextDependentMeasure(1.0);
        measure.set_unspecified_value();
        assert_eq!(measure.case_mem(), 2);
        assert!(measure.unspecified_value().is_some());
    }
}
