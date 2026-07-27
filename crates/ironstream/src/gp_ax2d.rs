//! `gp_Ax2d` — 2-D axis (a point and a unit direction), mirroring OpenCascade's
//! `gp_Ax2d` from package `TKMath/gp`.
//!
//! An axis in 2D is defined by:
//! * a **location** point (`gp_Pnt2d`)
//! * a **unit direction** (`gp_Dir2d`), kept normalised at all times
//!
//! The struct supports the full OCCT `gp_Ax2d` API: geometric queries
//! (`IsCoaxial`, `IsNormal`, `IsOpposite`, `IsParallel`) and geometric
//! transformations (`Translate`/`Translated`, `Rotate`/`Rotated`,
//! `Mirror`/`Mirrored`, `Scale`/`Scaled`, `Transform`/`Transformed`).

use crate::gp2d::Pnt2d;
use crate::gp_trsf2d::Trsf2d;

// ---------------------------------------------------------------------------
// Ax2d
// ---------------------------------------------------------------------------

/// A 2-D axis: a location point and a unit direction.
// occt-ref: gp_Ax2d
#[derive(Clone, Copy, Debug)]
pub struct Ax2d {
    /// The origin of the axis.
    location: Pnt2d,
    /// The unit direction of the axis (always normalised).
    direction: Pnt2d,
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Rotate a 2-D vector by `angle` radians (counter-clockwise).
#[inline]
fn rotate_vec(v: Pnt2d, angle: f64) -> Pnt2d {
    let (s, c) = angle.sin_cos();
    Pnt2d::new(c * v.x - s * v.y, s * v.x + c * v.y)
}

/// Reflect a point through an axis defined by an origin and a unit direction.
///
/// The formula for reflecting point `p` across the line through `o` with
/// unit direction `d` is:
///   p' = 2 * (o + ((p - o)·d) * d) - p
///      = 2*o + 2*((p-o)·d)*d - p
#[inline]
fn reflect_point_through_line(p: Pnt2d, o: Pnt2d, d: Pnt2d) -> Pnt2d {
    let rel = Pnt2d::new(p.x - o.x, p.y - o.y);
    let proj_len = rel.x * d.x + rel.y * d.y;
    // foot = o + proj_len * d
    let foot = Pnt2d::new(o.x + proj_len * d.x, o.y + proj_len * d.y);
    // reflection = 2*foot - p
    Pnt2d::new(2.0 * foot.x - p.x, 2.0 * foot.y - p.y)
}

/// Reflect a unit direction vector across a line with unit direction `d`.
///
/// Reflection of vector `v` across a line with direction `d`:
///   v' = 2*(v·d)*d - v
#[inline]
fn reflect_dir_through_dir(v: Pnt2d, d: Pnt2d) -> Pnt2d {
    let dot = v.x * d.x + v.y * d.y;
    Pnt2d::new(2.0 * dot * d.x - v.x, 2.0 * dot * d.y - v.y)
}

// ---------------------------------------------------------------------------
// Ax2d — constructor and accessors
// ---------------------------------------------------------------------------

impl Ax2d {
    /// `gp_Ax2d(P, V)` — construct from a location point and a direction.
    /// The direction is normalised on construction.
    pub fn new(location: Pnt2d, direction: Pnt2d) -> Self {
        Self {
            location,
            direction: direction.normalized(),
        }
    }

    /// `Location` — return the location point.
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.location
    }

    /// `Direction` — return the unit direction.
    #[inline]
    pub fn direction(&self) -> Pnt2d {
        self.direction
    }

    /// `SetLocation(P)` — change the location point.
    #[inline]
    pub fn set_location(&mut self, p: Pnt2d) {
        self.location = p;
    }

    /// `SetDirection(V)` — change the direction (normalises `v`).
    #[inline]
    pub fn set_direction(&mut self, v: Pnt2d) {
        self.direction = v.normalized();
    }

    // -----------------------------------------------------------------------
    // Geometric queries
    // -----------------------------------------------------------------------

    /// `IsCoaxial(other, linear_tol, angular_tol)` — true when the two axes
    /// are parallel (or anti-parallel) **and** the location of `other` lies on
    /// `self` (within `linear_tol`).
    ///
    /// OCCT checks: directions are parallel AND the distance from `other.location`
    /// to the line defined by `self` is ≤ `linear_tol`.
    pub fn is_coaxial(&self, other: &Ax2d, linear_tol: f64, angular_tol: f64) -> bool {
        if !self.is_parallel(other, angular_tol) {
            return false;
        }
        // Distance from other.location to the line through self.location with
        // direction self.direction (using the 2-D cross-product magnitude).
        let rel = Pnt2d::new(
            other.location.x - self.location.x,
            other.location.y - self.location.y,
        );
        let cross = (rel.x * self.direction.y - rel.y * self.direction.x).abs();
        cross <= linear_tol
    }

    /// `IsNormal(other, angular_tol)` — true when the directions are
    /// perpendicular within `angular_tol`.
    pub fn is_normal(&self, other: &Ax2d, angular_tol: f64) -> bool {
        let dot = (self.direction.x * other.direction.x
            + self.direction.y * other.direction.y)
            .abs();
        // dot = |cos θ|; perpendicular ⟺ dot ≈ 0
        dot <= angular_tol.sin()
    }

    /// `IsOpposite(other, angular_tol)` — true when the directions point in
    /// opposite directions within `angular_tol`.
    pub fn is_opposite(&self, other: &Ax2d, angular_tol: f64) -> bool {
        let dot = self.direction.x * other.direction.x
            + self.direction.y * other.direction.y;
        // anti-parallel ⟺ dot ≈ -1
        (-dot - 1.0).abs() <= angular_tol.sin()
    }

    /// `IsParallel(other, angular_tol)` — true when the directions are parallel
    /// or anti-parallel within `angular_tol`.
    pub fn is_parallel(&self, other: &Ax2d, angular_tol: f64) -> bool {
        // |cross| = |sin θ|; parallel ⟺ |sin θ| ≈ 0
        let cross = (self.direction.x * other.direction.y
            - self.direction.y * other.direction.x)
            .abs();
        cross <= angular_tol.sin()
    }

    // -----------------------------------------------------------------------
    // In-place transformations
    // -----------------------------------------------------------------------

    /// `Translate(V)` — shift the location by vector `v`; direction unchanged.
    pub fn translate(&mut self, v: Pnt2d) {
        self.location = Pnt2d::new(self.location.x + v.x, self.location.y + v.y);
    }

    /// `Translate(P1, P2)` — shift the location by the vector P1 → P2.
    pub fn translate_2pts(&mut self, p1: Pnt2d, p2: Pnt2d) {
        self.translate(Pnt2d::new(p2.x - p1.x, p2.y - p1.y));
    }

    /// `Rotate(P, angle)` — rotate the axis about point `p` by `angle` radians.
    pub fn rotate(&mut self, p: Pnt2d, angle: f64) {
        // Rotate the location about p.
        let rel = Pnt2d::new(self.location.x - p.x, self.location.y - p.y);
        let rotated_rel = rotate_vec(rel, angle);
        self.location = Pnt2d::new(p.x + rotated_rel.x, p.y + rotated_rel.y);
        // Rotate the direction (pure linear, no translation).
        self.direction = rotate_vec(self.direction, angle);
    }

    /// `Mirror(P)` — point symmetry about `p` (in-place).
    pub fn mirror_point(&mut self, p: Pnt2d) {
        // Reflect location through p: loc' = 2*p - loc
        self.location = Pnt2d::new(
            2.0 * p.x - self.location.x,
            2.0 * p.y - self.location.y,
        );
        // Direction negates under point symmetry.
        self.direction = Pnt2d::new(-self.direction.x, -self.direction.y);
    }

    /// `Mirror(A)` — axial symmetry through line `a` (in-place).
    pub fn mirror_axis(&mut self, a: &Ax2d) {
        self.location = reflect_point_through_line(self.location, a.location, a.direction);
        self.direction = reflect_dir_through_dir(self.direction, a.direction);
    }

    /// `Scale(P, s)` — uniform scaling by `s` about centre `p` (in-place).
    /// The direction is unchanged (but may be reversed if `s < 0`).
    pub fn scale(&mut self, p: Pnt2d, s: f64) {
        self.location = Pnt2d::new(
            p.x + s * (self.location.x - p.x),
            p.y + s * (self.location.y - p.y),
        );
        // A negative scale reverses the direction.
        if s < 0.0 {
            self.direction = Pnt2d::new(-self.direction.x, -self.direction.y);
        }
        // The direction remains unit length (magnitude of scale only affects
        // the location).
    }

    /// `Transform(T)` — apply a 2-D affine transform (in-place).
    pub fn transform(&mut self, t: &Trsf2d) {
        self.location = t.apply(self.location);
        self.direction = t.apply_vector(self.direction).normalized();
    }

    // -----------------------------------------------------------------------
    // Functional (value-returning) transformations
    // -----------------------------------------------------------------------

    /// `Translated(V)` — return a translated copy.
    pub fn translated(&self, v: Pnt2d) -> Ax2d {
        let mut ax = *self;
        ax.translate(v);
        ax
    }

    /// `Translated(P1, P2)` — return a copy translated by P1 → P2.
    pub fn translated_2pts(&self, p1: Pnt2d, p2: Pnt2d) -> Ax2d {
        let mut ax = *self;
        ax.translate_2pts(p1, p2);
        ax
    }

    /// `Rotated(P, angle)` — return a rotated copy.
    pub fn rotated(&self, p: Pnt2d, angle: f64) -> Ax2d {
        let mut ax = *self;
        ax.rotate(p, angle);
        ax
    }

    /// `Mirrored(P)` — return a point-mirrored copy.
    pub fn mirrored_point(&self, p: Pnt2d) -> Ax2d {
        let mut ax = *self;
        ax.mirror_point(p);
        ax
    }

    /// `Mirrored(A)` — return an axially-mirrored copy.
    pub fn mirrored_axis(&self, a: &Ax2d) -> Ax2d {
        let mut ax = *self;
        ax.mirror_axis(a);
        ax
    }

    /// `Scaled(P, s)` — return a scaled copy.
    pub fn scaled(&self, p: Pnt2d, s: f64) -> Ax2d {
        let mut ax = *self;
        ax.scale(p, s);
        ax
    }

    /// `Transformed(T)` — return a transformed copy.
    pub fn transformed(&self, t: &Trsf2d) -> Ax2d {
        let mut ax = *self;
        ax.transform(t);
        ax
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    fn near(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol:.2e})");
    }

    #[test]
    fn constructor_normalises_direction() {
        let ax = Ax2d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(3.0, 0.0));
        near(ax.direction().x, 1.0, 1e-12);
        near(ax.direction().y, 0.0, 1e-12);
        near(ax.location().x, 1.0, 1e-12);
        near(ax.location().y, 2.0, 1e-12);
    }

    #[test]
    fn translate_shifts_location() {
        let mut ax = Ax2d::new(Pnt2d::new(1.0, 1.0), Pnt2d::new(1.0, 0.0));
        ax.translate(Pnt2d::new(3.0, -1.0));
        near(ax.location().x, 4.0, 1e-12);
        near(ax.location().y, 0.0, 1e-12);
        near(ax.direction().x, 1.0, 1e-12);
        near(ax.direction().y, 0.0, 1e-12);
    }

    #[test]
    fn rotate_90() {
        let ax = Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0));
        let r = ax.rotated(Pnt2d::origin(), FRAC_PI_2);
        near(r.location().x, 0.0, 1e-12);
        near(r.location().y, 1.0, 1e-12);
        near(r.direction().x, 0.0, 1e-12);
        near(r.direction().y, 1.0, 1e-12);
    }

    #[test]
    fn mirror_through_origin() {
        let ax = Ax2d::new(Pnt2d::new(2.0, 3.0), Pnt2d::new(1.0, 0.0));
        let m = ax.mirrored_point(Pnt2d::origin());
        near(m.location().x, -2.0, 1e-12);
        near(m.location().y, -3.0, 1e-12);
        near(m.direction().x, -1.0, 1e-12);
        near(m.direction().y, 0.0, 1e-12);
    }

    #[test]
    fn scale_about_origin() {
        let ax = Ax2d::new(Pnt2d::new(2.0, 0.0), Pnt2d::new(0.0, 1.0));
        let s = ax.scaled(Pnt2d::origin(), 3.0);
        near(s.location().x, 6.0, 1e-12);
        near(s.location().y, 0.0, 1e-12);
        near(s.direction().x, 0.0, 1e-12);
        near(s.direction().y, 1.0, 1e-12);
    }

    #[test]
    fn is_parallel_same_dir() {
        let a = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        let b = Ax2d::new(Pnt2d::new(0.0, 5.0), Pnt2d::new(1.0, 0.0));
        assert!(a.is_parallel(&b, 1e-10));
        assert!(!a.is_normal(&b, 1e-10));
    }

    #[test]
    fn is_normal_perpendicular() {
        let a = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        let b = Ax2d::new(Pnt2d::origin(), Pnt2d::new(0.0, 1.0));
        assert!(a.is_normal(&b, 1e-10));
        assert!(!a.is_parallel(&b, 1e-10));
    }

    #[test]
    fn is_opposite_reversed() {
        let a = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        let b = Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(-1.0, 0.0));
        assert!(a.is_opposite(&b, 1e-10));
        assert!(a.is_parallel(&b, 1e-10));
        assert!(!a.is_normal(&b, 1e-10));
    }
}
