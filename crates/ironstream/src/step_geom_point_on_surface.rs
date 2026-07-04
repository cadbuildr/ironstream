// FILE: step_geom_point_on_surface.rs
// occt: StepGeom_PointOnSurface

/// Represents a point on a surface
pub struct StepGeomPointOnSurface {
    name: String,
    surface_id: i32,
    u_param: f64,
    v_param: f64,
}

impl StepGeomPointOnSurface {
    pub fn new(name: String, surface_id: i32, u: f64, v: f64) -> Self {
        StepGeomPointOnSurface {
            name,
            surface_id,
            u_param: u,
            v_param: v,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn surface_id(&self) -> i32 {
        self.surface_id
    }

    pub fn u_param(&self) -> f64 {
        self.u_param
    }

    pub fn v_param(&self) -> f64 {
        self.v_param
    }

    pub fn set_params(&mut self, u: f64, v: f64) {
        self.u_param = u;
        self.v_param = v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_point_on_surface() {
        let point = StepGeomPointOnSurface::new("PointOnSurface1".to_string(), 1, 0.25, 0.75);
        assert_eq!(point.name(), "PointOnSurface1");
        assert_eq!(point.surface_id(), 1);
        assert_eq!(point.u_param(), 0.25);
        assert_eq!(point.v_param(), 0.75);
    }

    #[test]
    fn test_set_params() {
        let mut point = StepGeomPointOnSurface::new("PointOnSurface1".to_string(), 1, 0.25, 0.75);
        point.set_params(0.5, 0.5);
        assert_eq!(point.u_param(), 0.5);
        assert_eq!(point.v_param(), 0.5);
    }
}
