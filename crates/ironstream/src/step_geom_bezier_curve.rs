// FILE: step_geom_bezier_curve.rs
// occt: StepGeom_BezierCurve

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CartesianPoint {
    name: String,
}

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

#[derive(Clone)]
pub struct BezierCurve {
    name: Arc<String>,
    degree: i32,
    control_points_list: Option<Vec<Arc<Mutex<CartesianPoint>>>>,
    curve_form: BSplineCurveForm,
    closed_curve: StepDataLogical,
    self_intersect: StepDataLogical,
}

impl BezierCurve {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            degree: 0,
            control_points_list: None,
            curve_form: BSplineCurveForm::Unspecified,
            closed_curve: StepDataLogical::Unknown,
            self_intersect: StepDataLogical::Unknown,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        degree: i32,
        control_points_list: Option<Vec<Arc<Mutex<CartesianPoint>>>>,
        curve_form: BSplineCurveForm,
        closed_curve: StepDataLogical,
        self_intersect: StepDataLogical,
    ) {
        self.name = Arc::new(name);
        self.degree = degree;
        self.control_points_list = control_points_list;
        self.curve_form = curve_form;
        self.closed_curve = closed_curve;
        self.self_intersect = self_intersect;
    }

    pub fn set_degree(&mut self, degree: i32) {
        self.degree = degree;
    }

    pub fn degree(&self) -> i32 {
        self.degree
    }

    pub fn set_control_points_list(&mut self, points: Vec<Arc<Mutex<CartesianPoint>>>) {
        self.control_points_list = Some(points);
    }

    pub fn control_points_list(&self) -> Option<Vec<Arc<Mutex<CartesianPoint>>>> {
        self.control_points_list.clone()
    }

    pub fn control_points_list_value(&self, num: i32) -> Option<Arc<Mutex<CartesianPoint>>> {
        self.control_points_list
            .as_ref()
            .and_then(|pts| pts.get((num - 1) as usize).cloned())
    }

    pub fn nb_control_points_list(&self) -> i32 {
        self.control_points_list.as_ref().map_or(0, |pts| pts.len() as i32)
    }

    pub fn set_curve_form(&mut self, form: BSplineCurveForm) {
        self.curve_form = form;
    }

    pub fn curve_form(&self) -> BSplineCurveForm {
        self.curve_form.clone()
    }

    pub fn set_closed_curve(&mut self, closed: StepDataLogical) {
        self.closed_curve = closed;
    }

    pub fn closed_curve(&self) -> StepDataLogical {
        self.closed_curve.clone()
    }

    pub fn set_self_intersect(&mut self, self_intersect: StepDataLogical) {
        self.self_intersect = self_intersect;
    }

    pub fn self_intersect(&self) -> StepDataLogical {
        self.self_intersect.clone()
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for BezierCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let curve = BezierCurve::new();
        assert_eq!(curve.degree(), 0);
    }

    #[test]
    fn test_init() {
        let mut curve = BezierCurve::new();
        curve.init(
            "bezier".to_string(),
            3,
            None,
            BSplineCurveForm::CircularArc,
            StepDataLogical::False,
            StepDataLogical::False,
        );
        assert_eq!(curve.name(), "bezier");
        assert_eq!(curve.degree(), 3);
    }
}
