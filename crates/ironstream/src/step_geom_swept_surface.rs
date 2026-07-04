// FILE: step_geom_swept_surface.rs
// occt: StepGeom_SweptSurface

/// Represents a surface created by sweeping a curve
pub struct StepGeomSweptSurface {
    name: String,
    /// The curve being swept
    swept_curve_id: i32,
}

impl StepGeomSweptSurface {
    pub fn new(name: String, swept_curve_id: i32) -> Self {
        StepGeomSweptSurface {
            name,
            swept_curve_id,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn swept_curve_id(&self) -> i32 {
        self.swept_curve_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_swept_surface() {
        let surface = StepGeomSweptSurface::new("SweptSurface1".to_string(), 1);
        assert_eq!(surface.name(), "SweptSurface1");
        assert_eq!(surface.swept_curve_id(), 1);
    }
}
