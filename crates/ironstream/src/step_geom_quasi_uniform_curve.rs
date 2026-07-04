// FILE: step_geom_quasi_uniform_curve.rs
// occt: StepGeom_QuasiUniformCurve

/// Represents a quasi-uniform B-spline curve
pub struct StepGeomQuasiUniformCurve {
    name: String,
    degree: i32,
    /// Number of control points
    nb_control_points: i32,
}

impl StepGeomQuasiUniformCurve {
    pub fn new(name: String, degree: i32, nb_control_points: i32) -> Self {
        StepGeomQuasiUniformCurve {
            name,
            degree,
            nb_control_points,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn degree(&self) -> i32 {
        self.degree
    }

    pub fn nb_control_points(&self) -> i32 {
        self.nb_control_points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_curve() {
        let curve = StepGeomQuasiUniformCurve::new("Curve1".to_string(), 3, 10);
        assert_eq!(curve.name(), "Curve1");
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.nb_control_points(), 10);
    }
}
