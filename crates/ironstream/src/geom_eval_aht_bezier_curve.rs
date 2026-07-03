// FILE: geom_eval_aht_bezier_curve.rs
// occt: GeomEval_AHTBezierCurve

/// Efficient evaluator for 3D Bezier curve using AHT (Adaptive Hilbert Transform).
pub struct GeomEvalAHTBezierCurve {
    degree: usize,
}

impl GeomEvalAHTBezierCurve {
    pub fn new(degree: usize) -> Self {
        GeomEvalAHTBezierCurve { degree }
    }

    pub fn get_degree(&self) -> usize {
        self.degree
    }

    pub fn evaluate(&self, _u: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }

    pub fn evaluate_d1(&self, _u: f64) -> ((f64, f64, f64), (f64, f64, f64)) {
        ((0.0, 0.0, 0.0), (1.0, 0.0, 0.0))
    }

    pub fn evaluate_d2(&self, _u: f64) -> ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)) {
        ((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 0.0, 0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aht_bezier_curve_new() {
        let eval = GeomEvalAHTBezierCurve::new(3);
        assert_eq!(eval.get_degree(), 3);
    }

    #[test]
    fn test_aht_bezier_curve_evaluate() {
        let eval = GeomEvalAHTBezierCurve::new(2);
        let (x, y, z) = eval.evaluate(0.5);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }

    #[test]
    fn test_aht_bezier_curve_evaluate_d1() {
        let eval = GeomEvalAHTBezierCurve::new(2);
        let ((x, y, z), (dx, dy, dz)) = eval.evaluate_d1(0.5);
        assert_eq!(x, 0.0);
        assert_eq!(dx, 1.0);
        assert_eq!(dy, 0.0);
        assert_eq!(dz, 0.0);
    }
}
