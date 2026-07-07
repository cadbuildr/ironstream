// FILE: open_gl_vec.rs
// occt: OpenGl_Vec

//! Tool classes for selecting appropriate vector and matrix types.

/// 2D vector of floats
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, other: &Vec2f) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn len_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.len();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Self { x: 0.0, y: 0.0 }
        }
    }
}

impl Default for Vec2f {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// 3D vector of floats
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3f) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn len_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.len();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }
}

impl Default for Vec3f {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// 4D vector of floats
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn dot(&self, other: &Vec4f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn len_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.len();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
                w: self.w / len,
            }
        } else {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            }
        }
    }

    pub fn xyz(&self) -> Vec3f {
        Vec3f {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Default for Vec4f {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

/// 2D vector of doubles
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, other: &Vec2d) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.len();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Self { x: 0.0, y: 0.0 }
        }
    }
}

impl Default for Vec2d {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// 3D vector of doubles
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Vec3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3d) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.len();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }
}

impl Default for Vec3d {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// 4D vector of doubles
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4d {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn dot(&self, other: &Vec4d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.len();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
                w: self.w / len,
            }
        } else {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            }
        }
    }

    pub fn xyz(&self) -> Vec3d {
        Vec3d {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Default for Vec4d {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2f() {
        let v = Vec2f::new(3.0, 4.0);
        assert_eq!(v.len_sq(), 25.0);
        assert_eq!(v.len(), 5.0);

        let v2 = Vec2f::new(1.0, 0.0);
        assert_eq!(v2.dot(&Vec2f::new(2.0, 3.0)), 2.0);
    }

    #[test]
    fn test_vec3f_cross() {
        let v1 = Vec3f::new(1.0, 0.0, 0.0);
        let v2 = Vec3f::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);

        assert_eq!(cross.x, 0.0);
        assert_eq!(cross.y, 0.0);
        assert_eq!(cross.z, 1.0);
    }

    #[test]
    fn test_vec3f_normalize() {
        let v = Vec3f::new(3.0, 4.0, 0.0);
        let norm = v.normalized();

        assert!((norm.len() - 1.0).abs() < 0.001);
        assert!((norm.x - 0.6).abs() < 0.001);
        assert!((norm.y - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_vec4f_xyz() {
        let v = Vec4f::new(1.0, 2.0, 3.0, 4.0);
        let xyz = v.xyz();

        assert_eq!(xyz.x, 1.0);
        assert_eq!(xyz.y, 2.0);
        assert_eq!(xyz.z, 3.0);
    }

    #[test]
    fn test_vec3d() {
        let v = Vec3d::new(1.0, 1.0, 1.0);
        assert!((v.len() - (3.0_f64).sqrt()).abs() < 0.001);

        let v2 = Vec3d::new(1.0, 0.0, 0.0);
        assert_eq!(v.dot(&v2), 1.0);
    }

    #[test]
    fn test_default_vectors() {
        let v2f = Vec2f::default();
        assert_eq!(v2f.x, 0.0);
        assert_eq!(v2f.y, 0.0);

        let v4d = Vec4d::default();
        assert_eq!(v4d.w, 0.0);
    }
}
