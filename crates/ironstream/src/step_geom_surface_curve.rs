// FILE: step_geom_surface_curve.rs
// occt: StepGeom_SurfaceCurve

/// Represents a curve on a surface
pub struct StepGeomSurfaceCurve {
    name: String,
    curve_3d_id: i32,
    surface_id: i32,
    /// Optional parametric curve
    pcurve_id: Option<i32>,
}

impl StepGeomSurfaceCurve {
    pub fn new(name: String, curve_3d_id: i32, surface_id: i32) -> Self {
        StepGeomSurfaceCurve {
            name,
            curve_3d_id,
            surface_id,
            pcurve_id: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn curve_3d_id(&self) -> i32 {
        self.curve_3d_id
    }

    pub fn surface_id(&self) -> i32 {
        self.surface_id
    }

    pub fn pcurve_id(&self) -> Option<i32> {
        self.pcurve_id
    }

    pub fn set_pcurve_id(&mut self, pcurve_id: i32) {
        self.pcurve_id = Some(pcurve_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_surface_curve() {
        let curve = StepGeomSurfaceCurve::new("SurfaceCurve1".to_string(), 1, 2);
        assert_eq!(curve.name(), "SurfaceCurve1");
        assert_eq!(curve.curve_3d_id(), 1);
        assert_eq!(curve.surface_id(), 2);
        assert!(curve.pcurve_id().is_none());
    }

    #[test]
    fn test_set_pcurve_id() {
        let mut curve = StepGeomSurfaceCurve::new("SurfaceCurve1".to_string(), 1, 2);
        curve.set_pcurve_id(3);
        assert_eq!(curve.pcurve_id(), Some(3));
    }
}
