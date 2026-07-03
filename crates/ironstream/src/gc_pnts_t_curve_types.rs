// FILE: gc_pnts_t_curve_types.rs
// occt: GCPnts_TCurveTypes

/// Template-based curve types for GCPnts.
pub struct GcPntsTCurveTypes;

impl GcPntsTCurveTypes {
    pub fn max_segment_length() -> f64 {
        1e10
    }

    pub fn min_segment_length() -> f64 {
        1e-10
    }

    pub fn max_deflection() -> f64 {
        1e10
    }

    pub fn min_deflection() -> f64 {
        1e-10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limits() {
        assert!(GcPntsTCurveTypes::max_segment_length() > GcPntsTCurveTypes::min_segment_length());
        assert!(GcPntsTCurveTypes::max_deflection() > GcPntsTCurveTypes::min_deflection());
    }
}
