// FILE: aspect_xr_analog_action_data.rs
// occt: Aspect_XRAnalogActionData

/// Simple 3D floating-point vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    /// Create a new 3D vector.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f { x, y, z }
    }

    /// Create a zero vector.
    pub fn zero() -> Self {
        Vec3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Check if this vector is equal to another within floating-point tolerance.
    pub fn is_equal(&self, other: Vec3f, tolerance: f32) -> bool {
        (self.x - other.x).abs() <= tolerance
            && (self.y - other.y).abs() <= tolerance
            && (self.z - other.z).abs() <= tolerance
    }
}

/// Analog input XR action data.
#[derive(Debug, Clone)]
pub struct AspectXRAnalogActionData {
    /// The origin that caused this action's current state.
    pub active_origin: u64,
    /// Time relative to now when this event happened. Will be negative to indicate a past time.
    pub update_time: f32,
    /// The current state of this action (x, y, z axes).
    pub vec_xyz: Vec3f,
    /// Deltas since the previous update.
    pub delta_xyz: Vec3f,
    /// Whether or not this action is currently available to be bound in the active action set.
    pub is_active: bool,
}

impl AspectXRAnalogActionData {
    /// Create a new analog action data with default values.
    pub fn new() -> Self {
        AspectXRAnalogActionData {
            active_origin: 0,
            update_time: 0.0,
            vec_xyz: Vec3f::zero(),
            delta_xyz: Vec3f::zero(),
            is_active: false,
        }
    }

    /// Check if delta is non-zero.
    pub fn is_changed(&self) -> bool {
        !self.delta_xyz.is_equal(Vec3f::zero(), 1e-6)
    }
}

impl Default for AspectXRAnalogActionData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3f_creation() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vec3f_zero() {
        let v = Vec3f::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_vec3f_is_equal() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(1.0, 2.0, 3.0);
        assert!(v1.is_equal(v2, 0.0001));

        let v3 = Vec3f::new(1.0, 2.0, 3.5);
        assert!(!v1.is_equal(v3, 0.1));
    }

    #[test]
    fn test_analog_action_data_creation() {
        let data = AspectXRAnalogActionData::new();
        assert_eq!(data.active_origin, 0);
        assert_eq!(data.update_time, 0.0);
        assert!(!data.is_active);
        assert!(data.vec_xyz.is_equal(Vec3f::zero(), 0.0001));
    }

    #[test]
    fn test_analog_action_data_is_changed_false() {
        let data = AspectXRAnalogActionData::new();
        assert!(!data.is_changed());
    }

    #[test]
    fn test_analog_action_data_is_changed_true() {
        let mut data = AspectXRAnalogActionData::new();
        data.delta_xyz = Vec3f::new(0.1, 0.2, 0.3);
        assert!(data.is_changed());
    }

    #[test]
    fn test_analog_action_data_is_changed_nearly_zero() {
        let mut data = AspectXRAnalogActionData::new();
        data.delta_xyz = Vec3f::new(1e-7, 1e-7, 1e-7);
        assert!(!data.is_changed());
    }

    #[test]
    fn test_analog_action_data_default() {
        let data = AspectXRAnalogActionData::default();
        assert_eq!(data.active_origin, 0);
        assert!(!data.is_active);
    }
}
