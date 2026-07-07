// FILE: step_geom_b_spline_surface_with_knots_and_rational_b_spline_surface.rs
// occt: StepGeom_BSplineSurfaceWithKnotsAndRationalBSplineSurface

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
    Unspecified,
}

#[derive(Clone)]
pub struct BSplineSurfaceWithKnotsAndRationalBSplineSurface {
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
    weights_data: Option<Vec<Vec<f64>>>,
}

impl BSplineSurfaceWithKnotsAndRationalBSplineSurface {
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
            weights_data: None,
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
        weights_data: Option<Vec<Vec<f64>>>,
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
        self.weights_data = weights_data;
    }

    pub fn set_weights_data(&mut self, weights: Vec<Vec<f64>>) {
        self.weights_data = Some(weights);
    }

    pub fn weights_data(&self) -> Option<Vec<Vec<f64>>> {
        self.weights_data.clone()
    }

    pub fn weights_data_value(&self, num1: i32, num2: i32) -> Option<f64> {
        self.weights_data.as_ref().and_then(|w| {
            w.get((num1 - 1) as usize)
                .and_then(|row| row.get((num2 - 1) as usize).copied())
        })
    }

    pub fn nb_weights_data_i(&self) -> i32 {
        self.weights_data.as_ref().map_or(0, |w| w.len() as i32)
    }

    pub fn nb_weights_data_j(&self) -> i32 {
        self.weights_data
            .as_ref()
            .and_then(|w| w.first().map(|row| row.len() as i32))
            .unwrap_or(0)
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }

    pub fn u_degree(&self) -> i32 {
        self.u_degree
    }

    pub fn v_degree(&self) -> i32 {
        self.v_degree
    }
}

impl Default for BSplineSurfaceWithKnotsAndRationalBSplineSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let surf = BSplineSurfaceWithKnotsAndRationalBSplineSurface::new();
        assert_eq!(surf.u_degree(), 0);
        assert_eq!(surf.v_degree(), 0);
    }

    #[test]
    fn test_weights_data() {
        let mut surf = BSplineSurfaceWithKnotsAndRationalBSplineSurface::new();
        surf.set_weights_data(vec![vec![1.0, 0.5], vec![0.5, 1.0]]);
        assert_eq!(surf.nb_weights_data_i(), 2);
        assert_eq!(surf.nb_weights_data_j(), 2);
        assert_eq!(surf.weights_data_value(1, 2), Some(0.5));
    }
}
