// FILE: proj_lib_project_on_plane.rs
// occt: ProjLib_ProjectOnPlane

/// Project curves onto a plane.
pub struct ProjLibProjectOnPlane {
    plane_normal: (f64, f64, f64),
    plane_position: (f64, f64, f64),
}

impl ProjLibProjectOnPlane {
    /// Create a plane projection tool.
    pub fn new(normal: (f64, f64, f64), position: (f64, f64, f64)) -> Self {
        ProjLibProjectOnPlane {
            plane_normal: normal,
            plane_position: position,
        }
    }

    /// Get the plane normal.
    pub fn plane_normal(&self) -> (f64, f64, f64) {
        self.plane_normal
    }

    /// Get the plane position.
    pub fn plane_position(&self) -> (f64, f64, f64) {
        self.plane_position
    }

    /// Project a point onto the plane.
    pub fn project_point(&self, p: (f64, f64, f64)) -> (f64, f64, f64) {
        let dx = p.0 - self.plane_position.0;
        let dy = p.1 - self.plane_position.1;
        let dz = p.2 - self.plane_position.2;

        let dot = dx * self.plane_normal.0 + dy * self.plane_normal.1 + dz * self.plane_normal.2;

        (
            p.0 - dot * self.plane_normal.0,
            p.1 - dot * self.plane_normal.1,
            p.2 - dot * self.plane_normal.2,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_projector() {
        let proj =
            ProjLibProjectOnPlane::new((0.0, 0.0, 1.0), (0.0, 0.0, 0.0));
        assert_eq!(proj.plane_normal(), (0.0, 0.0, 1.0));
        assert_eq!(proj.plane_position(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_project_point_on_xy_plane() {
        let proj = ProjLibProjectOnPlane::new((0.0, 0.0, 1.0), (0.0, 0.0, 0.0));
        let p = (1.0, 2.0, 5.0);
        let proj_p = proj.project_point(p);
        assert!((proj_p.0 - 1.0).abs() < 1e-6);
        assert!((proj_p.1 - 2.0).abs() < 1e-6);
        assert!((proj_p.2 - 0.0).abs() < 1e-6);
    }
}
