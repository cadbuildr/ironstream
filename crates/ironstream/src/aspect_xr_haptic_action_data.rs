// FILE: aspect_xr_haptic_action_data.rs
// occt: Aspect_XRHapticActionData

/// Haptic output XR action data.
#[derive(Clone, Copy, Debug, Default)]
pub struct AspectXRHapticActionData {
    /// Delay in seconds before start
    pub delay: f32,
    /// Duration in seconds
    pub duration: f32,
    /// Vibration frequency
    pub frequency: f32,
    /// Vibration amplitude
    pub amplitude: f32,
}

impl AspectXRHapticActionData {
    /// Create a new haptic action data with default values (all zeros).
    pub fn new() -> Self {
        Self {
            delay: 0.0,
            duration: 0.0,
            frequency: 0.0,
            amplitude: 0.0,
        }
    }

    /// Return true if data is valid (not empty).
    /// Valid means duration > 0, amplitude > 0, frequency > 0, and delay >= 0.
    pub fn is_valid(&self) -> bool {
        self.duration > 0.0 && self.amplitude > 0.0 && self.frequency > 0.0 && self.delay >= 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_invalid() {
        let data = AspectXRHapticActionData::default();
        assert!(!data.is_valid());
    }

    #[test]
    fn test_valid_data() {
        let data = AspectXRHapticActionData {
            delay: 0.1,
            duration: 0.5,
            frequency: 100.0,
            amplitude: 0.8,
        };
        assert!(data.is_valid());
    }

    #[test]
    fn test_zero_duration_invalid() {
        let data = AspectXRHapticActionData {
            delay: 0.0,
            duration: 0.0,
            frequency: 100.0,
            amplitude: 0.8,
        };
        assert!(!data.is_valid());
    }

    #[test]
    fn test_negative_delay_invalid() {
        let data = AspectXRHapticActionData {
            delay: -0.1,
            duration: 0.5,
            frequency: 100.0,
            amplitude: 0.8,
        };
        assert!(!data.is_valid());
    }

    #[test]
    fn test_zero_frequency_invalid() {
        let data = AspectXRHapticActionData {
            delay: 0.0,
            duration: 0.5,
            frequency: 0.0,
            amplitude: 0.8,
        };
        assert!(!data.is_valid());
    }

    #[test]
    fn test_zero_amplitude_invalid() {
        let data = AspectXRHapticActionData {
            delay: 0.0,
            duration: 0.5,
            frequency: 100.0,
            amplitude: 0.0,
        };
        assert!(!data.is_valid());
    }
}
