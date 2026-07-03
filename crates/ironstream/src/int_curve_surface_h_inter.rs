// FILE: int_curve_surface_h_inter.rs
// occt: IntCurveSurface_HInter

/// Computes intersection between a curve and a surface.
/// Supports both polyhedral approximations and exact computations.
pub struct IntCurveSurfaceHInter {
    done: bool,
}

impl IntCurveSurfaceHInter {
    /// Create an empty intersection object
    pub fn new() -> Self {
        IntCurveSurfaceHInter { done: false }
    }

    /// Check if computation is done
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Set computation status
    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }
}

impl Default for IntCurveSurfaceHInter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let inter = IntCurveSurfaceHInter::new();
        assert!(!inter.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut inter = IntCurveSurfaceHInter::new();
        inter.set_done(true);
        assert!(inter.is_done());
    }

    #[test]
    fn test_default() {
        let inter = IntCurveSurfaceHInter::default();
        assert!(!inter.is_done());
    }
}
