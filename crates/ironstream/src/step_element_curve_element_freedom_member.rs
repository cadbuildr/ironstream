// FILE: step_element_curve_element_freedom_member.rs
// occt: StepElement_CurveElementFreedomMember

pub struct CurveElementFreedomMember {
    pub freedom_type: Option<CurveElementFreedomType>,
    pub application_name: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveElementFreedomType {
    TranslationX,
    TranslationY,
    TranslationZ,
    RotationX,
    RotationY,
    RotationZ,
}

impl CurveElementFreedomMember {
    pub fn new() -> Self {
        CurveElementFreedomMember {
            freedom_type: None,
            application_name: None,
        }
    }

    pub fn set_freedom_type(&mut self, ftype: CurveElementFreedomType) {
        self.freedom_type = Some(ftype);
    }

    pub fn get_freedom_type(&self) -> Option<CurveElementFreedomType> {
        self.freedom_type
    }

    pub fn set_application_name(&mut self, name: String) {
        self.application_name = Some(name);
    }

    pub fn get_application_name(&self) -> Option<&str> {
        self.application_name.as_deref()
    }

    pub fn case_mem(&self) -> i32 {
        match self.freedom_type {
            Some(_) => 1,
            None => match self.application_name {
                Some(_) => 2,
                None => 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let member = CurveElementFreedomMember::new();
        assert!(member.freedom_type.is_none());
        assert!(member.application_name.is_none());
    }

    #[test]
    fn test_set_freedom_type() {
        let mut member = CurveElementFreedomMember::new();
        member.set_freedom_type(CurveElementFreedomType::TranslationX);
        assert_eq!(
            member.get_freedom_type(),
            Some(CurveElementFreedomType::TranslationX)
        );
    }

    #[test]
    fn test_case_mem() {
        let mut member = CurveElementFreedomMember::new();
        assert_eq!(member.case_mem(), 0);
        member.set_freedom_type(CurveElementFreedomType::RotationY);
        assert_eq!(member.case_mem(), 1);
    }

    #[test]
    fn test_set_application_name() {
        let mut member = CurveElementFreedomMember::new();
        member.set_application_name("custom_freedom".to_string());
        assert_eq!(member.get_application_name(), Some("custom_freedom"));
    }
}
