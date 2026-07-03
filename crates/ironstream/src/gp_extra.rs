// FILE: gp_extra.rs
// occt: gp_Quaternion, gp_EulerSequence, gp_XY, gp_Mat2d

use std::f64::consts::PI;

/// Euler angle sequence conventions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EulerSequence {
    // Intrinsic rotations (OCCT naming)
    XYZ,
    XZY,
    YXZ,
    YZX,
    ZXY,
    ZYX,
    // Extrinsic / aerospace
    ZXZ,
    ZYZ,
    Yaw_Pitch_Roll,
}

impl Default for EulerSequence {
    fn default() -> Self { Self::ZYX }
}

// occt: gp_XY — 2D coordinate pair
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpXY {
    pub x: f64,
    pub y: f64,
}

impl GpXY {
    pub fn new(x: f64, y: f64) -> Self { Self { x, y } }
    pub fn origin() -> Self { Self { x: 0.0, y: 0.0 } }

    pub fn add(&self, other: &GpXY) -> Self { Self { x: self.x + other.x, y: self.y + other.y } }
    pub fn sub(&self, other: &GpXY) -> Self { Self { x: self.x - other.x, y: self.y - other.y } }
    pub fn scale(&self, s: f64) -> Self { Self { x: self.x * s, y: self.y * s } }
    pub fn dot(&self, other: &GpXY) -> f64 { self.x * other.x + self.y * other.y }
    pub fn cross(&self, other: &GpXY) -> f64 { self.x * other.y - self.y * other.x }
    pub fn modulus(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
    pub fn square_modulus(&self) -> f64 { self.x * self.x + self.y * self.y }
    pub fn is_equal(&self, other: &GpXY, tol: f64) -> bool {
        (self.x - other.x).abs() < tol && (self.y - other.y).abs() < tol
    }

    pub fn normalized(&self) -> Option<Self> {
        let m = self.modulus();
        if m < 1e-14 { None } else { Some(self.scale(1.0 / m)) }
    }

    pub fn rotated(&self, angle: f64) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self { x: self.x * c - self.y * s, y: self.x * s + self.y * c }
    }
}

// occt: gp_Mat2d — 2×2 matrix
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpMat2d {
    pub m: [[f64; 2]; 2],
}

impl GpMat2d {
    pub fn identity() -> Self {
        Self { m: [[1.0, 0.0], [0.0, 1.0]] }
    }

    pub fn zero() -> Self {
        Self { m: [[0.0; 2]; 2] }
    }

    pub fn new(a00: f64, a01: f64, a10: f64, a11: f64) -> Self {
        Self { m: [[a00, a01], [a10, a11]] }
    }

    pub fn rotation(angle: f64) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self::new(c, -s, s, c)
    }

    pub fn value(&self, row: usize, col: usize) -> f64 { self.m[row][col] }

    pub fn set_value(&mut self, row: usize, col: usize, v: f64) {
        self.m[row][col] = v;
    }

    pub fn determinant(&self) -> f64 {
        self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0]
    }

    pub fn transposed(&self) -> Self {
        Self::new(self.m[0][0], self.m[1][0], self.m[0][1], self.m[1][1])
    }

    pub fn inverted(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < 1e-14 { return None; }
        Some(Self::new(
            self.m[1][1] / det,
            -self.m[0][1] / det,
            -self.m[1][0] / det,
            self.m[0][0] / det,
        ))
    }

    pub fn multiplied(&self, other: &GpMat2d) -> Self {
        Self::new(
            self.m[0][0] * other.m[0][0] + self.m[0][1] * other.m[1][0],
            self.m[0][0] * other.m[0][1] + self.m[0][1] * other.m[1][1],
            self.m[1][0] * other.m[0][0] + self.m[1][1] * other.m[1][0],
            self.m[1][0] * other.m[0][1] + self.m[1][1] * other.m[1][1],
        )
    }

    pub fn multiply_xy(&self, v: &GpXY) -> GpXY {
        GpXY::new(
            self.m[0][0] * v.x + self.m[0][1] * v.y,
            self.m[1][0] * v.x + self.m[1][1] * v.y,
        )
    }

    pub fn is_identity(&self, tol: f64) -> bool {
        let id = Self::identity();
        (0..2).all(|r| (0..2).all(|c| (self.m[r][c] - id.m[r][c]).abs() < tol))
    }
}

impl Default for GpMat2d {
    fn default() -> Self { Self::identity() }
}

// occt: gp_Quaternion
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl GpQuaternion {
    pub fn identity() -> Self { Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 } }

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self { Self { x, y, z, w } }

    /// From axis-angle (axis need not be normalized).
    pub fn from_axis_angle(axis: [f64; 3], angle: f64) -> Self {
        let len = (axis[0]*axis[0] + axis[1]*axis[1] + axis[2]*axis[2]).sqrt();
        if len < 1e-14 { return Self::identity(); }
        let s = (angle * 0.5).sin() / len;
        Self {
            x: axis[0] * s,
            y: axis[1] * s,
            z: axis[2] * s,
            w: (angle * 0.5).cos(),
        }
    }

    /// Construct from Euler angles (ZYX / yaw-pitch-roll by default).
    pub fn from_euler_angles(seq: EulerSequence, a1: f64, a2: f64, a3: f64) -> Self {
        let q1 = match seq {
            EulerSequence::ZYX | EulerSequence::Yaw_Pitch_Roll => {
                let qz = Self::from_axis_angle([0.0, 0.0, 1.0], a1);
                let qy = Self::from_axis_angle([0.0, 1.0, 0.0], a2);
                let qx = Self::from_axis_angle([1.0, 0.0, 0.0], a3);
                qz.multiplied(&qy).multiplied(&qx)
            }
            EulerSequence::XYZ => {
                let qx = Self::from_axis_angle([1.0, 0.0, 0.0], a1);
                let qy = Self::from_axis_angle([0.0, 1.0, 0.0], a2);
                let qz = Self::from_axis_angle([0.0, 0.0, 1.0], a3);
                qx.multiplied(&qy).multiplied(&qz)
            }
            _ => {
                let qx = Self::from_axis_angle([1.0, 0.0, 0.0], a1);
                let qy = Self::from_axis_angle([0.0, 1.0, 0.0], a2);
                let qz = Self::from_axis_angle([0.0, 0.0, 1.0], a3);
                qx.multiplied(&qy).multiplied(&qz)
            }
        };
        q1
    }

    pub fn norm(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let n = self.norm();
        if n < 1e-14 { return Self::identity(); }
        Self { x: self.x/n, y: self.y/n, z: self.z/n, w: self.w/n }
    }

    pub fn conjugate(&self) -> Self {
        Self { x: -self.x, y: -self.y, z: -self.z, w: self.w }
    }

    pub fn inverted(&self) -> Self {
        let n2 = self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w;
        if n2 < 1e-28 { return Self::identity(); }
        let c = self.conjugate();
        Self { x: c.x/n2, y: c.y/n2, z: c.z/n2, w: c.w/n2 }
    }

    pub fn multiplied(&self, other: &GpQuaternion) -> Self {
        Self {
            x: self.w*other.x + self.x*other.w + self.y*other.z - self.z*other.y,
            y: self.w*other.y - self.x*other.z + self.y*other.w + self.z*other.x,
            z: self.w*other.z + self.x*other.y - self.y*other.x + self.z*other.w,
            w: self.w*other.w - self.x*other.x - self.y*other.y - self.z*other.z,
        }
    }

    pub fn dot(&self, other: &GpQuaternion) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z + self.w*other.w
    }

    pub fn slerp(&self, other: &GpQuaternion, t: f64) -> Self {
        let mut d = self.dot(other);
        let other_adj = if d < 0.0 { d = -d; GpQuaternion { x: -other.x, y: -other.y, z: -other.z, w: -other.w } } else { *other };
        let (scale0, scale1) = if d > 1.0 - 1e-10 {
            (1.0 - t, t)
        } else {
            let theta = d.acos();
            let s = theta.sin();
            (((1.0 - t) * theta).sin() / s, (t * theta).sin() / s)
        };
        GpQuaternion {
            x: scale0 * self.x + scale1 * other_adj.x,
            y: scale0 * self.y + scale1 * other_adj.y,
            z: scale0 * self.z + scale1 * other_adj.z,
            w: scale0 * self.w + scale1 * other_adj.w,
        }
    }

    pub fn rotate_vector(&self, v: [f64; 3]) -> [f64; 3] {
        let qv = GpQuaternion { x: v[0], y: v[1], z: v[2], w: 0.0 };
        let res = self.multiplied(&qv).multiplied(&self.conjugate());
        [res.x, res.y, res.z]
    }

    pub fn get_axis_angle(&self) -> ([f64; 3], f64) {
        let n = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
        if n < 1e-14 {
            return ([0.0, 0.0, 1.0], 0.0);
        }
        let angle = 2.0 * self.w.clamp(-1.0, 1.0).acos();
        ([self.x/n, self.y/n, self.z/n], angle)
    }

    pub fn to_rotation_matrix(&self) -> [[f64; 3]; 3] {
        let q = self.normalized();
        let (x, y, z, w) = (q.x, q.y, q.z, q.w);
        [
            [1.0 - 2.0*(y*y + z*z),  2.0*(x*y - w*z),         2.0*(x*z + w*y)],
            [2.0*(x*y + w*z),         1.0 - 2.0*(x*x + z*z),  2.0*(y*z - w*x)],
            [2.0*(x*z - w*y),         2.0*(y*z + w*x),         1.0 - 2.0*(x*x + y*y)],
        ]
    }

    pub fn is_equal(&self, other: &GpQuaternion, tol: f64) -> bool {
        (self.x - other.x).abs() < tol
            && (self.y - other.y).abs() < tol
            && (self.z - other.z).abs() < tol
            && (self.w - other.w).abs() < tol
    }
}

impl Default for GpQuaternion {
    fn default() -> Self { Self::identity() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gp_xy_ops() {
        let a = GpXY::new(3.0, 4.0);
        assert!((a.modulus() - 5.0).abs() < 1e-10);
        let b = a.normalized().unwrap();
        assert!((b.modulus() - 1.0).abs() < 1e-10);
        let c = a.rotated(PI / 2.0);
        assert!((c.x + 4.0).abs() < 1e-10);
        assert!((c.y - 3.0).abs() < 1e-10);
    }

    #[test]
    fn gp_mat2d_ops() {
        let id = GpMat2d::identity();
        assert_eq!(id.determinant(), 1.0);
        let inv = id.inverted().unwrap();
        assert!(inv.is_identity(1e-10));
        let rot = GpMat2d::rotation(PI / 2.0);
        let v = GpXY::new(1.0, 0.0);
        let vr = rot.multiply_xy(&v);
        assert!((vr.x).abs() < 1e-10);
        assert!((vr.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn quaternion_identity() {
        let q = GpQuaternion::identity();
        assert!((q.norm() - 1.0).abs() < 1e-10);
        let v = q.rotate_vector([1.0, 0.0, 0.0]);
        assert!((v[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn quaternion_rotation() {
        let q = GpQuaternion::from_axis_angle([0.0, 0.0, 1.0], PI / 2.0);
        let v = q.rotate_vector([1.0, 0.0, 0.0]);
        assert!(v[0].abs() < 1e-10);
        assert!((v[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn quaternion_slerp() {
        let q0 = GpQuaternion::identity();
        let q1 = GpQuaternion::from_axis_angle([0.0, 0.0, 1.0], PI);
        let qm = q0.slerp(&q1, 0.5);
        let (_, angle) = qm.get_axis_angle();
        assert!((angle - PI / 2.0).abs() < 1e-6);
    }

    #[test]
    fn quaternion_to_rotation_matrix() {
        let q = GpQuaternion::identity();
        let m = q.to_rotation_matrix();
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((m[i][j] - expected).abs() < 1e-10);
            }
        }
    }
}
