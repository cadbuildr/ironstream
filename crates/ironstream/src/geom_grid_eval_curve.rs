// FILE: geom_grid_eval_curve.rs
// occt: GeomGridEval_Curve

pub struct GeomGridEvalCurve;

impl GeomGridEvalCurve {
    pub fn new() -> Self {
        GeomGridEvalCurve
    }

    pub fn evaluate(&self, _u: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }
}

impl Default for GeomGridEvalCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _eval = GeomGridEvalCurve::new();
    }

    #[test]
    fn test_evaluate() {
        let eval = GeomGridEvalCurve::new();
        let (x, y, z) = eval.evaluate(0.5);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }
}
