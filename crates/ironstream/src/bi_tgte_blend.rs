// FILE: bi_tgte_blend.rs
// occt: BiTgte_Blend

/// Blending operations between tangent geometry.
/// Handles smooth blending between geometric elements.
#[derive(Debug, Clone)]
pub struct BiTgteBlend {
    /// Blend radius
    radius: f64,
    /// Whether blend is computed
    is_done: bool,
}

impl BiTgteBlend {
    /// Create a new Blend with radius
    pub fn new(radius: f64) -> Self {
        Self {
            radius,
            is_done: false,
        }
    }

    /// Get blend radius
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Check if blend is computed
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Mark blend as done
    pub fn set_done(&mut self) {
        self.is_done = true;
    }
}

impl Default for BiTgteBlend {
    fn default() -> Self {
        Self::new(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let blend = BiTgteBlend::new(2.0);
        assert_eq!(blend.radius(), 2.0);
        assert!(!blend.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut blend = BiTgteBlend::new(1.5);
        blend.set_done();
        assert!(blend.is_done());
    }

    #[test]
    fn test_default() {
        let blend = BiTgteBlend::default();
        assert_eq!(blend.radius(), 0.0);
    }
}
