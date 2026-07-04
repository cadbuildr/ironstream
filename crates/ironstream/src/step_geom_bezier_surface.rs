// FILE: step_geom_bezier_surface.rs
// occt: StepGeom_BezierSurface

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BSplineSurfaceForm {
    PlaneSurface,
    CylindricalSurface,
    Unspecified,
}

#[derive(Clone)]
pub struct BezierSurface {
    name: Arc<String>,
    u_degree: i32,
    v_degree: i32,
    control_points_list: Option<Vec<Vec<Arc<Mutex<CartesianPoint>>>>>,
    surface_form: BSplineSurfaceForm,
    u_closed: StepDataLogical,
    v_closed: StepDataLogical,
    self_intersect: StepDataLogical,
}

impl BezierSurface {
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
    ) {
        self.name = Arc::new(name);
        self.u_degree = u_degree;
        self.v_degree = v_degree;
        self.control_points_list = control_points_list;
        self.surface_form = surface_form;
        self.u_closed = u_closed;
        self.v_closed = v_closed;
        self.self_intersect = self_intersect;
    }

    pub fn set_u_degree(&mut self, degree: i32) {
        self.u_degree = degree;
    }

    pub fn u_degree(&self) -> i32 {
        self.u_degree
    }

    pub fn set_v_degree(&mut self, degree: i32) {
        self.v_degree = degree;
    }

    pub fn v_degree(&self) -> i32 {
        self.v_degree
    }

    pub fn set_control_points_list(
        &mut self,
        points: Vec<Vec<Arc<Mutex<CartesianPoint>>>>,
    ) {
        self.control_points_list = Some(points);
    }

    pub fn control_points_list(
        &self,
    ) -> Option<Vec<Vec<Arc<Mutex<CartesianPoint>>>>> {
        self.control_points_list.clone()
    }

    pub fn control_points_list_value(
        &self,
        num1: i32,
        num2: i32,
    ) -> Option<Arc<Mutex<CartesianPoint>>> {
        self.control_points_list.as_ref().and_then(|pts| {
            pts.get((num1 - 1) as usize)
                .and_then(|row| row.get((num2 - 1) as usize).cloned())
        })
    }

    pub fn nb_control_points_list_i(&self) -> i32 {
        self.control_points_list.as_ref().map_or(0, |pts| pts.len() as i32)
    }

    pub fn nb_control_points_list_j(&self) -> i32 {
        self.control_points_list
            .as_ref()
            .and_then(|pts| pts.first().map(|row| row.len() as i32))
            .unwrap_or(0)
    }

    pub fn set_surface_form(&mut self, form: BSplineSurfaceForm) {
        self.surface_form = form;
    }

    pub fn surface_form(&self) -> BSplineSurfaceForm {
        self.surface_form.clone()
    }

    pub fn set_u_closed(&mut self, closed: StepDataLogical) {
        self.u_closed = closed;
    }

    pub fn u_closed(&self) -> StepDataLogical {
        self.u_closed.clone()
    }

    pub fn set_v_closed(&mut self, closed: StepDataLogical) {
        self.v_closed = closed;
    }

    pub fn v_closed(&self) -> StepDataLogical {
        self.v_closed.clone()
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

impl Default for BezierSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let surf = BezierSurface::new();
        assert_eq!(surf.u_degree(), 0);
        assert_eq!(surf.v_degree(), 0);
    }

    #[test]
    fn test_init() {
        let mut surf = BezierSurface::new();
        surf.init(
            "bezier_surf".to_string(),
            3,
            3,
            None,
            BSplineSurfaceForm::PlaneSurface,
            StepDataLogical::False,
            StepDataLogical::False,
            StepDataLogical::False,
        );
        assert_eq!(surf.u_degree(), 3);
        assert_eq!(surf.v_degree(), 3);
    }
}
