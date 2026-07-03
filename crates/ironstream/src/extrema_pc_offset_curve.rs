// FILE: extrema_pc_offset_curve.rs
// occt: ExtremaPC_OffsetCurve

//! Find extrema between a point and an offset curve.

pub struct OffsetCurveExtrema {
    offset_distance: f64,
}

impl OffsetCurveExtrema {
    pub fn new(offset_distance: f64) -> Self {
        OffsetCurveExtrema { offset_distance }
    }

    pub fn extrema_parameter(&self, _point: (f64, f64, f64)) -> Option<f64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let extrema = OffsetCurveExtrema::new(1.5);
        assert_eq!(extrema.offset_distance, 1.5);
    }
}
