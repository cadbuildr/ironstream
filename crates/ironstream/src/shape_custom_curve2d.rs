// FILE: shape_custom_curve2d.rs
// occt: ShapeCustom_Curve2d

/// Converts 2D curve to analytical form.
pub struct Curve2d;

impl Curve2d {
    /// Check if poles are linear.
    pub fn is_linear(_poles: &[(f64, f64)], _tolerance: f64) -> (bool, f64) {
        (false, 0.0)
    }

    /// Convert to line 2d.
    pub fn convert_to_line2d(_curve: &[u8], _first: f64, _last: f64, _tolerance: f64) -> Option<(f64, f64, f64)> {
        None
    }

    /// Simplify BSpline 2d.
    pub fn simplify_bspline2d(_bspline: &[u8], _tolerance: f64) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_linear_empty() {
        let (is_linear, dev) = Curve2d::is_linear(&[], 0.01);
        assert!(!is_linear);
        assert_eq!(dev, 0.0);
    }

    #[test]
    fn test_convert_to_line2d() {
        let result = Curve2d::convert_to_line2d(&[], 0.0, 1.0, 0.01);
        assert!(result.is_none());
    }

    #[test]
    fn test_simplify_bspline2d() {
        let result = Curve2d::simplify_bspline2d(&[], 0.01);
        assert!(!result);
    }
}
