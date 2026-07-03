// FILE: aspect_tracked_device_pose.rs
// occt: Aspect_TrackedDevicePose

/// Tracked device pose (position and orientation).
#[derive(Debug, Clone, Copy)]
pub struct AspectTrackedDevicePose {
    pub position: (f64, f64, f64),
    pub orientation: (f64, f64, f64, f64), // Quaternion
}

impl AspectTrackedDevicePose {
    /// Create device pose.
    pub fn new(position: (f64, f64, f64), orientation: (f64, f64, f64, f64)) -> Self {
        AspectTrackedDevicePose {
            position,
            orientation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pose = AspectTrackedDevicePose::new((1.0, 2.0, 3.0), (0.0, 0.0, 0.0, 1.0));
        assert_eq!(pose.position, (1.0, 2.0, 3.0));
    }
}
