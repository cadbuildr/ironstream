// FILE: geom_vector_o.rs
// occt: Geom_Vector

//! Abstract class describing the common behavior of vectors in 3D space.
//! The Geom package provides two concrete classes: Geom_Direction (unit vector)
//! and Geom_VectorWithMagnitude.

pub trait Vector {
    /// Returns the X coordinate.
    fn x(&self) -> f64;

    /// Returns the Y coordinate.
    fn y(&self) -> f64;

    /// Returns the Z coordinate.
    fn z(&self) -> f64;

    /// Returns the magnitude of the vector.
    fn magnitude(&self) -> f64;

    /// Returns the square of the magnitude.
    fn square_magnitude(&self) -> f64;

    /// Get coordinates as a tuple.
    fn coord(&self) -> (f64, f64, f64) {
        (self.x(), self.y(), self.z())
    }
}

pub struct VectorImpl {
    x: f64,
    y: f64,
    z: f64,
}

impl VectorImpl {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VectorImpl { x, y, z }
    }

    pub fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn reversed(&self) -> Self {
        VectorImpl {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// Compute dot product with another vector.
    pub fn dot(&self, other: &VectorImpl) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Compute cross product with another vector.
    pub fn cross(&self, other: &VectorImpl) -> VectorImpl {
        VectorImpl {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Compute angle in radians with another vector (0 to PI).
    pub fn angle(&self, other: &VectorImpl) -> f64 {
        let mag_self = self.magnitude();
        let mag_other = other.magnitude();
        if mag_self < 1e-12 || mag_other < 1e-12 {
            0.0
        } else {
            let cos_angle = self.dot(other) / (mag_self * mag_other);
            cos_angle.clamp(-1.0, 1.0).acos()
        }
    }
}

impl Vector for VectorImpl {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn square_magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_construction() {
        let vec = VectorImpl::new(3.0, 4.0, 0.0);
        assert_eq!(vec.x(), 3.0);
        assert_eq!(vec.y(), 4.0);
        assert_eq!(vec.z(), 0.0);
    }

    #[test]
    fn test_magnitude() {
        let vec = VectorImpl::new(3.0, 4.0, 0.0);
        assert_eq!(vec.magnitude(), 5.0);
    }

    #[test]
    fn test_square_magnitude() {
        let vec = VectorImpl::new(3.0, 4.0, 0.0);
        assert_eq!(vec.square_magnitude(), 25.0);
    }

    #[test]
    fn test_reverse() {
        let mut vec = VectorImpl::new(1.0, 2.0, 3.0);
        vec.reverse();
        assert_eq!((vec.x(), vec.y(), vec.z()), (-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_reversed() {
        let vec = VectorImpl::new(1.0, 2.0, 3.0);
        let rev = vec.reversed();
        assert_eq!((rev.x(), rev.y(), rev.z()), (-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_dot_product() {
        let v1 = VectorImpl::new(1.0, 2.0, 3.0);
        let v2 = VectorImpl::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0); // 1*4 + 2*5 + 3*6
    }

    #[test]
    fn test_cross_product() {
        let v1 = VectorImpl::new(1.0, 0.0, 0.0);
        let v2 = VectorImpl::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);
        assert_eq!((cross.x(), cross.y(), cross.z()), (0.0, 0.0, 1.0));
    }

    #[test]
    fn test_angle_parallel() {
        let v1 = VectorImpl::new(1.0, 0.0, 0.0);
        let v2 = VectorImpl::new(2.0, 0.0, 0.0);
        assert!(v1.angle(&v2).abs() < 1e-10);
    }

    #[test]
    fn test_angle_perpendicular() {
        let v1 = VectorImpl::new(1.0, 0.0, 0.0);
        let v2 = VectorImpl::new(0.0, 1.0, 0.0);
        assert!((v1.angle(&v2) - std::f64::consts::PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_coord() {
        let vec = VectorImpl::new(1.5, 2.5, 3.5);
        assert_eq!(vec.coord(), (1.5, 2.5, 3.5));
    }
}
