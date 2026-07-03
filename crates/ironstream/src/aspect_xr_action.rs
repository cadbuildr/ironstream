// FILE: aspect_xr_action.rs
// occt: Aspect_XRAction
// occt: Aspect_XRActionType

use std::sync::Arc;

/// XR action type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

/// XR action definition.
pub struct AspectXRAction {
    id: String,
    action_type: AspectXRActionType,
    raw_handle: u64,
}

impl AspectXRAction {
    /// Create a new XR action with the given id and type.
    pub fn new(id: impl Into<String>, action_type: AspectXRActionType) -> Self {
        AspectXRAction {
            id: id.into(),
            action_type,
            raw_handle: 0,
        }
    }

    /// Return the action id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the action type.
    pub fn action_type(&self) -> AspectXRActionType {
        self.action_type
    }

    /// Return TRUE if action is defined (has a valid handle).
    pub fn is_valid(&self) -> bool {
        self.raw_handle != 0
    }

    /// Return the action handle.
    pub fn raw_handle(&self) -> u64 {
        self.raw_handle
    }

    /// Set the action handle.
    pub fn set_raw_handle(&mut self, handle: u64) {
        self.raw_handle = handle;
    }
}

/// Type alias for a reference-counted XR action.
pub type AspectXRActionHandle = Arc<AspectXRAction>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xr_action_creation() {
        let action = AspectXRAction::new("test_action", AspectXRActionType::InputDigital);
        assert_eq!(action.id(), "test_action");
        assert_eq!(action.action_type(), AspectXRActionType::InputDigital);
        assert!(!action.is_valid());
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
    fn test_xr_action_handle_operations() {
        let mut action = AspectXRAction::new("pose_action", AspectXRActionType::InputPose);
        assert!(!action.is_valid());

        action.set_raw_handle(12345);
        assert!(action.is_valid());
        assert_eq!(action.raw_handle(), 12345);
    }

    #[test]
    fn test_xr_action_handle_wrapper() {
        let action = AspectXRActionHandle::new(
            AspectXRAction::new("haptic_action", AspectXRActionType::OutputHaptic)
        );
        assert_eq!(action.id(), "haptic_action");
        assert_eq!(action.action_type(), AspectXRActionType::OutputHaptic);
    }
}
