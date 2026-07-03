// FILE: geom_eval_t_bezier_curve.rs
// occt: GeomEval_TBezierCurve

pub struct GeomEvalTBezierCurve {
    degree: usize,
}

impl GeomEvalTBezierCurve {
    pub fn new(degree: usize) -> Self {
        GeomEvalTBezierCurve { degree }
    }

    pub fn get_degree(&self) -> usize {
        self.degree
    }

    pub fn evaluate(&self, _u: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_bezier_curve_new() {
        let eval = GeomEvalTBezierCurve::new(3);
        assert_eq!(eval.get_degree(), 3);
    }

    #[test]
    fn test_t_bezier_curve_evaluate() {
        let eval = GeomEvalTBezierCurve::new(2);
        let (x, y, z) = eval.evaluate(0.5);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }
}
