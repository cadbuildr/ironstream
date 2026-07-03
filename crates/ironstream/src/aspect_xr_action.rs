// FILE: aspect_xr_action.rs
// occt: Aspect_XRAction

use crate::aspect_xr_action_type::AspectXRActionType;

/// XR action definition.
#[derive(Debug, Clone)]
pub struct AspectXRAction {
    /// Action ID
    id: String,
    /// Action handle
    raw_handle: u64,
    /// Action type
    action_type: AspectXRActionType,
}

impl AspectXRAction {
    /// Create a new XR action.
    /// @param id action identifier
    /// @param action_type action type
    pub fn new(id: String, action_type: AspectXRActionType) -> Self {
        AspectXRAction {
            id,
            raw_handle: 0,
            action_type,
        }
    }

    /// Return action ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return action type.
    pub fn action_type(&self) -> AspectXRActionType {
        self.action_type
    }

    /// Return TRUE if action is defined (has non-zero handle).
    pub fn is_valid(&self) -> bool {
        self.raw_handle != 0
    }

    /// Return action handle.
    pub fn raw_handle(&self) -> u64 {
        self.raw_handle
    }

    /// Set action handle.
    pub fn set_raw_handle(&mut self, handle: u64) {
        self.raw_handle = handle;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_action() {
        let action = AspectXRAction::new(
            "test_action".to_string(),
            AspectXRActionType::InputDigital,
        );
        assert_eq!(action.id(), "test_action");
        assert_eq!(action.action_type(), AspectXRActionType::InputDigital);
    }

    #[test]
    fn test_action_validity() {
        let mut action = AspectXRAction::new(
            "test_action".to_string(),
            AspectXRActionType::InputAnalog,
        );
        assert!(!action.is_valid());

        action.set_raw_handle(12345);
        assert!(action.is_valid());
        assert_eq!(action.raw_handle(), 12345);
    }

    #[test]
    fn test_action_id() {
        let action = AspectXRAction::new(
            "my_grip_action".to_string(),
            AspectXRActionType::InputPose,
        );
        assert_eq!(action.id(), "my_grip_action");
    }

    #[test]
    fn test_action_types() {
        let digital = AspectXRAction::new(
            "digital".to_string(),
            AspectXRActionType::InputDigital,
        );
        assert_eq!(digital.action_type(), AspectXRActionType::InputDigital);

        let analog = AspectXRAction::new("analog".to_string(), AspectXRActionType::InputAnalog);
        assert_eq!(analog.action_type(), AspectXRActionType::InputAnalog);

        let pose = AspectXRAction::new("pose".to_string(), AspectXRActionType::InputPose);
        assert_eq!(pose.action_type(), AspectXRActionType::InputPose);

        let haptic =
            AspectXRAction::new("haptic".to_string(), AspectXRActionType::OutputHaptic);
        assert_eq!(haptic.action_type(), AspectXRActionType::OutputHaptic);
    }

    #[test]
    fn test_action_handle_lifecycle() {
        let mut action = AspectXRAction::new(
            "lifecycle_test".to_string(),
            AspectXRActionType::InputSkeletal,
        );

        assert!(!action.is_valid());
        assert_eq!(action.raw_handle(), 0);

        action.set_raw_handle(99999);
        assert!(action.is_valid());
        assert_eq!(action.raw_handle(), 99999);
    }

    #[test]
    fn test_action_clone() {
        let mut action = AspectXRAction::new(
            "clone_test".to_string(),
            AspectXRActionType::InputDigital,
        );
        action.set_raw_handle(54321);

        let cloned = action.clone();
        assert_eq!(cloned.id(), action.id());
        assert_eq!(cloned.action_type(), action.action_type());
        assert_eq!(cloned.raw_handle(), action.raw_handle());
        assert_eq!(cloned.is_valid(), action.is_valid());
    }
}
