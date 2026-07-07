// FILE: step_geom_su_parameters.rs
// occt: StepGeom_SuParameters

/// Represents surface parameters for U direction
pub struct StepGeomSuParameters {
    /// U parameter value
    u: f64,
    /// Surface ID
    surface_id: i32,
}

impl StepGeomSuParameters {
    pub fn new(u: f64, surface_id: i32) -> Self {
        StepGeomSuParameters { u, surface_id }
    }

    pub fn u(&self) -> f64 {
        self.u
    }

    pub fn surface_id(&self) -> i32 {
        self.surface_id
    }

    pub fn set_u(&mut self, u: f64) {
        self.u = u;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_parameters() {
        let params = StepGeomSuParameters::new(0.5, 1);
        assert_eq!(params.u(), 0.5);
        assert_eq!(params.surface_id(), 1);
    }

    #[test]
    fn test_set_u() {
        let mut params = StepGeomSuParameters::new(0.5, 1);
        params.set_u(0.75);
        assert_eq!(params.u(), 0.75);
    }
}
