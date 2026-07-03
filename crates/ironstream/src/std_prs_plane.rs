// FILE: std_prs_plane.rs
// occt: StdPrs_Plane

/// Plane presentation.
#[derive(Clone, Debug)]
pub struct Plane {
    pub plane_id: u32,
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub size: f64,
}

impl Plane {
    pub fn new(plane_id: u32, origin: [f64; 3], normal: [f64; 3], size: f64) -> Self {
        let len = (normal[0]*normal[0] + normal[1]*normal[1] + normal[2]*normal[2]).sqrt();
        let n = if len > 1e-14 {
            [normal[0]/len, normal[1]/len, normal[2]/len]
        } else {
            [0.0, 0.0, 1.0]
        };
        Self {
            plane_id,
            origin,
            normal: n,
            size: size.max(0.1),
        }
    }

    pub fn set_size(&mut self, size: f64) {
        self.size = size.max(0.1);
    }

    pub fn set_origin(&mut self, origin: [f64; 3]) {
        self.origin = origin;
    }

    pub fn size(&self) -> f64 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_plane() {
        let plane = Plane::new(1, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 10.0);
        assert_eq!(plane.plane_id, 1);
        assert_eq!(plane.size(), 10.0);
    }

    #[test]
    fn normalizes_normal() {
        let plane = Plane::new(1, [0.0, 0.0, 0.0], [0.0, 0.0, 5.0], 10.0);
        let n = plane.normal;
        assert!((n[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn set_size() {
        let mut plane = Plane::new(1, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 10.0);
        plane.set_size(20.0);
        assert_eq!(plane.size(), 20.0);
    }
}
