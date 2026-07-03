// FILE: extrema_pc_bezier_curve.rs
// occt: ExtremaPC_BezierCurve

//! Find extrema between a point and a Bezier curve.

pub struct BezierCurveExtrema {
    degree: i32,
    poles: Vec<(f64, f64, f64)>,
}

impl BezierCurveExtrema {
    pub fn new(degree: i32, poles: Vec<(f64, f64, f64)>) -> Self {
        BezierCurveExtrema { degree, poles }
    }

    pub fn extrema_parameter(&self, _point: (f64, f64, f64)) -> Vec<f64> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let poles = vec![(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)];
        let extrema = BezierCurveExtrema::new(1, poles);
        assert_eq!(extrema.degree, 1);
        assert_eq!(extrema.poles.len(), 2);
    }
}
