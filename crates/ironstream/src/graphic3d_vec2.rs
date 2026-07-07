// FILE: graphic3d_vec2.rs
// occt: Graphic3d_Vec2

//! 2D vector types for graphics.
//! Deprecated: Use NCollection_Vec2 directly.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2f { x, y }
    }

    pub fn zero() -> Self {
        Vec2f { x: 0.0, y: 0.0 }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2d { x, y }
    }

    pub fn zero() -> Self {
        Vec2d { x: 0.0, y: 0.0 }
    }
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2i { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2f_creation() {
        let v = Vec2f::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn test_vec2f_magnitude() {
        let v = Vec2f::new(3.0, 4.0);
        assert!((v.magnitude() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_vec2f_zero() {
        let v = Vec2f::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
    }

    #[test]
    fn test_vec2d_creation() {
        let v = Vec2d::new(1.5, 2.5);
        assert_eq!(v.x, 1.5);
        assert_eq!(v.y, 2.5);
    }

    #[test]
    fn test_vec2i_creation() {
        let v = Vec2i::new(10, 20);
        assert_eq!(v.x, 10);
        assert_eq!(v.y, 20);
    }

    #[test]
    fn test_vec2_equality() {
        let v1 = Vec2f::new(1.0, 2.0);
        let v2 = Vec2f::new(1.0, 2.0);
        assert_eq!(v1, v2);
    }
}
