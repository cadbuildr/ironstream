// FILE: step_geom_b_spline_surface_with_knots.rs
// occt: StepGeom_BSplineSurfaceWithKnots

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CartesianPoint {
    name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KnotType {
    UniformKnots = 0,
    Unspecified = 1,
    QuasiUniformKnots = 2,
    PiecewiseBezierKnots = 3,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StepDataLogical {
    True,
    False,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BSplineSurfaceForm {
    PlaneSurface,
    CylindricalSurface,
    ConicalSurface,
    SphericalSurface,
    ToroidalSurface,
    Unspecified,
}

#[derive(Clone)]
pub struct BSplineSurfaceWithKnots {
    name: Arc<String>,
    u_degree: i32,
    v_degree: i32,
    control_points_list: Option<Vec<Vec<Arc<Mutex<CartesianPoint>>>>>,
    surface_form: BSplineSurfaceForm,
    u_closed: StepDataLogical,
    v_closed: StepDataLogical,
    self_intersect: StepDataLogical,
    u_knot_multiplicities: Option<Vec<i32>>,
    v_knot_multiplicities: Option<Vec<i32>>,
    u_knots: Option<Vec<f64>>,
    v_knots: Option<Vec<f64>>,
    knot_spec: KnotType,
}

impl BSplineSurfaceWithKnots {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            u_degree: 0,
            v_degree: 0,
            control_points_list: None,
            surface_form: BSplineSurfaceForm::Unspecified,
            u_closed: StepDataLogical::Unknown,
            v_closed: StepDataLogical::Unknown,
            self_intersect: StepDataLogical::Unknown,
            u_knot_multiplicities: None,
            v_knot_multiplicities: None,
            u_knots: None,
            v_knots: None,
            knot_spec: KnotType::Unspecified,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        u_degree: i32,
        v_degree: i32,
        control_points_list: Option<Vec<Vec<Arc<Mutex<CartesianPoint>>>>>,
        surface_form: BSplineSurfaceForm,
        u_closed: StepDataLogical,
        v_closed: StepDataLogical,
        self_intersect: StepDataLogical,
        u_knot_multiplicities: Option<Vec<i32>>,
        v_knot_multiplicities: Option<Vec<i32>>,
        u_knots: Option<Vec<f64>>,
        v_knots: Option<Vec<f64>>,
        knot_spec: KnotType,
    ) {
        self.name = Arc::new(name);
        self.u_degree = u_degree;
        self.v_degree = v_degree;
        self.control_points_list = control_points_list;
        self.surface_form = surface_form;
        self.u_closed = u_closed;
        self.v_closed = v_closed;
        self.self_intersect = self_intersect;
        self.u_knot_multiplicities = u_knot_multiplicities;
        self.v_knot_multiplicities = v_knot_multiplicities;
        self.u_knots = u_knots;
        self.v_knots = v_knots;
        self.knot_spec = knot_spec;
    }

    pub fn set_u_knot_multiplicities(&mut self, mults: Vec<i32>) {
        self.u_knot_multiplicities = Some(mults);
    }

    pub fn u_knot_multiplicities(&self) -> Option<Vec<i32>> {
        self.u_knot_multiplicities.clone()
    }

    pub fn u_knot_multiplicities_value(&self, num: i32) -> Option<i32> {
        self.u_knot_multiplicities
            .as_ref()
            .and_then(|m| m.get((num - 1) as usize).copied())
    }

    pub fn nb_u_knot_multiplicities(&self) -> i32 {
        self.u_knot_multiplicities.as_ref().map_or(0, |m| m.len() as i32)
    }

    pub fn set_v_knot_multiplicities(&mut self, mults: Vec<i32>) {
        self.v_knot_multiplicities = Some(mults);
    }

    pub fn v_knot_multiplicities(&self) -> Option<Vec<i32>> {
        self.v_knot_multiplicities.clone()
    }

    pub fn v_knot_multiplicities_value(&self, num: i32) -> Option<i32> {
        self.v_knot_multiplicities
            .as_ref()
            .and_then(|m| m.get((num - 1) as usize).copied())
    }

    pub fn nb_v_knot_multiplicities(&self) -> i32 {
        self.v_knot_multiplicities.as_ref().map_or(0, |m| m.len() as i32)
    }

    pub fn set_u_knots(&mut self, knots: Vec<f64>) {
        self.u_knots = Some(knots);
    }

    pub fn u_knots(&self) -> Option<Vec<f64>> {
        self.u_knots.clone()
    }

    pub fn u_knots_value(&self, num: i32) -> Option<f64> {
        self.u_knots
            .as_ref()
            .and_then(|k| k.get((num - 1) as usize).copied())
    }

    pub fn nb_u_knots(&self) -> i32 {
        self.u_knots.as_ref().map_or(0, |k| k.len() as i32)
    }

    pub fn set_v_knots(&mut self, knots: Vec<f64>) {
        self.v_knots = Some(knots);
    }

    pub fn v_knots(&self) -> Option<Vec<f64>> {
        self.v_knots.clone()
    }

    pub fn v_knots_value(&self, num: i32) -> Option<f64> {
        self.v_knots
            .as_ref()
            .and_then(|k| k.get((num - 1) as usize).copied())
    }

    pub fn nb_v_knots(&self) -> i32 {
        self.v_knots.as_ref().map_or(0, |k| k.len() as i32)
    }

    pub fn set_knot_spec(&mut self, spec: KnotType) {
        self.knot_spec = spec;
    }

    pub fn knot_spec(&self) -> KnotType {
        self.knot_spec
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for BSplineSurfaceWithKnots {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let surf = BSplineSurfaceWithKnots::new();
        assert_eq!(surf.u_degree, 0);
        assert_eq!(surf.knot_spec(), KnotType::Unspecified);
    }

    #[test]
    fn test_knots() {
        let mut surf = BSplineSurfaceWithKnots::new();
        surf.set_u_knots(vec![0.0, 0.5, 1.0]);
        assert_eq!(surf.nb_u_knots(), 3);
        assert_eq!(surf.u_knots_value(2), Some(0.5));
    }
}
