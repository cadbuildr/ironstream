// FILE: step_geom_b_spline_curve.rs
// occt: StepGeom_BSplineCurve

use std::sync::{Arc, Mutex};

/// Placeholder for StepData_Logical enum (OCCT tri-state logic)
#[derive(Clone, Debug, PartialEq)]
pub enum StepDataLogical {
    True,
    False,
    Unknown,
}

/// Placeholder for CartesianPoint
#[derive(Clone)]
pub struct CartesianPoint {
    name: String,
}

/// Simplified BSplineCurveForm enum
#[derive(Clone, Debug, PartialEq)]
pub enum BSplineCurveForm {
    PolylineForm,
    CircularArc,
    EllipticArc,
    ParabolicArc,
    HyperbolicArc,
    Unspecified,
}

/// BSplineCurve: A rational B-spline curve definition.
#[derive(Clone)]
pub struct BSplineCurve {
    name: Arc<String>,
    degree: i32,
    control_points_list: Option<Vec<Arc<Mutex<CartesianPoint>>>>,
    curve_form: BSplineCurveForm,
    closed_curve: StepDataLogical,
    self_intersect: StepDataLogical,
}

impl BSplineCurve {
    /// Creates a new BSplineCurve.
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

    /// Initializes the BSplineCurve with all parameters.
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

    /// Sets the degree of the curve.
    pub fn set_degree(&mut self, degree: i32) {
        self.degree = degree;
    }

    /// Returns the degree.
    pub fn degree(&self) -> i32 {
        self.degree
    }

    /// Sets the control points list.
    pub fn set_control_points_list(&mut self, control_points_list: Vec<Arc<Mutex<CartesianPoint>>>) {
        self.control_points_list = Some(control_points_list);
    }

    /// Returns the control points list.
    pub fn control_points_list(&self) -> Option<Vec<Arc<Mutex<CartesianPoint>>>> {
        self.control_points_list.clone()
    }

    /// Returns a single control point by index (1-based).
    pub fn control_points_list_value(&self, num: i32) -> Option<Arc<Mutex<CartesianPoint>>> {
        self.control_points_list
            .as_ref()
            .and_then(|pts| pts.get((num - 1) as usize).cloned())
    }

    /// Returns the number of control points.
    pub fn nb_control_points_list(&self) -> i32 {
        self.control_points_list.as_ref().map_or(0, |pts| pts.len() as i32)
    }

    /// Sets the curve form.
    pub fn set_curve_form(&mut self, curve_form: BSplineCurveForm) {
        self.curve_form = curve_form;
    }

    /// Returns the curve form.
    pub fn curve_form(&self) -> BSplineCurveForm {
        self.curve_form.clone()
    }

    /// Sets whether the curve is closed.
    pub fn set_closed_curve(&mut self, closed_curve: StepDataLogical) {
        self.closed_curve = closed_curve;
    }

    /// Returns whether the curve is closed.
    pub fn closed_curve(&self) -> StepDataLogical {
        self.closed_curve.clone()
    }

    /// Sets whether the curve self-intersects.
    pub fn set_self_intersect(&mut self, self_intersect: StepDataLogical) {
        self.self_intersect = self_intersect;
    }

    /// Returns whether the curve self-intersects.
    pub fn self_intersect(&self) -> StepDataLogical {
        self.self_intersect.clone()
    }

    /// Returns the name.
    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for BSplineCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_spline_curve_creation() {
        let curve = BSplineCurve::new();
        assert_eq!(curve.degree(), 0);
        assert_eq!(curve.curve_form(), BSplineCurveForm::Unspecified);
    }

    #[test]
    fn test_b_spline_curve_init() {
        let mut curve = BSplineCurve::new();
        curve.init(
            "test_curve".to_string(),
            3,
            None,
            BSplineCurveForm::CircularArc,
            StepDataLogical::True,
            StepDataLogical::False,
        );
        assert_eq!(curve.name(), "test_curve");
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.curve_form(), BSplineCurveForm::CircularArc);
        assert_eq!(curve.closed_curve(), StepDataLogical::True);
        assert_eq!(curve.self_intersect(), StepDataLogical::False);
    }

    #[test]
    fn test_b_spline_curve_control_points() {
        let mut curve = BSplineCurve::new();
        let pts = vec![
            Arc::new(Mutex::new(CartesianPoint {
                name: "p1".to_string(),
            })),
            Arc::new(Mutex::new(CartesianPoint {
                name: "p2".to_string(),
            })),
        ];
        curve.set_control_points_list(pts);
        assert_eq!(curve.nb_control_points_list(), 2);
    }
}
