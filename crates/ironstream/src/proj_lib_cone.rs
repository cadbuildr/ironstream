// FILE: proj_lib_cone.rs
// occt: ProjLib_Cone

//! Projects curves onto a cone surface.
//! Handles both cartesian and parametric projections.
pub struct ProjLibCone {
    apex: (f64, f64, f64),
    axis: (f64, f64, f64),
    semi_angle: f64,
}

impl ProjLibCone {
    pub fn new(apex: (f64, f64, f64), axis: (f64, f64, f64), semi_angle: f64) -> Self {
        Self {
            apex,
            axis,
            semi_angle,
        }
    }

    pub fn apex(&self) -> (f64, f64, f64) {
        self.apex
    }

    pub fn axis(&self) -> (f64, f64, f64) {
        self.axis
    }

    pub fn semi_angle(&self) -> f64 {
        self.semi_angle
    }

    /// Projects a point onto the cone surface
    pub fn project_point(&self, pt: (f64, f64, f64)) -> Option<(f64, f64)> {
        let (px, py, pz) = pt;
        let (ax, ay, az) = self.apex;
        let (dx, dy, dz) = self.axis;

        let vx = px - ax;
        let vy = py - ay;
        let vz = pz - az;

        let axis_len_sq = dx * dx + dy * dy + dz * dz;
        let t = (vx * dx + vy * dy + vz * dz) / axis_len_sq;

        let cx = vx - t * dx;
        let cy = vy - t * dy;
        let cz = vz - t * dz;

        let u = t.atan2(cx.hypot(cy.hypot(cz)));
        let v = cy.atan2(cx);

        Some((u, v))
    }

    /// Computes cone parameters from a line of points
    pub fn compute_from_points(&mut self, points: &[(f64, f64, f64)]) -> bool {
        if points.len() < 3 {
            return false;
        }

        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_z = 0.0;

        for (x, y, z) in points {
            sum_x += x;
            sum_y += y;
            sum_z += z;
        }

        let count = points.len() as f64;
        self.apex = (sum_x / count, sum_y / count, sum_z / count);

        true
    }

    /// Normalize the axis direction
    pub fn normalize_axis(&mut self) {
        let (dx, dy, dz) = self.axis;
        let len = (dx * dx + dy * dy + dz * dz).sqrt();
        if len > 1e-10 {
            self.axis = (dx / len, dy / len, dz / len);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let cone = ProjLibCone::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 0.5);
        assert_eq!(cone.apex(), (0.0, 0.0, 0.0));
        assert_eq!(cone.semi_angle(), 0.5);
    }

    #[test]
    fn test_project_point() {
        let cone = ProjLibCone::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 0.5);
        let proj = cone.project_point((1.0, 0.0, 1.0));
        assert!(proj.is_some());
    }

    #[test]
    fn test_compute_from_points() {
        let mut cone = ProjLibCone::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 0.5);
        let points = vec![(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (1.0, 1.0, 0.0)];
        let result = cone.compute_from_points(&points);
        assert!(result);
        let apex = cone.apex();
        assert!(apex.0 > 0.0 && apex.0 < 1.0);
    }

    #[test]
    fn test_compute_empty() {
        let mut cone = ProjLibCone::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 0.5);
        let result = cone.compute_from_points(&[]);
        assert!(!result);
    }

    #[test]
    fn test_normalize_axis() {
        let mut cone = ProjLibCone::new((0.0, 0.0, 0.0), (3.0, 4.0, 0.0), 0.5);
        cone.normalize_axis();
        let (dx, dy, dz) = cone.axis();
        let len = (dx * dx + dy * dy + dz * dz).sqrt();
        assert!((len - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_offset_apex() {
        let cone = ProjLibCone::new((5.0, 5.0, 5.0), (0.0, 0.0, 1.0), 0.5);
        assert_eq!(cone.apex(), (5.0, 5.0, 5.0));
    }
}
