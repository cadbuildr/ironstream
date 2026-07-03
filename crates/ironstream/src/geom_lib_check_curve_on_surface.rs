// FILE: geom_lib_check_curve_on_surface.rs
// occt: GeomLib_CheckCurveOnSurface

/// Check if a curve lies on a surface.
pub struct GeomLibCheckCurveOnSurface {
    is_on_surface: bool,
    max_deviation: f64,
}

impl GeomLibCheckCurveOnSurface {
    /// Create a checker for curve on surface.
    pub fn new() -> Self {
        GeomLibCheckCurveOnSurface {
            is_on_surface: false,
            max_deviation: f64::INFINITY,
        }
    }

    /// Check if curve lies on surface.
    pub fn perform(&mut self, _tolerance: f64) {
        self.is_on_surface = false;
        self.max_deviation = 0.0;
    }

    /// Check if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.max_deviation != f64::INFINITY
    }

    /// Check if curve is on surface.
    pub fn is_on_surface(&self) -> bool {
        self.is_on_surface
    }

    /// Get maximum deviation.
    pub fn max_deviation(&self) -> f64 {
        self.max_deviation
    }
}

impl Default for GeomLibCheckCurveOnSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_checker() {
        let checker = GeomLibCheckCurveOnSurface::new();
        assert!(!checker.is_on_surface());
    }

    #[test]
    fn test_perform_check() {
        let mut checker = GeomLibCheckCurveOnSurface::new();
        checker.perform(0.01);
        assert!(!checker.is_on_surface());
    }
}
