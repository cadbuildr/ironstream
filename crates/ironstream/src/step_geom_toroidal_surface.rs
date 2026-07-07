// FILE: step_geom_toroidal_surface.rs
// occt: StepGeom_ToroidalSurface

/// Represents a toroidal surface (torus)
pub struct StepGeomToroidalSurface {
    name: String,
    /// Position/placement
    position_id: i32,
    /// Major radius
    major_radius: f64,
    /// Minor radius
    minor_radius: f64,
}

impl StepGeomToroidalSurface {
    pub fn new(
        name: String,
        position_id: i32,
        major_radius: f64,
        minor_radius: f64,
    ) -> Self {
        StepGeomToroidalSurface {
            name,
            position_id,
            major_radius,
            minor_radius,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn position_id(&self) -> i32 {
        self.position_id
    }

    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    pub fn set_major_radius(&mut self, radius: f64) {
        self.major_radius = radius;
    }

    pub fn set_minor_radius(&mut self, radius: f64) {
        self.minor_radius = radius;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_torus() {
        let torus = StepGeomToroidalSurface::new("Torus1".to_string(), 1, 10.0, 2.0);
        assert_eq!(torus.name(), "Torus1");
        assert_eq!(torus.major_radius(), 10.0);
        assert_eq!(torus.minor_radius(), 2.0);
    }

    #[test]
    fn test_set_radii() {
        let mut torus = StepGeomToroidalSurface::new("Torus1".to_string(), 1, 10.0, 2.0);
        torus.set_major_radius(15.0);
        torus.set_minor_radius(3.0);
        assert_eq!(torus.major_radius(), 15.0);
        assert_eq!(torus.minor_radius(), 3.0);
    }
}
