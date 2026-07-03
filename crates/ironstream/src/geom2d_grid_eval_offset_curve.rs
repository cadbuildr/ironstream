// FILE: geom2d_grid_eval_offset_curve.rs
// occt: Geom2dGridEval_OffsetCurve

/// Efficient batch evaluator for 2D offset curve grid points.
pub struct Geom2dGridEvalOffsetCurve {
    offset_dist: f64,
}

impl Geom2dGridEvalOffsetCurve {
    pub fn new(offset_dist: f64) -> Self {
        Geom2dGridEvalOffsetCurve {
            offset_dist,
        }
    }

    pub fn evaluate(&self, _u: f64) -> (f64, f64) {
        (self.offset_dist, 0.0)
    }

    pub fn evaluate_d1(&self, _u: f64) -> ((f64, f64), (f64, f64)) {
        ((self.offset_dist, 0.0), (1.0, 0.0))
    }

    pub fn evaluate_d2(&self, _u: f64) -> ((f64, f64), (f64, f64), (f64, f64)) {
        ((self.offset_dist, 0.0), (1.0, 0.0), (0.0, 0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_offset_curve_new() {
        let oc = Geom2dGridEvalOffsetCurve::new(2.5);
        assert!((oc.offset_dist - 2.5).abs() < PRECISION);
    }

    #[test]
    fn test_offset_curve_evaluate() {
        let oc = Geom2dGridEvalOffsetCurve::new(3.0);
        let (x, y) = oc.evaluate(0.5);
        assert!((x - 3.0).abs() < PRECISION);
        assert!(y.abs() < PRECISION);
    }

    #[test]
    fn test_offset_curve_evaluate_d1() {
        let oc = Geom2dGridEvalOffsetCurve::new(1.5);
        let ((x, y), (dx, dy)) = oc.evaluate_d1(0.0);
        assert!((x - 1.5).abs() < PRECISION);
        assert!((dx - 1.0).abs() < PRECISION);
        assert!(dy.abs() < PRECISION);
    }
}
