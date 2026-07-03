//! 2D vector primitives mirroring OpenCascade's `gp_Vec2d` and `gp_Dir2d`
//! classes from the TKMath package.
//!
//! `Vec2d` is a general-purpose 2D vector with arbitrary (including zero)
//! magnitude. `Dir2d` wraps a normalised `Vec2d` and is always unit-length.
//!
//! No external crates are used; only `std` (`f64::sqrt`, `f64::atan2`,
//! `f64::sin`, `f64::cos`).

// ── Vec2d ─────────────────────────────────────────────────────────────────────

/// A 2D vector with arbitrary magnitude.
// occt: gp_Vec2d
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    /// Construct from Cartesian coordinates (`gp_Vec2d(x, y)`).
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// The zero vector.
    #[inline]
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    // ── Algebra ───────────────────────────────────────────────────────────────

    /// `Dot(other)` — scalar dot product.
    #[inline]
    pub fn dot(&self, other: &Vec2d) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// `Crossed(other)` — 2D "cross product" (z-component of the 3D cross),
    /// i.e. the signed area of the parallelogram spanned by `self` and `other`.
    ///
    /// Mirrors `gp_Vec2d::Crossed`.
    #[inline]
    pub fn crossed(&self, other: &Vec2d) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// `Magnitude()` — Euclidean length.
    #[inline]
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// `Normalized()` — return a unit vector in the same direction.
    ///
    /// Returns `(1, 0)` for the zero vector (OCCT raises
    /// `Standard_ConstructionError`; we use a safe fallback).
    pub fn normalize(&self) -> Vec2d {
        let m = self.magnitude();
        if m < 1.0e-300 {
            Vec2d::new(1.0, 0.0)
        } else {
            Vec2d::new(self.x / m, self.y / m)
        }
    }

    /// `Multiplied(s)` — scale by a scalar factor.
    #[inline]
    pub fn scale(&self, s: f64) -> Vec2d {
        Vec2d::new(self.x * s, self.y * s)
    }

    /// `Added(other)` — component-wise addition.
    #[inline]
    pub fn add(&self, other: &Vec2d) -> Vec2d {
        Vec2d::new(self.x + other.x, self.y + other.y)
    }

    /// `Subtracted(other)` — component-wise subtraction.
    #[inline]
    pub fn sub(&self, other: &Vec2d) -> Vec2d {
        Vec2d::new(self.x - other.x, self.y - other.y)
    }

    // ── Angles ────────────────────────────────────────────────────────────────

    /// `Angle(other)` — signed angle in `(-π, π]` from `self` to `other`.
    ///
    /// The sign follows the CCW convention: positive when `self` must rotate
    /// CCW to reach `other`. Mirrors `gp_Vec2d::Angle`.
    ///
    /// Returns `NaN` if either vector is zero (OCCT raises
    /// `Standard_DomainError`).
    pub fn angle(&self, other: &Vec2d) -> f64 {
        // atan2(cross, dot) gives the signed angle from self to other.
        let cross = self.crossed(other);
        let dot = self.dot(other);
        let m1 = self.magnitude();
        let m2 = other.magnitude();
        f64::atan2(cross / (m1 * m2), dot / (m1 * m2))
    }

    /// `Rotated(angle)` — rotate CCW by `angle` radians.
    ///
    /// Mirrors `gp_Vec2d::Rotated(gp_Ax22d, angle)` applied about the origin.
    pub fn rotate(&self, angle: f64) -> Vec2d {
        let (s, c) = (angle.sin(), angle.cos());
        Vec2d::new(c * self.x - s * self.y, s * self.x + c * self.y)
    }
}

// ── std::ops trait impls for Vec2d ────────────────────────────────────────────

impl std::ops::Add for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn add(self, rhs: Vec2d) -> Vec2d {
        Vec2d::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn sub(self, rhs: Vec2d) -> Vec2d {
        Vec2d::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<f64> for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn mul(self, s: f64) -> Vec2d {
        Vec2d::new(self.x * s, self.y * s)
    }
}

impl std::ops::Neg for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn neg(self) -> Vec2d {
        Vec2d::new(-self.x, -self.y)
    }
}

// ── Dir2d ─────────────────────────────────────────────────────────────────────

/// A 2D unit-direction vector. Always normalised; wraps a `Vec2d`.
// occt: gp_Dir2d
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dir2d {
    pub v: Vec2d,
}

impl Dir2d {
    /// Construct from `(x, y)`, normalising automatically.
    ///
    /// Returns `None` if the input vector has near-zero magnitude (mirrors
    /// `Standard_ConstructionError` from OCCT).
    pub fn new(x: f64, y: f64) -> Option<Self> {
        let raw = Vec2d::new(x, y);
        let m = raw.magnitude();
        if m < 1.0e-300 {
            None
        } else {
            Some(Self {
                v: Vec2d::new(x / m, y / m),
            })
        }
    }

    // ── Angles ────────────────────────────────────────────────────────────────

    /// `Angle(other)` — signed angle in `(-π, π]` from `self` to `other`.
    ///
    /// Mirrors `gp_Dir2d::Angle`.
    #[inline]
    pub fn angle(&self, other: &Dir2d) -> f64 {
        // Both are unit vectors so magnitudes are 1; atan2(cross, dot) directly.
        let cross = self.v.crossed(&other.v);
        let dot = self.v.dot(&other.v);
        f64::atan2(cross, dot)
    }

    // ── Transforms ────────────────────────────────────────────────────────────

    /// `Rotated(angle)` — rotate CCW by `angle` radians.
    ///
    /// The result is guaranteed to be unit-length (trig identities).
    /// Mirrors `gp_Dir2d::Rotated`.
    pub fn rotate(&self, angle: f64) -> Dir2d {
        let (s, c) = (angle.sin(), angle.cos());
        // sin²+cos² = 1 so the rotated vector is already unit-length.
        Dir2d {
            v: Vec2d::new(
                c * self.v.x - s * self.v.y,
                s * self.v.x + c * self.v.y,
            ),
        }
    }

    /// `Mirrored(other)` — reflect `self` about the direction `other`.
    ///
    /// Formula: d' = 2(d·ref)ref − d, where `ref` is the mirror axis unit
    /// direction. Mirrors `gp_Dir2d::Mirrored(gp_Dir2d)`.
    pub fn mirror(&self, other: &Dir2d) -> Dir2d {
        let dot = self.v.dot(&other.v);
        Dir2d {
            v: Vec2d::new(
                2.0 * dot * other.v.x - self.v.x,
                2.0 * dot * other.v.y - self.v.y,
            ),
        }
    }

    /// `Reversed()` — opposite direction.
    ///
    /// Mirrors `gp_Dir2d::Reversed`.
    #[inline]
    pub fn reverse(&self) -> Dir2d {
        Dir2d {
            v: Vec2d::new(-self.v.x, -self.v.y),
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, PI};

    // ── Vec2d ──────────────────────────────────────────────────────────────────

    #[test]
    fn vec2d_zero() {
        let z = Vec2d::zero();
        assert_eq!(z.x, 0.0);
        assert_eq!(z.y, 0.0);
    }

    #[test]
    fn vec2d_magnitude() {
        let v = Vec2d::new(3.0, 4.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-12);
    }

    #[test]
    fn vec2d_dot() {
        let a = Vec2d::new(1.0, 2.0);
        let b = Vec2d::new(3.0, 4.0);
        assert!((a.dot(&b) - 11.0).abs() < 1e-12);
    }

    #[test]
    fn vec2d_crossed() {
        let a = Vec2d::new(1.0, 0.0);
        let b = Vec2d::new(0.0, 1.0);
        // CCW: positive
        assert!((a.crossed(&b) - 1.0).abs() < 1e-12);
        // CW: negative
        assert!((b.crossed(&a) + 1.0).abs() < 1e-12);
    }

    #[test]
    fn vec2d_normalize() {
        let v = Vec2d::new(0.0, 5.0);
        let n = v.normalize();
        assert!((n.x).abs() < 1e-12);
        assert!((n.y - 1.0).abs() < 1e-12);
    }

    #[test]
    fn vec2d_scale() {
        let v = Vec2d::new(1.0, 2.0).scale(3.0);
        assert!((v.x - 3.0).abs() < 1e-12);
        assert!((v.y - 6.0).abs() < 1e-12);
    }

    #[test]
    fn vec2d_add_sub() {
        let a = Vec2d::new(1.0, 2.0);
        let b = Vec2d::new(3.0, 4.0);
        let s = a.add(&b);
        assert!((s.x - 4.0).abs() < 1e-12 && (s.y - 6.0).abs() < 1e-12);
        let d = b.sub(&a);
        assert!((d.x - 2.0).abs() < 1e-12 && (d.y - 2.0).abs() < 1e-12);
    }

    #[test]
    fn vec2d_angle_90() {
        let x = Vec2d::new(1.0, 0.0);
        let y = Vec2d::new(0.0, 1.0);
        let a = x.angle(&y);
        assert!((a - FRAC_PI_2).abs() < 1e-12);
    }

    #[test]
    fn vec2d_angle_signed() {
        // Clockwise from x to -y should be -π/2.
        let x = Vec2d::new(1.0, 0.0);
        let neg_y = Vec2d::new(0.0, -1.0);
        let a = x.angle(&neg_y);
        assert!((a + FRAC_PI_2).abs() < 1e-12);
    }

    #[test]
    fn vec2d_rotate_90() {
        let v = Vec2d::new(1.0, 0.0).rotate(FRAC_PI_2);
        assert!((v.x).abs() < 1e-12);
        assert!((v.y - 1.0).abs() < 1e-12);
    }

    // ── Dir2d ──────────────────────────────────────────────────────────────────

    #[test]
    fn dir2d_new_unit() {
        let d = Dir2d::new(0.0, 5.0).unwrap();
        assert!((d.v.magnitude() - 1.0).abs() < 1e-12);
        assert!((d.v.x).abs() < 1e-12);
        assert!((d.v.y - 1.0).abs() < 1e-12);
    }

    #[test]
    fn dir2d_new_zero_returns_none() {
        assert!(Dir2d::new(0.0, 0.0).is_none());
    }

    #[test]
    fn dir2d_angle() {
        let dx = Dir2d::new(1.0, 0.0).unwrap();
        let dy = Dir2d::new(0.0, 1.0).unwrap();
        let a = dx.angle(&dy);
        assert!((a - FRAC_PI_2).abs() < 1e-12);
    }

    #[test]
    fn dir2d_rotate() {
        let d = Dir2d::new(1.0, 0.0).unwrap().rotate(FRAC_PI_2);
        assert!((d.v.x).abs() < 1e-12);
        assert!((d.v.y - 1.0).abs() < 1e-12);
    }

    #[test]
    fn dir2d_mirror() {
        // Mirroring (1, 0) about the diagonal (1, 1)/√2 should give (0, 1).
        let d = Dir2d::new(1.0, 0.0).unwrap();
        let axis = Dir2d::new(1.0, 1.0).unwrap();
        let m = d.mirror(&axis);
        assert!((m.v.x).abs() < 1e-12);
        assert!((m.v.y - 1.0).abs() < 1e-12);
    }

    #[test]
    fn dir2d_reverse() {
        let d = Dir2d::new(1.0, 0.0).unwrap().reverse();
        assert!((d.v.x + 1.0).abs() < 1e-12);
        assert!((d.v.y).abs() < 1e-12);
    }

    #[test]
    fn dir2d_remains_unit_after_rotate() {
        let d = Dir2d::new(3.0, 4.0).unwrap().rotate(1.23456);
        assert!((d.v.magnitude() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn dir2d_angle_180() {
        let d = Dir2d::new(1.0, 0.0).unwrap();
        let neg = d.reverse();
        let a = d.angle(&neg);
        assert!((a.abs() - PI).abs() < 1e-12);
    }
}
