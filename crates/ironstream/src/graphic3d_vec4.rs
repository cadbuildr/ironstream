// FILE: graphic3d_vec4.rs
// occt: Graphic3d_Vec4

//! 4D vector types for graphics.
//! Deprecated: Use NCollection_Vec4 directly.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4f { x, y, z, w }
    }

    pub fn zero() -> Self {
        Vec4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn dot(&self, other: &Vec4f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

impl Vec4d {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vec4d { x, y, z, w }
    }

    pub fn zero() -> Self {
        Vec4d { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec4f_creation() {
        let v = Vec4f::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.w, 4.0);
    }

    #[test]
    fn test_vec4f_magnitude() {
        let v = Vec4f::new(1.0, 2.0, 2.0, 0.0);
        assert_eq!(v.magnitude(), 3.0);
    }

    #[test]
    fn test_vec4f_dot() {
        let v1 = Vec4f::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4f::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(v1.dot(&v2), 10.0);
    }

    #[test]
    fn test_vec4f_zero() {
        let v = Vec4f::zero();
        assert_eq!(v.magnitude(), 0.0);
    }

    #[test]
    fn test_vec4d_creation() {
        let v = Vec4d::new(1.5, 2.5, 3.5, 4.5);
        assert_eq!(v.x, 1.5);
        assert_eq!(v.w, 4.5);
    }

    #[test]
    fn test_vec4_equality() {
        let v1 = Vec4f::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4f::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v1, v2);
    }
}
