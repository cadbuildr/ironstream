//! `Geom2d_CartesianPoint` -- a persistent 2D Cartesian point, wrapping
//! `gp_Pnt2d`. Mirrors OpenCascade's `Geom2d_CartesianPoint` from package
//! `TKG2d`.
//!
//! The struct stores a `gp_Pnt2d` (`Pnt2d` in IronStream) internally and
//! exposes the same interface as the OCCT class:
//!
//! * `new(x, y)` / `from_pnt2d(p)` — constructors.
//! * `set_coord(x, y)` / `set_x(x)` / `set_y(y)` — coordinate setters.
//! * `x()` / `y()` / `coord()` — coordinate getters.
//! * `pnt2d()` — returns the underlying `gp_Pnt2d` value.
//! * `distance(other)` — Euclidean distance to another point.
//! * `transform(t)` / `transformed(t)` — in-place and copy transform.

use crate::gp2d::{Pnt2d, Trsf2d};

/// A persistent 2D Cartesian point (`Geom2d_CartesianPoint`).
///
/// Wraps a [`Pnt2d`] (`gp_Pnt2d`) and provides the full OCCT
/// `Geom2d_CartesianPoint` API.
// occt: Geom2d_CartesianPoint
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CartesianPoint2d {
    pnt: Pnt2d,
}

impl CartesianPoint2d {
    /// `Geom2d_CartesianPoint(X, Y)` — creates a point from two coordinates.
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pnt: Pnt2d::new(x, y),
        }
    }

    /// `Geom2d_CartesianPoint(const gp_Pnt2d& P)` — creates from an existing
    /// `gp_Pnt2d`.
    #[inline]
    pub fn from_pnt2d(p: Pnt2d) -> Self {
        Self { pnt: p }
    }

    /// `SetCoord(X, Y)` — sets both coordinates at once.
    #[inline]
    pub fn set_coord(&mut self, x: f64, y: f64) {
        self.pnt.x = x;
        self.pnt.y = y;
    }

    /// `SetX(X)` — sets the X coordinate.
    #[inline]
    pub fn set_x(&mut self, x: f64) {
        self.pnt.x = x;
    }

    /// `SetY(Y)` — sets the Y coordinate.
    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.pnt.y = y;
    }

    /// `X()` — returns the X coordinate.
    #[inline]
    pub fn x(&self) -> f64 {
        self.pnt.x
    }

    /// `Y()` — returns the Y coordinate.
    #[inline]
    pub fn y(&self) -> f64 {
        self.pnt.y
    }

    /// `Coord(X, Y)` — returns both coordinates as a tuple `(x, y)`.
    #[inline]
    pub fn coord(&self) -> (f64, f64) {
        (self.pnt.x, self.pnt.y)
    }

    /// `Pnt2d()` — returns the underlying `gp_Pnt2d`.
    #[inline]
    pub fn pnt2d(&self) -> Pnt2d {
        self.pnt
    }

    /// `Distance(const Geom2d_CartesianPoint& Other)` — Euclidean distance
    /// to another `CartesianPoint2d`.
    #[inline]
    pub fn distance(&self, other: &CartesianPoint2d) -> f64 {
        self.pnt.distance(other.pnt)
    }

    /// `Transform(const gp_Trsf2d& T)` — applies the 2D affine transform
    /// in place, replacing the stored point with `T.apply(pnt)`.
    #[inline]
    pub fn transform(&mut self, t: &Trsf2d) {
        self.pnt = t.apply(self.pnt);
    }

    /// `Transformed(T)` — non-mutating variant; returns a new transformed
    /// point leaving `self` unchanged.
    #[inline]
    pub fn transformed(&self, t: &Trsf2d) -> CartesianPoint2d {
        CartesianPoint2d {
            pnt: t.apply(self.pnt),
        }
    }

    /// `Copy()` — a deep, independent copy (equivalent to `Clone` for
    /// a `Copy` type, provided for API symmetry with OCCT).
    #[inline]
    pub fn copy(&self) -> CartesianPoint2d {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precision::CONFUSION;

    #[test]
    fn construct_xy() {
        let p = CartesianPoint2d::new(3.0, 4.0);
        assert!((p.x() - 3.0).abs() < CONFUSION);
        assert!((p.y() - 4.0).abs() < CONFUSION);
    }

    #[test]
    fn construct_from_pnt2d() {
        let gp = Pnt2d::new(-1.0, 2.5);
        let p = CartesianPoint2d::from_pnt2d(gp);
        assert_eq!(p.pnt2d(), gp);
    }

    #[test]
    fn set_coord_updates_both() {
        let mut p = CartesianPoint2d::new(0.0, 0.0);
        p.set_coord(7.0, -3.0);
        assert!((p.x() - 7.0).abs() < CONFUSION);
        assert!((p.y() + 3.0).abs() < CONFUSION);
    }

    #[test]
    fn distance_pythagoras() {
        let a = CartesianPoint2d::new(0.0, 0.0);
        let b = CartesianPoint2d::new(3.0, 4.0);
        assert!((a.distance(&b) - 5.0).abs() < CONFUSION);
    }

    #[test]
    fn transform_translation() {
        let t = Trsf2d::translation(Pnt2d::new(10.0, -5.0));
        let mut p = CartesianPoint2d::new(1.0, 2.0);
        p.transform(&t);
        assert!((p.x() - 11.0).abs() < CONFUSION);
        assert!((p.y() + 3.0).abs() < CONFUSION);
    }
}
