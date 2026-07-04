// FILE: step_element_element_volume.rs
// occt: StepElement_ElementVolume

/// Enumeration for element volume aspect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementVolume {
    Volume,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_volume() {
        let vol = ElementVolume::Volume;
        assert_eq!(vol, ElementVolume::Volume);
    }

    #[test]
    fn test_debug() {
        let vol = ElementVolume::Volume;
        assert_eq!(format!("{:?}", vol), "Volume");
    }

    #[test]
    fn test_copy() {
        let vol = ElementVolume::Volume;
        let vol2 = vol;
        assert_eq!(vol, vol2);
    }
}
