//! `gp` (2D part) — planar geometric primitives, mirroring OpenCascade's
//! `gp_Pnt2d`, `gp_Vec2d`, `gp_Dir2d`, `gp_Ax2d`, `gp_Trsf2d`.
//!
//! 2D points live in a surface's parametric / sketch plane; the topology layer
//! maps them to 3D through the owning surface or sketch frame.

use std::ops::{Add, Mul, Neg, Sub};

/// A point or vector in 2D (`gp_Pnt2d` / `gp_Vec2d`).
#[derive(Clone, Copy, Debug, PartialEq)]
// occt: gp_Pnt2d, gp_XY, gp_Vec2d, gp_Dir2d
pub struct Pnt2d {
    pub x: f64,
    pub y: f64,
}

/// Alias mirroring `gp_Vec2d`.
pub type Vec2d = Pnt2d;

impl Pnt2d {
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    #[inline]
    pub const fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    #[inline]
    pub fn dot(self, o: Pnt2d) -> f64 {
        self.x * o.x + self.y * o.y
    }
    /// 2D cross product (z of the 3D cross) — signed area form.
    #[inline]
    pub fn cross(self, o: Pnt2d) -> f64 {
        self.x * o.y - self.y * o.x
    }
    #[inline]
    pub fn norm(self) -> f64 {
        self.dot(self).sqrt()
    }
    #[inline]
    pub fn distance(self, o: Pnt2d) -> f64 {
        (self - o).norm()
    }
    #[inline]
    pub fn normalized(self) -> Pnt2d {
        let n = self.norm();
        if n < 1e-12 {
            Pnt2d::new(1.0, 0.0)
        } else {
            self * (1.0 / n)
        }
    }
    #[inline]
    pub fn lerp(self, o: Pnt2d, t: f64) -> Pnt2d {
        self + (o - self) * t
    }
    /// Rotate +90° (CCW).
    #[inline]
    pub fn perp(self) -> Pnt2d {
        Pnt2d::new(-self.y, self.x)
    }
    #[inline]
    pub fn angle(self) -> f64 {
        self.y.atan2(self.x)
    }
}

impl Add for Pnt2d {
    type Output = Pnt2d;
    #[inline]
    fn add(self, o: Pnt2d) -> Pnt2d {
        Pnt2d::new(self.x + o.x, self.y + o.y)
    }
}
impl Sub for Pnt2d {
    type Output = Pnt2d;
    #[inline]
    fn sub(self, o: Pnt2d) -> Pnt2d {
        Pnt2d::new(self.x - o.x, self.y - o.y)
    }
}
impl Mul<f64> for Pnt2d {
    type Output = Pnt2d;
    #[inline]
    fn mul(self, s: f64) -> Pnt2d {
        Pnt2d::new(self.x * s, self.y * s)
    }
}
impl Neg for Pnt2d {
    type Output = Pnt2d;
    #[inline]
    fn neg(self) -> Pnt2d {
        Pnt2d::new(-self.x, -self.y)
    }
}

/// A 2D axis (`gp_Ax2d`): a location and a unit direction.
#[derive(Clone, Copy, Debug)]
// occt: gp_Ax2d
pub struct Ax2d {
    pub location: Pnt2d,
    pub direction: Pnt2d,
}

impl Ax2d {
    pub fn new(location: Pnt2d, direction: Pnt2d) -> Self {
        Self {
            location,
            direction: direction.normalized(),
        }
    }
}

/// A 2D affine transform (`gp_Trsf2d`): 2x2 linear part + translation.
#[derive(Clone, Copy, Debug)]
// occt: gp_Trsf2d
pub struct Trsf2d {
    pub m: [[f64; 2]; 2],
    pub t: [f64; 2],
}

impl Default for Trsf2d {
    fn default() -> Self {
        Self::identity()
    }
}

impl Trsf2d {
    pub const fn identity() -> Self {
        Self {
            m: [[1.0, 0.0], [0.0, 1.0]],
            t: [0.0, 0.0],
        }
    }
    pub fn translation(v: Pnt2d) -> Self {
        Self {
            m: [[1.0, 0.0], [0.0, 1.0]],
            t: [v.x, v.y],
        }
    }
    pub fn rotation(center: Pnt2d, angle: f64) -> Self {
        let (s, c) = angle.sin_cos();
        let m = [[c, -s], [s, c]];
        let rc = [
            m[0][0] * center.x + m[0][1] * center.y,
            m[1][0] * center.x + m[1][1] * center.y,
        ];
        Self {
            m,
            t: [center.x - rc[0], center.y - rc[1]],
        }
    }
    #[inline]
    pub fn apply(&self, p: Pnt2d) -> Pnt2d {
        Pnt2d::new(
            self.m[0][0] * p.x + self.m[0][1] * p.y + self.t[0],
            self.m[1][0] * p.x + self.m[1][1] * p.y + self.t[1],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_and_perp() {
        let a = Pnt2d::new(1.0, 0.0);
        assert_eq!(a.perp(), Pnt2d::new(0.0, 1.0));
        assert_eq!(a.cross(Pnt2d::new(0.0, 1.0)), 1.0);
    }

    #[test]
    fn rotate_90() {
        let t = Trsf2d::rotation(Pnt2d::origin(), std::f64::consts::FRAC_PI_2);
        let p = t.apply(Pnt2d::new(1.0, 0.0));
        assert!((p.x).abs() < 1e-12 && (p.y - 1.0).abs() < 1e-12);
    }
}
