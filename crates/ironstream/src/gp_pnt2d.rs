//! `gp_Pnt2d` — 2D point, mirroring OpenCascade's `gp_Pnt2d` from package
//! `TKMath/gp`.
//!
//! A `Pnt2d` is a position in the 2-D plane.  Unlike a vector it has no
//! direction semantics; it participates in translations and similarity
//! transforms but not in pure linear maps.  The API mirrors OCCT's `gp_Pnt2d`
//! exactly (snake_case for Rust idiom).

use crate::gp2d::{Ax2d, Pnt2d};
use crate::gp_trsf2d::Trsf2d;

// occt: gp_Pnt2d

/// A point in the 2-D plane.
///
/// Wraps an `(x, y)` coordinate pair and exposes the full `gp_Pnt2d` API from
/// OpenCascade: coordinate accessors/setters, distance queries, geometric
/// transforms (translate, rotate, mirror, scale) in both mutating and
/// value-returning forms, and a tolerance-based equality test.
///
/// The underlying storage is [`Pnt2d`] (aliased to `gp_XY` / `gp_Vec2d` in
/// this crate); use [`Pnt2d::new`] directly when raw arithmetic is needed.
// occt: gp_Pnt2d
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpPnt2d {
    coord: Pnt2d,
}

impl GpPnt2d {
    // ── Construction ──────────────────────────────────────────────────────────

    /// `gp_Pnt2d(Xp, Yp)` — construct a point from two coordinates.
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self {
            coord: Pnt2d::new(x, y),
        }
    }

    /// Origin `(0, 0)`.
    #[inline]
    pub const fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Construct from a raw [`Pnt2d`] coordinate pair.
    #[inline]
    pub const fn from_pnt2d(p: Pnt2d) -> Self {
        Self { coord: p }
    }

    /// Return the underlying [`Pnt2d`] coordinate pair.
    #[inline]
    pub const fn as_pnt2d(self) -> Pnt2d {
        self.coord
    }

    // ── Coordinate setters ────────────────────────────────────────────────────

    /// `SetCoord(Xp, Yp)` — assign both coordinates.
    #[inline]
    pub fn set_coord(&mut self, x: f64, y: f64) {
        self.coord.x = x;
        self.coord.y = y;
    }

    /// `SetCoord(Index, Xi)` — assign a single coordinate by 1-based index.
    ///
    /// Panics when `index` is not 1 or 2, matching OCCT's
    /// `Standard_OutOfRange`.
    #[inline]
    pub fn set_coord_index(&mut self, index: usize, value: f64) {
        match index {
            1 => self.coord.x = value,
            2 => self.coord.y = value,
            _ => panic!("gp_Pnt2d::SetCoord — index {index} out of range [1, 2]"),
        }
    }

    /// `SetX(X)`.
    #[inline]
    pub fn set_x(&mut self, x: f64) {
        self.coord.x = x;
    }

    /// `SetY(Y)`.
    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.coord.y = y;
    }

    // ── Coordinate accessors ──────────────────────────────────────────────────

    /// `Coord(Xp, Yp)` — retrieve both coordinates as a tuple.
    #[inline]
    pub fn coord(self) -> (f64, f64) {
        (self.coord.x, self.coord.y)
    }

    /// `Coord(Index)` — retrieve a single coordinate by 1-based index.
    ///
    /// Panics when `index` is not 1 or 2.
    #[inline]
    pub fn coord_index(self, index: usize) -> f64 {
        match index {
            1 => self.coord.x,
            2 => self.coord.y,
            _ => panic!("gp_Pnt2d::Coord — index {index} out of range [1, 2]"),
        }
    }

    /// `X()`.
    #[inline]
    pub fn x(self) -> f64 {
        self.coord.x
    }

    /// `Y()`.
    #[inline]
    pub fn y(self) -> f64 {
        self.coord.y
    }

    // ── Distance ──────────────────────────────────────────────────────────────

    /// `Distance(Other)` — Euclidean distance to `other`.
    #[inline]
    pub fn distance(self, other: GpPnt2d) -> f64 {
        self.coord.distance(other.coord)
    }

    /// `SquareDistance(Other)` — squared Euclidean distance to `other`.
    #[inline]
    pub fn square_distance(self, other: GpPnt2d) -> f64 {
        let dx = self.coord.x - other.coord.x;
        let dy = self.coord.y - other.coord.y;
        dx * dx + dy * dy
    }

    // ── Equality ──────────────────────────────────────────────────────────────

    /// `IsEqual(Other, LinearTolerance)` — true when the Euclidean distance to
    /// `other` is at most `tol`.
    #[inline]
    pub fn is_equal(self, other: GpPnt2d, tol: f64) -> bool {
        self.distance(other) <= tol
    }

    // ── Translate ─────────────────────────────────────────────────────────────

    /// `Translate(V)` — move self by the vector `v` (in-place).
    #[inline]
    pub fn translate(&mut self, v: Pnt2d) {
        self.coord.x += v.x;
        self.coord.y += v.y;
    }

    /// `Translate(P1, P2)` — move self by the vector P1 → P2 (in-place).
    #[inline]
    pub fn translate_2pts(&mut self, p1: GpPnt2d, p2: GpPnt2d) {
        self.coord.x += p2.coord.x - p1.coord.x;
        self.coord.y += p2.coord.y - p1.coord.y;
    }

    /// `Translated(V)` — returns a copy translated by vector `v`.
    #[inline]
    pub fn translated(self, v: Pnt2d) -> GpPnt2d {
        GpPnt2d::new(self.coord.x + v.x, self.coord.y + v.y)
    }

    /// `Translated(P1, P2)` — returns a copy translated by P1 → P2.
    #[inline]
    pub fn translated_2pts(self, p1: GpPnt2d, p2: GpPnt2d) -> GpPnt2d {
        GpPnt2d::new(
            self.coord.x + p2.coord.x - p1.coord.x,
            self.coord.y + p2.coord.y - p1.coord.y,
        )
    }

    // ── Rotate ────────────────────────────────────────────────────────────────

    /// `Rotate(P, Ang)` — rotate self about point `center` by `angle` radians
    /// counter-clockwise (in-place).
    #[inline]
    pub fn rotate(&mut self, center: GpPnt2d, angle: f64) {
        *self = self.rotated(center, angle);
    }

    /// `Rotated(P, Ang)` — returns a copy rotated about `center` by `angle`
    /// radians counter-clockwise.
    #[inline]
    pub fn rotated(self, center: GpPnt2d, angle: f64) -> GpPnt2d {
        let (s, c) = angle.sin_cos();
        let dx = self.coord.x - center.coord.x;
        let dy = self.coord.y - center.coord.y;
        GpPnt2d::new(
            center.coord.x + c * dx - s * dy,
            center.coord.y + s * dx + c * dy,
        )
    }

    // ── Mirror ────────────────────────────────────────────────────────────────

    /// `Mirror(P)` — reflect self through point `p` (in-place).
    #[inline]
    pub fn mirror_point(&mut self, p: GpPnt2d) {
        *self = self.mirrored_point(p);
    }

    /// `Mirrored(P)` — returns a copy reflected through point `p`.
    #[inline]
    pub fn mirrored_point(self, p: GpPnt2d) -> GpPnt2d {
        GpPnt2d::new(
            2.0 * p.coord.x - self.coord.x,
            2.0 * p.coord.y - self.coord.y,
        )
    }

    /// `Mirror(A)` — reflect self across the axis `a` (in-place).
    #[inline]
    pub fn mirror_axis(&mut self, a: Ax2d) {
        *self = self.mirrored_axis(a);
    }

    /// `Mirrored(A)` — returns a copy reflected across the axis `a`.
    ///
    /// `a` is a line defined by a location point and a unit direction.  The
    /// reflection formula is the standard Householder-style 2D line reflection.
    pub fn mirrored_axis(self, a: Ax2d) -> GpPnt2d {
        let d = a.direction.normalized();
        let dx = d.x;
        let dy = d.y;
        // Reflect p across the line through a.location with direction (dx, dy).
        // rel = p - location
        let rx = self.coord.x - a.location.x;
        let ry = self.coord.y - a.location.y;
        // proj = (rel · d) * d
        let dot = rx * dx + ry * dy;
        let px = dot * dx;
        let py = dot * dy;
        // reflected_rel = 2*proj - rel
        let fx = 2.0 * px - rx;
        let fy = 2.0 * py - ry;
        GpPnt2d::new(a.location.x + fx, a.location.y + fy)
    }

    // ── Scale ─────────────────────────────────────────────────────────────────

    /// `Scale(P, S)` — scale self about `center` by factor `s` (in-place).
    #[inline]
    pub fn scale(&mut self, center: GpPnt2d, s: f64) {
        *self = self.scaled(center, s);
    }

    /// `Scaled(P, S)` — returns a copy scaled about `center` by factor `s`.
    #[inline]
    pub fn scaled(self, center: GpPnt2d, s: f64) -> GpPnt2d {
        GpPnt2d::new(
            center.coord.x + (self.coord.x - center.coord.x) * s,
            center.coord.y + (self.coord.y - center.coord.y) * s,
        )
    }

    // ── Transform ─────────────────────────────────────────────────────────────

    /// `Transform(T)` — apply the 2D affine transform `t` in place.
    #[inline]
    pub fn transform(&mut self, t: &Trsf2d) {
        self.coord = t.apply(self.coord);
    }

    /// `Transformed(T)` — returns a copy with the 2D affine transform `t`
    /// applied.
    #[inline]
    pub fn transformed(self, t: &Trsf2d) -> GpPnt2d {
        GpPnt2d::from_pnt2d(t.apply(self.coord))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precision::{ANGULAR, CONFUSION};
    use std::f64::consts::{FRAC_PI_2, PI};

    fn near(a: f64, b: f64) {
        assert!(
            (a - b).abs() < CONFUSION,
            "expected {b}, got {a} (diff {})",
            (a - b).abs()
        );
    }

    #[test]
    fn constructor_and_accessors() {
        let p = GpPnt2d::new(3.0, 4.0);
        near(p.x(), 3.0);
        near(p.y(), 4.0);
        let (x, y) = p.coord();
        near(x, 3.0);
        near(y, 4.0);
        near(p.coord_index(1), 3.0);
        near(p.coord_index(2), 4.0);
    }

    #[test]
    fn set_coord() {
        let mut p = GpPnt2d::origin();
        p.set_coord(5.0, -2.0);
        near(p.x(), 5.0);
        near(p.y(), -2.0);
        p.set_x(10.0);
        near(p.x(), 10.0);
        p.set_y(20.0);
        near(p.y(), 20.0);
        p.set_coord_index(1, 1.0);
        near(p.x(), 1.0);
        p.set_coord_index(2, 2.0);
        near(p.y(), 2.0);
    }

    #[test]
    fn distance_and_square_distance() {
        let a = GpPnt2d::new(0.0, 0.0);
        let b = GpPnt2d::new(3.0, 4.0);
        near(a.distance(b), 5.0);
        near(a.square_distance(b), 25.0);
    }

    #[test]
    fn is_equal() {
        let a = GpPnt2d::new(1.0, 2.0);
        let b = GpPnt2d::new(1.0 + CONFUSION * 0.5, 2.0);
        assert!(a.is_equal(b, CONFUSION));
        let c = GpPnt2d::new(1.0 + CONFUSION * 2.0, 2.0);
        assert!(!a.is_equal(c, CONFUSION));
    }

    #[test]
    fn translate() {
        let p = GpPnt2d::new(1.0, 2.0);
        let v = Pnt2d::new(3.0, 4.0);
        let q = p.translated(v);
        near(q.x(), 4.0);
        near(q.y(), 6.0);

        // in-place
        let mut r = p;
        r.translate(v);
        near(r.x(), 4.0);
        near(r.y(), 6.0);

        // two-point form
        let p1 = GpPnt2d::new(1.0, 1.0);
        let p2 = GpPnt2d::new(4.0, 5.0);
        let t = p.translated_2pts(p1, p2);
        near(t.x(), 4.0);
        near(t.y(), 6.0);

        let _ = ANGULAR; // use constant in test module
    }

    #[test]
    fn rotate_about_origin() {
        let p = GpPnt2d::new(1.0, 0.0);
        let center = GpPnt2d::origin();
        let q = p.rotated(center, FRAC_PI_2);
        assert!(q.x().abs() < CONFUSION * 10.0);
        assert!((q.y() - 1.0).abs() < CONFUSION * 10.0);

        // 180°
        let q2 = p.rotated(center, PI);
        assert!((q2.x() + 1.0).abs() < CONFUSION * 10.0);
        assert!(q2.y().abs() < CONFUSION * 10.0);

        // in-place
        let mut r = p;
        r.rotate(center, FRAC_PI_2);
        assert!(r.x().abs() < CONFUSION * 10.0);
        assert!((r.y() - 1.0).abs() < CONFUSION * 10.0);
    }

    #[test]
    fn rotate_about_non_origin() {
        // Rotating (2, 1) by 90° about (1, 1) should give (1, 2).
        let p = GpPnt2d::new(2.0, 1.0);
        let c = GpPnt2d::new(1.0, 1.0);
        let q = p.rotated(c, FRAC_PI_2);
        assert!((q.x() - 1.0).abs() < CONFUSION * 10.0, "x={}", q.x());
        assert!((q.y() - 2.0).abs() < CONFUSION * 10.0, "y={}", q.y());
    }

    #[test]
    fn mirror_point() {
        let p = GpPnt2d::new(3.0, 4.0);
        let center = GpPnt2d::new(1.0, 1.0);
        let q = p.mirrored_point(center);
        near(q.x(), -1.0);
        near(q.y(), -2.0);

        // in-place
        let mut r = p;
        r.mirror_point(center);
        near(r.x(), -1.0);
        near(r.y(), -2.0);
    }

    #[test]
    fn mirror_axis() {
        // Mirror across the X axis: (x, y) -> (x, -y).
        let axis = Ax2d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0));
        let p = GpPnt2d::new(3.0, 4.0);
        let q = p.mirrored_axis(axis);
        near(q.x(), 3.0);
        near(q.y(), -4.0);

        // Mirror across the Y axis: (x, y) -> (-x, y).
        let axis_y = Ax2d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(0.0, 1.0));
        let r = p.mirrored_axis(axis_y);
        near(r.x(), -3.0);
        near(r.y(), 4.0);

        // in-place
        let mut s = p;
        s.mirror_axis(axis);
        near(s.x(), 3.0);
        near(s.y(), -4.0);
    }

    #[test]
    fn scale() {
        let p = GpPnt2d::new(4.0, 6.0);
        let center = GpPnt2d::new(2.0, 2.0);
        let q = p.scaled(center, 2.0);
        near(q.x(), 6.0);
        near(q.y(), 10.0);

        // in-place
        let mut r = p;
        r.scale(center, 0.5);
        near(r.x(), 3.0);
        near(r.y(), 4.0);
    }

    #[test]
    fn transform() {
        // Use a rotation Trsf2d and verify Transform/Transformed.
        let mut t = Trsf2d::identity();
        t.set_rotation(Pnt2d::origin(), FRAC_PI_2);

        let p = GpPnt2d::new(1.0, 0.0);
        let q = p.transformed(&t);
        assert!(q.x().abs() < CONFUSION * 10.0, "x={}", q.x());
        assert!((q.y() - 1.0).abs() < CONFUSION * 10.0, "y={}", q.y());

        // in-place
        let mut r = p;
        r.transform(&t);
        assert!(r.x().abs() < CONFUSION * 10.0, "x={}", r.x());
        assert!((r.y() - 1.0).abs() < CONFUSION * 10.0, "y={}", r.y());

        // Translation transform
        let mut t2 = Trsf2d::identity();
        t2.set_translation(Pnt2d::new(5.0, 7.0));
        let s = GpPnt2d::new(1.0, 2.0).transformed(&t2);
        near(s.x(), 6.0);
        near(s.y(), 9.0);
    }

    #[test]
    fn from_pnt2d_roundtrip() {
        let raw = Pnt2d::new(7.0, -3.0);
        let p = GpPnt2d::from_pnt2d(raw);
        let back = p.as_pnt2d();
        assert_eq!(back, raw);
    }
}
