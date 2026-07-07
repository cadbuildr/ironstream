// FILE: step_fea_enumerated_degree_of_freedom.rs
// occt: StepFEA_EnumeratedDegreeOfFreedom

/// Enumeration representing STEP entity EnumeratedDegreeOfFreedom
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StepFeaEnumeratedDegreeOfFreedom {
    XTranslation,
    YTranslation,
    ZTranslation,
    XRotation,
    YRotation,
    ZRotation,
    Warp,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerated_degree_of_freedom_variants() {
        let x_trans = StepFeaEnumeratedDegreeOfFreedom::XTranslation;
        let y_trans = StepFeaEnumeratedDegreeOfFreedom::YTranslation;
        let z_trans = StepFeaEnumeratedDegreeOfFreedom::ZTranslation;
        let x_rot = StepFeaEnumeratedDegreeOfFreedom::XRotation;
        let y_rot = StepFeaEnumeratedDegreeOfFreedom::YRotation;
        let z_rot = StepFeaEnumeratedDegreeOfFreedom::ZRotation;
        let warp = StepFeaEnumeratedDegreeOfFreedom::Warp;

        assert_eq!(x_trans, StepFeaEnumeratedDegreeOfFreedom::XTranslation);
        assert_eq!(y_trans, StepFeaEnumeratedDegreeOfFreedom::YTranslation);
        assert_eq!(z_trans, StepFeaEnumeratedDegreeOfFreedom::ZTranslation);
        assert_eq!(x_rot, StepFeaEnumeratedDegreeOfFreedom::XRotation);
        assert_eq!(y_rot, StepFeaEnumeratedDegreeOfFreedom::YRotation);
        assert_eq!(z_rot, StepFeaEnumeratedDegreeOfFreedom::ZRotation);
        assert_eq!(warp, StepFeaEnumeratedDegreeOfFreedom::Warp);
    }

    #[test]
    fn test_enumerated_degree_of_freedom_clone() {
        let x_trans = StepFeaEnumeratedDegreeOfFreedom::XTranslation;
        let cloned = x_trans.clone();
        assert_eq!(x_trans, cloned);
    }

    #[test]
    fn test_enumerated_degree_of_freedom_distinct() {
        let x_trans = StepFeaEnumeratedDegreeOfFreedom::XTranslation;
        let y_trans = StepFeaEnumeratedDegreeOfFreedom::YTranslation;
        assert_ne!(x_trans, y_trans);
    }
}
