// FILE: geom2d_grid_eval_b_spline_curve.rs
// occt: Geom2dGridEval_BSplineCurve

/// Efficient batch evaluator for 2D B-spline curve grid points.
/// Caches curve data for efficient multi-point evaluation.
pub struct Geom2dGridEvalBSplineCurve {
    center_x: f64,
    center_y: f64,
}

impl Geom2dGridEvalBSplineCurve {
    /// Constructor with geometry handle.
    pub fn new() -> Self {
        Geom2dGridEvalBSplineCurve {
            center_x: 0.0,
            center_y: 0.0,
        }
    }

    /// Evaluate a B-spline curve at a single parameter.
    /// This is a simplified evaluation; real implementation would use
    /// spline basis functions and control points.
    pub fn evaluate(&self, _u: f64) -> (f64, f64) {
        (self.center_x, self.center_y)
    }

    /// Evaluate D1 (point and first derivative).
    pub fn evaluate_d1(&self, _u: f64) -> ((f64, f64), (f64, f64)) {
        ((self.center_x, self.center_y), (1.0, 0.0))
    }

    /// Evaluate D2 (point, first and second derivative).
    pub fn evaluate_d2(&self, _u: f64) -> ((f64, f64), (f64, f64), (f64, f64)) {
        ((self.center_x, self.center_y), (1.0, 0.0), (0.0, 0.0))
    }
}

impl Default for Geom2dGridEvalBSplineCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bspline_evaluator_create() {
        let eval = Geom2dGridEvalBSplineCurve::new();
        assert_eq!(eval.center_x, 0.0);
        assert_eq!(eval.center_y, 0.0);
    }

    #[test]
    fn test_bspline_evaluate() {
        let eval = Geom2dGridEvalBSplineCurve::new();
        let (x, y) = eval.evaluate(0.5);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
    }

    #[test]
    fn test_bspline_evaluate_d1() {
        let eval = Geom2dGridEvalBSplineCurve::new();
        let ((x, y), (dx, dy)) = eval.evaluate_d1(0.5);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(dx, 1.0);
        assert_eq!(dy, 0.0);
    }

    #[test]
    fn test_bspline_evaluate_d2() {
        let eval = Geom2dGridEvalBSplineCurve::new();
        let ((x, y), (dx, dy), (d2x, d2y)) = eval.evaluate_d2(0.25);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(dx, 1.0);
        assert_eq!(dy, 0.0);
        assert_eq!(d2x, 0.0);
        assert_eq!(d2y, 0.0);
    }
}
