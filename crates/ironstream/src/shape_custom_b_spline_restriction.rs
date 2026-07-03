// FILE: shape_custom_b_spline_restriction.rs
// occt: ShapeCustom_BSplineRestriction

/// BSpline restriction modification.
pub struct BSplineRestriction {
    tol3d: f64,
    tol2d: f64,
    max_degree: usize,
    max_segments: usize,
    approx_surface: bool,
    approx_curve3d: bool,
    approx_curve2d: bool,
}

impl BSplineRestriction {
    /// Create empty tool.
    pub fn new() -> Self {
        BSplineRestriction {
            tol3d: 0.0,
            tol2d: 0.0,
            max_degree: 0,
            max_segments: 0,
            approx_surface: false,
            approx_curve3d: false,
            approx_curve2d: false,
        }
    }

    /// Set tolerance 3d.
    pub fn set_tol3d(&mut self, tol: f64) {
        self.tol3d = tol;
    }

    /// Set tolerance 2d.
    pub fn set_tol2d(&mut self, tol: f64) {
        self.tol2d = tol;
    }

    /// Set max degree.
    pub fn set_max_degree(&mut self, deg: usize) {
        self.max_degree = deg;
    }

    /// Set max segments.
    pub fn set_max_segments(&mut self, seg: usize) {
        self.max_segments = seg;
    }
}

impl Default for BSplineRestriction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bs = BSplineRestriction::new();
        assert_eq!(bs.tol3d, 0.0);
        assert!(!bs.approx_surface);
    }

    #[test]
    fn test_set_tolerances() {
        let mut bs = BSplineRestriction::new();
        bs.set_tol3d(0.01);
        bs.set_tol2d(0.001);
        assert_eq!(bs.tol3d, 0.01);
        assert_eq!(bs.tol2d, 0.001);
    }

    #[test]
    fn test_set_degree() {
        let mut bs = BSplineRestriction::new();
        bs.set_max_degree(8);
        assert_eq!(bs.max_degree, 8);
    }
}
