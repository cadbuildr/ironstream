// FILE: n_collection_vec4.rs
// occt: NCollection_Vec4

/// 4-component vector.
#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub fn dot(&self, other: &Vec4) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl std::ops::Add for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl std::ops::Sub for Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.w, 4.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec4::new(1.0, 0.0, 0.0, 0.0);
        let v2 = Vec4::new(2.0, 0.0, 0.0, 0.0);
        assert_eq!(v1.dot(&v2), 2.0);
    }

    #[test]
    fn test_length() {
        let v = Vec4::new(3.0, 4.0, 0.0, 0.0);
        assert!((v.length() - 5.0).abs() < 1.0e-10);
    }

    #[test]
    fn test_add() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(5.0, 6.0, 7.0, 8.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 6.0);
        assert_eq!(v3.y, 8.0);
    }
}
