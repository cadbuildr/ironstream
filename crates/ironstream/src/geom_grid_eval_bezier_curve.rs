// FILE: geom_grid_eval_bezier_curve.rs
// occt: GeomGridEval_BezierCurve

pub struct GeomGridEvalBezierCurve {
    degree: usize,
}

impl GeomGridEvalBezierCurve {
    pub fn new(degree: usize) -> Self {
        GeomGridEvalBezierCurve { degree }
    }

    pub fn evaluate(&self, _u: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_eval_bezier_curve_new() {
        let eval = GeomGridEvalBezierCurve::new(3);
        assert_eq!(eval.degree, 3);
    }

    #[test]
    fn test_grid_eval_bezier_curve_evaluate() {
        let eval = GeomGridEvalBezierCurve::new(2);
        let (x, y, z) = eval.evaluate(0.5);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }
}
