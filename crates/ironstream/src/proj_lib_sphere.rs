// FILE: proj_lib_sphere.rs
// occt: ProjLib_Sphere

/// Sphere projection.
pub struct ProjLibSphere {
    radius: f64,
    center: (f64, f64, f64),
}

impl ProjLibSphere {
    /// Create a sphere projector.
    pub fn new(center: (f64, f64, f64), radius: f64) -> Self {
        ProjLibSphere { radius, center }
    }

    /// Get the radius.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Get the center.
    pub fn center(&self) -> (f64, f64, f64) {
        self.center
    }

    /// Project a point onto the sphere.
    pub fn project_point(&self, p: (f64, f64, f64)) -> (f64, f64, f64) {
        let dx = p.0 - self.center.0;
        let dy = p.1 - self.center.1;
        let dz = p.2 - self.center.2;

        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        if dist < 1e-10 {
            return self.center;
        }

        let scale = self.radius / dist;
        (
            self.center.0 + scale * dx,
            self.center.1 + scale * dy,
            self.center.2 + scale * dz,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sphere() {
        let sphere = ProjLibSphere::new((0.0, 0.0, 0.0), 1.0);
        assert_eq!(sphere.radius(), 1.0);
        assert_eq!(sphere.center(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_project_point() {
        let sphere = ProjLibSphere::new((0.0, 0.0, 0.0), 5.0);
        let p = (10.0, 0.0, 0.0);
        let proj_p = sphere.project_point(p);
        assert!((proj_p.0 - 5.0).abs() < 1e-6);
        assert!((proj_p.1 - 0.0).abs() < 1e-6);
        assert!((proj_p.2 - 0.0).abs() < 1e-6);
    }
}
