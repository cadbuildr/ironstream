// FILE: aspect_skydome_background.rs
// occt: Aspect_SkydomeBackground

/// Skydome background parameters.
pub struct AspectSkydomeBackground {
    zenith_top_color: (f32, f32, f32),
    zenith_bottom_color: (f32, f32, f32),
    horizon_color: (f32, f32, f32),
}

impl AspectSkydomeBackground {
    /// Create skydome background.
    pub fn new(
        zenith_top: (f32, f32, f32),
        zenith_bottom: (f32, f32, f32),
        horizon: (f32, f32, f32),
    ) -> Self {
        AspectSkydomeBackground {
            zenith_top_color: zenith_top,
            zenith_bottom_color: zenith_bottom,
            horizon_color: horizon,
        }
    }

    /// Get zenith top color.
    pub fn zenith_top_color(&self) -> (f32, f32, f32) {
        self.zenith_top_color
    }

    /// Get zenith bottom color.
    pub fn zenith_bottom_color(&self) -> (f32, f32, f32) {
        self.zenith_bottom_color
    }

    /// Get horizon color.
    pub fn horizon_color(&self) -> (f32, f32, f32) {
        self.horizon_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bg = AspectSkydomeBackground::new((0.0, 0.0, 1.0), (0.5, 0.7, 1.0), (1.0, 1.0, 1.0));
        assert_eq!(bg.zenith_top_color(), (0.0, 0.0, 1.0));
    }
}
