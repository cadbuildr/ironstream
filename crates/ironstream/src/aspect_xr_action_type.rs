// FILE: aspect_xr_action_type.rs
// occt: Aspect_XRActionType

/// XR action type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AspectXRActionType {
    /// Boolean input (like button).
    InputDigital = 0,
    /// Analog input (1/2/3 axes).
    InputAnalog = 1,
    /// Positional input.
    InputPose = 2,
    /// Skeletal input.
    InputSkeletal = 3,
    /// Haptic output (vibration).
    OutputHaptic = 4,
}

impl AspectXRActionType {
    /// Convert from numeric value to enum variant.
    pub fn from_value(value: u32) -> Option<Self> {
        match value {
            0 => Some(AspectXRActionType::InputDigital),
            1 => Some(AspectXRActionType::InputAnalog),
            2 => Some(AspectXRActionType::InputPose),
            3 => Some(AspectXRActionType::InputSkeletal),
            4 => Some(AspectXRActionType::OutputHaptic),
            _ => None,
        }
    }

    /// Get the numeric value of this action type.
    pub fn as_value(&self) -> u32 {
        *self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xr_action_type_values() {
        assert_eq!(AspectXRActionType::InputDigital.as_value(), 0);
        assert_eq!(AspectXRActionType::InputAnalog.as_value(), 1);
        assert_eq!(AspectXRActionType::InputPose.as_value(), 2);
        assert_eq!(AspectXRActionType::InputSkeletal.as_value(), 3);
        assert_eq!(AspectXRActionType::OutputHaptic.as_value(), 4);
    }

    #[test]
    fn test_xr_action_type_from_value() {
        assert_eq!(AspectXRActionType::from_value(0), Some(AspectXRActionType::InputDigital));
        assert_eq!(AspectXRActionType::from_value(1), Some(AspectXRActionType::InputAnalog));
        assert_eq!(AspectXRActionType::from_value(2), Some(AspectXRActionType::InputPose));
        assert_eq!(AspectXRActionType::from_value(3), Some(AspectXRActionType::InputSkeletal));
        assert_eq!(AspectXRActionType::from_value(4), Some(AspectXRActionType::OutputHaptic));
        assert_eq!(AspectXRActionType::from_value(999), None);
    }

    #[test]
    fn test_xr_action_type_round_trip() {
        for original in [
            AspectXRActionType::InputDigital,
            AspectXRActionType::InputAnalog,
            AspectXRActionType::InputPose,
            AspectXRActionType::InputSkeletal,
            AspectXRActionType::OutputHaptic,
        ] {
            let value = original.as_value();
            let restored = AspectXRActionType::from_value(value);
            assert_eq!(restored, Some(original));
        }
    }

    #[test]
    fn test_xr_action_type_copy() {
        let t1 = AspectXRActionType::InputAnalog;
        let t2 = t1;
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_xr_action_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(AspectXRActionType::InputDigital);
        set.insert(AspectXRActionType::InputAnalog);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&AspectXRActionType::InputDigital));
    }
}
