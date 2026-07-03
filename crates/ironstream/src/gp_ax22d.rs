//! `gp_Ax22d` — 2-D coordinate system (origin + X-axis + Y-axis), mirroring
//! OpenCascade's `gp_Ax22d` from package `TKMath/gp`.
//!
//! A `gp_Ax22d` defines a 2-D axis placement consisting of:
//! * a **location** (origin) point,
//! * a **unit X direction**,
//! * a **unit Y direction** (derived from X and the `IsDirect` sense).
//!
//! When `IsDirect` is `true` the frame is right-handed (Y = X rotated +90°);
//! when `false` it is left-handed (Y = X rotated –90°).  The full OCCT API is
//! implemented: accessors, geometric queries (`Angle`), and all transformations
//! (`Translate`/`Translated`, `Rotate`/`Rotated`, `Mirror`/`Mirrored`,
//! `Scale`/`Scaled`, `Transform`/`Transformed`).

use crate::gp2d::Pnt2d;
use crate::gp_ax2d::Ax2d;
use crate::gp_trsf2d::Trsf2d;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Rotate a 2-D vector by `angle` radians counter-clockwise.
#[inline]
fn rotate_vec(v: Pnt2d, angle: f64) -> Pnt2d {
    let (s, c) = angle.sin_cos();
    Pnt2d::new(c * v.x - s * v.y, s * v.x + c * v.y)
}

/// Reflect a point across the line through origin `o` with unit direction `d`.
#[inline]
fn reflect_pt(p: Pnt2d, o: Pnt2d, d: Pnt2d) -> Pnt2d {
    let rx = p.x - o.x;
    let ry = p.y - o.y;
    let dot = rx * d.x + ry * d.y;
    Pnt2d::new(
        o.x + 2.0 * dot * d.x - rx,
        o.y + 2.0 * dot * d.y - ry,
    )
}

/// Reflect a direction (unit vector) across a line with unit direction `d`.
#[inline]
fn reflect_dir(v: Pnt2d, d: Pnt2d) -> Pnt2d {
    let dot = v.x * d.x + v.y * d.y;
    Pnt2d::new(2.0 * dot * d.x - v.x, 2.0 * dot * d.y - v.y)
}

/// Derive the Y direction from an X direction and the handedness flag.
///
/// Right-handed (`is_direct = true`): Y = X rotated +90° → (−x.y, x.x).
/// Left-handed (`is_direct = false`): Y = X rotated −90° → (x.y, −x.x).
#[inline]
fn y_from_x(x: Pnt2d, is_direct: bool) -> Pnt2d {
    if is_direct {
        Pnt2d::new(-x.y, x.x)
    } else {
        Pnt2d::new(x.y, -x.x)
    }
}

// ---------------------------------------------------------------------------
// Ax22d
// ---------------------------------------------------------------------------

/// A 2-D coordinate system: origin, X direction, Y direction, and handedness.
// occt: gp_Ax22d
#[derive(Clone, Copy, Debug)]
pub struct Ax22d {
    /// Origin of the coordinate system.
    location: Pnt2d,
    /// Unit X direction.
    x_dir: Pnt2d,
    /// Unit Y direction (derived from `x_dir` and `is_direct`).
    y_dir: Pnt2d,
    /// `true` ↔ right-handed (direct sense), `false` ↔ left-handed.
    is_direct: bool,
}

impl Ax22d {
    // -----------------------------------------------------------------------
    // Constructors
    // -----------------------------------------------------------------------

    /// `gp_Ax22d(P, Xdir, IsDirect)` — construct from an origin point, an X
    /// direction and a handedness flag.  The X direction is normalised; the Y
    /// direction is derived from it and the handedness.
    pub fn new(origin: Pnt2d, x_dir: Pnt2d, is_direct: bool) -> Self {
        let x = x_dir.normalized();
        let y = y_from_x(x, is_direct);
        Self {
            location: origin,
            x_dir: x,
            y_dir: y,
            is_direct,
        }
    }

    /// Convenience constructor from an origin and an X-axis (`gp_Ax2d`).
    /// The frame is right-handed by default.
    pub fn from_ax2d(origin: Pnt2d, x_axis: Ax2d) -> Self {
        Self::new(origin, x_axis.direction(), true)
    }

    /// Standard right-handed frame centred at the origin with X = (1, 0).
    pub fn standard() -> Self {
        Self::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0), true)
    }

    // -----------------------------------------------------------------------
    // Accessors
    // -----------------------------------------------------------------------

    /// `Location` — return the origin point.
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.location
    }

    /// `XDirection` — return the unit X direction.
    #[inline]
    pub fn x_direction(&self) -> Pnt2d {
        self.x_dir
    }

    /// `YDirection` — return the unit Y direction.
    #[inline]
    pub fn y_direction(&self) -> Pnt2d {
        self.y_dir
    }

    /// `XAxis` — return the X axis as a `gp_Ax2d`.
    #[inline]
    pub fn x_axis(&self) -> Ax2d {
        Ax2d::new(self.location, self.x_dir)
    }

    /// `YAxis` — return the Y axis as a `gp_Ax2d`.
    #[inline]
    pub fn y_axis(&self) -> Ax2d {
        Ax2d::new(self.location, self.y_dir)
    }

    /// `IsDirect` — `true` if the frame is right-handed.
    #[inline]
    pub fn is_direct(&self) -> bool {
        self.is_direct
    }

    /// `Angle(other)` — angle in radians from this X axis to `other`'s X axis,
    /// measured counter-clockwise, in `(−π, π]`.
    pub fn angle(&self, other: &Ax22d) -> f64 {
        // angle of other.x_dir relative to self.x_dir
        let dx = other.x_dir;
        let sx = self.x_dir;
        // Use atan2(cross, dot) to get a signed angle.
        let cross = sx.x * dx.y - sx.y * dx.x; // sin of angle
        let dot = sx.x * dx.x + sx.y * dx.y; // cos of angle
        cross.atan2(dot)
    }

    // -----------------------------------------------------------------------
    // Setters
    // -----------------------------------------------------------------------

    /// `SetLocation(P)` — change the origin.
    #[inline]
    pub fn set_location(&mut self, p: Pnt2d) {
        self.location = p;
    }

    /// `SetXDirection(V)` — change the X direction (normalises `v`); the Y
    /// direction is recomputed from the handedness flag.
    pub fn set_x_direction(&mut self, v: Pnt2d) {
        self.x_dir = v.normalized();
        self.y_dir = y_from_x(self.x_dir, self.is_direct);
    }

    /// `SetYDirection(V)` — change the Y direction (normalises `v`); the X
    /// direction is recomputed from the handedness flag.
    pub fn set_y_direction(&mut self, v: Pnt2d) {
        self.y_dir = v.normalized();
        // In a right-handed frame Y = rot(X, +90°) so X = rot(Y, −90°).
        // In a left-handed frame Y = rot(X, −90°) so X = rot(Y, +90°).
        self.x_dir = if self.is_direct {
            Pnt2d::new(self.y_dir.y, -self.y_dir.x) // rot Y by -90°
        } else {
            Pnt2d::new(-self.y_dir.y, self.y_dir.x) // rot Y by +90°
        };
    }

    // -----------------------------------------------------------------------
    // In-place transformations
    // -----------------------------------------------------------------------

    /// `Translate(V)` — shift the origin by vector `v`; axes unchanged.
    pub fn translate(&mut self, v: Pnt2d) {
        self.location.x += v.x;
        self.location.y += v.y;
    }

    /// `Translate(P1, P2)` — shift the origin by the vector P1 → P2.
    pub fn translate_2pts(&mut self, p1: Pnt2d, p2: Pnt2d) {
        self.translate(Pnt2d::new(p2.x - p1.x, p2.y - p1.y));
    }

    /// `Rotate(P, angle)` — rotate the whole frame about point `p` by `angle`
    /// radians counter-clockwise.
    pub fn rotate(&mut self, p: Pnt2d, angle: f64) {
        // Rotate the origin.
        let rel = Pnt2d::new(self.location.x - p.x, self.location.y - p.y);
        let rotated_rel = rotate_vec(rel, angle);
        self.location = Pnt2d::new(p.x + rotated_rel.x, p.y + rotated_rel.y);
        // Rotate the axes.
        self.x_dir = rotate_vec(self.x_dir, angle);
        self.y_dir = rotate_vec(self.y_dir, angle);
    }

    /// `Mirror(P)` — point symmetry about `p` (in-place).
    ///
    /// Location is reflected through `p`; both axis directions are negated.
    /// The handedness is preserved (double negation of perpendicular axes).
    pub fn mirror_point(&mut self, p: Pnt2d) {
        self.location = Pnt2d::new(
            2.0 * p.x - self.location.x,
            2.0 * p.y - self.location.y,
        );
        self.x_dir = Pnt2d::new(-self.x_dir.x, -self.x_dir.y);
        self.y_dir = Pnt2d::new(-self.y_dir.x, -self.y_dir.y);
    }

    /// `Mirror(A)` — axial symmetry (line mirror) through axis `a` (in-place).
    ///
    /// Both the location and the axis directions are reflected.  The
    /// handedness flips because one reflection reverses orientation.
    pub fn mirror_axis(&mut self, a: &Ax2d) {
        let o = a.location();
        let d = a.direction();
        self.location = reflect_pt(self.location, o, d);
        self.x_dir = reflect_dir(self.x_dir, d);
        self.y_dir = reflect_dir(self.y_dir, d);
        // A single reflection reverses handedness.
        self.is_direct = !self.is_direct;
    }

    /// `Scale(P, s)` — uniform scaling by `s` about centre `p` (in-place).
    ///
    /// The origin is scaled; the axis directions are unchanged except that a
    /// negative scale reverses them (and flips handedness).
    pub fn scale(&mut self, p: Pnt2d, s: f64) {
        self.location = Pnt2d::new(
            p.x + s * (self.location.x - p.x),
            p.y + s * (self.location.y - p.y),
        );
        if s < 0.0 {
            self.x_dir = Pnt2d::new(-self.x_dir.x, -self.x_dir.y);
            self.y_dir = Pnt2d::new(-self.y_dir.x, -self.y_dir.y);
            self.is_direct = !self.is_direct;
        }
    }

    /// `Transform(T)` — apply a 2-D affine transform in place.
    ///
    /// The origin is fully transformed; the axis directions are transformed
    /// by the linear (vectorial) part only and re-normalised.  If the
    /// transform is orientation-reversing the handedness flag is flipped.
    pub fn transform(&mut self, t: &Trsf2d) {
        self.location = t.apply(self.location);
        self.x_dir = t.apply_vector(self.x_dir).normalized();
        self.y_dir = t.apply_vector(self.y_dir).normalized();
        if t.is_negative() {
            self.is_direct = !self.is_direct;
        }
    }

    // -----------------------------------------------------------------------
    // Value-returning (functional) transformations
    // -----------------------------------------------------------------------

    /// `Translated(V)` — return a translated copy.
    pub fn translated(&self, v: Pnt2d) -> Ax22d {
        let mut ax = *self;
        ax.translate(v);
        ax
    }

    /// `Translated(P1, P2)` — return a copy translated by the vector P1 → P2.
    pub fn translated_2pts(&self, p1: Pnt2d, p2: Pnt2d) -> Ax22d {
        let mut ax = *self;
        ax.translate_2pts(p1, p2);
        ax
    }

    /// `Rotated(P, angle)` — return a rotated copy.
    pub fn rotated(&self, p: Pnt2d, angle: f64) -> Ax22d {
        let mut ax = *self;
        ax.rotate(p, angle);
        ax
    }

    /// `Mirrored(P)` — return a point-mirrored copy.
    pub fn mirrored_point(&self, p: Pnt2d) -> Ax22d {
        let mut ax = *self;
        ax.mirror_point(p);
        ax
    }

    /// `Mirrored(A)` — return a line-mirrored copy.
    pub fn mirrored_axis(&self, a: &Ax2d) -> Ax22d {
        let mut ax = *self;
        ax.mirror_axis(a);
        ax
    }

    /// `Scaled(P, s)` — return a scaled copy.
    pub fn scaled(&self, p: Pnt2d, s: f64) -> Ax22d {
        let mut ax = *self;
        ax.scale(p, s);
        ax
    }

    /// `Transformed(T)` — return a transformed copy.
    pub fn transformed(&self, t: &Trsf2d) -> Ax22d {
        let mut ax = *self;
        ax.transform(t);
        ax
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp2d::Ax2d as Ax2dSimple;
    use crate::precision::{ANGULAR, CONFUSION};
    use std::f64::consts::{FRAC_PI_2, PI};

    fn near(a: f64, b: f64, tol: f64) {
        assert!(
            (a - b).abs() <= tol,
            "expected {a} ≈ {b} (tol {tol:.2e}), delta = {}",
            (a - b).abs()
        );
    }

    fn pnt_near(p: Pnt2d, x: f64, y: f64) {
        near(p.x, x, CONFUSION);
        near(p.y, y, CONFUSION);
    }

    // ── Construction & accessors ───────────────────────────────────────────────

    #[test]
    fn constructor_right_handed() {
        let ax = Ax22d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0), true);
        pnt_near(ax.location(), 1.0, 2.0);
        pnt_near(ax.x_direction(), 1.0, 0.0);
        pnt_near(ax.y_direction(), 0.0, 1.0); // +90° from X
        assert!(ax.is_direct());
    }

    #[test]
    fn constructor_left_handed() {
        let ax = Ax22d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0), false);
        pnt_near(ax.x_direction(), 1.0, 0.0);
        pnt_near(ax.y_direction(), 0.0, -1.0); // −90° from X
        assert!(!ax.is_direct());
    }

    #[test]
    fn constructor_normalises_x_direction() {
        let ax = Ax22d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(3.0, 4.0), true);
        let x = ax.x_direction();
        near((x.x * x.x + x.y * x.y).sqrt(), 1.0, CONFUSION);
        near(x.x, 0.6, CONFUSION);
        near(x.y, 0.8, CONFUSION);
    }

    // ── Axes ──────────────────────────────────────────────────────────────────

    #[test]
    fn x_axis_and_y_axis() {
        let ax = Ax22d::new(Pnt2d::new(2.0, 3.0), Pnt2d::new(0.0, 1.0), true);
        let xa = ax.x_axis();
        let ya = ax.y_axis();
        pnt_near(xa.location(), 2.0, 3.0);
        pnt_near(xa.direction(), 0.0, 1.0);
        pnt_near(ya.location(), 2.0, 3.0);
        // Y = X rotated +90°: (0,1) → (-1, 0)
        pnt_near(ya.direction(), -1.0, 0.0);
    }

    // ── Angle ─────────────────────────────────────────────────────────────────

    #[test]
    fn angle_between_axes() {
        let a = Ax22d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), true);
        let b = Ax22d::new(Pnt2d::origin(), Pnt2d::new(0.0, 1.0), true);
        near(a.angle(&b), FRAC_PI_2, ANGULAR * 100.0);
        near(b.angle(&a), -FRAC_PI_2, ANGULAR * 100.0);

        let c = Ax22d::new(Pnt2d::origin(), Pnt2d::new(-1.0, 0.0), true);
        near(a.angle(&c), PI, ANGULAR * 100.0);
        let _ = ANGULAR;
        let _ = CONFUSION;
    }

    // ── Translate ─────────────────────────────────────────────────────────────

    #[test]
    fn translate_in_place() {
        let mut ax = Ax22d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0), true);
        ax.translate(Pnt2d::new(3.0, -1.0));
        pnt_near(ax.location(), 4.0, 1.0);
        pnt_near(ax.x_direction(), 1.0, 0.0); // unchanged
        pnt_near(ax.y_direction(), 0.0, 1.0);
    }

    #[test]
    fn translated_functional() {
        let ax = Ax22d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0), true);
        let t = ax.translated(Pnt2d::new(5.0, 7.0));
        pnt_near(t.location(), 5.0, 7.0);
        pnt_near(t.x_direction(), 1.0, 0.0);
        // original unchanged
        pnt_near(ax.location(), 0.0, 0.0);
    }

    #[test]
    fn translate_2pts() {
        let ax = Ax22d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0), true);
        let t = ax.translated_2pts(Pnt2d::new(1.0, 1.0), Pnt2d::new(4.0, 5.0));
        pnt_near(t.location(), 3.0, 4.0);
    }

    // ── Rotate ────────────────────────────────────────────────────────────────

    #[test]
    fn rotate_90_about_origin() {
        let ax = Ax22d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0), true);
        let r = ax.rotated(Pnt2d::origin(), FRAC_PI_2);
        // location (1,0) rotated 90° → (0, 1)
        near(r.location().x, 0.0, CONFUSION);
        near(r.location().y, 1.0, CONFUSION);
        // x_dir (1,0) rotated 90° → (0,1)
        near(r.x_direction().x, 0.0, CONFUSION);
        near(r.x_direction().y, 1.0, CONFUSION);
        // y_dir (0,1) rotated 90° → (-1,0)
        near(r.y_direction().x, -1.0, CONFUSION);
        near(r.y_direction().y, 0.0, CONFUSION);
        assert!(r.is_direct());
    }

    // ── Mirror through point ──────────────────────────────────────────────────

    #[test]
    fn mirror_through_point() {
        let ax = Ax22d::new(Pnt2d::new(2.0, 3.0), Pnt2d::new(1.0, 0.0), true);
        let m = ax.mirrored_point(Pnt2d::origin());
        pnt_near(m.location(), -2.0, -3.0);
        pnt_near(m.x_direction(), -1.0, 0.0);
        pnt_near(m.y_direction(), 0.0, -1.0);
        // Handedness preserved (point mirror = rotation of 180°)
        assert!(m.is_direct());
    }

    // ── Mirror through axis ───────────────────────────────────────────────────

    #[test]
    fn mirror_through_x_axis() {
        // Mirror frame at (0,1) with X=(1,0) through the global X axis.
        let ax = Ax22d::new(Pnt2d::new(0.0, 1.0), Pnt2d::new(1.0, 0.0), true);
        let mirror = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        let m = ax.mirrored_axis(&mirror);
        pnt_near(m.location(), 0.0, -1.0);
        pnt_near(m.x_direction(), 1.0, 0.0); // X along mirror line — unchanged
        // Handedness flips → is_direct=false → Y = X rotated -90° = (0,-1)
        pnt_near(m.y_direction(), 0.0, -1.0);
        assert!(!m.is_direct());
    }

    // ── Scale ─────────────────────────────────────────────────────────────────

    #[test]
    fn scale_positive_about_origin() {
        let ax = Ax22d::new(Pnt2d::new(2.0, 0.0), Pnt2d::new(0.0, 1.0), true);
        let s = ax.scaled(Pnt2d::origin(), 3.0);
        pnt_near(s.location(), 6.0, 0.0);
        pnt_near(s.x_direction(), 0.0, 1.0); // unchanged
        pnt_near(s.y_direction(), -1.0, 0.0);
        assert!(s.is_direct());
    }

    #[test]
    fn scale_negative_flips_handedness() {
        let ax = Ax22d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0), true);
        let s = ax.scaled(Pnt2d::origin(), -2.0);
        pnt_near(s.location(), -2.0, 0.0);
        pnt_near(s.x_direction(), -1.0, 0.0); // negated
        pnt_near(s.y_direction(), 0.0, -1.0);
        assert!(!s.is_direct());
    }

    // ── Transform ─────────────────────────────────────────────────────────────

    #[test]
    fn transform_rotation() {
        let ax = Ax22d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0), true);
        let mut t = Trsf2d::identity();
        t.set_rotation(Pnt2d::origin(), FRAC_PI_2);
        let r = ax.transformed(&t);
        near(r.location().x, 0.0, CONFUSION);
        near(r.location().y, 1.0, CONFUSION);
        near(r.x_direction().x, 0.0, CONFUSION);
        near(r.x_direction().y, 1.0, CONFUSION);
        // orientation-preserving: handedness unchanged
        assert!(r.is_direct());
    }

    #[test]
    fn transform_mirror_flips_handedness() {
        let ax = Ax22d::new(Pnt2d::new(0.0, 1.0), Pnt2d::new(1.0, 0.0), true);
        let mut t = Trsf2d::identity();
        // Mirror about the X axis (use gp2d::Ax2d as required by Trsf2d)
        t.set_mirror_axis(Ax2dSimple::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)));
        let r = ax.transformed(&t);
        pnt_near(r.location(), 0.0, -1.0);
        assert!(!r.is_direct()); // handedness flipped
    }

    // ── Set direction setters ─────────────────────────────────────────────────

    #[test]
    fn set_x_direction_updates_y() {
        let mut ax = Ax22d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), true);
        ax.set_x_direction(Pnt2d::new(0.0, 1.0)); // X = (0,1)
        pnt_near(ax.x_direction(), 0.0, 1.0);
        pnt_near(ax.y_direction(), -1.0, 0.0); // Y = X rotated +90°
    }

    #[test]
    fn set_y_direction_updates_x() {
        let mut ax = Ax22d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), true);
        ax.set_y_direction(Pnt2d::new(0.0, 1.0)); // Y = (0,1)
        pnt_near(ax.y_direction(), 0.0, 1.0);
        // In a right-handed frame X = rot(Y, -90°): (0,1)→(1,0)
        pnt_near(ax.x_direction(), 1.0, 0.0);
    }
}
