// FILE: step_geom_spherical_surface.rs
// occt: StepGeom_SphericalSurface

/// Represents a spherical surface
pub struct StepGeomSphericalSurface {
    name: String,
    /// Position/placement
    position_id: i32,
    /// Radius
    radius: f64,
}

impl StepGeomSphericalSurface {
    pub fn new(name: String, position_id: i32, radius: f64) -> Self {
        StepGeomSphericalSurface {
            name,
            position_id,
            radius,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn position_id(&self) -> i32 {
        self.position_id
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sphere() {
        let sphere = StepGeomSphericalSurface::new("Sphere1".to_string(), 1, 5.0);
        assert_eq!(sphere.name(), "Sphere1");
        assert_eq!(sphere.radius(), 5.0);
    }

    #[test]
    fn test_set_radius() {
        let mut sphere = StepGeomSphericalSurface::new("Sphere1".to_string(), 1, 5.0);
        sphere.set_radius(10.0);
        assert_eq!(sphere.radius(), 10.0);
    }
}
