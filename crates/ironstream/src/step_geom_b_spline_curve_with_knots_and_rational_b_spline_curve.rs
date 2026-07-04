// FILE: step_geom_b_spline_curve_with_knots_and_rational_b_spline_curve.rs
// occt: StepGeom_BSplineCurveWithKnotsAndRationalBSplineCurve

use std::sync::{Arc, Mutex};

/// Placeholder types
#[derive(Clone)]
pub struct CartesianPoint {
    name: String,
}

#[derive(Clone)]
pub struct BSplineCurveWithKnots;

#[derive(Clone)]
pub struct RationalBSplineCurve;

#[derive(Clone, Debug, PartialEq)]
pub enum StepDataLogical {
    True,
    False,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BSplineCurveForm {
    PolylineForm,
    CircularArc,
    EllipticArc,
    ParabolicArc,
    HyperbolicArc,
    Unspecified,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KnotType {
    UniformKnots = 0,
    Unspecified = 1,
    QuasiUniformKnots = 2,
    PiecewiseBezierKnots = 3,
}

/// BSplineCurveWithKnotsAndRationalBSplineCurve: Combined representation.
#[derive(Clone)]
pub struct BSplineCurveWithKnotsAndRationalBSplineCurve {
    name: Arc<String>,
    degree: i32,
    control_points_list: Option<Vec<Arc<Mutex<CartesianPoint>>>>,
    curve_form: BSplineCurveForm,
    closed_curve: StepDataLogical,
    self_intersect: StepDataLogical,
    bspline_curve_with_knots: Option<Arc<Mutex<BSplineCurveWithKnots>>>,
    rational_bspline_curve: Option<Arc<Mutex<RationalBSplineCurve>>>,
    knot_multiplicities: Option<Vec<i32>>,
    knots: Option<Vec<f64>>,
    knot_spec: KnotType,
    weights_data: Option<Vec<f64>>,
}

impl BSplineCurveWithKnotsAndRationalBSplineCurve {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            degree: 0,
            control_points_list: None,
            curve_form: BSplineCurveForm::Unspecified,
            closed_curve: StepDataLogical::Unknown,
            self_intersect: StepDataLogical::Unknown,
            bspline_curve_with_knots: None,
            rational_bspline_curve: None,
            knot_multiplicities: None,
            knots: None,
            knot_spec: KnotType::Unspecified,
            weights_data: None,
        }
    }

    pub fn init_with_curves(
        &mut self,
        name: String,
        degree: i32,
        control_points_list: Option<Vec<Arc<Mutex<CartesianPoint>>>>,
        curve_form: BSplineCurveForm,
        closed_curve: StepDataLogical,
        self_intersect: StepDataLogical,
        bspline_curve_with_knots: Option<Arc<Mutex<BSplineCurveWithKnots>>>,
        rational_bspline_curve: Option<Arc<Mutex<RationalBSplineCurve>>>,
    ) {
        self.name = Arc::new(name);
        self.degree = degree;
        self.control_points_list = control_points_list;
        self.curve_form = curve_form;
        self.closed_curve = closed_curve;
        self.self_intersect = self_intersect;
        self.bspline_curve_with_knots = bspline_curve_with_knots;
        self.rational_bspline_curve = rational_bspline_curve;
    }

    pub fn init_with_knots_and_weights(
        &mut self,
        name: String,
        degree: i32,
        control_points_list: Option<Vec<Arc<Mutex<CartesianPoint>>>>,
        curve_form: BSplineCurveForm,
        closed_curve: StepDataLogical,
        self_intersect: StepDataLogical,
        knot_multiplicities: Option<Vec<i32>>,
        knots: Option<Vec<f64>>,
        knot_spec: KnotType,
        weights_data: Option<Vec<f64>>,
    ) {
        self.name = Arc::new(name);
        self.degree = degree;
        self.control_points_list = control_points_list;
        self.curve_form = curve_form;
        self.closed_curve = closed_curve;
        self.self_intersect = self_intersect;
        self.knot_multiplicities = knot_multiplicities;
        self.knots = knots;
        self.knot_spec = knot_spec;
        self.weights_data = weights_data;
    }

    pub fn set_bspline_curve_with_knots(
        &mut self,
        curve: Arc<Mutex<BSplineCurveWithKnots>>,
    ) {
        self.bspline_curve_with_knots = Some(curve);
    }

    pub fn bspline_curve_with_knots(
        &self,
    ) -> Option<Arc<Mutex<BSplineCurveWithKnots>>> {
        self.bspline_curve_with_knots.clone()
    }

    pub fn set_rational_bspline_curve(
        &mut self,
        curve: Arc<Mutex<RationalBSplineCurve>>,
    ) {
        self.rational_bspline_curve = Some(curve);
    }

    pub fn rational_bspline_curve(
        &self,
    ) -> Option<Arc<Mutex<RationalBSplineCurve>>> {
        self.rational_bspline_curve.clone()
    }

    pub fn set_knot_multiplicities(&mut self, mults: Vec<i32>) {
        self.knot_multiplicities = Some(mults);
    }

    pub fn knot_multiplicities(&self) -> Option<Vec<i32>> {
        self.knot_multiplicities.clone()
    }

    pub fn knot_multiplicities_value(&self, num: i32) -> Option<i32> {
        self.knot_multiplicities
            .as_ref()
            .and_then(|m| m.get((num - 1) as usize).copied())
    }

    pub fn nb_knot_multiplicities(&self) -> i32 {
        self.knot_multiplicities.as_ref().map_or(0, |m| m.len() as i32)
    }

    pub fn set_knots(&mut self, knots: Vec<f64>) {
        self.knots = Some(knots);
    }

    pub fn knots(&self) -> Option<Vec<f64>> {
        self.knots.clone()
    }

    pub fn knots_value(&self, num: i32) -> Option<f64> {
        self.knots
            .as_ref()
            .and_then(|k| k.get((num - 1) as usize).copied())
    }

    pub fn nb_knots(&self) -> i32 {
        self.knots.as_ref().map_or(0, |k| k.len() as i32)
    }

    pub fn set_knot_spec(&mut self, spec: KnotType) {
        self.knot_spec = spec;
    }

    pub fn knot_spec(&self) -> KnotType {
        self.knot_spec
    }

    pub fn set_weights_data(&mut self, weights: Vec<f64>) {
        self.weights_data = Some(weights);
    }

    pub fn weights_data(&self) -> Option<Vec<f64>> {
        self.weights_data.clone()
    }

    pub fn weights_data_value(&self, num: i32) -> Option<f64> {
        self.weights_data
            .as_ref()
            .and_then(|w| w.get((num - 1) as usize).copied())
    }

    pub fn nb_weights_data(&self) -> i32 {
        self.weights_data.as_ref().map_or(0, |w| w.len() as i32)
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }

    pub fn degree(&self) -> i32 {
        self.degree
    }
}

impl Default for BSplineCurveWithKnotsAndRationalBSplineCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let curve = BSplineCurveWithKnotsAndRationalBSplineCurve::new();
        assert_eq!(curve.degree(), 0);
        assert_eq!(curve.knot_spec(), KnotType::Unspecified);
    }

    #[test]
    fn test_weights_data() {
        let mut curve = BSplineCurveWithKnotsAndRationalBSplineCurve::new();
        curve.set_weights_data(vec![1.0, 0.5, 1.0]);
        assert_eq!(curve.nb_weights_data(), 3);
        assert_eq!(curve.weights_data_value(2), Some(0.5));
    }

    #[test]
    fn test_init_with_knots_and_weights() {
        let mut curve = BSplineCurveWithKnotsAndRationalBSplineCurve::new();
        curve.init_with_knots_and_weights(
            "rational_curve".to_string(),
            2,
            None,
            BSplineCurveForm::CircularArc,
            StepDataLogical::True,
            StepDataLogical::False,
            Some(vec![2, 1, 2]),
            Some(vec![0.0, 0.5, 1.0]),
            KnotType::UniformKnots,
            Some(vec![1.0, 0.5, 1.0]),
        );
        assert_eq!(curve.name(), "rational_curve");
        assert_eq!(curve.nb_weights_data(), 3);
    }
}
