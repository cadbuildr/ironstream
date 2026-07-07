// FILE: gp_vec2f.rs
// occt: gp_Vec2f

/// 2D vector with single-precision (f32) components.
/// Represents a vector in 2D space (X, Y).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GpVec2f {
    pub x: f32,
    pub y: f32,
}

impl GpVec2f {
    /// Create a new 2D vector with given components.
    pub fn new(x: f32, y: f32) -> Self {
        GpVec2f { x, y }
    }

    /// Create a zero vector (0, 0).
    pub fn zero() -> Self {
        GpVec2f { x: 0.0, y: 0.0 }
    }

    /// Create a unit vector along X axis (1, 0).
    pub fn unit_x() -> Self {
        GpVec2f { x: 1.0, y: 0.0 }
    }

    /// Create a unit vector along Y axis (0, 1).
    pub fn unit_y() -> Self {
        GpVec2f { x: 0.0, y: 1.0 }
    }

    /// Calculate the magnitude (length) of the vector.
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Calculate the squared magnitude (avoids sqrt, faster).
    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Return a normalized vector (magnitude = 1.0).
    /// Returns zero vector if magnitude is near-zero.
    pub fn normalized(&self) -> Self {
        let mag = self.magnitude();
        if mag < 1e-7 {
            GpVec2f::zero()
        } else {
            GpVec2f {
                x: self.x / mag,
                y: self.y / mag,
            }
        }
    }

    /// Normalize this vector in-place.
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag >= 1e-7 {
            self.x /= mag;
            self.y /= mag;
        }
    }

    /// Calculate the dot product with another vector.
    pub fn dot(&self, other: &GpVec2f) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Calculate the cross product (returns scalar for 2D).
    /// Result is self.x * other.y - self.y * other.x
    pub fn cross(&self, other: &GpVec2f) -> f32 {
        self.x * other.y - self.y * other.x
    }

    /// Add another vector to this one.
    pub fn add(&self, other: &GpVec2f) -> Self {
        GpVec2f {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Subtract another vector from this one.
    pub fn subtract(&self, other: &GpVec2f) -> Self {
        GpVec2f {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Multiply this vector by a scalar.
    pub fn scale(&self, scalar: f32) -> Self {
        GpVec2f {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    /// Calculate distance between two points (represented as vectors).
    pub fn distance(&self, other: &GpVec2f) -> f32 {
        self.subtract(other).magnitude()
    }

    /// Check if this vector is approximately equal to another (with tolerance).
    pub fn is_equal(&self, other: &GpVec2f, tolerance: f32) -> bool {
        (self.x - other.x).abs() < tolerance && (self.y - other.y).abs() < tolerance
    }
}

impl std::ops::Add for GpVec2f {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        GpVec2f::add(&self, &other)
    }
}

impl std::ops::Sub for GpVec2f {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self.subtract(&other)
    }
}

impl std::ops::Mul<f32> for GpVec2f {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        self.scale(scalar)
    }
}

impl std::ops::Mul<GpVec2f> for f32 {
    type Output = GpVec2f;
    fn mul(self, vec: GpVec2f) -> GpVec2f {
        vec.scale(self)
    }
}

impl std::ops::Neg for GpVec2f {
    type Output = Self;
    fn neg(self) -> Self {
        GpVec2f {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = GpVec2f::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn test_zero() {
        let v = GpVec2f::zero();
        assert_eq!(v, GpVec2f::new(0.0, 0.0));
    }

    #[test]
    fn test_unit_x() {
        let v = GpVec2f::unit_x();
        assert_eq!(v, GpVec2f::new(1.0, 0.0));
    }

    #[test]
    fn test_unit_y() {
        let v = GpVec2f::unit_y();
        assert_eq!(v, GpVec2f::new(0.0, 1.0));
    }

    #[test]
    fn test_magnitude_3_4_5() {
        let v = GpVec2f::new(3.0, 4.0);
        assert!((v.magnitude() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_magnitude_zero() {
        let v = GpVec2f::zero();
        assert!(v.magnitude() < 1e-6);
    }

    #[test]
    fn test_magnitude_squared() {
        let v = GpVec2f::new(3.0, 4.0);
        assert_eq!(v.magnitude_squared(), 25.0);
    }

    #[test]
    fn test_normalized_3_4_5() {
        let v = GpVec2f::new(3.0, 4.0);
        let n = v.normalized();
        assert!((n.magnitude() - 1.0).abs() < 0.001);
        assert!((n.x - 0.6).abs() < 0.001);
        assert!((n.y - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_normalize_in_place() {
        let mut v = GpVec2f::new(3.0, 4.0);
        v.normalize();
        assert!((v.magnitude() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_normalized_zero_vector() {
        let v = GpVec2f::zero();
        let n = v.normalized();
        assert_eq!(n, GpVec2f::zero());
    }

    #[test]
    fn test_dot_product_perpendicular() {
        let v1 = GpVec2f::unit_x();
        let v2 = GpVec2f::unit_y();
        assert!(v1.dot(&v2) < 1e-6);
    }

    #[test]
    fn test_dot_product_parallel() {
        let v1 = GpVec2f::new(2.0, 0.0);
        let v2 = GpVec2f::new(3.0, 0.0);
        assert_eq!(v1.dot(&v2), 6.0);
    }

    #[test]
    fn test_cross_product() {
        let v1 = GpVec2f::new(1.0, 0.0);
        let v2 = GpVec2f::new(0.0, 1.0);
        assert_eq!(v1.cross(&v2), 1.0);
    }

    #[test]
    fn test_cross_product_zero() {
        let v1 = GpVec2f::new(2.0, 0.0);
        let v2 = GpVec2f::new(3.0, 0.0);
        assert_eq!(v1.cross(&v2), 0.0);
    }

    #[test]
    fn test_add() {
        let v1 = GpVec2f::new(1.0, 2.0);
        let v2 = GpVec2f::new(3.0, 4.0);
        let result = v1.add(&v2);
        assert_eq!(result, GpVec2f::new(4.0, 6.0));
    }

    #[test]
    fn test_subtract() {
        let v1 = GpVec2f::new(5.0, 6.0);
        let v2 = GpVec2f::new(1.0, 2.0);
        let result = v1.subtract(&v2);
        assert_eq!(result, GpVec2f::new(4.0, 4.0));
    }

    #[test]
    fn test_scale() {
        let v = GpVec2f::new(2.0, 3.0);
        let result = v.scale(2.0);
        assert_eq!(result, GpVec2f::new(4.0, 6.0));
    }

    #[test]
    fn test_distance() {
        let v1 = GpVec2f::new(0.0, 0.0);
        let v2 = GpVec2f::new(3.0, 4.0);
        assert!((v1.distance(&v2) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_is_equal_true() {
        let v1 = GpVec2f::new(1.0, 2.0);
        let v2 = GpVec2f::new(1.001, 2.001);
        assert!(v1.is_equal(&v2, 0.01));
    }

    #[test]
    fn test_is_equal_false() {
        let v1 = GpVec2f::new(1.0, 2.0);
        let v2 = GpVec2f::new(1.1, 2.0);
        assert!(!v1.is_equal(&v2, 0.01));
    }

    #[test]
    fn test_operator_add() {
        let v1 = GpVec2f::new(1.0, 2.0);
        let v2 = GpVec2f::new(3.0, 4.0);
        let result = v1 + v2;
        assert_eq!(result, GpVec2f::new(4.0, 6.0));
    }

    #[test]
    fn test_operator_sub() {
        let v1 = GpVec2f::new(5.0, 6.0);
        let v2 = GpVec2f::new(1.0, 2.0);
        let result = v1 - v2;
        assert_eq!(result, GpVec2f::new(4.0, 4.0));
    }

    #[test]
    fn test_operator_mul_scalar_right() {
        let v = GpVec2f::new(2.0, 3.0);
        let result = v * 2.0;
        assert_eq!(result, GpVec2f::new(4.0, 6.0));
    }

    #[test]
    fn test_operator_mul_scalar_left() {
        let v = GpVec2f::new(2.0, 3.0);
        let result = 3.0 * v;
        assert_eq!(result, GpVec2f::new(6.0, 9.0));
    }

    #[test]
    fn test_operator_neg() {
        let v = GpVec2f::new(1.0, -2.0);
        let result = -v;
        assert_eq!(result, GpVec2f::new(-1.0, 2.0));
    }
}
