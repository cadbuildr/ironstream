// FILE: extrema_pc_b_spline_curve.rs
// occt: ExtremaPC_BSplineCurve

//! Find extrema (min/max distance) between a point and a B-spline curve.

pub struct BSplineCurveExtrema {
    degree: i32,
    knots: Vec<f64>,
    poles: Vec<(f64, f64, f64)>,
}

impl BSplineCurveExtrema {
    pub fn new(degree: i32) -> Self {
        BSplineCurveExtrema {
            degree,
            knots: vec![],
            poles: vec![],
        }
    }

    pub fn set_knots(&mut self, knots: Vec<f64>) {
        self.knots = knots;
    }

    pub fn set_poles(&mut self, poles: Vec<(f64, f64, f64)>) {
        self.poles = poles;
    }

    pub fn extrema_distance(&self, _point: (f64, f64, f64)) -> Option<(f64, f64)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let extrema = BSplineCurveExtrema::new(3);
        assert_eq!(extrema.degree, 3);
    }

    #[test]
    fn test_set_knots() {
        let mut extrema = BSplineCurveExtrema::new(3);
        extrema.set_knots(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]);
        assert_eq!(extrema.knots.len(), 8);
    }
}
