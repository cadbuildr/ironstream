//! `geom_point2d` — standalone 2-D point types with no external dependencies.
//!
//! Provides two complementary types that mirror OCCT's layered point API:
//!
//! * [`Pnt2d`] — a lightweight value type corresponding to `gp_Pnt2d`.
//! * [`CartesianPoint2d`] — a persistent wrapper corresponding to
//!   `Geom2d_CartesianPoint`.
//!
//! Free functions:
//! * [`midpoint_2d`] — arithmetic midpoint of two [`Pnt2d`] values.
//! * [`barycenter_2d`] — weighted barycenter of a slice of [`Pnt2d`] values.
//!
//! This module is intentionally zero-dependency: it uses `std` only.

// ── Pnt2d ─────────────────────────────────────────────────────────────────────

/// A point in the 2-D plane, mirroring `gp_Pnt2d` from OpenCASCADE.
///
/// Stores a bare `(x, y)` coordinate pair.  Unlike a direction or vector,
/// a `Pnt2d` has position semantics; it participates in translations,
/// reflections, and scaling but not in pure linear maps.
// occt-ref: gp_Pnt2d
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pnt2d {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
}

impl Pnt2d {
    /// `gp_Pnt2d(X, Y)` — construct a point from two coordinates.
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Return the origin `(0, 0)`.
    #[inline]
    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// `Distance(Other)` — Euclidean distance to `other`.
    #[inline]
    pub fn distance(&self, other: &Pnt2d) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// `Translate(dx, dy)` — shift the point in place by the given offsets.
    #[inline]
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    /// `Mirror(Y-axis)` — reflect across the Y axis, returning a new point.
    ///
    /// Equivalent to `(−x, y)`.
    #[inline]
    pub fn mirror_x(&self) -> Pnt2d {
        Pnt2d {
            x: -self.x,
            y: self.y,
        }
    }

    /// `Mirror(X-axis)` — reflect across the X axis, returning a new point.
    ///
    /// Equivalent to `(x, −y)`.
    #[inline]
    pub fn mirror_y(&self) -> Pnt2d {
        Pnt2d {
            x: self.x,
            y: -self.y,
        }
    }

    /// `Scaled(factor)` — uniform scale about the origin, returning a new
    /// point.
    #[inline]
    pub fn scale(&self, factor: f64) -> Pnt2d {
        Pnt2d {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

// ── CartesianPoint2d ──────────────────────────────────────────────────────────

/// A persistent 2-D Cartesian point, wrapping [`Pnt2d`] and mirroring
/// `Geom2d_CartesianPoint` from OpenCASCADE.
///
/// This type adds named coordinate accessors / mutators on top of the bare
/// [`Pnt2d`] struct.
// occt-ref: Geom2d_CartesianPoint
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CartesianPoint2d {
    /// The underlying `gp_Pnt2d`-equivalent coordinate pair.
    pub coords: Pnt2d,
}

impl CartesianPoint2d {
    /// `Geom2d_CartesianPoint(X, Y)` — construct from two coordinates.
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            coords: Pnt2d::new(x, y),
        }
    }

    /// `X()` — return the X coordinate.
    #[inline]
    pub fn x(&self) -> f64 {
        self.coords.x
    }

    /// `Y()` — return the Y coordinate.
    #[inline]
    pub fn y(&self) -> f64 {
        self.coords.y
    }

    /// `SetX(V)` — set the X coordinate.
    #[inline]
    pub fn set_x(&mut self, v: f64) {
        self.coords.x = v;
    }

    /// `SetY(V)` — set the Y coordinate.
    #[inline]
    pub fn set_y(&mut self, v: f64) {
        self.coords.y = v;
    }
}

// ── Free functions ────────────────────────────────────────────────────────────

/// Return the arithmetic midpoint of two points.
///
/// Equivalent to `barycenter_2d(&[*p1, *p2], &[1.0, 1.0])` but faster.
#[inline]
pub fn midpoint_2d(p1: &Pnt2d, p2: &Pnt2d) -> Pnt2d {
    Pnt2d {
        x: (p1.x + p2.x) * 0.5,
        y: (p1.y + p2.y) * 0.5,
    }
}

/// Return the weighted barycenter of a set of points.
///
/// # Panics
///
/// Panics when `pts` and `weights` have different lengths, or when `pts` is
/// empty, or when the total weight is zero.
pub fn barycenter_2d(pts: &[Pnt2d], weights: &[f64]) -> Pnt2d {
    assert_eq!(
        pts.len(),
        weights.len(),
        "barycenter_2d: pts and weights must have the same length"
    );
    assert!(
        !pts.is_empty(),
        "barycenter_2d: at least one point is required"
    );

    let mut wx = 0.0_f64;
    let mut wy = 0.0_f64;
    let mut total = 0.0_f64;

    for (p, &w) in pts.iter().zip(weights.iter()) {
        wx += p.x * w;
        wy += p.y * w;
        total += w;
    }

    assert!(
        total != 0.0,
        "barycenter_2d: total weight must not be zero"
    );

    Pnt2d {
        x: wx / total,
        y: wy / total,
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-10;

    fn near(a: f64, b: f64) {
        assert!(
            (a - b).abs() < EPS,
            "expected {b}, got {a} (diff {})",
            (a - b).abs()
        );
    }

    // ── Pnt2d ──────────────────────────────────────────────────────────────

    #[test]
    fn pnt2d_new_and_origin() {
        let p = Pnt2d::new(3.0, 4.0);
        near(p.x, 3.0);
        near(p.y, 4.0);

        let o = Pnt2d::origin();
        near(o.x, 0.0);
        near(o.y, 0.0);
    }

    #[test]
    fn pnt2d_distance() {
        let a = Pnt2d::new(0.0, 0.0);
        let b = Pnt2d::new(3.0, 4.0);
        near(a.distance(&b), 5.0);
        near(b.distance(&a), 5.0);
        near(a.distance(&a), 0.0);
    }

    #[test]
    fn pnt2d_translate() {
        let mut p = Pnt2d::new(1.0, 2.0);
        p.translate(3.0, -1.0);
        near(p.x, 4.0);
        near(p.y, 1.0);
    }

    #[test]
    fn pnt2d_mirror_x() {
        let p = Pnt2d::new(3.0, 4.0);
        let q = p.mirror_x();
        near(q.x, -3.0);
        near(q.y, 4.0);
    }

    #[test]
    fn pnt2d_mirror_y() {
        let p = Pnt2d::new(3.0, 4.0);
        let q = p.mirror_y();
        near(q.x, 3.0);
        near(q.y, -4.0);
    }

    #[test]
    fn pnt2d_scale() {
        let p = Pnt2d::new(2.0, 5.0);
        let q = p.scale(3.0);
        near(q.x, 6.0);
        near(q.y, 15.0);

        // scale by zero -> origin
        let z = p.scale(0.0);
        near(z.x, 0.0);
        near(z.y, 0.0);
    }

    // ── CartesianPoint2d ───────────────────────────────────────────────────

    #[test]
    fn cartesian_point2d_new_and_accessors() {
        let p = CartesianPoint2d::new(7.0, -3.0);
        near(p.x(), 7.0);
        near(p.y(), -3.0);
    }

    #[test]
    fn cartesian_point2d_set_x_y() {
        let mut p = CartesianPoint2d::new(0.0, 0.0);
        p.set_x(5.0);
        p.set_y(-9.0);
        near(p.x(), 5.0);
        near(p.y(), -9.0);
    }

    #[test]
    fn cartesian_point2d_coords_field() {
        let p = CartesianPoint2d::new(1.0, 2.0);
        near(p.coords.x, 1.0);
        near(p.coords.y, 2.0);
    }

    // ── midpoint_2d ────────────────────────────────────────────────────────

    #[test]
    fn midpoint_basic() {
        let a = Pnt2d::new(0.0, 0.0);
        let b = Pnt2d::new(4.0, 6.0);
        let m = midpoint_2d(&a, &b);
        near(m.x, 2.0);
        near(m.y, 3.0);
    }

    #[test]
    fn midpoint_same_point() {
        let p = Pnt2d::new(3.0, 7.0);
        let m = midpoint_2d(&p, &p);
        near(m.x, 3.0);
        near(m.y, 7.0);
    }

    // ── barycenter_2d ──────────────────────────────────────────────────────

    #[test]
    fn barycenter_equal_weights() {
        let pts = [Pnt2d::new(0.0, 0.0), Pnt2d::new(4.0, 0.0), Pnt2d::new(2.0, 4.0)];
        let weights = [1.0, 1.0, 1.0];
        let b = barycenter_2d(&pts, &weights);
        near(b.x, 2.0);
        near(b.y, 4.0 / 3.0);
    }

    #[test]
    fn barycenter_unequal_weights() {
        // All weight on the second point -> barycenter is that point.
        let pts = [Pnt2d::new(0.0, 0.0), Pnt2d::new(5.0, 5.0)];
        let weights = [0.0, 1.0];
        let b = barycenter_2d(&pts, &weights);
        near(b.x, 5.0);
        near(b.y, 5.0);
    }

    #[test]
    fn barycenter_single_point() {
        let pts = [Pnt2d::new(3.0, -2.0)];
        let weights = [42.0];
        let b = barycenter_2d(&pts, &weights);
        near(b.x, 3.0);
        near(b.y, -2.0);
    }

    #[test]
    #[should_panic(expected = "same length")]
    fn barycenter_length_mismatch_panics() {
        let pts = [Pnt2d::new(0.0, 0.0)];
        let weights = [1.0, 2.0];
        let _ = barycenter_2d(&pts, &weights);
    }

    #[test]
    #[should_panic(expected = "at least one point")]
    fn barycenter_empty_panics() {
        let _ = barycenter_2d(&[], &[]);
    }

    #[test]
    #[should_panic(expected = "total weight must not be zero")]
    fn barycenter_zero_weight_panics() {
        let pts = [Pnt2d::new(1.0, 2.0)];
        let weights = [0.0];
        let _ = barycenter_2d(&pts, &weights);
    }
}
