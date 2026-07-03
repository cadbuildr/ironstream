// FILE: v3d_plane.rs
// occt: V3d_Plane

//! A plane in 3D space used for view operations and clipping.

#[derive(Clone, Debug)]
pub struct Plane {
    pub plane_id: u32,
    pub normal: [f64; 3],
    pub position: [f64; 3],
    pub is_active: bool,
}

impl Plane {
    pub fn new(plane_id: u32, normal: [f64; 3], position: [f64; 3]) -> Self {
        let len = (normal[0]*normal[0] + normal[1]*normal[1] + normal[2]*normal[2]).sqrt();
        let n = if len > 1e-14 {
            [normal[0]/len, normal[1]/len, normal[2]/len]
        } else {
            [0.0, 0.0, 1.0]
        };
        Plane {
            plane_id,
            normal: n,
            position,
            is_active: false,
        }
    }

    pub fn with_activity(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn distance_to_point(&self, p: [f64; 3]) -> f64 {
        let dx = p[0] - self.position[0];
        let dy = p[1] - self.position[1];
        let dz = p[2] - self.position[2];
        (dx * self.normal[0] + dy * self.normal[1] + dz * self.normal[2]).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_creation() {
        let plane = Plane::new(1, [0.0, 0.0, 1.0], [0.0, 0.0, 0.0]);
        assert_eq!(plane.plane_id, 1);
        assert!(!plane.is_active);
        assert!((plane.normal[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn plane_normalization() {
        let plane = Plane::new(1, [0.0, 0.0, 5.0], [0.0, 0.0, 0.0]);
        assert!((plane.normal[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn plane_distance() {
        let plane = Plane::new(1, [0.0, 0.0, 1.0], [0.0, 0.0, 5.0]);
        let dist = plane.distance_to_point([0.0, 0.0, 8.0]);
        assert!((dist - 3.0).abs() < 1e-10);
    }

    #[test]
    fn plane_activity() {
        let mut plane = Plane::new(1, [0.0, 0.0, 1.0], [0.0, 0.0, 0.0]);
        plane.activate();
        assert!(plane.is_active);
        plane.deactivate();
        assert!(!plane.is_active);
    }
}
