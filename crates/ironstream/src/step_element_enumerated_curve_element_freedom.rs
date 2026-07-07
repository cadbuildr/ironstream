// FILE: step_element_enumerated_curve_element_freedom.rs
// occt: StepElement_EnumeratedCurveElementFreedom

/// Enumeration for curve element freedom constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnumeratedCurveElementFreedom {
    XTranslation,
    YTranslation,
    ZTranslation,
    XRotation,
    YRotation,
    ZRotation,
    Warp,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_variants() {
        assert_ne!(EnumeratedCurveElementFreedom::XTranslation, EnumeratedCurveElementFreedom::YTranslation);
        assert_ne!(EnumeratedCurveElementFreedom::XRotation, EnumeratedCurveElementFreedom::Warp);
        assert_eq!(EnumeratedCurveElementFreedom::None, EnumeratedCurveElementFreedom::None);
    }

    #[test]
    fn test_copy() {
        let freedom = EnumeratedCurveElementFreedom::ZRotation;
        let freedom2 = freedom;
        assert_eq!(freedom, freedom2);
    }

    #[test]
    fn test_debug() {
        let freedom = EnumeratedCurveElementFreedom::Warp;
        assert_eq!(format!("{:?}", freedom), "Warp");
    }
}
