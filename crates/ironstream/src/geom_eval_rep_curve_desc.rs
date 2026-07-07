// FILE: geom_eval_rep_curve_desc.rs
// occt: GeomEval_RepCurveDesc

/// Representative curve descriptor for evaluation.
pub struct RepCurveDesc {
    curve_type: String,
}

impl RepCurveDesc {
    pub fn new(curve_type: &str) -> Self {
        Self {
            curve_type: curve_type.to_string(),
        }
    }

    pub fn curve_type(&self) -> &str {
        &self.curve_type
    }
}
