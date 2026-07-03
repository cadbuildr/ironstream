// FILE: aspect_xr_pose_action_data.rs
// occt: Aspect_XRPoseActionData

/// Pose input XR action data.
///
/// Represents the pose state of an XR action, including the device's orientation
/// and whether the action is currently active in the action set.
#[derive(Clone, Debug)]
pub struct AspectXrPoseActionData {
    /// The pose state (position and orientation)
    pub pose: AspectTrackedDevicePose,
    /// The origin that caused this action's current state
    pub active_origin: u64,
    /// Whether this action is currently available to be bound in the active action set
    pub is_active: bool,
}

impl AspectXrPoseActionData {
    /// Create a new XR pose action data with default values.
    pub fn new() -> Self {
        Self {
            pose: AspectTrackedDevicePose::default(),
            active_origin: 0,
            is_active: false,
        }
    }
}

impl Default for AspectXrPoseActionData {
    fn default() -> Self {
        Self::new()
    }
}

/// A minimal TrackedDevicePose representation for testing purposes.
/// In a real implementation, this would contain full pose data (position, orientation).
#[derive(Clone, Debug, Default)]
pub struct AspectTrackedDevicePose {
    /// Placeholder for pose data
    pub _reserved: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xr_pose_action_data_construction() {
        let data = AspectXrPoseActionData::new();
        assert_eq!(data.active_origin, 0);
        assert!(!data.is_active);
    }

    #[test]
    fn test_xr_pose_action_data_default() {
        let data = AspectXrPoseActionData::default();
        assert_eq!(data.active_origin, 0);
        assert!(!data.is_active);
    }

    #[test]
    fn test_xr_pose_action_data_mutation() {
        let mut data = AspectXrPoseActionData::new();
        data.active_origin = 12345;
        data.is_active = true;
        assert_eq!(data.active_origin, 12345);
        assert!(data.is_active);
    }

    #[test]
    fn test_xr_pose_action_data_clone() {
        let data1 = AspectXrPoseActionData::new();
        let data2 = data1.clone();
        assert_eq!(data1.active_origin, data2.active_origin);
        assert_eq!(data1.is_active, data2.is_active);
    }
}
