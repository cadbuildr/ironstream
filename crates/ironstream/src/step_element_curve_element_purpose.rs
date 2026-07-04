// FILE: step_element_curve_element_purpose.rs
// occt: StepElement_CurveElementPurpose

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CurveElementPurpose {
    Enumerated(EnumeratedCurveElementPurpose),
    ApplicationDefined(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumeratedCurveElementPurpose {
    Beam,
    Cable,
    Spring,
    Rib,
}

impl CurveElementPurpose {
    pub fn case_num(&self) -> i32 {
        match self {
            CurveElementPurpose::Enumerated(_) => 1,
            CurveElementPurpose::ApplicationDefined(_) => 2,
        }
    }

    pub fn is_enumerated(&self) -> bool {
        matches!(self, CurveElementPurpose::Enumerated(_))
    }

    pub fn is_application_defined(&self) -> bool {
        matches!(self, CurveElementPurpose::ApplicationDefined(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num() {
        let enum_purpose = CurveElementPurpose::Enumerated(EnumeratedCurveElementPurpose::Beam);
        let app_purpose = CurveElementPurpose::ApplicationDefined("custom".to_string());
        assert_eq!(enum_purpose.case_num(), 1);
        assert_eq!(app_purpose.case_num(), 2);
    }

    #[test]
    fn test_is_enumerated() {
        let enum_purpose = CurveElementPurpose::Enumerated(EnumeratedCurveElementPurpose::Cable);
        assert!(enum_purpose.is_enumerated());
        assert!(!enum_purpose.is_application_defined());
    }

    #[test]
    fn test_is_application_defined() {
        let app_purpose = CurveElementPurpose::ApplicationDefined("app_def".to_string());
        assert!(app_purpose.is_application_defined());
        assert!(!app_purpose.is_enumerated());
    }
}
