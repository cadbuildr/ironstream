// FILE: step_element_enumerated_curve_element_purpose.rs
// occt: StepElement_EnumeratedCurveElementPurpose

/// Enumeration for curve element purpose in FEA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnumeratedCurveElementPurpose {
    Axial,
    YYBending,
    ZZBending,
    Torsion,
    XYShear,
    XZShear,
    Warping,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_variants() {
        assert_ne!(EnumeratedCurveElementPurpose::Axial, EnumeratedCurveElementPurpose::YYBending);
        assert_ne!(EnumeratedCurveElementPurpose::Torsion, EnumeratedCurveElementPurpose::Warping);
        assert_eq!(EnumeratedCurveElementPurpose::Axial, EnumeratedCurveElementPurpose::Axial);
    }

    #[test]
    fn test_copy() {
        let purpose = EnumeratedCurveElementPurpose::Warping;
        let purpose2 = purpose;
        assert_eq!(purpose, purpose2);
    }

    #[test]
    fn test_debug() {
        let purpose = EnumeratedCurveElementPurpose::Torsion;
        assert_eq!(format!("{:?}", purpose), "Torsion");
    }
}
