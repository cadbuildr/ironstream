// FILE: step_geom_b_spline_curve_with_knots.rs
// occt: StepGeom_BSplineCurveWithKnots

use std::sync::{Arc, Mutex};

/// Knot type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KnotType {
    UniformKnots = 0,
    Unspecified = 1,
    QuasiUniformKnots = 2,
    PiecewiseBezierKnots = 3,
}

/// Placeholder for CartesianPoint
#[derive(Clone)]
pub struct CartesianPoint {
    name: String,
}

/// Placeholder for StepData_Logical
#[derive(Clone, Debug, PartialEq)]
pub enum StepDataLogical {
    True,
    False,
    Unknown,
}

/// Placeholder for BSplineCurveForm
#[derive(Clone, Debug, PartialEq)]
pub enum BSplineCurveForm {
    PolylineForm,
    CircularArc,
    EllipticArc,
    ParabolicArc,
    HyperbolicArc,
    Unspecified,
}

/// BSplineCurveWithKnots: A B-spline curve with explicit knot vectors and multiplicities.
#[derive(Clone)]
pub struct BSplineCurveWithKnots {
    name: Arc<String>,
    degree: i32,
    control_points_list: Option<Vec<Arc<Mutex<CartesianPoint>>>>,
    curve_form: BSplineCurveForm,
    closed_curve: StepDataLogical,
    self_intersect: StepDataLogical,
    knot_multiplicities: Option<Vec<i32>>,
    knots: Option<Vec<f64>>,
    knot_spec: KnotType,
}

impl BSplineCurveWithKnots {
    /// Creates a new BSplineCurveWithKnots.
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            degree: 0,
            control_points_list: None,
            curve_form: BSplineCurveForm::Unspecified,
            closed_curve: StepDataLogical::Unknown,
            self_intersect: StepDataLogical::Unknown,
            knot_multiplicities: None,
            knots: None,
            knot_spec: KnotType::Unspecified,
        }
    }

    /// Initializes with all parameters.
    pub fn init(
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
    }

    /// Sets knot multiplicities.
    pub fn set_knot_multiplicities(&mut self, knot_multiplicities: Vec<i32>) {
        self.knot_multiplicities = Some(knot_multiplicities);
    }

    /// Returns knot multiplicities.
    pub fn knot_multiplicities(&self) -> Option<Vec<i32>> {
        self.knot_multiplicities.clone()
    }

    /// Returns a single knot multiplicity by index (1-based).
    pub fn knot_multiplicities_value(&self, num: i32) -> Option<i32> {
        self.knot_multiplicities
            .as_ref()
            .and_then(|mults| mults.get((num - 1) as usize).copied())
    }

    /// Returns the number of knot multiplicities.
    pub fn nb_knot_multiplicities(&self) -> i32 {
        self.knot_multiplicities.as_ref().map_or(0, |m| m.len() as i32)
    }

    /// Sets knots.
    pub fn set_knots(&mut self, knots: Vec<f64>) {
        self.knots = Some(knots);
    }

    /// Returns knots.
    pub fn knots(&self) -> Option<Vec<f64>> {
        self.knots.clone()
    }

    /// Returns a single knot by index (1-based).
    pub fn knots_value(&self, num: i32) -> Option<f64> {
        self.knots
            .as_ref()
            .and_then(|k| k.get((num - 1) as usize).copied())
    }

    /// Returns the number of knots.
    pub fn nb_knots(&self) -> i32 {
        self.knots.as_ref().map_or(0, |k| k.len() as i32)
    }

    /// Sets knot specification type.
    pub fn set_knot_spec(&mut self, knot_spec: KnotType) {
        self.knot_spec = knot_spec;
    }

    /// Returns knot specification type.
    pub fn knot_spec(&self) -> KnotType {
        self.knot_spec
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }

    pub fn degree(&self) -> i32 {
        self.degree
    }

    pub fn closed_curve(&self) -> StepDataLogical {
        self.closed_curve.clone()
    }

    pub fn self_intersect(&self) -> StepDataLogical {
        self.self_intersect.clone()
    }
}

impl Default for BSplineCurveWithKnots {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_spline_curve_with_knots_creation() {
        let curve = BSplineCurveWithKnots::new();
        assert_eq!(curve.degree(), 0);
        assert_eq!(curve.knot_spec(), KnotType::Unspecified);
    }

    #[test]
    fn test_b_spline_curve_with_knots_init() {
        let mut curve = BSplineCurveWithKnots::new();
        curve.init(
            "test".to_string(),
            2,
            None,
            BSplineCurveForm::CircularArc,
            StepDataLogical::True,
            StepDataLogical::False,
            Some(vec![1, 2, 1]),
            Some(vec![0.0, 0.5, 1.0]),
            KnotType::UniformKnots,
        );
        assert_eq!(curve.name(), "test");
        assert_eq!(curve.degree(), 2);
        assert_eq!(curve.nb_knot_multiplicities(), 3);
        assert_eq!(curve.nb_knots(), 3);
    }

    #[test]
    fn test_b_spline_curve_with_knots_knot_values() {
        let mut curve = BSplineCurveWithKnots::new();
        curve.set_knots(vec![0.0, 0.5, 1.0]);
        assert_eq!(curve.knots_value(1), Some(0.0));
        assert_eq!(curve.knots_value(2), Some(0.5));
        assert_eq!(curve.knots_value(3), Some(1.0));
    }

    #[test]
    fn test_b_spline_curve_with_knots_multiplicity_values() {
        let mut curve = BSplineCurveWithKnots::new();
        curve.set_knot_multiplicities(vec![2, 1, 2]);
        assert_eq!(curve.knot_multiplicities_value(1), Some(2));
        assert_eq!(curve.knot_multiplicities_value(2), Some(1));
        assert_eq!(curve.knot_multiplicities_value(3), Some(2));
    }
}
