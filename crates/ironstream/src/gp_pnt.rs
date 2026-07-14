//! `gp_Pnt` — 3D point, mirroring OpenCascade's `gp_Pnt` from package
//! `TKMath/gp`.
//!
//! A `Pnt` is a position in 3-D space.  Unlike a vector it has no direction
//! semantics; it participates in translations and similarity transforms but
//! not in pure linear maps.  The API mirrors OCCT's `gp_Pnt` exactly
//! (snake_case for Rust idiom).

// occt-ref: gp_Pnt

/// A point in 3-D space.
///
/// Stores an `(x, y, z)` coordinate triple and exposes the full `gp_Pnt` API
/// from OpenCascade: coordinate accessors, distance queries, geometric
/// operations (translate, scale, mirror), a tolerance-based equality test, a
/// barycenter constructor, and free-function helpers ([`midpoint`],
/// [`distance_3pts`]).
///
/// Only `std` is used (`f64::sqrt`); no external crates are required.
// occt-ref: gp_Pnt
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pnt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Pnt {
    // ── Construction ──────────────────────────────────────────────────────────

    /// `gp_Pnt(Xp, Yp, Zp)` — construct a point from three coordinates.
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Origin `(0, 0, 0)`.
    #[inline]
    pub const fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    // ── Coordinate accessors ──────────────────────────────────────────────────

    /// `Coord()` — return coordinates as `[x, y, z]`.
    #[inline]
    pub fn coord(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    // ── Distance ──────────────────────────────────────────────────────────────

    /// `Distance(Other)` — Euclidean distance to `other`.
    #[inline]
    pub fn distance(&self, other: &Pnt) -> f64 {
        self.square_distance(other).sqrt()
    }

    /// `SquareDistance(Other)` — squared Euclidean distance to `other`.
    #[inline]
    pub fn square_distance(&self, other: &Pnt) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }

    // ── Equality ──────────────────────────────────────────────────────────────

    /// `IsEqual(Other, LinearTolerance)` — true when the Euclidean distance to
    /// `other` is at most `tol`.
    #[inline]
    pub fn is_equal(&self, other: &Pnt, tol: f64) -> bool {
        self.distance(other) <= tol
    }

    // ── Translate ─────────────────────────────────────────────────────────────

    /// `Translate(V)` — move self by the vector `v` given as `[dx, dy, dz]`
    /// (in-place).
    #[inline]
    pub fn translate(&mut self, v: [f64; 3]) {
        self.x += v[0];
        self.y += v[1];
        self.z += v[2];
    }

    // ── Scale ─────────────────────────────────────────────────────────────────

    /// `Scaled(Center, S)` — returns a copy of self scaled about `center` by
    /// `factor`.
    ///
    /// Each coordinate is mapped: `coord_new = center + (coord - center) * factor`.
    #[inline]
    pub fn scale_from(&self, center: &Pnt, factor: f64) -> Pnt {
        Pnt::new(
            center.x + (self.x - center.x) * factor,
            center.y + (self.y - center.y) * factor,
            center.z + (self.z - center.z) * factor,
        )
    }

    // ── Mirror ────────────────────────────────────────────────────────────────

    /// `Mirrored(Center)` — returns a copy of self reflected through the point
    /// `center`.
    ///
    /// Formula: `coord_new = 2 * center - coord`.
    #[inline]
    pub fn mirror_point(&self, center: &Pnt) -> Pnt {
        Pnt::new(
            2.0 * center.x - self.x,
            2.0 * center.y - self.y,
            2.0 * center.z - self.z,
        )
    }

    // ── Barycenter ────────────────────────────────────────────────────────────

    /// Weighted barycenter of two points.
    ///
    /// Returns `(p1 * w1 + p2 * w2) / (w1 + w2)`.
    ///
    /// Panics in debug mode if `w1 + w2` is zero (degenerate weights).
    #[inline]
    pub fn barycenter(p1: &Pnt, w1: f64, p2: &Pnt, w2: f64) -> Pnt {
        let total = w1 + w2;
        debug_assert!(total != 0.0, "gp_Pnt::barycenter — total weight is zero");
        let inv = 1.0 / total;
        Pnt::new(
            (p1.x * w1 + p2.x * w2) * inv,
            (p1.y * w1 + p2.y * w2) * inv,
            (p1.z * w1 + p2.z * w2) * inv,
        )
    }
}

// ── Free functions ─────────────────────────────────────────────────────────────

/// Midpoint of two points — equivalent to `Pnt::barycenter(p1, 1.0, p2, 1.0)`.
#[inline]
pub fn midpoint(p1: &Pnt, p2: &Pnt) -> Pnt {
    Pnt::new(
        (p1.x + p2.x) * 0.5,
        (p1.y + p2.y) * 0.5,
        (p1.z + p2.z) * 0.5,
    )
}

/// Pairwise distances between three points: returns `(d12, d23, d13)`.
///
/// - `d12` = distance from `p1` to `p2`
/// - `d23` = distance from `p2` to `p3`
/// - `d13` = distance from `p1` to `p3`
///
/// The ordering mirrors `gp_Pnt::Distance` usage in OCCT tests where the
/// consecutive-pair distances come first.
#[inline]
pub fn distance_3pts(p1: &Pnt, p2: &Pnt, p3: &Pnt) -> (f64, f64, f64) {
    (p1.distance(p2), p2.distance(p3), p1.distance(p3))
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-10;

    fn near(a: f64, b: f64) {
        assert!(
            (a - b).abs() < TOL,
            "expected {b}, got {a} (diff {})",
            (a - b).abs()
        );
    }

    #[test]
    fn constructor_and_coord() {
        let p = Pnt::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
        assert_eq!(p.coord(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn origin() {
        let o = Pnt::origin();
        assert_eq!(o.coord(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn distance_and_square_distance() {
        let a = Pnt::origin();
        let b = Pnt::new(1.0, 2.0, 2.0);
        near(a.distance(&b), 3.0);
        near(a.square_distance(&b), 9.0);
    }

    #[test]
    fn is_equal() {
        let a = Pnt::new(1.0, 0.0, 0.0);
        let b = Pnt::new(1.0 + TOL * 0.5, 0.0, 0.0);
        assert!(a.is_equal(&b, TOL));
        let c = Pnt::new(1.0 + TOL * 2.0, 0.0, 0.0);
        assert!(!a.is_equal(&c, TOL));
    }

    #[test]
    fn translate() {
        let mut p = Pnt::new(1.0, 2.0, 3.0);
        p.translate([1.0, -1.0, 2.0]);
        near(p.x, 2.0);
        near(p.y, 1.0);
        near(p.z, 5.0);
    }

    #[test]
    fn scale_from() {
        let p = Pnt::new(4.0, 6.0, 8.0);
        let center = Pnt::new(2.0, 2.0, 2.0);
        let q = p.scale_from(&center, 2.0);
        near(q.x, 6.0);
        near(q.y, 10.0);
        near(q.z, 14.0);
    }

    #[test]
    fn mirror_point() {
        let p = Pnt::new(3.0, 4.0, 5.0);
        let c = Pnt::new(1.0, 1.0, 1.0);
        let q = p.mirror_point(&c);
        near(q.x, -1.0);
        near(q.y, -2.0);
        near(q.z, -3.0);
    }

    #[test]
    fn barycenter_equal_weights() {
        let p1 = Pnt::new(0.0, 0.0, 0.0);
        let p2 = Pnt::new(4.0, 6.0, 8.0);
        let b = Pnt::barycenter(&p1, 1.0, &p2, 1.0);
        near(b.x, 2.0);
        near(b.y, 3.0);
        near(b.z, 4.0);
    }

    #[test]
    fn barycenter_unequal_weights() {
        let p1 = Pnt::new(0.0, 0.0, 0.0);
        let p2 = Pnt::new(3.0, 3.0, 3.0);
        // w1=1, w2=2 => result = (0*1 + 3*2)/3 = 2
        let b = Pnt::barycenter(&p1, 1.0, &p2, 2.0);
        near(b.x, 2.0);
        near(b.y, 2.0);
        near(b.z, 2.0);
    }

    #[test]
    fn midpoint_free_fn() {
        let p1 = Pnt::new(0.0, 0.0, 0.0);
        let p2 = Pnt::new(2.0, 4.0, 6.0);
        let m = midpoint(&p1, &p2);
        near(m.x, 1.0);
        near(m.y, 2.0);
        near(m.z, 3.0);
    }

    #[test]
    fn distance_3pts_free_fn() {
        // 3-4-5 right triangle in XY plane.
        let a = Pnt::origin();
        let b = Pnt::new(3.0, 0.0, 0.0);
        let c = Pnt::new(0.0, 4.0, 0.0);
        let (d12, d23, d13) = distance_3pts(&a, &b, &c);
        near(d12, 3.0); // a→b
        near(d13, 4.0); // a→c
        near(d23, 5.0); // b→c
    }
}
