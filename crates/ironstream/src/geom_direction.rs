//! `Geom_Direction` -- a persistent 3D unit direction, wrapping `gp_Dir`.
//! Mirrors OpenCascade's `Geom_Direction` from package `TKG3d`.
//!
//! A direction is a unit vector in 3D space.  Every mutating operation that
//! takes raw coordinates or a [`Vec3`] normalizes the input so that the stored
//! vector always has magnitude 1.
//!
//! ## API mapping (OCCT → IronStream)
//!
//! | OCCT                              | IronStream                                |
//! |-----------------------------------|-------------------------------------------|
//! | `Geom_Direction(X, Y, Z)`         | `GeomDirection::new(x, y, z)`            |
//! | `Geom_Direction(gp_Dir)`          | `GeomDirection::from_dir(d)`             |
//! | `SetCoord(X, Y, Z)`               | `set_coord(x, y, z)`                     |
//! | `SetDir(gp_Dir)`                  | `set_dir(d)`                             |
//! | `X()` / `Y()` / `Z()`            | `x()` / `y()` / `z()`                   |
//! | `Coord(X, Y, Z)`                  | `coord()`                                |
//! | `Dir()`                           | `dir()`                                  |
//! | `Crossed(Other)`                  | `crossed(other)`                         |
//! | `Dot(Other)`                      | `dot(other)`                             |
//! | `IsOpposite(Other, AngTol)`       | `is_opposite(other, ang_tol)`            |
//! | `IsParallel(Other, AngTol)`       | `is_parallel(other, ang_tol)`            |
//! | `Angle(Other)`                    | `angle(other)`                           |
//! | `Transform(T)`                    | `transform(t)` / `transformed(t)`        |
//! | `Reversed()` / `Reverse()`        | `reversed()` / `reverse()`              |
//! | `Copy()`                          | `copy()`                                 |

use crate::gp::{Trsf, Vec3};

/// A persistent unit direction in 3D space (`Geom_Direction`).
///
/// Stores a [`Vec3`] (`gp_Dir`) that is guaranteed to have magnitude 1.
// occt: Geom_Direction
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomDirection {
    dir: Vec3,
}

impl GeomDirection {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// `Geom_Direction(X, Y, Z)` — constructs from three coordinates, which are
    /// normalized immediately.
    ///
    /// # Panics
    /// Panics if `(x, y, z)` is the zero vector (mirrors OCCT's
    /// `Standard_ConstructionError`).
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let n = (x * x + y * y + z * z).sqrt();
        assert!(
            n > 0.0,
            "Geom_Direction: cannot build a direction from the null vector"
        );
        Self {
            dir: Vec3::new(x / n, y / n, z / n),
        }
    }

    /// `Geom_Direction(const gp_Dir& V)` — constructs from an already-unit
    /// `gp_Dir` (represented as [`Vec3`]).  The vector is re-normalized for
    /// safety.
    #[inline]
    pub fn from_dir(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }

    // ── Setters ───────────────────────────────────────────────────────────────

    /// `SetCoord(X, Y, Z)` — replaces the stored direction with the normalized
    /// version of `(x, y, z)`.
    ///
    /// # Panics
    /// Panics if `(x, y, z)` is the zero vector.
    #[inline]
    pub fn set_coord(&mut self, x: f64, y: f64, z: f64) {
        *self = Self::new(x, y, z);
    }

    /// `SetDir(const gp_Dir&)` — replaces the stored direction, normalizing
    /// the given `gp_Dir`.
    #[inline]
    pub fn set_dir(&mut self, v: Vec3) {
        *self = Self::from_dir(v);
    }

    // ── Getters ───────────────────────────────────────────────────────────────

    /// `X()` — the X component of the unit vector.
    #[inline]
    pub fn x(&self) -> f64 {
        self.dir.x
    }

    /// `Y()` — the Y component of the unit vector.
    #[inline]
    pub fn y(&self) -> f64 {
        self.dir.y
    }

    /// `Z()` — the Z component of the unit vector.
    #[inline]
    pub fn z(&self) -> f64 {
        self.dir.z
    }

    /// `Coord(X, Y, Z)` — all three components as `(x, y, z)`.
    #[inline]
    pub fn coord(&self) -> (f64, f64, f64) {
        (self.dir.x, self.dir.y, self.dir.z)
    }

    /// `Dir()` — the underlying `gp_Dir` as a [`Vec3`] (unit vector).
    #[inline]
    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    // ── Geometric operations ──────────────────────────────────────────────────

    /// `Crossed(Other)` — the 3D cross product of the two unit directions.
    ///
    /// Returns the vector perpendicular to both `self` and `other`, with
    /// magnitude `sin(angle)`.  The result is **not** normalized (it is a
    /// `gp_Vec`, not a `gp_Dir`).
    #[inline]
    pub fn crossed(&self, other: &GeomDirection) -> Vec3 {
        self.dir.cross(other.dir)
    }

    /// `Dot(Other)` — the dot product of the two unit directions.
    ///
    /// Because both are unit vectors the result is `cos(angle)`.
    #[inline]
    pub fn dot(&self, other: &GeomDirection) -> f64 {
        self.dir.dot(other.dir)
    }

    /// `IsOpposite(Other, AngularTolerance)` — returns `true` when the angle
    /// between `self` and the **reverse** of `other` is within `ang_tol`
    /// radians (i.e. the two directions are nearly anti-parallel).
    ///
    /// Uses the same approximation as OCCT:
    /// `dot(self, other) <= -(1 - ang_tol²/2)`.
    #[inline]
    pub fn is_opposite(&self, other: &GeomDirection, ang_tol: f64) -> bool {
        let d = self.dot(other);
        // cos(π - ang_tol) ≈ -(1 - ang_tol²/2) for small tolerances
        d <= -(1.0 - 0.5 * ang_tol * ang_tol)
    }

    /// `IsParallel(Other, AngularTolerance)` — returns `true` when `self` is
    /// parallel (co-directional **or** anti-parallel) to `other` within the
    /// given angular tolerance.
    ///
    /// The test is `|sin(angle)| ≤ sin(ang_tol)`, implemented as
    /// `|crossed|.norm() ≤ sin(ang_tol)`.
    #[inline]
    pub fn is_parallel(&self, other: &GeomDirection, ang_tol: f64) -> bool {
        self.crossed(other).norm() <= ang_tol.sin()
    }

    /// `Angle(Other)` — the unsigned angle in radians between `self` and
    /// `other`, in the range `[0, π]`.
    ///
    /// Computed as `acos(dot)` clamped to `[-1, 1]` for numerical safety.
    #[inline]
    pub fn angle(&self, other: &GeomDirection) -> f64 {
        self.dot(other).clamp(-1.0, 1.0).acos()
    }

    // ── Reversal ──────────────────────────────────────────────────────────────

    /// `Reverse()` — reverses the direction in place.
    #[inline]
    pub fn reverse(&mut self) {
        self.dir = Vec3::new(-self.dir.x, -self.dir.y, -self.dir.z);
    }

    /// `Reversed()` — returns a copy of this direction, reversed.
    #[inline]
    pub fn reversed(&self) -> GeomDirection {
        GeomDirection {
            dir: Vec3::new(-self.dir.x, -self.dir.y, -self.dir.z),
        }
    }

    // ── Transform ─────────────────────────────────────────────────────────────

    /// `Transform(T)` — applies the linear part of the 3D transform `t` to the
    /// direction (no translation), then re-normalizes.
    ///
    /// # Panics
    /// Panics if the linear part of `t` maps the direction to the zero vector
    /// (degenerate transform).
    #[inline]
    pub fn transform(&mut self, t: &Trsf) {
        let v = t.apply_vector(self.dir);
        *self = Self::new(v.x, v.y, v.z);
    }

    /// `Transformed(T)` — returns a new direction obtained by applying `t`,
    /// leaving `self` unchanged.
    #[inline]
    pub fn transformed(&self, t: &Trsf) -> GeomDirection {
        let mut c = *self;
        c.transform(t);
        c
    }

    // ── Copy ─────────────────────────────────────────────────────────────────

    /// `Copy()` — returns an independent copy (equivalent to `Clone` for a
    /// `Copy` type; provided for API symmetry with OCCT).
    #[inline]
    pub fn copy(&self) -> GeomDirection {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precision::{ANGULAR, CONFUSION};
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

    #[test]
    fn construct_normalizes() {
        let d = GeomDirection::new(3.0, 0.0, 4.0);
        let mag = (d.x() * d.x() + d.y() * d.y() + d.z() * d.z()).sqrt();
        assert!((mag - 1.0).abs() < CONFUSION);
    }

    #[test]
    fn unit_x() {
        let d = GeomDirection::new(1.0, 0.0, 0.0);
        assert!((d.x() - 1.0).abs() < CONFUSION);
        assert!(d.y().abs() < CONFUSION);
        assert!(d.z().abs() < CONFUSION);
    }

    #[test]
    fn set_coord_normalizes() {
        let mut d = GeomDirection::new(1.0, 0.0, 0.0);
        d.set_coord(0.0, 0.0, 5.0);
        assert!(d.x().abs() < CONFUSION);
        assert!(d.y().abs() < CONFUSION);
        assert!((d.z() - 1.0).abs() < CONFUSION);
    }

    #[test]
    fn crossed_perpendicular_magnitude() {
        let dx = GeomDirection::new(1.0, 0.0, 0.0);
        let dy = GeomDirection::new(0.0, 1.0, 0.0);
        // cross(X, Y) = Z, magnitude = sin(90°) = 1
        let c = dx.crossed(&dy);
        assert!((c.norm() - 1.0).abs() < CONFUSION);
        assert!(c.x.abs() < CONFUSION);
        assert!(c.y.abs() < CONFUSION);
        assert!((c.z - 1.0).abs() < CONFUSION);
    }

    #[test]
    fn dot_parallel() {
        let d = GeomDirection::new(1.0, 0.0, 0.0);
        assert!((d.dot(&d) - 1.0).abs() < CONFUSION);
    }

    #[test]
    fn angle_90_degrees() {
        let dx = GeomDirection::new(1.0, 0.0, 0.0);
        let dy = GeomDirection::new(0.0, 1.0, 0.0);
        let a = dx.angle(&dy);
        assert!((a - FRAC_PI_2).abs() < ANGULAR);
    }

    #[test]
    fn angle_45_degrees() {
        let dx = GeomDirection::new(1.0, 0.0, 0.0);
        let d45 = GeomDirection::new(1.0, 1.0, 0.0);
        let a = dx.angle(&d45);
        assert!((a - FRAC_PI_4).abs() < CONFUSION);
    }

    #[test]
    fn angle_antiparallel_is_pi() {
        let dx = GeomDirection::new(1.0, 0.0, 0.0);
        let neg = GeomDirection::new(-1.0, 0.0, 0.0);
        let a = dx.angle(&neg);
        assert!((a - PI).abs() < CONFUSION);
    }

    #[test]
    fn is_opposite_antiparallel() {
        let d = GeomDirection::new(1.0, 0.0, 0.0);
        let neg = GeomDirection::new(-1.0, 0.0, 0.0);
        assert!(d.is_opposite(&neg, 1e-7));
        assert!(!d.is_opposite(&d, 1e-7));
    }

    #[test]
    fn is_parallel() {
        let d = GeomDirection::new(1.0, 0.0, 0.0);
        let neg = GeomDirection::new(-1.0, 0.0, 0.0);
        assert!(d.is_parallel(&neg, 1e-7));
        let perp = GeomDirection::new(0.0, 1.0, 0.0);
        assert!(!d.is_parallel(&perp, 1e-7));
    }

    #[test]
    fn transform_rotation_90() {
        use crate::gp::{Ax1, Pnt};
        let mut d = GeomDirection::new(1.0, 0.0, 0.0);
        let axis = Ax1::new(Pnt::origin(), Vec3::new(0.0, 0.0, 1.0));
        let t = Trsf::rotation(axis, FRAC_PI_2);
        d.transform(&t);
        assert!(d.x().abs() < CONFUSION);
        assert!((d.y() - 1.0).abs() < CONFUSION);
        assert!(d.z().abs() < CONFUSION);
    }

    #[test]
    fn reversed_direction() {
        let d = GeomDirection::new(1.0, 0.0, 0.0);
        let r = d.reversed();
        assert!((r.x() + 1.0).abs() < CONFUSION);
        assert!(r.y().abs() < CONFUSION);
        assert!(r.z().abs() < CONFUSION);
    }

    #[test]
    fn coord_tuple() {
        let d = GeomDirection::new(3.0, 0.0, 4.0);
        let (x, y, z) = d.coord();
        assert!((x - d.x()).abs() < ANGULAR);
        assert!((y - d.y()).abs() < ANGULAR);
        assert!((z - d.z()).abs() < ANGULAR);
    }

    #[test]
    fn copy_is_independent() {
        let original = GeomDirection::new(1.0, 0.0, 0.0);
        let mut c = original.copy();
        c.set_coord(0.0, 1.0, 0.0);
        assert!((original.x() - 1.0).abs() < CONFUSION);
        assert!(original.y().abs() < CONFUSION);
        assert!(original.z().abs() < CONFUSION);
    }

    #[test]
    fn from_dir_round_trip() {
        let v = Vec3::new(0.0, 0.0, 1.0);
        let d = GeomDirection::from_dir(v);
        let back = d.dir();
        assert!(back.x.abs() < CONFUSION);
        assert!(back.y.abs() < CONFUSION);
        assert!((back.z - 1.0).abs() < CONFUSION);
    }
}
