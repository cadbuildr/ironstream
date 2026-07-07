// FILE: step_element_volume_element_purpose.rs
// occt: StepElement_VolumeElementPurpose

/// Representation of STEP SELECT type VolumeElementPurpose
#[derive(Clone, Debug, PartialEq)]
pub enum VolumeElementPurpose {
    EnumeratedVolumeElementPurpose(String),
    ApplicationDefinedElementPurpose(String),
}

impl VolumeElementPurpose {
    pub fn case_num(&self) -> i32 {
        0
    }

    pub fn case_mem(&self) -> i32 {
        match self {
            VolumeElementPurpose::EnumeratedVolumeElementPurpose(_) => 1,
            VolumeElementPurpose::ApplicationDefinedElementPurpose(_) => 2,
        }
    }

    pub fn enumerated_volume_element_purpose(&self) -> Option<&str> {
        match self {
            VolumeElementPurpose::EnumeratedVolumeElementPurpose(v) => Some(v),
            _ => None,
        }
    }

    pub fn set_enumerated_volume_element_purpose(&mut self, val: String) {
        *self = VolumeElementPurpose::EnumeratedVolumeElementPurpose(val);
    }

    pub fn application_defined_element_purpose(&self) -> Option<&str> {
        match self {
            VolumeElementPurpose::ApplicationDefinedElementPurpose(v) => Some(v),
            _ => None,
        }
    }

    pub fn set_application_defined_element_purpose(&mut self, val: String) {
        *self = VolumeElementPurpose::ApplicationDefinedElementPurpose(val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_mem() {
        let purpose = VolumeElementPurpose::EnumeratedVolumeElementPurpose("Stress".to_string());
        assert_eq!(purpose.case_mem(), 1);

        let purpose2 = VolumeElementPurpose::ApplicationDefinedElementPurpose("Custom".to_string());
        assert_eq!(purpose2.case_mem(), 2);
    }

    #[test]
    fn test_enumerated() {
        let purpose = VolumeElementPurpose::EnumeratedVolumeElementPurpose("StressDisplacement".to_string());
        assert_eq!(
            purpose.enumerated_volume_element_purpose(),
            Some("StressDisplacement")
        );
        assert!(purpose.application_defined_element_purpose().is_none());
    }

    #[test]
    fn test_application_defined() {
        let purpose = VolumeElementPurpose::ApplicationDefinedElementPurpose("MyVolumePurpose".to_string());
        assert_eq!(
            purpose.application_defined_element_purpose(),
            Some("MyVolumePurpose")
        );
        assert!(purpose.enumerated_volume_element_purpose().is_none());
    }

    #[test]
    fn test_setters() {
        let mut purpose = VolumeElementPurpose::EnumeratedVolumeElementPurpose("".to_string());
        purpose.set_application_defined_element_purpose("NewPurpose".to_string());
        assert_eq!(purpose.case_mem(), 2);
        assert_eq!(purpose.application_defined_element_purpose(), Some("NewPurpose"));
    }
}
