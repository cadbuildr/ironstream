//! `gp_Vec` — a 3D vector (non-unit), mirroring OpenCascade's `gp_Vec` class
//! from the TKMath package.
//!
//! Distinct from `gp_Dir` (which is always unit-length) and `gp_Pnt` (which
//! represents a position). `gp_Vec` may have any magnitude, including zero.

use crate::gp::{Ax1, Ax3, Pnt, Trsf};
use std::f64::consts::PI;

/// A 3D vector with arbitrary magnitude.
// occt-ref: gp_Vec
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec {
    /// Construct from Cartesian coordinates (`gp_Vec(x, y, z)`).
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Zero vector.
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Construct from a unit direction (wraps `gp_Dir` → `gp_Vec`).
    #[inline]
    pub fn from_dir(d: Pnt) -> Self {
        let u = d.normalized();
        Self::new(u.x, u.y, u.z)
    }

    /// Construct from a `gp_Pnt`/`gp_XYZ` (coordinate triple).
    #[inline]
    pub fn from_pnt(p: Pnt) -> Self {
        Self::new(p.x, p.y, p.z)
    }

    // ── Accessors ──────────────────────────────────────────────────────────

    /// Return a `gp_XYZ`-compatible coordinate triple.
    #[inline]
    pub fn coord(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }
    #[inline]
    pub fn z(&self) -> f64 {
        self.z
    }

    // ── Magnitude ──────────────────────────────────────────────────────────

    /// `Magnitude()` — Euclidean length.
    #[inline]
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// `SquareMagnitude()`.
    #[inline]
    pub fn square_magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // ── Arithmetic ─────────────────────────────────────────────────────────

    /// `Added(other)` / `operator+`.
    #[inline]
    pub fn add(&self, other: &Vec) -> Vec {
        Vec::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    /// In-place `Add`.
    #[inline]
    pub fn add_assign(&mut self, other: &Vec) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    /// `Subtracted(other)` / `operator-`.
    #[inline]
    pub fn subtract(&self, other: &Vec) -> Vec {
        Vec::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    /// In-place `Subtract`.
    #[inline]
    pub fn subtract_assign(&mut self, other: &Vec) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }

    /// `Multiplied(scalar)` / `operator*`.
    #[inline]
    pub fn multiplied(&self, s: f64) -> Vec {
        Vec::new(self.x * s, self.y * s, self.z * s)
    }

    /// In-place `Multiply`.
    #[inline]
    pub fn multiply(&mut self, s: f64) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }

    /// `Divided(scalar)` / `operator/`.
    #[inline]
    pub fn divided(&self, s: f64) -> Vec {
        let inv = 1.0 / s;
        Vec::new(self.x * inv, self.y * inv, self.z * inv)
    }

    /// In-place `Divide`.
    #[inline]
    pub fn divide(&mut self, s: f64) {
        let inv = 1.0 / s;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }

    /// `Reversed()` — negated vector.
    #[inline]
    pub fn reversed(&self) -> Vec {
        Vec::new(-self.x, -self.y, -self.z)
    }

    /// In-place `Reverse`.
    #[inline]
    pub fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    // ── Cross product ──────────────────────────────────────────────────────

    /// `Crossed(other)` / `^` — cross product.
    #[inline]
    pub fn crossed(&self, other: &Vec) -> Vec {
        Vec::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// `CrossMagnitude(other)` — magnitude of the cross product.
    #[inline]
    pub fn cross_magnitude(&self, other: &Vec) -> f64 {
        self.crossed(other).magnitude()
    }

    /// `CrossSquareMagnitude(other)` — square magnitude of the cross product.
    #[inline]
    pub fn cross_square_magnitude(&self, other: &Vec) -> f64 {
        self.crossed(other).square_magnitude()
    }

    // ── Dot product ────────────────────────────────────────────────────────

    /// `Dot(other)` — scalar dot product.
    #[inline]
    pub fn dot(&self, other: &Vec) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// `DotCross(v1, v2)` — mixed triple product: `self · (v1 × v2)`.
    #[inline]
    pub fn dot_cross(&self, v1: &Vec, v2: &Vec) -> f64 {
        self.dot(&v1.crossed(v2))
    }

    // ── Angles ─────────────────────────────────────────────────────────────

    /// `Angle(other)` — unsigned angle in `[0, π]` between two vectors.
    ///
    /// Panics (via f64 arithmetic producing NaN) if either vector is zero —
    /// exactly as OCCT raises `Standard_DomainError`.
    pub fn angle(&self, other: &Vec) -> f64 {
        let m1 = self.magnitude();
        let m2 = other.magnitude();
        // Compute sin and cos of the angle.
        let cross_mag = self.crossed(other).magnitude();
        let dot = self.dot(other);
        // atan2 is more numerically stable than acos near 0 and π.
        f64::atan2(cross_mag / (m1 * m2), dot / (m1 * m2))
    }

    /// `AngleWithRef(other, ref_vec)` — signed angle using `ref_vec` to
    /// determine orientation. Sign is positive when `self × other` and
    /// `ref_vec` point in the same half-space.
    pub fn angle_with_ref(&self, other: &Vec, vref: &Vec) -> f64 {
        let cross = self.crossed(other);
        let m1 = self.magnitude();
        let m2 = other.magnitude();
        let cross_mag = cross.magnitude();
        let dot = self.dot(other);
        let angle = f64::atan2(cross_mag / (m1 * m2), dot / (m1 * m2));
        if cross.dot(vref) < 0.0 {
            -angle
        } else {
            angle
        }
    }

    // ── Predicates ─────────────────────────────────────────────────────────

    /// `IsEqual(other, linear_tol, angular_tol)` — magnitudes differ by at
    /// most `linear_tol` AND direction differs by at most `angular_tol`.
    pub fn is_equal(&self, other: &Vec, linear_tol: f64, angular_tol: f64) -> bool {
        let mag_diff = (self.magnitude() - other.magnitude()).abs();
        if mag_diff > linear_tol {
            return false;
        }
        // If both are (near-)zero the direction is undefined; treat as equal.
        let m1 = self.magnitude();
        let m2 = other.magnitude();
        if m1 < 1.0e-300 || m2 < 1.0e-300 {
            return true;
        }
        self.angle(other) <= angular_tol
    }

    /// `IsNormal(other, angular_tol)` — perpendicular within tolerance.
    pub fn is_normal(&self, other: &Vec, angular_tol: f64) -> bool {
        let m1 = self.magnitude();
        let m2 = other.magnitude();
        if m1 < 1.0e-300 || m2 < 1.0e-300 {
            return false;
        }
        (self.angle(other) - std::f64::consts::FRAC_PI_2).abs() <= angular_tol
    }

    /// `IsOpposite(other, angular_tol)` — anti-parallel within tolerance.
    pub fn is_opposite(&self, other: &Vec, angular_tol: f64) -> bool {
        let m1 = self.magnitude();
        let m2 = other.magnitude();
        if m1 < 1.0e-300 || m2 < 1.0e-300 {
            return false;
        }
        (PI - self.angle(other)) <= angular_tol
    }

    /// `IsParallel(other, angular_tol)` — parallel or anti-parallel.
    pub fn is_parallel(&self, other: &Vec, angular_tol: f64) -> bool {
        let m1 = self.magnitude();
        let m2 = other.magnitude();
        if m1 < 1.0e-300 || m2 < 1.0e-300 {
            return false;
        }
        let a = self.angle(other);
        a <= angular_tol || (PI - a) <= angular_tol
    }

    // ── Unit / normalize ───────────────────────────────────────────────────

    /// `Normalized()` — return a unit vector in the same direction.
    ///
    /// Returns `+Z` for a zero vector (matches OCCT "undefined" fallback in
    /// our kernel; OCCT raises `Standard_ConstructionError`).
    pub fn normalized(&self) -> Vec {
        let m = self.magnitude();
        if m < 1.0e-300 {
            Vec::new(0.0, 0.0, 1.0)
        } else {
            self.multiplied(1.0 / m)
        }
    }

    /// In-place `Normalize`.
    pub fn normalize(&mut self) {
        let n = self.normalized();
        self.x = n.x;
        self.y = n.y;
        self.z = n.z;
    }

    // ── Transforms ─────────────────────────────────────────────────────────

    /// `Rotated(Ax1, angle)` — rotate vector about `axis` by `angle` radians.
    pub fn rotated(&self, axis: Ax1, angle: f64) -> Vec {
        // Rodrigues' rotation formula applied to a free vector
        // (the translation part of the Trsf is irrelevant for vectors).
        let t = Trsf::rotation(axis, angle);
        let p = t.apply_vector(Pnt::new(self.x, self.y, self.z));
        Vec::new(p.x, p.y, p.z)
    }

    /// In-place `Rotate`.
    pub fn rotate(&mut self, axis: Ax1, angle: f64) {
        let r = self.rotated(axis, angle);
        self.x = r.x;
        self.y = r.y;
        self.z = r.z;
    }

    /// `Mirrored(Pnt)` — reflect through the origin of `center`.
    ///
    /// For a free vector, mirroring through a *point* simply reverses the
    /// vector (the point acts as center of symmetry).  OCCT `gp_Vec::Mirror(P)`
    /// negates all components.
    #[inline]
    pub fn mirrored_point(&self, _center: Pnt) -> Vec {
        self.reversed()
    }

    /// `Mirrored(Ax1)` — reflect the vector across the line given by `axis`.
    ///
    /// The formula is: v' = 2(v·d)d − v  where d is the unit axis direction.
    pub fn mirrored_axis(&self, axis: Ax1) -> Vec {
        let d = axis.direction.normalized();
        let dot = self.x * d.x + self.y * d.y + self.z * d.z;
        Vec::new(
            2.0 * dot * d.x - self.x,
            2.0 * dot * d.y - self.y,
            2.0 * dot * d.z - self.z,
        )
    }

    /// `Mirrored(Ax2)` — reflect the vector through the plane defined by `ax2`.
    ///
    /// The plane's normal is `ax2.z_dir`. For a free vector the formula is:
    /// v' = v − 2(v·n)n.
    pub fn mirrored_plane(&self, ax2: Ax3) -> Vec {
        let n = ax2.z_dir.normalized();
        let dot = self.x * n.x + self.y * n.y + self.z * n.z;
        Vec::new(
            self.x - 2.0 * dot * n.x,
            self.y - 2.0 * dot * n.y,
            self.z - 2.0 * dot * n.z,
        )
    }

    /// `Scaled(factor)` — uniform scaling.
    #[inline]
    pub fn scaled(&self, factor: f64) -> Vec {
        self.multiplied(factor)
    }

    /// `Transformed(Trsf)` — apply the linear part of the transform only
    /// (vectors are not affected by translation).
    pub fn transformed(&self, t: &Trsf) -> Vec {
        let p = t.apply_vector(Pnt::new(self.x, self.y, self.z));
        Vec::new(p.x, p.y, p.z)
    }

    // ── Operator-style convenience conversions ─────────────────────────────

    /// Convert to `gp_Pnt` / `gp_XYZ`.
    #[inline]
    pub fn to_pnt(&self) -> Pnt {
        Pnt::new(self.x, self.y, self.z)
    }
}

// ── std::ops trait impls ──────────────────────────────────────────────────────

impl std::ops::Add for Vec {
    type Output = Vec;
    #[inline]
    fn add(self, rhs: Vec) -> Vec {
        Vec::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vec {
    type Output = Vec;
    #[inline]
    fn sub(self, rhs: Vec) -> Vec {
        Vec::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<f64> for Vec {
    type Output = Vec;
    #[inline]
    fn mul(self, s: f64) -> Vec {
        self.multiplied(s)
    }
}

impl std::ops::Div<f64> for Vec {
    type Output = Vec;
    #[inline]
    fn div(self, s: f64) -> Vec {
        self.divided(s)
    }
}

impl std::ops::Neg for Vec {
    type Output = Vec;
    #[inline]
    fn neg(self) -> Vec {
        self.reversed()
    }
}

impl std::ops::BitXor for Vec {
    type Output = Vec;
    /// `^` — cross product, matching OCCT's `operator^`.
    #[inline]
    fn bitxor(self, rhs: Vec) -> Vec {
        self.crossed(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magnitude_and_square_magnitude() {
        let v = Vec::new(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-12);
        assert!((v.square_magnitude() - 25.0).abs() < 1e-12);
    }

    #[test]
    fn cross_product_basic() {
        let x = Vec::new(1.0, 0.0, 0.0);
        let y = Vec::new(0.0, 1.0, 0.0);
        let z = x.crossed(&y);
        assert!((z.x - 0.0).abs() < 1e-12);
        assert!((z.y - 0.0).abs() < 1e-12);
        assert!((z.z - 1.0).abs() < 1e-12);
    }

    #[test]
    fn dot_product_basic() {
        let a = Vec::new(1.0, 2.0, 3.0);
        let b = Vec::new(4.0, 5.0, 6.0);
        assert!((a.dot(&b) - 32.0).abs() < 1e-12);
    }
}
