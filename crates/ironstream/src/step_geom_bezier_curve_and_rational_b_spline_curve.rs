// FILE: step_geom_bezier_curve_and_rational_b_spline_curve.rs
// occt: StepGeom_BezierCurveAndRationalBSplineCurve

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CartesianPoint {
    name: String,
}

#[derive(Clone)]
pub struct BezierCurve;

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
    Unspecified,
}

#[derive(Clone)]
pub struct BezierCurveAndRationalBSplineCurve {
    name: Arc<String>,
    degree: i32,
    control_points_list: Option<Vec<Arc<Mutex<CartesianPoint>>>>,
    curve_form: BSplineCurveForm,
    closed_curve: StepDataLogical,
    self_intersect: StepDataLogical,
    bezier_curve: Option<Arc<Mutex<BezierCurve>>>,
    rational_bspline_curve: Option<Arc<Mutex<RationalBSplineCurve>>>,
    weights_data: Option<Vec<f64>>,
}

impl BezierCurveAndRationalBSplineCurve {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            degree: 0,
            control_points_list: None,
            curve_form: BSplineCurveForm::Unspecified,
            closed_curve: StepDataLogical::Unknown,
            self_intersect: StepDataLogical::Unknown,
            bezier_curve: None,
            rational_bspline_curve: None,
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
        bezier_curve: Option<Arc<Mutex<BezierCurve>>>,
        rational_bspline_curve: Option<Arc<Mutex<RationalBSplineCurve>>>,
    ) {
        self.name = Arc::new(name);
        self.degree = degree;
        self.control_points_list = control_points_list;
        self.curve_form = curve_form;
        self.closed_curve = closed_curve;
        self.self_intersect = self_intersect;
        self.bezier_curve = bezier_curve;
        self.rational_bspline_curve = rational_bspline_curve;
    }

    pub fn init_with_weights(
        &mut self,
        name: String,
        degree: i32,
        control_points_list: Option<Vec<Arc<Mutex<CartesianPoint>>>>,
        curve_form: BSplineCurveForm,
        closed_curve: StepDataLogical,
        self_intersect: StepDataLogical,
        weights_data: Option<Vec<f64>>,
    ) {
        self.name = Arc::new(name);
        self.degree = degree;
        self.control_points_list = control_points_list;
        self.curve_form = curve_form;
        self.closed_curve = closed_curve;
        self.self_intersect = self_intersect;
        self.weights_data = weights_data;
    }

    pub fn set_bezier_curve(&mut self, curve: Arc<Mutex<BezierCurve>>) {
        self.bezier_curve = Some(curve);
    }

    pub fn bezier_curve(&self) -> Option<Arc<Mutex<BezierCurve>>> {
        self.bezier_curve.clone()
    }

    pub fn set_rational_bspline_curve(&mut self, curve: Arc<Mutex<RationalBSplineCurve>>) {
        self.rational_bspline_curve = Some(curve);
    }

    pub fn rational_bspline_curve(&self) -> Option<Arc<Mutex<RationalBSplineCurve>>> {
        self.rational_bspline_curve.clone()
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

impl Default for BezierCurveAndRationalBSplineCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let curve = BezierCurveAndRationalBSplineCurve::new();
        assert_eq!(curve.degree(), 0);
    }

    #[test]
    fn test_weights() {
        let mut curve = BezierCurveAndRationalBSplineCurve::new();
        curve.set_weights_data(vec![1.0, 0.707, 0.707, 1.0]);
        assert_eq!(curve.nb_weights_data(), 4);
        assert_eq!(curve.weights_data_value(2), Some(0.707));
    }
}
