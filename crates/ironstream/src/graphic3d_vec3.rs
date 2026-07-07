// FILE: graphic3d_vec3.rs
// occt: Graphic3d_Vec3

//! 3D vector types for graphics.
//! Deprecated: Use NCollection_Vec3 directly.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3f { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: &Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3d { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3d { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Vec3i {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Vec3i { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3f_creation() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vec3f_magnitude() {
        let v = Vec3f::new(0.0, 0.0, 5.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_vec3f_dot() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_vec3f_zero() {
        let v = Vec3f::zero();
        assert_eq!(v.magnitude(), 0.0);
    }

    #[test]
    fn test_vec3d_creation() {
        let v = Vec3d::new(1.5, 2.5, 3.5);
        assert_eq!(v.x, 1.5);
    }

    #[test]
    fn test_vec3i_creation() {
        let v = Vec3i::new(10, 20, 30);
        assert_eq!(v.z, 30);
    }
}
