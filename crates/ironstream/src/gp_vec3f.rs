// FILE: gp_vec3f.rs
// occt: gp_Vec3f

/// 3D vector with single-precision (f32) components.
/// Represents a vector in 3D space (X, Y, Z).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GpVec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl GpVec3f {
    /// Create a new 3D vector with given components.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        GpVec3f { x, y, z }
    }

    /// Create a zero vector (0, 0, 0).
    pub fn zero() -> Self {
        GpVec3f { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Create a unit vector along X axis (1, 0, 0).
    pub fn unit_x() -> Self {
        GpVec3f { x: 1.0, y: 0.0, z: 0.0 }
    }

    /// Create a unit vector along Y axis (0, 1, 0).
    pub fn unit_y() -> Self {
        GpVec3f { x: 0.0, y: 1.0, z: 0.0 }
    }

    /// Create a unit vector along Z axis (0, 0, 1).
    pub fn unit_z() -> Self {
        GpVec3f { x: 0.0, y: 0.0, z: 1.0 }
    }

    /// Calculate the magnitude (length) of the vector.
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Calculate the squared magnitude (avoids sqrt, faster).
    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Return a normalized vector (magnitude = 1.0).
    /// Returns zero vector if magnitude is near-zero.
    pub fn normalized(&self) -> Self {
        let mag = self.magnitude();
        if mag < 1e-7 {
            GpVec3f::zero()
        } else {
            GpVec3f {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        }
    }

    /// Normalize this vector in-place.
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag >= 1e-7 {
            self.x /= mag;
            self.y /= mag;
            self.z /= mag;
        }
    }

    /// Calculate the dot product with another vector.
    pub fn dot(&self, other: &GpVec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculate the cross product.
    /// Result is the 3D cross product vector.
    pub fn cross(&self, other: &GpVec3f) -> GpVec3f {
        GpVec3f {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Add another vector to this one.
    pub fn add(&self, other: &GpVec3f) -> Self {
        GpVec3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// Subtract another vector from this one.
    pub fn subtract(&self, other: &GpVec3f) -> Self {
        GpVec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    /// Multiply this vector by a scalar.
    pub fn scale(&self, scalar: f32) -> Self {
        GpVec3f {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    /// Calculate distance between two points (represented as vectors).
    pub fn distance(&self, other: &GpVec3f) -> f32 {
        self.subtract(other).magnitude()
    }

    /// Check if this vector is approximately equal to another (with tolerance).
    pub fn is_equal(&self, other: &GpVec3f, tolerance: f32) -> bool {
        (self.x - other.x).abs() < tolerance
            && (self.y - other.y).abs() < tolerance
            && (self.z - other.z).abs() < tolerance
    }

    /// Calculate the angle between this vector and another (in radians).
    pub fn angle(&self, other: &GpVec3f) -> f32 {
        let dot = self.dot(other);
        let mag_product = self.magnitude() * other.magnitude();
        if mag_product < 1e-7 {
            0.0
        } else {
            let cos_angle = (dot / mag_product).clamp(-1.0, 1.0);
            cos_angle.acos()
        }
    }
}

impl std::ops::Add for GpVec3f {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        GpVec3f::add(&self, &other)
    }
}

impl std::ops::Sub for GpVec3f {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self.subtract(&other)
    }
}

impl std::ops::Mul<f32> for GpVec3f {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        self.scale(scalar)
    }
}

impl std::ops::Mul<GpVec3f> for f32 {
    type Output = GpVec3f;
    fn mul(self, vec: GpVec3f) -> GpVec3f {
        vec.scale(self)
    }
}

impl std::ops::Neg for GpVec3f {
    type Output = Self;
    fn neg(self) -> Self {
        GpVec3f {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = GpVec3f::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_zero() {
        let v = GpVec3f::zero();
        assert_eq!(v, GpVec3f::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_unit_x() {
        let v = GpVec3f::unit_x();
        assert_eq!(v, GpVec3f::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_unit_y() {
        let v = GpVec3f::unit_y();
        assert_eq!(v, GpVec3f::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_unit_z() {
        let v = GpVec3f::unit_z();
        assert_eq!(v, GpVec3f::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_magnitude_1_2_2() {
        let v = GpVec3f::new(1.0, 2.0, 2.0);
        assert!((v.magnitude() - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_magnitude_zero() {
        let v = GpVec3f::zero();
        assert!(v.magnitude() < 1e-6);
    }

    #[test]
    fn test_magnitude_squared() {
        let v = GpVec3f::new(1.0, 2.0, 2.0);
        assert_eq!(v.magnitude_squared(), 9.0);
    }

    #[test]
    fn test_normalized_1_2_2() {
        let v = GpVec3f::new(1.0, 2.0, 2.0);
        let n = v.normalized();
        assert!((n.magnitude() - 1.0).abs() < 0.001);
        assert!((n.x - 1.0/3.0).abs() < 0.001);
        assert!((n.y - 2.0/3.0).abs() < 0.001);
        assert!((n.z - 2.0/3.0).abs() < 0.001);
    }

    #[test]
    fn test_normalize_in_place() {
        let mut v = GpVec3f::new(1.0, 2.0, 2.0);
        v.normalize();
        assert!((v.magnitude() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_normalized_zero_vector() {
        let v = GpVec3f::zero();
        let n = v.normalized();
        assert_eq!(n, GpVec3f::zero());
    }

    #[test]
    fn test_dot_product_perpendicular() {
        let v1 = GpVec3f::unit_x();
        let v2 = GpVec3f::unit_y();
        assert!(v1.dot(&v2) < 1e-6);
    }

    #[test]
    fn test_dot_product_parallel() {
        let v1 = GpVec3f::new(2.0, 0.0, 0.0);
        let v2 = GpVec3f::new(3.0, 0.0, 0.0);
        assert_eq!(v1.dot(&v2), 6.0);
    }

    #[test]
    fn test_cross_product_x_cross_y() {
        let v1 = GpVec3f::unit_x();
        let v2 = GpVec3f::unit_y();
        let result = v1.cross(&v2);
        assert_eq!(result, GpVec3f::unit_z());
    }

    #[test]
    fn test_cross_product_y_cross_z() {
        let v1 = GpVec3f::unit_y();
        let v2 = GpVec3f::unit_z();
        let result = v1.cross(&v2);
        assert_eq!(result, GpVec3f::unit_x());
    }

    #[test]
    fn test_cross_product_z_cross_x() {
        let v1 = GpVec3f::unit_z();
        let v2 = GpVec3f::unit_x();
        let result = v1.cross(&v2);
        assert_eq!(result, GpVec3f::unit_y());
    }

    #[test]
    fn test_cross_product_parallel_zero() {
        let v1 = GpVec3f::new(2.0, 0.0, 0.0);
        let v2 = GpVec3f::new(3.0, 0.0, 0.0);
        let result = v1.cross(&v2);
        assert_eq!(result, GpVec3f::zero());
    }

    #[test]
    fn test_add() {
        let v1 = GpVec3f::new(1.0, 2.0, 3.0);
        let v2 = GpVec3f::new(4.0, 5.0, 6.0);
        let result = v1.add(&v2);
        assert_eq!(result, GpVec3f::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_subtract() {
        let v1 = GpVec3f::new(5.0, 6.0, 7.0);
        let v2 = GpVec3f::new(1.0, 2.0, 3.0);
        let result = v1.subtract(&v2);
        assert_eq!(result, GpVec3f::new(4.0, 4.0, 4.0));
    }

    #[test]
    fn test_scale() {
        let v = GpVec3f::new(2.0, 3.0, 4.0);
        let result = v.scale(2.0);
        assert_eq!(result, GpVec3f::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_distance() {
        let v1 = GpVec3f::new(0.0, 0.0, 0.0);
        let v2 = GpVec3f::new(1.0, 2.0, 2.0);
        assert!((v1.distance(&v2) - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_is_equal_true() {
        let v1 = GpVec3f::new(1.0, 2.0, 3.0);
        let v2 = GpVec3f::new(1.001, 2.001, 3.001);
        assert!(v1.is_equal(&v2, 0.01));
    }

    #[test]
    fn test_is_equal_false() {
        let v1 = GpVec3f::new(1.0, 2.0, 3.0);
        let v2 = GpVec3f::new(1.1, 2.0, 3.0);
        assert!(!v1.is_equal(&v2, 0.01));
    }

    #[test]
    fn test_angle_perpendicular() {
        let v1 = GpVec3f::unit_x();
        let v2 = GpVec3f::unit_y();
        let angle = v1.angle(&v2);
        assert!((angle - std::f32::consts::PI / 2.0).abs() < 0.001);
    }

    #[test]
    fn test_angle_parallel() {
        let v1 = GpVec3f::unit_x();
        let v2 = GpVec3f::unit_x();
        let angle = v1.angle(&v2);
        assert!(angle < 1e-6);
    }

    #[test]
    fn test_angle_opposite() {
        let v1 = GpVec3f::unit_x();
        let v2 = GpVec3f::new(-1.0, 0.0, 0.0);
        let angle = v1.angle(&v2);
        assert!((angle - std::f32::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_operator_add() {
        let v1 = GpVec3f::new(1.0, 2.0, 3.0);
        let v2 = GpVec3f::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_eq!(result, GpVec3f::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_operator_sub() {
        let v1 = GpVec3f::new(5.0, 6.0, 7.0);
        let v2 = GpVec3f::new(1.0, 2.0, 3.0);
        let result = v1 - v2;
        assert_eq!(result, GpVec3f::new(4.0, 4.0, 4.0));
    }

    #[test]
    fn test_operator_mul_scalar_right() {
        let v = GpVec3f::new(2.0, 3.0, 4.0);
        let result = v * 2.0;
        assert_eq!(result, GpVec3f::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_operator_mul_scalar_left() {
        let v = GpVec3f::new(2.0, 3.0, 4.0);
        let result = 3.0 * v;
        assert_eq!(result, GpVec3f::new(6.0, 9.0, 12.0));
    }

    #[test]
    fn test_operator_neg() {
        let v = GpVec3f::new(1.0, -2.0, 3.0);
        let result = -v;
        assert_eq!(result, GpVec3f::new(-1.0, 2.0, -3.0));
    }
}
