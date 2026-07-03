// FILE: int_polyh_point.rs
// occt: IntPolyh_Point

/// A 3D point for polyhedron representation
#[derive(Clone, Copy, Debug)]
pub struct IntPolyhPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl IntPolyhPoint {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        IntPolyhPoint { x, y, z }
    }

    pub fn distance_to(&self, other: &IntPolyhPoint) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn dot_product(&self, other: &IntPolyhPoint) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(&self, other: &IntPolyhPoint) -> IntPolyhPoint {
        IntPolyhPoint {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Default for IntPolyhPoint {
    fn default() -> Self {
        IntPolyhPoint { x: 0.0, y: 0.0, z: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let pt = IntPolyhPoint::new(1.0, 2.0, 3.0);
        assert_eq!(pt.x, 1.0);
        assert_eq!(pt.y, 2.0);
        assert_eq!(pt.z, 3.0);
    }

    #[test]
    fn test_distance() {
        let pt1 = IntPolyhPoint::new(0.0, 0.0, 0.0);
        let pt2 = IntPolyhPoint::new(3.0, 4.0, 0.0);
        assert!((pt1.distance_to(&pt2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_dot_product() {
        let pt1 = IntPolyhPoint::new(1.0, 0.0, 0.0);
        let pt2 = IntPolyhPoint::new(2.0, 0.0, 0.0);
        assert_eq!(pt1.dot_product(&pt2), 2.0);
    }

    #[test]
    fn test_cross_product() {
        let pt1 = IntPolyhPoint::new(1.0, 0.0, 0.0);
        let pt2 = IntPolyhPoint::new(0.0, 1.0, 0.0);
        let cross = pt1.cross_product(&pt2);
        assert_eq!(cross.z, 1.0);
    }

    #[test]
    fn test_default() {
        let pt = IntPolyhPoint::default();
        assert_eq!(pt.x, 0.0);
        assert_eq!(pt.y, 0.0);
        assert_eq!(pt.z, 0.0);
    }
}
