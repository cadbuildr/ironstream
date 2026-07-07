// FILE: step_fea_element_volume.rs
// occt: StepFEA_ElementVolume

/// Enumeration representing STEP entity ElementVolume
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepFeaElementVolume {
    Volume,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_volume_variant() {
        let volume = StepFeaElementVolume::Volume;
        assert_eq!(volume, StepFeaElementVolume::Volume);
    }

    #[test]
    fn test_element_volume_clone() {
        let volume = StepFeaElementVolume::Volume;
        let cloned = volume.clone();
        assert_eq!(volume, cloned);
    }
}
