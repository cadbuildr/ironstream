// FILE: plate_d1.rs
// occt: Plate_D1

use std::ops::{Add, Mul, Neg};

#[derive(Clone, Copy, Debug)]
pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl XYZ {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        XYZ { x, y, z }
    }

    pub fn dot(&self, other: &XYZ) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &XYZ) -> XYZ {
        XYZ {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn modulus(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn cross_magnitude(&self, other: &XYZ) -> f64 {
        self.cross(other).modulus()
    }

    pub fn normalize(&mut self) {
        let m = self.modulus();
        if m > 1e-15 {
            self.x /= m;
            self.y /= m;
            self.z /= m;
        }
    }
}

impl Add for XYZ {
    type Output = XYZ;
    fn add(self, other: XYZ) -> XYZ {
        XYZ {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f64> for XYZ {
    type Output = XYZ;
    fn mul(self, scalar: f64) -> XYZ {
        XYZ {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<XYZ> for f64 {
    type Output = XYZ;
    fn mul(self, xyz: XYZ) -> XYZ {
        xyz * self
    }
}

impl Neg for XYZ {
    type Output = XYZ;
    fn neg(self) -> XYZ {
        XYZ {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct XY {
    pub x: f64,
    pub y: f64,
}

impl XY {
    pub fn new(x: f64, y: f64) -> Self {
        XY { x, y }
    }
}

/// Plate_D1: Define an order 1 derivative of a 3D-valued function of a 2D variable
#[derive(Clone, Copy, Debug)]
pub struct PlateD1 {
    pub du: XYZ,
    pub dv: XYZ,
}

impl PlateD1 {
    pub fn new(du: XYZ, dv: XYZ) -> Self {
        PlateD1 { du, dv }
    }

    pub fn du(&self) -> &XYZ {
        &self.du
    }

    pub fn dv(&self) -> &XYZ {
        &self.dv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xyz_creation() {
        let v = XYZ::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_xyz_dot() {
        let v1 = XYZ::new(1.0, 2.0, 3.0);
        let v2 = XYZ::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0); // 1*4 + 2*5 + 3*6
    }

    #[test]
    fn test_xyz_cross() {
        let v1 = XYZ::new(1.0, 0.0, 0.0);
        let v2 = XYZ::new(0.0, 1.0, 0.0);
        let result = v1.cross(&v2);
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 1.0);
    }

    #[test]
    fn test_plate_d1() {
        let du = XYZ::new(1.0, 0.0, 0.0);
        let dv = XYZ::new(0.0, 1.0, 0.0);
        let d1 = PlateD1::new(du, dv);
        assert_eq!(d1.du().x, 1.0);
        assert_eq!(d1.dv().y, 1.0);
    }
}
