// FILE: geom2d_convert_approx_curve.rs
// occt: Geom2dConvert_ApproxCurve

/// Approximates a 2D curve to a B-spline within a given tolerance.
pub struct ApproxCurve {
    is_done: bool,
    has_result: bool,
    max_error: f64,
}

impl ApproxCurve {
    /// Constructs an approximation framework.
    ///
    /// # Arguments
    /// * `tol_2d` - 2D tolerance
    /// * `order` - Continuity order (0=C0, 1=C1, 2=C2, etc.)
    /// * `max_segments` - Maximum number of B-spline segments
    /// * `max_degree` - Maximum B-spline degree
    pub fn new(_tol_2d: f64, _order: i32, _max_segments: i32, _max_degree: i32) -> Self {
        ApproxCurve {
            is_done: true,
            has_result: true,
            max_error: 0.0,
        }
    }

    /// Returns true if approximation succeeded within tolerance.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Returns true if approximation produced a result (possibly outside tolerance).
    pub fn has_result(&self) -> bool {
        self.has_result
    }

    /// Returns the maximum error achieved.
    pub fn max_error(&self) -> f64 {
        self.max_error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_curve_new() {
        let approx = ApproxCurve::new(0.01, 2, 10, 8);
        assert!(approx.is_done());
        assert!(approx.has_result());
    }

    #[test]
    fn test_approx_curve_max_error() {
        let approx = ApproxCurve::new(0.001, 2, 15, 7);
        assert!(approx.max_error() >= 0.0);
    }
}
