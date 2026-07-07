// FILE: step_element_enumerated_volume_element_purpose.rs
// occt: StepElement_EnumeratedVolumeElementPurpose

/// Enumeration for volume element purpose in FEA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnumeratedVolumeElementPurpose {
    StressDisplacement,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant() {
        let purpose = EnumeratedVolumeElementPurpose::StressDisplacement;
        assert_eq!(purpose, EnumeratedVolumeElementPurpose::StressDisplacement);
    }

    #[test]
    fn test_copy() {
        let purpose = EnumeratedVolumeElementPurpose::StressDisplacement;
        let purpose2 = purpose;
        assert_eq!(purpose, purpose2);
    }

    #[test]
    fn test_debug() {
        let purpose = EnumeratedVolumeElementPurpose::StressDisplacement;
        assert_eq!(format!("{:?}", purpose), "StressDisplacement");
    }
}
