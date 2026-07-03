// FILE: geom2d_gcc_q_curve.rs
// occt: Geom2dGcc_QCurve

/// Wrapper for qualified curve with parameter range.
#[derive(Debug, Clone)]
pub struct Geom2dGccQCurve {
    pub min_param: f64,
    pub max_param: f64,
}

impl Geom2dGccQCurve {
    pub fn new(min_param: f64, max_param: f64) -> Self {
        Geom2dGccQCurve { min_param, max_param }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q_curve() {
        let qc = Geom2dGccQCurve::new(0.0, 1.0);
        assert_eq!(qc.min_param, 0.0);
        assert_eq!(qc.max_param, 1.0);
    }
}
