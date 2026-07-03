// FILE: geom_grid_eval_offset_curve.rs
// occt: GeomGridEval_OffsetCurve

//! Evaluates an offset curve on a parameter grid.

pub struct OffsetCurveEval {
    offset_distance: f64,
}

impl OffsetCurveEval {
    pub fn new(offset_distance: f64) -> Self {
        OffsetCurveEval { offset_distance }
    }

    pub fn set_offset(&mut self, offset: f64) {
        self.offset_distance = offset;
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
        let eval = OffsetCurveEval::new(1.5);
        assert_eq!(eval.offset_distance, 1.5);
    }

    #[test]
    fn test_set_offset() {
        let mut eval = OffsetCurveEval::new(1.0);
        eval.set_offset(2.5);
        assert_eq!(eval.offset_distance, 2.5);
    }
}
