// FILE: step_element_surface_element_purpose.rs
// occt: StepElement_SurfaceElementPurpose

/// Representation of STEP SELECT type SurfaceElementPurpose
#[derive(Clone, Debug, PartialEq)]
pub enum SurfaceElementPurpose {
    EnumeratedSurfaceElementPurpose(String),
    ApplicationDefinedElementPurpose(String),
}

impl SurfaceElementPurpose {
    pub fn case_num(&self) -> i32 {
        0
    }

    pub fn case_mem(&self) -> i32 {
        match self {
            SurfaceElementPurpose::EnumeratedSurfaceElementPurpose(_) => 1,
            SurfaceElementPurpose::ApplicationDefinedElementPurpose(_) => 2,
        }
    }

    pub fn enumerated_surface_element_purpose(&self) -> Option<&str> {
        match self {
            SurfaceElementPurpose::EnumeratedSurfaceElementPurpose(v) => Some(v),
            _ => None,
        }
    }

    pub fn set_enumerated_surface_element_purpose(&mut self, val: String) {
        *self = SurfaceElementPurpose::EnumeratedSurfaceElementPurpose(val);
    }

    pub fn application_defined_element_purpose(&self) -> Option<&str> {
        match self {
            SurfaceElementPurpose::ApplicationDefinedElementPurpose(v) => Some(v),
            _ => None,
        }
    }

    pub fn set_application_defined_element_purpose(&mut self, val: String) {
        *self = SurfaceElementPurpose::ApplicationDefinedElementPurpose(val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_mem() {
        let purpose = SurfaceElementPurpose::EnumeratedSurfaceElementPurpose("Membrane".to_string());
        assert_eq!(purpose.case_mem(), 1);

        let purpose2 = SurfaceElementPurpose::ApplicationDefinedElementPurpose("Custom".to_string());
        assert_eq!(purpose2.case_mem(), 2);
    }

    #[test]
    fn test_enumerated() {
        let purpose = SurfaceElementPurpose::EnumeratedSurfaceElementPurpose("Bending".to_string());
        assert_eq!(
            purpose.enumerated_surface_element_purpose(),
            Some("Bending")
        );
        assert!(purpose.application_defined_element_purpose().is_none());
    }

    #[test]
    fn test_application_defined() {
        let purpose = SurfaceElementPurpose::ApplicationDefinedElementPurpose("MyPurpose".to_string());
        assert_eq!(
            purpose.application_defined_element_purpose(),
            Some("MyPurpose")
        );
        assert!(purpose.enumerated_surface_element_purpose().is_none());
    }

    #[test]
    fn test_setters() {
        let mut purpose = SurfaceElementPurpose::EnumeratedSurfaceElementPurpose("".to_string());
        purpose.set_application_defined_element_purpose("NewPurpose".to_string());
        assert_eq!(purpose.case_mem(), 2);
        assert_eq!(purpose.application_defined_element_purpose(), Some("NewPurpose"));
    }
}
