// FILE: step_geom_seam_curve.rs
// occt: StepGeom_SeamCurve

/// Represents a seam curve on a surface (edge where parameter wraps around)
pub struct StepGeomSeamCurve {
    name: String,
    surface_id: i32,
    pcurve_id: i32,
}

impl StepGeomSeamCurve {
    pub fn new(name: String, surface_id: i32, pcurve_id: i32) -> Self {
        StepGeomSeamCurve {
            name,
            surface_id,
            pcurve_id,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn surface_id(&self) -> i32 {
        self.surface_id
    }

    pub fn pcurve_id(&self) -> i32 {
        self.pcurve_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_seam_curve() {
        let seam = StepGeomSeamCurve::new("Seam1".to_string(), 1, 2);
        assert_eq!(seam.name(), "Seam1");
        assert_eq!(seam.surface_id(), 1);
        assert_eq!(seam.pcurve_id(), 2);
    }
}
