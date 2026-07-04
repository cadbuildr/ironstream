// FILE: step_geom_quasi_uniform_surface_and_rational_b_spline_surface.rs
// occt: StepGeom_QuasiUniformSurfaceAndRationalBSplineSurface

/// Represents a quasi-uniform rational B-spline surface
pub struct StepGeomQuasiUniformSurfaceAndRationalBSplineSurface {
    name: String,
    u_degree: i32,
    v_degree: i32,
    nb_u_control_points: i32,
    nb_v_control_points: i32,
    /// Weights for the control points
    weights: Vec<f64>,
}

impl StepGeomQuasiUniformSurfaceAndRationalBSplineSurface {
    pub fn new(
        name: String,
        u_degree: i32,
        v_degree: i32,
        nb_u: i32,
        nb_v: i32,
    ) -> Self {
        let total_points = (nb_u * nb_v) as usize;
        StepGeomQuasiUniformSurfaceAndRationalBSplineSurface {
            name,
            u_degree,
            v_degree,
            nb_u_control_points: nb_u,
            nb_v_control_points: nb_v,
            weights: vec![1.0; total_points],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn u_degree(&self) -> i32 {
        self.u_degree
    }

    pub fn v_degree(&self) -> i32 {
        self.v_degree
    }

    pub fn nb_u_control_points(&self) -> i32 {
        self.nb_u_control_points
    }

    pub fn nb_v_control_points(&self) -> i32 {
        self.nb_v_control_points
    }

    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_surface() {
        let surface = StepGeomQuasiUniformSurfaceAndRationalBSplineSurface::new(
            "Surface1".to_string(),
            3,
            3,
            10,
            10,
        );
        assert_eq!(surface.name(), "Surface1");
        assert_eq!(surface.u_degree(), 3);
        assert_eq!(surface.v_degree(), 3);
        assert_eq!(surface.weights().len(), 100);
    }
}
