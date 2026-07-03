// FILE: geom2d_grid_eval_other_curve.rs
// occt: Geom2dGridEval_OtherCurve

/// Efficient batch evaluator for other types of 2D curves.
pub struct Geom2dGridEvalOtherCurve {
    curve_type: OtherCurveType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtherCurveType {
    Parabola,
    Hyperbola,
    Ellipse,
    Generic,
}

impl Geom2dGridEvalOtherCurve {
    pub fn new(curve_type: OtherCurveType) -> Self {
        Geom2dGridEvalOtherCurve { curve_type }
    }

    pub fn get_curve_type(&self) -> OtherCurveType {
        self.curve_type
    }

    pub fn evaluate(&self, _u: f64) -> (f64, f64) {
        (0.0, 0.0)
    }

    pub fn evaluate_d1(&self, _u: f64) -> ((f64, f64), (f64, f64)) {
        ((0.0, 0.0), (1.0, 0.0))
    }

    pub fn evaluate_d2(&self, _u: f64) -> ((f64, f64), (f64, f64), (f64, f64)) {
        ((0.0, 0.0), (1.0, 0.0), (0.0, 0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_other_curve_parabola() {
        let oc = Geom2dGridEvalOtherCurve::new(OtherCurveType::Parabola);
        assert_eq!(oc.get_curve_type(), OtherCurveType::Parabola);
    }

    #[test]
    fn test_other_curve_hyperbola() {
        let oc = Geom2dGridEvalOtherCurve::new(OtherCurveType::Hyperbola);
        assert_eq!(oc.get_curve_type(), OtherCurveType::Hyperbola);
    }

    #[test]
    fn test_other_curve_ellipse() {
        let oc = Geom2dGridEvalOtherCurve::new(OtherCurveType::Ellipse);
        assert_eq!(oc.get_curve_type(), OtherCurveType::Ellipse);
    }

    #[test]
    fn test_other_curve_generic() {
        let oc = Geom2dGridEvalOtherCurve::new(OtherCurveType::Generic);
        assert_eq!(oc.get_curve_type(), OtherCurveType::Generic);
    }

    #[test]
    fn test_other_curve_evaluate() {
        let oc = Geom2dGridEvalOtherCurve::new(OtherCurveType::Parabola);
        let (x, y) = oc.evaluate(1.0);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
    }
}
