// FILE: aspect_xr_tracked_device_role.rs
// occt: Aspect_XRTrackedDeviceRole

/// Predefined tracked devices.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AspectXrTrackedDeviceRole {
    /// Head-mounted display / head position
    Head = 0,
    /// Left hand controller
    LeftHand = 1,
    /// Right hand controller
    RightHand = 2,
    /// Other devices
    Other = 3,
}

/// Number of tracked device roles (used for array sizing)
pub const ASPECT_XR_TRACKED_DEVICE_ROLE_NB: usize = AspectXrTrackedDeviceRole::Other as usize + 1;

impl AspectXrTrackedDeviceRole {
    /// Convert from integer to enum value
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(AspectXrTrackedDeviceRole::Head),
            1 => Some(AspectXrTrackedDeviceRole::LeftHand),
            2 => Some(AspectXrTrackedDeviceRole::RightHand),
            3 => Some(AspectXrTrackedDeviceRole::Other),
            _ => None,
        }
    }

    /// Convert enum to integer
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xr_tracked_device_role_values() {
        assert_eq!(AspectXrTrackedDeviceRole::Head as u32, 0);
        assert_eq!(AspectXrTrackedDeviceRole::LeftHand as u32, 1);
        assert_eq!(AspectXrTrackedDeviceRole::RightHand as u32, 2);
        assert_eq!(AspectXrTrackedDeviceRole::Other as u32, 3);
    }

    #[test]
    fn test_xr_tracked_device_role_nb() {
        assert_eq!(ASPECT_XR_TRACKED_DEVICE_ROLE_NB, 4);
    }

    #[test]
    fn test_xr_tracked_device_role_from_u32() {
        assert_eq!(AspectXrTrackedDeviceRole::from_u32(0), Some(AspectXrTrackedDeviceRole::Head));
        assert_eq!(AspectXrTrackedDeviceRole::from_u32(1), Some(AspectXrTrackedDeviceRole::LeftHand));
        assert_eq!(AspectXrTrackedDeviceRole::from_u32(2), Some(AspectXrTrackedDeviceRole::RightHand));
        assert_eq!(AspectXrTrackedDeviceRole::from_u32(3), Some(AspectXrTrackedDeviceRole::Other));
        assert_eq!(AspectXrTrackedDeviceRole::from_u32(4), None);
        assert_eq!(AspectXrTrackedDeviceRole::from_u32(255), None);
    }

    #[test]
    fn test_xr_tracked_device_role_as_u32() {
        assert_eq!(AspectXrTrackedDeviceRole::Head.as_u32(), 0);
        assert_eq!(AspectXrTrackedDeviceRole::LeftHand.as_u32(), 1);
        assert_eq!(AspectXrTrackedDeviceRole::RightHand.as_u32(), 2);
        assert_eq!(AspectXrTrackedDeviceRole::Other.as_u32(), 3);
    }

    #[test]
    fn test_xr_tracked_device_role_clone_copy() {
        let role = AspectXrTrackedDeviceRole::LeftHand;
        let role_copy = role;
        assert_eq!(role, role_copy);
    }

    #[test]
    fn test_xr_tracked_device_role_ord() {
        assert!(AspectXrTrackedDeviceRole::Head < AspectXrTrackedDeviceRole::LeftHand);
        assert!(AspectXrTrackedDeviceRole::LeftHand < AspectXrTrackedDeviceRole::RightHand);
        assert!(AspectXrTrackedDeviceRole::RightHand < AspectXrTrackedDeviceRole::Other);
    }
}
