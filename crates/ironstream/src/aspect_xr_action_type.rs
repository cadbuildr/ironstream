// FILE: aspect_xr_action_type.rs
// occt: Aspect_XRActionType

/// XR action type enumeration.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AspectXRActionType {
    /// Boolean input (like button)
    InputDigital = 0,
    /// Analog input (1/2/3 axes)
    InputAnalog = 1,
    /// Positional input
    InputPose = 2,
    /// Skeletal input
    InputSkeletal = 3,
    /// Haptic output (vibration)
    OutputHaptic = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_type_values() {
        assert_eq!(AspectXRActionType::InputDigital as u32, 0);
        assert_eq!(AspectXRActionType::InputAnalog as u32, 1);
        assert_eq!(AspectXRActionType::InputPose as u32, 2);
        assert_eq!(AspectXRActionType::InputSkeletal as u32, 3);
        assert_eq!(AspectXRActionType::OutputHaptic as u32, 4);
    }

    #[test]
    fn test_action_type_equality() {
        let type1 = AspectXRActionType::InputDigital;
        let type2 = AspectXRActionType::InputDigital;
        assert_eq!(type1, type2);

        let type3 = AspectXRActionType::InputAnalog;
        assert_ne!(type1, type3);
    }

    #[test]
    fn test_action_type_clone() {
        let action_type = AspectXRActionType::InputAnalog;
        let cloned = action_type;
        assert_eq!(action_type, cloned);
    }

    #[test]
    fn test_action_type_ordering() {
        assert!(AspectXRActionType::InputDigital < AspectXRActionType::InputAnalog);
        assert!(AspectXRActionType::InputAnalog < AspectXRActionType::InputPose);
        assert!(AspectXRActionType::InputPose < AspectXRActionType::InputSkeletal);
        assert!(AspectXRActionType::InputSkeletal < AspectXRActionType::OutputHaptic);
    }

    #[test]
    fn test_action_type_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(AspectXRActionType::InputDigital);
        set.insert(AspectXRActionType::InputAnalog);

        assert!(set.contains(&AspectXRActionType::InputDigital));
        assert!(set.contains(&AspectXRActionType::InputAnalog));
        assert!(!set.contains(&AspectXRActionType::InputPose));
    }

    #[test]
    fn test_action_type_debug() {
        let action_type = AspectXRActionType::OutputHaptic;
        let debug_str = format!("{:?}", action_type);
        assert!(debug_str.contains("OutputHaptic"));
    }
}
