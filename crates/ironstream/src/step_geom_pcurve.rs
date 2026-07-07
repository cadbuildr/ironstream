// FILE: step_geom_pcurve.rs
// occt: StepGeom_Pcurve

/// Parametric curve (P-curve) in STEP format.
/// A parametric curve is a curve defined in the parametric space of a surface.
pub struct StepGeomPcurve {
    id: i32,
    surface_id: i32,
}

impl StepGeomPcurve {
    pub fn new(id: i32, surface_id: i32) -> Self {
        StepGeomPcurve { id, surface_id }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn surface_id(&self) -> i32 {
        self.surface_id
    }

    pub fn set_surface_id(&mut self, surface_id: i32) {
        self.surface_id = surface_id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pcurve() {
        let pcurve = StepGeomPcurve::new(1, 10);
        assert_eq!(pcurve.id(), 1);
        assert_eq!(pcurve.surface_id(), 10);
    }

    #[test]
    fn test_set_surface_id() {
        let mut pcurve = StepGeomPcurve::new(1, 10);
        pcurve.set_surface_id(20);
        assert_eq!(pcurve.surface_id(), 20);
    }
}
