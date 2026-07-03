// FILE: extrema_g_curve_locator_o.rs
// occt: Extrema_GCurveLocator

/// Locates extrema on generic curves.
pub struct GCurveLocator {
    initialized: bool,
}

impl GCurveLocator {
    /// Initialize locator.
    pub fn new() -> Self {
        GCurveLocator {
            initialized: false,
        }
    }

    /// Returns initialization status.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Initialize locator.
    pub fn init(&mut self) {
        self.initialized = true;
    }
}

impl Default for GCurveLocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let locator = GCurveLocator::new();
        assert!(!locator.is_initialized());
    }

    #[test]
    fn test_init() {
        let mut locator = GCurveLocator::new();
        locator.init();
        assert!(locator.is_initialized());
    }

    #[test]
    fn test_default() {
        let locator = GCurveLocator::default();
        assert!(!locator.is_initialized());
    }
}
