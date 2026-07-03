//! Pure-Rust zero-dependency stubs for 3-D vector and unit-direction types,
//! mirroring OpenCascade's `gp_Vec` and `gp_Dir` from the TKMath package.
//!
//! Also contains `GeomVectorWithMagnitude` mirroring `Geom_VectorWithMagnitude`.
//!
//! No external crates are used; only `std` (f64::sqrt, f64::acos).

use crate::gp::{Pnt, Trsf};

// ── Vec3 ──────────────────────────────────────────────────────────────────────

/// A 3-D vector with arbitrary magnitude.
// occt: gp_Vec
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Construct from Cartesian coordinates (`gp_Vec(x, y, z)`).
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Zero vector.
    #[inline]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Scalar (dot) product with `other`.
    ///
    /// Mirrors `gp_Vec::Dot`.
    #[inline]
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product `self × other`.
    ///
    /// Mirrors `gp_Vec::Crossed`.
    #[inline]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Euclidean length.
    ///
    /// Mirrors `gp_Vec::Magnitude`.
    #[inline]
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Return a unit-length copy of this vector.
    ///
    /// Returns the zero vector unchanged if the magnitude is zero (avoids
    /// division by zero; OCCT raises `Standard_ConstructionError` instead).
    ///
    /// Mirrors `gp_Vec::Normalized`.
    #[inline]
    pub fn normalize(&self) -> Vec3 {
        let m = self.magnitude();
        if m < f64::MIN_POSITIVE {
            *self
        } else {
            Vec3::new(self.x / m, self.y / m, self.z / m)
        }
    }

    /// Return this vector uniformly scaled by `s`.
    ///
    /// Mirrors `gp_Vec::Multiplied`.
    #[inline]
    pub fn scale(&self, s: f64) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }

    /// Component-wise addition.
    ///
    /// Mirrors `gp_Vec::Added`.
    #[inline]
    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    /// Component-wise subtraction.
    ///
    /// Mirrors `gp_Vec::Subtracted`.
    #[inline]
    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    /// Unsigned angle in radians between `self` and `other`, in `[0, π]`.
    ///
    /// Uses `acos` clamped to `[-1, 1]` to avoid NaN from floating-point
    /// rounding.  Returns `0.0` if either vector has zero magnitude.
    ///
    /// Mirrors `gp_Vec::Angle`.
    #[inline]
    pub fn angle(&self, other: &Vec3) -> f64 {
        let m1 = self.magnitude();
        let m2 = other.magnitude();
        if m1 < f64::MIN_POSITIVE || m2 < f64::MIN_POSITIVE {
            return 0.0;
        }
        let cos_a = (self.dot(other) / (m1 * m2)).clamp(-1.0, 1.0);
        cos_a.acos()
    }

    /// Return `true` if `self` and `other` are parallel (or anti-parallel)
    /// within the angular tolerance `tol` (in radians).
    ///
    /// Mirrors `gp_Vec::IsParallel`.
    #[inline]
    pub fn is_parallel(&self, other: &Vec3, tol: f64) -> bool {
        let a = self.angle(other);
        a <= tol || (core::f64::consts::PI - a) <= tol
    }
}

// ── Dir3 ──────────────────────────────────────────────────────────────────────

/// A unit-length 3-D direction vector.
///
/// Invariant: `v.magnitude() == 1.0` (within floating-point precision).
// occt: gp_Dir
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dir3 {
    /// The underlying unit vector.
    pub v: Vec3,
}

impl Dir3 {
    /// Construct from components, normalizing them.
    ///
    /// Returns `None` if the magnitude of `(x, y, z)` is too small to
    /// normalize safely (mirrors `Standard_ConstructionError` in OCCT).
    ///
    /// Mirrors `gp_Dir(x, y, z)`.
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Option<Self> {
        let v = Vec3::new(x, y, z);
        let m = v.magnitude();
        if m < f64::MIN_POSITIVE {
            None
        } else {
            Some(Self {
                v: Vec3::new(x / m, y / m, z / m),
            })
        }
    }

    /// The X-axis direction `(1, 0, 0)`.
    ///
    /// Mirrors `gp::DX()`.
    #[inline]
    pub fn x_axis() -> Self {
        Self {
            v: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// The Y-axis direction `(0, 1, 0)`.
    ///
    /// Mirrors `gp::DY()`.
    #[inline]
    pub fn y_axis() -> Self {
        Self {
            v: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    /// The Z-axis direction `(0, 0, 1)`.
    ///
    /// Mirrors `gp::DZ()`.
    #[inline]
    pub fn z_axis() -> Self {
        Self {
            v: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    /// Unsigned angle in radians between `self` and `other`, in `[0, π]`.
    ///
    /// Because both directions are unit-length the dot product equals the
    /// cosine directly.  The result is clamped to `[-1, 1]` before `acos`
    /// to guard against floating-point overshoot.
    ///
    /// Mirrors `gp_Dir::Angle`.
    #[inline]
    pub fn angle(&self, other: &Dir3) -> f64 {
        let cos_a = self.v.dot(&other.v).clamp(-1.0, 1.0);
        cos_a.acos()
    }

    /// Scalar (dot) product with another unit direction.
    ///
    /// Equivalent to `cos(angle(other))`.
    ///
    /// Mirrors `gp_Dir::Dot`.
    #[inline]
    pub fn dot(&self, other: &Dir3) -> f64 {
        self.v.dot(&other.v)
    }

    /// Cross product `self × other`.
    ///
    /// The result is **not** renormalized; callers that need a `Dir3` must
    /// wrap it themselves (OCCT's `gp_Dir::Crossed` raises an error if the
    /// result is degenerate — we silently return the zero-magnitude vector
    /// in `v` in that case).
    ///
    /// Mirrors `gp_Dir::Crossed`.
    #[inline]
    pub fn cross(&self, other: &Dir3) -> Dir3 {
        let c = self.v.cross(&other.v);
        // Best-effort: renormalize if possible, otherwise keep whatever we get.
        let m = c.magnitude();
        let v = if m < f64::MIN_POSITIVE {
            c
        } else {
            c.scale(1.0 / m)
        };
        Dir3 { v }
    }

    /// Return the opposite direction (negate all components).
    ///
    /// Mirrors `gp_Dir::Reversed`.
    #[inline]
    pub fn reverse(&self) -> Dir3 {
        Dir3 {
            v: Vec3::new(-self.v.x, -self.v.y, -self.v.z),
        }
    }
}

// ── GeomVectorWithMagnitude ───────────────────────────────────────────────────

/// A free 3-D vector that retains its magnitude, mirroring
/// `Geom_VectorWithMagnitude` from OpenCascade's TKG3d package.
///
/// Unlike a unit-direction type, this vector can have any (non-negative)
/// length. The OCCT class inherits from `Geom_Vector`; here we provide the
/// most-used subset of its methods.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomVectorWithMagnitude {
    v: Pnt,
}

impl GeomVectorWithMagnitude {
    /// Construct from Cartesian coordinates (`Geom_VectorWithMagnitude(x,y,z)`).
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { v: Pnt::new(x, y, z) }
    }

    /// Construct from a `gp_Pnt` / `gp_Vec` value.
    #[inline]
    pub fn from_vec(p: Pnt) -> Self {
        Self { v: p }
    }

    // ---------------------------------------------------------------------- getters

    #[inline] pub fn x(&self) -> f64 { self.v.x }
    #[inline] pub fn y(&self) -> f64 { self.v.y }
    #[inline] pub fn z(&self) -> f64 { self.v.z }

    /// Return the underlying `gp_Pnt`/`gp_Vec` value.
    #[inline] pub fn vec(&self) -> Pnt { self.v }

    /// Euclidean length (`Magnitude`).
    #[inline]
    pub fn magnitude(&self) -> f64 {
        self.v.norm()
    }

    /// Square of the Euclidean length (`SquareMagnitude`).
    #[inline]
    pub fn square_magnitude(&self) -> f64 {
        self.v.dot(self.v)
    }

    // ---------------------------------------------------------------------- non-mutating arithmetic

    /// Return `self + other` as a new vector (`Added`).
    #[inline]
    pub fn added(&self, other: &Self) -> Self {
        Self { v: self.v + other.v }
    }

    /// Return `self - other` as a new vector (`Subtracted`).
    #[inline]
    pub fn subtracted(&self, other: &Self) -> Self {
        Self { v: self.v - other.v }
    }

    /// Return `self * scalar` as a new vector (`Multiplied`).
    #[inline]
    pub fn multiplied(&self, s: f64) -> Self {
        Self { v: self.v * s }
    }

    /// Dot product (`Dot`).
    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self.v.dot(other.v)
    }

    /// Cross product `self × other` (`Crossed`).
    #[inline]
    pub fn crossed(&self, other: &Self) -> Self {
        Self { v: self.v.cross(other.v) }
    }

    /// Magnitude of the cross product `|self × other|` (`CrossedMagnitude`).
    #[inline]
    pub fn crossed_magnitude(&self, other: &Self) -> f64 {
        self.crossed(other).magnitude()
    }

    /// Square of the magnitude of the cross product (`CrossedSquareMagnitude`).
    #[inline]
    pub fn crossed_square_magnitude(&self, other: &Self) -> f64 {
        self.crossed(other).square_magnitude()
    }

    /// Return a unit-length copy (`Normalized`).
    ///
    /// Returns the zero vector unchanged if magnitude is (near) zero — OCCT
    /// raises `Standard_ConstructionError` in that case.
    #[inline]
    pub fn normalized(&self) -> Self {
        let m = self.magnitude();
        if m < f64::MIN_POSITIVE {
            *self
        } else {
            Self { v: self.v * (1.0 / m) }
        }
    }

    /// Return the negated vector (`Reversed`).
    #[inline]
    pub fn reversed(&self) -> Self {
        Self { v: self.v * -1.0 }
    }

    /// Apply a `gp_Trsf` and return the transformed vector.
    ///
    /// Only the linear (rotation + scale) part is applied; translation is
    /// ignored because a free vector is not a point.
    #[inline]
    pub fn transformed(&self, t: &Trsf) -> Self {
        Self { v: t.apply_vector(self.v) }
    }

    // ---------------------------------------------------------------------- in-place mutations

    /// In-place `self += other` (`Add`).
    #[inline]
    pub fn add(&mut self, other: &Self) {
        self.v = self.v + other.v;
    }

    /// In-place `self -= other` (`Subtract`).
    #[inline]
    pub fn subtract(&mut self, other: &Self) {
        self.v = self.v - other.v;
    }

    /// In-place scalar multiplication (`Multiply`).
    #[inline]
    pub fn multiply(&mut self, s: f64) {
        self.v = self.v * s;
    }

    /// In-place normalization (`Normalize`).
    #[inline]
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    /// In-place negation (`Reverse`).
    #[inline]
    pub fn reverse(&mut self) {
        self.v = self.v * -1.0;
    }

    /// In-place transform (`Transform`).
    #[inline]
    pub fn transform(&mut self, t: &Trsf) {
        *self = self.transformed(t);
    }

    /// Return an independent copy (`Copy`).
    #[inline]
    pub fn copy(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::{FRAC_PI_2, PI};

    // ── Vec3 tests ────────────────────────────────────────────────────────────

    #[test]
    fn vec3_zero() {
        let v = Vec3::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn vec3_magnitude() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-12);
    }

    #[test]
    fn vec3_dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert!((a.dot(&b) - 32.0).abs() < 1e-12);
    }

    #[test]
    fn vec3_cross() {
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        let z = x.cross(&y);
        assert!((z.x).abs() < 1e-12);
        assert!((z.y).abs() < 1e-12);
        assert!((z.z - 1.0).abs() < 1e-12);
    }

    #[test]
    fn vec3_normalize() {
        let v = Vec3::new(0.0, 3.0, 4.0);
        let n = v.normalize();
        assert!((n.magnitude() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn vec3_scale_add_sub() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let s = a.scale(2.0);
        assert!((s.x - 2.0).abs() < 1e-12);
        let sum = a.add(&b);
        assert!((sum.x - 5.0).abs() < 1e-12);
        let diff = b.sub(&a);
        assert!((diff.x - 3.0).abs() < 1e-12);
    }

    #[test]
    fn vec3_angle_orthogonal() {
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        assert!((x.angle(&y) - FRAC_PI_2).abs() < 1e-12);
    }

    #[test]
    fn vec3_angle_antiparallel() {
        let v = Vec3::new(1.0, 0.0, 0.0);
        let neg = Vec3::new(-1.0, 0.0, 0.0);
        assert!((v.angle(&neg) - PI).abs() < 1e-12);
    }

    #[test]
    fn vec3_is_parallel() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(2.0, 0.0, 0.0);
        assert!(a.is_parallel(&b, 1e-10));
        let c = Vec3::new(0.0, 1.0, 0.0);
        assert!(!a.is_parallel(&c, 1e-10));
    }

    // ── Dir3 tests ────────────────────────────────────────────────────────────

    #[test]
    fn dir3_new_unit() {
        let d = Dir3::new(0.0, 3.0, 4.0).unwrap();
        assert!((d.v.magnitude() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn dir3_new_zero_returns_none() {
        assert!(Dir3::new(0.0, 0.0, 0.0).is_none());
    }

    #[test]
    fn dir3_axes() {
        let x = Dir3::x_axis();
        let y = Dir3::y_axis();
        let z = Dir3::z_axis();
        assert!((x.v.x - 1.0).abs() < 1e-12);
        assert!((y.v.y - 1.0).abs() < 1e-12);
        assert!((z.v.z - 1.0).abs() < 1e-12);
    }

    #[test]
    fn dir3_angle() {
        let x = Dir3::x_axis();
        let y = Dir3::y_axis();
        assert!((x.angle(&y) - FRAC_PI_2).abs() < 1e-12);
        assert!((x.angle(&x)).abs() < 1e-12);
    }

    #[test]
    fn dir3_dot() {
        let x = Dir3::x_axis();
        let y = Dir3::y_axis();
        assert!((x.dot(&y)).abs() < 1e-12);
        assert!((x.dot(&x) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn dir3_cross() {
        let x = Dir3::x_axis();
        let y = Dir3::y_axis();
        let z = x.cross(&y);
        assert!((z.v.z - 1.0).abs() < 1e-12);
    }

    #[test]
    fn dir3_reverse() {
        let x = Dir3::x_axis();
        let neg_x = x.reverse();
        assert!((neg_x.v.x + 1.0).abs() < 1e-12);
    }
}
