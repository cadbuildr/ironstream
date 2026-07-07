// FILE: step_geom_surface_curve_and_bounded_curve.rs
// occt: StepGeom_SurfaceCurveAndBoundedCurve

/// Represents a curve that is both a surface curve and a bounded curve
pub struct StepGeomSurfaceCurveAndBoundedCurve {
    name: String,
    surface_id: i32,
    /// Is bounded (has defined start and end)
    is_bounded: bool,
}

impl StepGeomSurfaceCurveAndBoundedCurve {
    pub fn new(name: String, surface_id: i32) -> Self {
        StepGeomSurfaceCurveAndBoundedCurve {
            name,
            surface_id,
            is_bounded: true,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn surface_id(&self) -> i32 {
        self.surface_id
    }

    pub fn is_bounded(&self) -> bool {
        self.is_bounded
    }

    pub fn set_bounded(&mut self, bounded: bool) {
        self.is_bounded = bounded;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_curve() {
        let curve = StepGeomSurfaceCurveAndBoundedCurve::new("Curve1".to_string(), 1);
        assert_eq!(curve.name(), "Curve1");
        assert_eq!(curve.surface_id(), 1);
        assert!(curve.is_bounded());
    }

    #[test]
    fn test_set_bounded() {
        let mut curve = StepGeomSurfaceCurveAndBoundedCurve::new("Curve1".to_string(), 1);
        curve.set_bounded(false);
        assert!(!curve.is_bounded());
    }
}
