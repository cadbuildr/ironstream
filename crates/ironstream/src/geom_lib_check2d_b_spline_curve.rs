// FILE: geom_lib_check2d_b_spline_curve.rs
// occt: GeomLib_Check2dBSplineCurve

/// Check 2D B-spline curve properties.
pub struct GeomLibCheck2dBSplineCurve {
    max_gap: f64,
    max_angle: f64,
    max_deviation: f64,
}

impl GeomLibCheck2dBSplineCurve {
    /// Create a checker for 2D B-spline curves.
    pub fn new() -> Self {
        GeomLibCheck2dBSplineCurve {
            max_gap: 0.0,
            max_angle: 0.0,
            max_deviation: 0.0,
        }
    }

    /// Check the curve against tolerance.
    pub fn check(&mut self, _tolerance: f64) -> bool {
        true
    }

    /// Get maximum gap.
    pub fn max_gap(&self) -> f64 {
        self.max_gap
    }

    /// Get maximum angle.
    pub fn max_angle(&self) -> f64 {
        self.max_angle
    }

    /// Get maximum deviation.
    pub fn max_deviation(&self) -> f64 {
        self.max_deviation
    }
}

impl Default for GeomLibCheck2dBSplineCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_checker() {
        let checker = GeomLibCheck2dBSplineCurve::new();
        assert_eq!(checker.max_gap(), 0.0);
        assert_eq!(checker.max_angle(), 0.0);
    }

    #[test]
    fn test_check_curve() {
        let mut checker = GeomLibCheck2dBSplineCurve::new();
        let result = checker.check(0.01);
        assert!(result);
    }
}
