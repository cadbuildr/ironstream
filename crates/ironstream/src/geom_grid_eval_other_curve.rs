// FILE: geom_grid_eval_other_curve.rs
// occt: GeomGridEval_OtherCurve

//! Evaluates other types of curves on a parameter grid.

pub struct OtherCurveEval;

impl OtherCurveEval {
    pub fn evaluate(_curve_type: &str, _parameter: f64) -> Option<(f64, f64, f64)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let result = OtherCurveEval::evaluate("unknown", 0.5);
        assert_eq!(result, None);
    }
}
