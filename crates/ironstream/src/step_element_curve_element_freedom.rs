// FILE: step_element_curve_element_freedom.rs
// occt: StepElement_CurveElementFreedom

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CurveElementFreedom {
    Enumerated(EnumeratedCurveElementFreedom),
    ApplicationDefined(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumeratedCurveElementFreedom {
    TranslationX,
    TranslationY,
    TranslationZ,
    RotationX,
    RotationY,
    RotationZ,
}

impl CurveElementFreedom {
    pub fn case_num(&self) -> i32 {
        match self {
            CurveElementFreedom::Enumerated(_) => 1,
            CurveElementFreedom::ApplicationDefined(_) => 2,
        }
    }

    pub fn is_enumerated(&self) -> bool {
        matches!(self, CurveElementFreedom::Enumerated(_))
    }

    pub fn is_application_defined(&self) -> bool {
        matches!(self, CurveElementFreedom::ApplicationDefined(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num() {
        let enum_freedom =
            CurveElementFreedom::Enumerated(EnumeratedCurveElementFreedom::TranslationX);
        let app_freedom = CurveElementFreedom::ApplicationDefined("custom".to_string());
        assert_eq!(enum_freedom.case_num(), 1);
        assert_eq!(app_freedom.case_num(), 2);
    }

    #[test]
    fn test_is_enumerated() {
        let enum_freedom =
            CurveElementFreedom::Enumerated(EnumeratedCurveElementFreedom::RotationZ);
        assert!(enum_freedom.is_enumerated());
        assert!(!enum_freedom.is_application_defined());
    }

    #[test]
    fn test_is_application_defined() {
        let app_freedom = CurveElementFreedom::ApplicationDefined("app_def".to_string());
        assert!(app_freedom.is_application_defined());
        assert!(!app_freedom.is_enumerated());
    }
}
