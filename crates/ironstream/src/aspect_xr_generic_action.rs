// FILE: aspect_xr_generic_action.rs
// occt: Aspect_XRGenericAction

/// Generic XR action enumeration.
///
/// Defines standard XR actions that can be bound to various input devices.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AspectXRGenericAction {
    /// Headset is on/off head
    IsHeadsetOn = 0,
    /// Application menu button pressed/released
    InputAppMenu = 1,
    /// System menu button pressed/released
    InputSysMenu = 2,
    /// Trigger squeezing [0..1], 1 to click
    InputTriggerPull = 3,
    /// Trigger clicked/released
    InputTriggerClick = 4,
    /// Grip state on/off
    InputGripClick = 5,
    /// Trackpad 2D position [-1,+1] with X and Y axes
    InputTrackPadPosition = 6,
    /// Trackpad touched/untouched
    InputTrackPadTouch = 7,
    /// Trackpad clicked/released
    InputTrackPadClick = 8,
    /// Thumbstick 2D position [-1,+1] with X and Y axes
    InputThumbstickPosition = 9,
    /// Thumbstick touched/untouched
    InputThumbstickTouch = 10,
    /// Thumbstick clicked/released
    InputThumbstickClick = 11,
    /// Base position of hand
    InputPoseBase = 12,
    /// Front position of hand
    InputPoseFront = 13,
    /// Position of main handgrip
    InputPoseHandGrip = 14,
    /// Position of main fingertip
    InputPoseFingerTip = 15,
    /// Haptic output (vibration)
    OutputHaptic = 16,
}

/// Total number of generic actions
pub const ASPECT_XR_GENERIC_ACTION_NB: u32 = 17; // OutputHaptic + 1

impl AspectXRGenericAction {
    /// Return the numeric value associated with this action.
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    /// Check if this is a haptic output action.
    pub fn is_haptic(&self) -> bool {
        *self == AspectXRGenericAction::OutputHaptic
    }

    /// Check if this is a position/pose action.
    pub fn is_pose(&self) -> bool {
        matches!(
            self,
            AspectXRGenericAction::InputPoseBase
                | AspectXRGenericAction::InputPoseFront
                | AspectXRGenericAction::InputPoseHandGrip
                | AspectXRGenericAction::InputPoseFingerTip
        )
    }

    /// Check if this is a trackpad action.
    pub fn is_trackpad(&self) -> bool {
        matches!(
            self,
            AspectXRGenericAction::InputTrackPadPosition
                | AspectXRGenericAction::InputTrackPadTouch
                | AspectXRGenericAction::InputTrackPadClick
        )
    }

    /// Check if this is a thumbstick action.
    pub fn is_thumbstick(&self) -> bool {
        matches!(
            self,
            AspectXRGenericAction::InputThumbstickPosition
                | AspectXRGenericAction::InputThumbstickTouch
                | AspectXRGenericAction::InputThumbstickClick
        )
    }

    /// Check if this is a trigger action.
    pub fn is_trigger(&self) -> bool {
        matches!(
            self,
            AspectXRGenericAction::InputTriggerPull | AspectXRGenericAction::InputTriggerClick
        )
    }

    /// Return the count of all generic actions.
    pub fn count() -> u32 {
        ASPECT_XR_GENERIC_ACTION_NB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_values() {
        assert_eq!(AspectXRGenericAction::IsHeadsetOn as u32, 0);
        assert_eq!(AspectXRGenericAction::InputAppMenu as u32, 1);
        assert_eq!(AspectXRGenericAction::InputSysMenu as u32, 2);
        assert_eq!(AspectXRGenericAction::InputTriggerPull as u32, 3);
        assert_eq!(AspectXRGenericAction::InputTriggerClick as u32, 4);
        assert_eq!(AspectXRGenericAction::InputGripClick as u32, 5);
        assert_eq!(AspectXRGenericAction::InputTrackPadPosition as u32, 6);
        assert_eq!(AspectXRGenericAction::InputTrackPadTouch as u32, 7);
        assert_eq!(AspectXRGenericAction::InputTrackPadClick as u32, 8);
        assert_eq!(AspectXRGenericAction::InputThumbstickPosition as u32, 9);
        assert_eq!(AspectXRGenericAction::InputThumbstickTouch as u32, 10);
        assert_eq!(AspectXRGenericAction::InputThumbstickClick as u32, 11);
        assert_eq!(AspectXRGenericAction::InputPoseBase as u32, 12);
        assert_eq!(AspectXRGenericAction::InputPoseFront as u32, 13);
        assert_eq!(AspectXRGenericAction::InputPoseHandGrip as u32, 14);
        assert_eq!(AspectXRGenericAction::InputPoseFingerTip as u32, 15);
        assert_eq!(AspectXRGenericAction::OutputHaptic as u32, 16);
    }

    #[test]
    fn test_count() {
        assert_eq!(ASPECT_XR_GENERIC_ACTION_NB, 17);
        assert_eq!(AspectXRGenericAction::count(), 17);
    }

    #[test]
    fn test_as_u32() {
        assert_eq!(AspectXRGenericAction::IsHeadsetOn.as_u32(), 0);
        assert_eq!(AspectXRGenericAction::OutputHaptic.as_u32(), 16);
    }

    #[test]
    fn test_is_haptic() {
        assert!(AspectXRGenericAction::OutputHaptic.is_haptic());
        assert!(!AspectXRGenericAction::InputAppMenu.is_haptic());
        assert!(!AspectXRGenericAction::InputTriggerPull.is_haptic());
    }

    #[test]
    fn test_is_pose() {
        assert!(AspectXRGenericAction::InputPoseBase.is_pose());
        assert!(AspectXRGenericAction::InputPoseFront.is_pose());
        assert!(AspectXRGenericAction::InputPoseHandGrip.is_pose());
        assert!(AspectXRGenericAction::InputPoseFingerTip.is_pose());
        assert!(!AspectXRGenericAction::InputTriggerClick.is_pose());
    }

    #[test]
    fn test_is_trackpad() {
        assert!(AspectXRGenericAction::InputTrackPadPosition.is_trackpad());
        assert!(AspectXRGenericAction::InputTrackPadTouch.is_trackpad());
        assert!(AspectXRGenericAction::InputTrackPadClick.is_trackpad());
        assert!(!AspectXRGenericAction::InputThumbstickPosition.is_trackpad());
    }

    #[test]
    fn test_is_thumbstick() {
        assert!(AspectXRGenericAction::InputThumbstickPosition.is_thumbstick());
        assert!(AspectXRGenericAction::InputThumbstickTouch.is_thumbstick());
        assert!(AspectXRGenericAction::InputThumbstickClick.is_thumbstick());
        assert!(!AspectXRGenericAction::InputTrackPadPosition.is_thumbstick());
    }

    #[test]
    fn test_is_trigger() {
        assert!(AspectXRGenericAction::InputTriggerPull.is_trigger());
        assert!(AspectXRGenericAction::InputTriggerClick.is_trigger());
        assert!(!AspectXRGenericAction::InputGripClick.is_trigger());
    }

    #[test]
    fn test_equality() {
        let action1 = AspectXRGenericAction::OutputHaptic;
        let action2 = AspectXRGenericAction::OutputHaptic;
        assert_eq!(action1, action2);

        let action3 = AspectXRGenericAction::IsHeadsetOn;
        assert_ne!(action1, action3);
    }

    #[test]
    fn test_ordering() {
        assert!(AspectXRGenericAction::IsHeadsetOn < AspectXRGenericAction::InputAppMenu);
        assert!(AspectXRGenericAction::InputAppMenu < AspectXRGenericAction::OutputHaptic);
    }

    #[test]
    fn test_clone() {
        let action = AspectXRGenericAction::InputTriggerClick;
        let cloned = action;
        assert_eq!(action, cloned);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(AspectXRGenericAction::InputTriggerClick);
        set.insert(AspectXRGenericAction::InputGripClick);

        assert!(set.contains(&AspectXRGenericAction::InputTriggerClick));
        assert!(set.contains(&AspectXRGenericAction::InputGripClick));
        assert!(!set.contains(&AspectXRGenericAction::InputAppMenu));
    }

    #[test]
    fn test_all_pose_actions() {
        let poses = vec![
            AspectXRGenericAction::InputPoseBase,
            AspectXRGenericAction::InputPoseFront,
            AspectXRGenericAction::InputPoseHandGrip,
            AspectXRGenericAction::InputPoseFingerTip,
        ];

        for pose in poses {
            assert!(pose.is_pose());
        }
    }

    #[test]
    fn test_debug_format() {
        let action = AspectXRGenericAction::OutputHaptic;
        let debug_str = format!("{:?}", action);
        assert!(debug_str.contains("OutputHaptic"));
    }
}
