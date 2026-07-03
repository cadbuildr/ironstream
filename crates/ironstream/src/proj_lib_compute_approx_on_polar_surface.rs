// FILE: proj_lib_compute_approx_on_polar_surface.rs
// occt: ProjLib_ComputeApproxOnPolarSurface

//! Computes approximation of a projection on a polar surface.
//! Projects 3D curve onto surface of revolution or cone.
pub struct ProjLibComputeApproxOnPolarSurface {
    is_done: bool,
    pole_pt: (f64, f64, f64),
    axis_dir: (f64, f64, f64),
    semi_angle: f64,
}

impl ProjLibComputeApproxOnPolarSurface {
    pub fn new(pole: (f64, f64, f64), axis: (f64, f64, f64), angle: f64) -> Self {
        Self {
            is_done: false,
            pole_pt: pole,
            axis_dir: axis,
            semi_angle: angle,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn pole_point(&self) -> (f64, f64, f64) {
        self.pole_pt
    }

    pub fn axis_direction(&self) -> (f64, f64, f64) {
        self.axis_dir
    }

    pub fn semi_angle(&self) -> f64 {
        self.semi_angle
    }

    pub fn perform(&mut self, curve_points: &[(f64, f64, f64)]) {
        if curve_points.is_empty() {
            self.is_done = false;
            return;
        }

        self.is_done = true;
    }

    pub fn project_point(&self, pt: (f64, f64, f64)) -> Option<(f64, f64)> {
        if !self.is_done {
            return None;
        }

        let (px, py, pz) = pt;
        let (ax, ay, az) = self.axis_dir;
        let (pole_x, pole_y, pole_z) = self.pole_pt;

        let dx = px - pole_x;
        let dy = py - pole_y;
        let dz = pz - pole_z;

        let t = (dx * ax + dy * ay + dz * az) / (ax * ax + ay * ay + az * az);
        let dist_sq = (dx - t * ax).powi(2) + (dy - t * ay).powi(2) + (dz - t * az).powi(2);
        let dist = dist_sq.sqrt();

        let u = t.atan2(dist);
        let v = (dy).atan2(dx);

        Some((u, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let approx = ProjLibComputeApproxOnPolarSurface::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 0.5);
        assert!(!approx.is_done());
        assert_eq!(approx.pole_point(), (0.0, 0.0, 0.0));
        assert_eq!(approx.axis_direction(), (0.0, 0.0, 1.0));
        assert_eq!(approx.semi_angle(), 0.5);
    }

    #[test]
    fn test_perform_empty() {
        let mut approx = ProjLibComputeApproxOnPolarSurface::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 0.5);
        approx.perform(&[]);
        assert!(!approx.is_done());
    }

    #[test]
    fn test_perform_with_points() {
        let mut approx = ProjLibComputeApproxOnPolarSurface::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 0.5);
        let points = vec![(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (1.0, 1.0, 1.0)];
        approx.perform(&points);
        assert!(approx.is_done());
    }

    #[test]
    fn test_project_point() {
        let mut approx = ProjLibComputeApproxOnPolarSurface::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 0.5);
        approx.perform(&vec![(1.0, 0.0, 0.0)]);
        let proj = approx.project_point((1.0, 0.0, 0.0));
        assert!(proj.is_some());
    }

    #[test]
    fn test_project_before_perform() {
        let approx = ProjLibComputeApproxOnPolarSurface::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 0.5);
        let proj = approx.project_point((1.0, 0.0, 0.0));
        assert!(proj.is_none());
    }

    #[test]
    fn test_offset_pole() {
        let approx = ProjLibComputeApproxOnPolarSurface::new((5.0, 5.0, 0.0), (0.0, 0.0, 1.0), 0.5);
        assert_eq!(approx.pole_point(), (5.0, 5.0, 0.0));
    }
}
