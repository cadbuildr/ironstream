// FILE: proj_lib_torus.rs
// occt: ProjLib_Torus

/// Torus projection.
pub struct ProjLibTorus {
    major_radius: f64,
    minor_radius: f64,
    center: (f64, f64, f64),
}

impl ProjLibTorus {
    /// Create a torus projector.
    pub fn new(center: (f64, f64, f64), major_radius: f64, minor_radius: f64) -> Self {
        ProjLibTorus {
            major_radius,
            minor_radius,
            center,
        }
    }

    /// Get the major radius.
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// Get the minor radius.
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    /// Get the center.
    pub fn center(&self) -> (f64, f64, f64) {
        self.center
    }

    /// Project a point onto the torus.
    pub fn project_point(&self, p: (f64, f64, f64)) -> (f64, f64, f64) {
        let dx = p.0 - self.center.0;
        let dy = p.1 - self.center.1;
        let dz = p.2 - self.center.2;

        // Distance in XY plane
        let r_xy = (dx * dx + dy * dy).sqrt();
        if r_xy < 1e-10 {
            // On the axis
            return (
                self.center.0 + self.major_radius,
                self.center.1,
                p.2,
            );
        }

        // Normalize XY components
        let ux = dx / r_xy;
        let uy = dy / r_xy;

        // Project onto torus
        let dist_z = dz;
        let dist_r = r_xy - self.major_radius;

        let scale = (dist_r * dist_r + dist_z * dist_z).sqrt();
        if scale < 1e-10 {
            return (
                self.center.0 + self.major_radius * ux,
                self.center.1 + self.major_radius * uy,
                self.center.2,
            );
        }

        let scale_r = self.minor_radius / scale;
        (
            self.center.0 + (self.major_radius + scale_r * dist_r) * ux,
            self.center.1 + (self.major_radius + scale_r * dist_r) * uy,
            self.center.2 + scale_r * dist_z,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_torus() {
        let torus = ProjLibTorus::new((0.0, 0.0, 0.0), 5.0, 2.0);
        assert_eq!(torus.major_radius(), 5.0);
        assert_eq!(torus.minor_radius(), 2.0);
        assert_eq!(torus.center(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_project_point_on_major_circle() {
        let torus = ProjLibTorus::new((0.0, 0.0, 0.0), 5.0, 2.0);
        let p = (10.0, 0.0, 0.0); // On major circle
        let proj_p = torus.project_point(p);
        assert!((proj_p.0 - 7.0).abs() < 0.1);
        assert!((proj_p.1 - 0.0).abs() < 1e-6);
    }
}
