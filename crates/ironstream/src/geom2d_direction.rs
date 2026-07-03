//! `Geom2d_Direction` -- a persistent 2D unit direction, wrapping `gp_Dir2d`.
//! Mirrors OpenCascade's `Geom2d_Direction` from package `TKG2d`.
//!
//! A direction is a unit vector in the plane.  Every mutating operation that
//! takes raw coordinates or a `Pnt2d` normalizes the input so that the stored
//! vector always has magnitude 1.
//!
//! ## API mapping (OCCT → IronStream)
//!
//! | OCCT                              | IronStream                                |
//! |-----------------------------------|-------------------------------------------|
//! | `Geom2d_Direction(X, Y)`          | `Direction2d::new(x, y)`                  |
//! | `Geom2d_Direction(gp_Dir2d)`      | `Direction2d::from_dir2d(d)`              |
//! | `SetCoord(X, Y)`                  | `set_coord(x, y)`                         |
//! | `SetDir2d(gp_Dir2d)`              | `set_dir2d(d)`                            |
//! | `X()` / `Y()`                     | `x()` / `y()`                             |
//! | `Coord(X, Y)`                     | `coord()`                                 |
//! | `Dir2d()`                         | `dir2d()`                                 |
//! | `Crossed(Other)`                  | `crossed(other)`                          |
//! | `Dot(Other)`                      | `dot(other)`                              |
//! | `IsOpposite(Other, AngTol)`       | `is_opposite(other, ang_tol)`             |
//! | `IsParallel(Other, AngTol)`       | `is_parallel(other, ang_tol)`             |
//! | `Angle(Other)`                    | `angle(other)`                            |
//! | `Transform(T)`                    | `transform(t)` / `transformed(t)`         |
//! | `Reversed()` / `Reverse()`        | `reversed()` / `reverse()`               |
//! | `Copy()`                          | `copy()`                                  |

use crate::gp2d::{Pnt2d, Trsf2d};

/// A persistent unit direction in 2D space (`Geom2d_Direction`).
///
/// Stores a [`Pnt2d`] (`gp_Dir2d`) that is guaranteed to have magnitude 1.
// occt: Geom2d_Direction
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Direction2d {
    dir: Pnt2d,
}

impl Direction2d {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// `Geom2d_Direction(X, Y)` — constructs from two coordinates, which are
    /// normalized immediately.
    ///
    /// # Panics
    /// Panics if `(x, y)` is the zero vector (mirrors OCCT's
    /// `Standard_ConstructionError`).
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        let n = (x * x + y * y).sqrt();
        assert!(
            n > 0.0,
            "Geom2d_Direction: cannot build a direction from the null vector"
        );
        Self {
            dir: Pnt2d::new(x / n, y / n),
        }
    }

    /// `Geom2d_Direction(const gp_Dir2d& V)` — constructs from an already-unit
    /// `gp_Dir2d` (represented as [`Pnt2d`]).  The vector is re-normalized for
    /// safety.
    #[inline]
    pub fn from_dir2d(v: Pnt2d) -> Self {
        Self::new(v.x, v.y)
    }

    // ── Setters ───────────────────────────────────────────────────────────────

    /// `SetCoord(X, Y)` — replaces the stored direction with the normalized
    /// version of `(x, y)`.
    ///
    /// # Panics
    /// Panics if `(x, y)` is the zero vector.
    #[inline]
    pub fn set_coord(&mut self, x: f64, y: f64) {
        *self = Self::new(x, y);
    }

    /// `SetDir2d(const gp_Dir2d&)` — replaces the stored direction, normalizing
    /// the given `gp_Dir2d`.
    #[inline]
    pub fn set_dir2d(&mut self, v: Pnt2d) {
        *self = Self::from_dir2d(v);
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

    /// `Coord(X, Y)` — both components as `(x, y)`.
    #[inline]
    pub fn coord(&self) -> (f64, f64) {
        (self.dir.x, self.dir.y)
    }

    /// `Dir2d()` — the underlying `gp_Dir2d` as a [`Pnt2d`].
    #[inline]
    pub fn dir2d(&self) -> Pnt2d {
        self.dir
    }

    // ── Geometric operations ──────────────────────────────────────────────────

    /// `Crossed(Other)` — the 2D cross product (signed Z-component of the 3D
    /// cross product), i.e. `self.x * other.y - self.y * other.x`.
    ///
    /// Because both directions are unit vectors the result is `sin(angle)`.
    #[inline]
    pub fn crossed(&self, other: &Direction2d) -> f64 {
        self.dir.cross(other.dir)
    }

    /// `Dot(Other)` — the dot product of the two unit directions.
    ///
    /// Because both are unit vectors the result is `cos(angle)`.
    #[inline]
    pub fn dot(&self, other: &Direction2d) -> f64 {
        self.dir.dot(other.dir)
    }

    /// `IsOpposite(Other, AngularTolerance)` — returns `true` when the angle
    /// between `self` and the **reverse** of `other` is within `ang_tol`
    /// radians.
    ///
    /// Equivalent to checking whether `self.dot(other) < -(cos(ang_tol))`,
    /// i.e. `self · other ≤ -(1 - ang_tol²/2)` for small tolerances.  OCCT
    /// uses the exact formula `dot <= -(1 - ang_tol²/2)`.
    #[inline]
    pub fn is_opposite(&self, other: &Direction2d, ang_tol: f64) -> bool {
        let d = self.dot(other);
        // cos(π - ang_tol) = -cos(ang_tol) ≈ -(1 - ang_tol²/2) for small tol
        d <= -(1.0 - 0.5 * ang_tol * ang_tol)
    }

    /// `IsParallel(Other, AngularTolerance)` — returns `true` when `self` is
    /// parallel (co-directional **or** anti-parallel) to `other` within the
    /// given angular tolerance.
    ///
    /// The test is `|sin(angle)| ≤ sin(ang_tol)`, approximated for small
    /// tolerances as `|crossed| ≤ ang_tol`.
    #[inline]
    pub fn is_parallel(&self, other: &Direction2d, ang_tol: f64) -> bool {
        self.crossed(other).abs() <= ang_tol.sin()
    }

    /// `Angle(Other)` — the signed angle in radians from `self` to `other`,
    /// in the range `]-π, π]`.  The sign is positive for a CCW rotation.
    ///
    /// Implemented as `atan2(crossed, dot)` — this is numerically stable for
    /// all direction pairs and matches OCCT's `gp_Dir2d::Angle`.
    #[inline]
    pub fn angle(&self, other: &Direction2d) -> f64 {
        let c = self.crossed(other); // sin of angle
        let d = self.dot(other); // cos of angle
        c.atan2(d)
    }

    // ── Reversal ──────────────────────────────────────────────────────────────

    /// `Reverse()` — reverses the direction in place.
    #[inline]
    pub fn reverse(&mut self) {
        self.dir = Pnt2d::new(-self.dir.x, -self.dir.y);
    }

    /// `Reversed()` — returns a copy of this direction, reversed.
    #[inline]
    pub fn reversed(&self) -> Direction2d {
        Direction2d {
            dir: Pnt2d::new(-self.dir.x, -self.dir.y),
        }
    }

    // ── Transform ─────────────────────────────────────────────────────────────

    /// `Transform(T)` — applies the linear part of the 2D transform `t` to the
    /// direction (no translation), then re-normalizes.
    ///
    /// # Panics
    /// Panics if the linear part of `t` maps the direction to the zero vector
    /// (degenerate transform).
    #[inline]
    pub fn transform(&mut self, t: &Trsf2d) {
        let d = self.dir;
        let tx = t.m[0][0] * d.x + t.m[0][1] * d.y;
        let ty = t.m[1][0] * d.x + t.m[1][1] * d.y;
        *self = Self::new(tx, ty);
    }

    /// `Transformed(T)` — returns a new direction obtained by applying `t`,
    /// leaving `self` unchanged.
    #[inline]
    pub fn transformed(&self, t: &Trsf2d) -> Direction2d {
        let mut c = *self;
        c.transform(t);
        c
    }

    // ── Copy ─────────────────────────────────────────────────────────────────

    /// `Copy()` — returns an independent copy (equivalent to `Clone` for a
    /// `Copy` type; provided for API symmetry with OCCT).
    #[inline]
    pub fn copy(&self) -> Direction2d {
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
        let d = Direction2d::new(3.0, 4.0);
        let mag = (d.x() * d.x() + d.y() * d.y()).sqrt();
        assert!((mag - 1.0).abs() < CONFUSION);
    }

    #[test]
    fn unit_x() {
        let d = Direction2d::new(1.0, 0.0);
        assert!((d.x() - 1.0).abs() < CONFUSION);
        assert!(d.y().abs() < CONFUSION);
    }

    #[test]
    fn set_coord_normalizes() {
        let mut d = Direction2d::new(1.0, 0.0);
        d.set_coord(0.0, 5.0);
        assert!(d.x().abs() < CONFUSION);
        assert!((d.y() - 1.0).abs() < CONFUSION);
    }

    #[test]
    fn crossed_perpendicular() {
        let dx = Direction2d::new(1.0, 0.0);
        let dy = Direction2d::new(0.0, 1.0);
        // sin(90°) = 1
        assert!((dx.crossed(&dy) - 1.0).abs() < CONFUSION);
    }

    #[test]
    fn dot_parallel() {
        let d = Direction2d::new(1.0, 0.0);
        // cos(0°) = 1
        assert!((d.dot(&d) - 1.0).abs() < CONFUSION);
    }

    #[test]
    fn angle_90_degrees() {
        let dx = Direction2d::new(1.0, 0.0);
        let dy = Direction2d::new(0.0, 1.0);
        let a = dx.angle(&dy);
        assert!((a - FRAC_PI_2).abs() < ANGULAR);
    }

    #[test]
    fn is_parallel() {
        let d = Direction2d::new(1.0, 0.0);
        let neg = Direction2d::new(-1.0, 0.0);
        assert!(d.is_parallel(&neg, 1e-7));
        let perp = Direction2d::new(0.0, 1.0);
        assert!(!d.is_parallel(&perp, 1e-7));
    }

    #[test]
    fn is_opposite() {
        let d = Direction2d::new(1.0, 0.0);
        let neg = Direction2d::new(-1.0, 0.0);
        assert!(d.is_opposite(&neg, 1e-7));
        assert!(!d.is_opposite(&d, 1e-7));
    }

    #[test]
    fn transform_rotation() {
        let mut d = Direction2d::new(1.0, 0.0);
        let t = Trsf2d::rotation(Pnt2d::new(0.0, 0.0), FRAC_PI_2);
        d.transform(&t);
        assert!(d.x().abs() < CONFUSION);
        assert!((d.y() - 1.0).abs() < CONFUSION);
    }

    #[test]
    fn reversed_direction() {
        let d = Direction2d::new(1.0, 0.0);
        let r = d.reversed();
        assert!((r.x() + 1.0).abs() < CONFUSION);
        assert!(r.y().abs() < CONFUSION);
    }

    #[test]
    fn angle_negative_for_cw() {
        let dx = Direction2d::new(1.0, 0.0);
        let dy_neg = Direction2d::new(0.0, -1.0);
        // -90° (CW from +X to -Y)
        let a = dx.angle(&dy_neg);
        assert!((a + FRAC_PI_2).abs() < ANGULAR);
    }

    #[test]
    fn angle_45_degrees() {
        let dx = Direction2d::new(1.0, 0.0);
        let d45 = Direction2d::new(1.0, 1.0);
        let a = dx.angle(&d45);
        assert!((a - FRAC_PI_4).abs() < CONFUSION);
    }

    #[test]
    fn coord_tuple() {
        let d = Direction2d::new(3.0, 4.0);
        let (x, y) = d.coord();
        assert!((x - d.x()).abs() < ANGULAR);
        assert!((y - d.y()).abs() < ANGULAR);
    }

    #[test]
    fn copy_is_independent() {
        let original = Direction2d::new(1.0, 0.0);
        let mut c = original.copy();
        c.set_coord(0.0, 1.0);
        assert!((original.x() - 1.0).abs() < CONFUSION);
        assert!(original.y().abs() < CONFUSION);
    }

    #[test]
    fn angle_antiparallel_is_pi() {
        let dx = Direction2d::new(1.0, 0.0);
        let neg = Direction2d::new(-1.0, 0.0);
        let a = dx.angle(&neg).abs();
        assert!((a - PI).abs() < CONFUSION);
    }

    #[test]
    fn from_dir2d_round_trip() {
        let p = Pnt2d::new(0.0, 1.0);
        let d = Direction2d::from_dir2d(p);
        assert!((d.dir2d().x - 0.0).abs() < CONFUSION);
        assert!((d.dir2d().y - 1.0).abs() < CONFUSION);
    }
}
