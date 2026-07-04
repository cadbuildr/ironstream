// FILE: step_geom_bezier_surface_and_rational_b_spline_surface.rs
// occt: StepGeom_BezierSurfaceAndRationalBSplineSurface

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CartesianPoint {
    name: String,
}

#[derive(Clone)]
pub struct BezierSurface;

#[derive(Clone)]
pub struct RationalBSplineSurface;

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
pub struct BezierSurfaceAndRationalBSplineSurface {
    name: Arc<String>,
    u_degree: i32,
    v_degree: i32,
    control_points_list: Option<Vec<Vec<Arc<Mutex<CartesianPoint>>>>>,
    surface_form: BSplineSurfaceForm,
    u_closed: StepDataLogical,
    v_closed: StepDataLogical,
    self_intersect: StepDataLogical,
    bezier_surface: Option<Arc<Mutex<BezierSurface>>>,
    rational_bspline_surface: Option<Arc<Mutex<RationalBSplineSurface>>>,
    weights_data: Option<Vec<Vec<f64>>>,
}

impl BezierSurfaceAndRationalBSplineSurface {
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
            bezier_surface: None,
            rational_bspline_surface: None,
            weights_data: None,
        }
    }

    pub fn init_with_surfaces(
        &mut self,
        name: String,
        u_degree: i32,
        v_degree: i32,
        control_points_list: Option<Vec<Vec<Arc<Mutex<CartesianPoint>>>>>,
        surface_form: BSplineSurfaceForm,
        u_closed: StepDataLogical,
        v_closed: StepDataLogical,
        self_intersect: StepDataLogical,
        bezier_surface: Option<Arc<Mutex<BezierSurface>>>,
        rational_bspline_surface: Option<Arc<Mutex<RationalBSplineSurface>>>,
    ) {
        self.name = Arc::new(name);
        self.u_degree = u_degree;
        self.v_degree = v_degree;
        self.control_points_list = control_points_list;
        self.surface_form = surface_form;
        self.u_closed = u_closed;
        self.v_closed = v_closed;
        self.self_intersect = self_intersect;
        self.bezier_surface = bezier_surface;
        self.rational_bspline_surface = rational_bspline_surface;
    }

    pub fn init_with_weights(
        &mut self,
        name: String,
        u_degree: i32,
        v_degree: i32,
        control_points_list: Option<Vec<Vec<Arc<Mutex<CartesianPoint>>>>>,
        surface_form: BSplineSurfaceForm,
        u_closed: StepDataLogical,
        v_closed: StepDataLogical,
        self_intersect: StepDataLogical,
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
        self.weights_data = weights_data;
    }

    pub fn set_bezier_surface(&mut self, surf: Arc<Mutex<BezierSurface>>) {
        self.bezier_surface = Some(surf);
    }

    pub fn bezier_surface(&self) -> Option<Arc<Mutex<BezierSurface>>> {
        self.bezier_surface.clone()
    }

    pub fn set_rational_bspline_surface(&mut self, surf: Arc<Mutex<RationalBSplineSurface>>) {
        self.rational_bspline_surface = Some(surf);
    }

    pub fn rational_bspline_surface(&self) -> Option<Arc<Mutex<RationalBSplineSurface>>> {
        self.rational_bspline_surface.clone()
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

impl Default for BezierSurfaceAndRationalBSplineSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let surf = BezierSurfaceAndRationalBSplineSurface::new();
        assert_eq!(surf.u_degree(), 0);
    }

    #[test]
    fn test_weights() {
        let mut surf = BezierSurfaceAndRationalBSplineSurface::new();
        surf.set_weights_data(vec![vec![1.0, 0.5], vec![0.5, 1.0]]);
        assert_eq!(surf.nb_weights_data_i(), 2);
        assert_eq!(surf.weights_data_value(1, 1), Some(1.0));
    }
}
