// FILE: aspect_xr_digital_action_data.rs
// occt: Aspect_XRDigitalActionData

/// Digital input XR action data.
///
/// Contains state information for digital input devices (buttons and switches).
#[derive(Debug, Clone)]
pub struct AspectXRDigitalActionData {
    /// The origin that caused this action's current state
    pub active_origin: u64,
    /// Time relative to now when this event happened. Will be negative to indicate a past time.
    pub update_time: f32,
    /// Whether or not this action is currently available to be bound in the active action set
    pub is_active: bool,
    /// The current state of this action; will be true if currently pressed
    pub is_pressed: bool,
    /// This is true if the state has changed since the last frame
    pub is_changed: bool,
}

impl AspectXRDigitalActionData {
    /// Create a new digital action data with default values.
    pub fn new() -> Self {
        AspectXRDigitalActionData {
            active_origin: 0,
            update_time: 0.0,
            is_active: false,
            is_pressed: false,
            is_changed: false,
        }
    }
}

impl Default for AspectXRDigitalActionData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digital_action_data_creation() {
        let data = AspectXRDigitalActionData::new();
        assert_eq!(data.active_origin, 0);
        assert_eq!(data.update_time, 0.0);
        assert!(!data.is_active);
        assert!(!data.is_pressed);
        assert!(!data.is_changed);
    }

    #[test]
    fn test_digital_action_data_press_state() {
        let mut data = AspectXRDigitalActionData::new();
        assert!(!data.is_pressed);

        data.is_pressed = true;
        assert!(data.is_pressed);
    }

    #[test]
    fn test_digital_action_data_state_change() {
        let mut data = AspectXRDigitalActionData::new();
        assert!(!data.is_changed);

        data.is_pressed = true;
        data.is_changed = true;
        assert!(data.is_changed);
    }

    #[test]
    fn test_digital_action_data_active() {
        let mut data = AspectXRDigitalActionData::new();
        assert!(!data.is_active);

        data.is_active = true;
        assert!(data.is_active);
    }

    #[test]
    fn test_digital_action_data_update_time() {
        let mut data = AspectXRDigitalActionData::new();
        data.update_time = -0.5;
        assert_eq!(data.update_time, -0.5);
    }

    #[test]
    fn test_digital_action_data_default() {
        let data = AspectXRDigitalActionData::default();
        assert_eq!(data.active_origin, 0);
        assert_eq!(data.update_time, 0.0);
        assert!(!data.is_active);
        assert!(!data.is_pressed);
        assert!(!data.is_changed);
    }

    #[test]
    fn test_digital_action_data_full_state() {
        let mut data = AspectXRDigitalActionData::new();
        data.active_origin = 12345;
        data.update_time = -0.123;
        data.is_active = true;
        data.is_pressed = true;
        data.is_changed = true;

        assert_eq!(data.active_origin, 12345);
        assert_eq!(data.update_time, -0.123);
        assert!(data.is_active);
        assert!(data.is_pressed);
        assert!(data.is_changed);
    }
}
