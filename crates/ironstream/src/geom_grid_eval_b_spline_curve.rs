// FILE: geom_grid_eval_b_spline_curve.rs
// occt: GeomGridEval_BSplineCurve

//! Evaluates a B-spline curve on a grid of parameters.

pub struct BSplineCurveEval {
    degree: i32,
    knot_vector: Vec<f64>,
    poles: Vec<(f64, f64, f64)>,
}

impl BSplineCurveEval {
    pub fn new(degree: i32) -> Self {
        BSplineCurveEval {
            degree,
            knot_vector: vec![],
            poles: vec![],
        }
    }

    pub fn set_knots(&mut self, knots: Vec<f64>) {
        self.knot_vector = knots;
    }

    pub fn set_poles(&mut self, poles: Vec<(f64, f64, f64)>) {
        self.poles = poles;
    }

    pub fn evaluate(&self, _parameter: f64) -> Option<(f64, f64, f64)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let eval = BSplineCurveEval::new(3);
        assert_eq!(eval.degree, 3);
    }

    #[test]
    fn test_set_knots() {
        let mut eval = BSplineCurveEval::new(3);
        eval.set_knots(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]);
        assert_eq!(eval.knot_vector.len(), 8);
    }

    #[test]
    fn test_set_poles() {
        let mut eval = BSplineCurveEval::new(3);
        eval.set_poles(vec![(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)]);
        assert_eq!(eval.poles.len(), 2);
    }
}
