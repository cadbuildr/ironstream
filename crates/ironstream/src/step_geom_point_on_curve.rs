// FILE: step_geom_point_on_curve.rs
// occt: StepGeom_PointOnCurve

/// Represents a point on a curve
pub struct StepGeomPointOnCurve {
    name: String,
    curve_id: i32,
    parameter: f64,
}

impl StepGeomPointOnCurve {
    pub fn new(name: String, curve_id: i32, parameter: f64) -> Self {
        StepGeomPointOnCurve {
            name,
            curve_id,
            parameter,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn curve_id(&self) -> i32 {
        self.curve_id
    }

    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    pub fn set_parameter(&mut self, parameter: f64) {
        self.parameter = parameter;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_point_on_curve() {
        let point = StepGeomPointOnCurve::new("PointOnCurve1".to_string(), 1, 0.5);
        assert_eq!(point.name(), "PointOnCurve1");
        assert_eq!(point.curve_id(), 1);
        assert_eq!(point.parameter(), 0.5);
    }

    #[test]
    fn test_set_parameter() {
        let mut point = StepGeomPointOnCurve::new("PointOnCurve1".to_string(), 1, 0.5);
        point.set_parameter(0.75);
        assert_eq!(point.parameter(), 0.75);
    }
}
