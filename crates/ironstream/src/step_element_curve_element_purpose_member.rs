// FILE: step_element_curve_element_purpose_member.rs
// occt: StepElement_CurveElementPurposeMember

pub struct CurveElementPurposeMember {
    pub purpose_type: Option<CurveElementPurposeType>,
    pub application_name: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveElementPurposeType {
    Beam,
    Cable,
    Spring,
    Rib,
}

impl CurveElementPurposeMember {
    pub fn new() -> Self {
        CurveElementPurposeMember {
            purpose_type: None,
            application_name: None,
        }
    }

    pub fn set_purpose_type(&mut self, ptype: CurveElementPurposeType) {
        self.purpose_type = Some(ptype);
    }

    pub fn get_purpose_type(&self) -> Option<CurveElementPurposeType> {
        self.purpose_type
    }

    pub fn set_application_name(&mut self, name: String) {
        self.application_name = Some(name);
    }

    pub fn get_application_name(&self) -> Option<&str> {
        self.application_name.as_deref()
    }

    pub fn case_mem(&self) -> i32 {
        match self.purpose_type {
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
        let member = CurveElementPurposeMember::new();
        assert!(member.purpose_type.is_none());
        assert!(member.application_name.is_none());
    }

    #[test]
    fn test_set_purpose_type() {
        let mut member = CurveElementPurposeMember::new();
        member.set_purpose_type(CurveElementPurposeType::Beam);
        assert_eq!(member.get_purpose_type(), Some(CurveElementPurposeType::Beam));
    }

    #[test]
    fn test_case_mem() {
        let mut member = CurveElementPurposeMember::new();
        assert_eq!(member.case_mem(), 0);
        member.set_purpose_type(CurveElementPurposeType::Spring);
        assert_eq!(member.case_mem(), 1);
    }

    #[test]
    fn test_set_application_name() {
        let mut member = CurveElementPurposeMember::new();
        member.set_application_name("custom_purpose".to_string());
        assert_eq!(member.get_application_name(), Some("custom_purpose"));
    }
}
