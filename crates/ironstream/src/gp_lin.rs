// FILE: gp_lin.rs
use crate::gp::{Ax1, Ax3, Pnt, Trsf};

// occt: gp_Lin
#[derive(Clone, Copy, Debug)]
pub struct Lin {
    pos: Ax1,
}

impl Lin {
    #[inline]
    pub fn new(pos: Ax1) -> Self {
        Self { pos }
    }

    #[inline]
    pub fn from_point_dir(p: Pnt, d: Pnt) -> Self {
        Self { pos: Ax1::new(p, d) }
    }

    // --- getters ---

    #[inline]
    pub fn position(&self) -> Ax1 {
        self.pos
    }

    #[inline]
    pub fn location(&self) -> Pnt {
        self.pos.location
    }

    #[inline]
    pub fn direction(&self) -> Pnt {
        self.pos.direction
    }

    // --- setters ---

    #[inline]
    pub fn set_position(&mut self, a: Ax1) {
        self.pos = a;
    }

    #[inline]
    pub fn set_location(&mut self, p: Pnt) {
        self.pos.location = p;
    }

    #[inline]
    pub fn set_direction(&mut self, d: Pnt) {
        self.pos.direction = d.normalized();
    }

    // --- geometric queries ---

    /// Point at parameter t: `location + t * direction`.
    #[inline]
    pub fn point_at(&self, t: f64) -> Pnt {
        self.pos.location + self.pos.direction * t
    }

    /// Signed angle in [-PI, PI] between the two line directions.
    pub fn angle(&self, other: &Lin) -> f64 {
        let a = self.pos.direction;
        let b = other.pos.direction;
        let cross = a.cross(b);
        let dot = a.dot(b).clamp(-1.0, 1.0);
        let mag = cross.norm();
        mag.atan2(dot)
    }

    #[inline]
    pub fn contains(&self, p: Pnt, tol: f64) -> bool {
        self.distance(p) <= tol
    }

    /// Perpendicular distance from point p to this line: ||(p - loc) x dir||
    pub fn distance(&self, p: Pnt) -> f64 {
        let v = p - self.pos.location;
        v.cross(self.pos.direction).norm()
    }

    #[inline]
    pub fn square_distance(&self, p: Pnt) -> f64 {
        let d = self.distance(p);
        d * d
    }

    /// Minimum distance between two lines; 0 if intersecting, else skew distance.
    pub fn distance_to_line(&self, other: &Lin) -> f64 {
        let d1 = self.pos.direction;
        let d2 = other.pos.direction;
        let cross = d1.cross(d2);
        let denom = cross.norm();
        if denom < 1e-12 {
            // Parallel lines: perpendicular offset between them.
            self.distance(other.pos.location)
        } else {
            let w = other.pos.location - self.pos.location;
            (w.dot(cross) / denom).abs()
        }
    }

    /// Line through p that is perpendicular to self.
    pub fn normal(&self, p: Pnt) -> Lin {
        let v = p - self.pos.location;
        let dir_self = self.pos.direction;
        // Component of (p - loc) perpendicular to self gives the normal direction.
        let perp = v - dir_self * v.dot(dir_self);
        let norm_dir = if perp.norm() < 1e-12 {
            dir_self.any_perpendicular()
        } else {
            perp.normalized()
        };
        Lin::from_point_dir(p, norm_dir)
    }

    pub fn is_parallel(&self, other: &Lin, ang_tol: f64) -> bool {
        self.pos.direction.is_parallel(other.pos.direction, ang_tol)
    }

    pub fn is_normal(&self, other: &Lin, ang_tol: f64) -> bool {
        self.pos.direction.is_normal(other.pos.direction, ang_tol)
    }

    pub fn is_opposite(&self, other: &Lin, ang_tol: f64) -> bool {
        self.pos.direction.is_opposite(other.pos.direction, ang_tol)
    }

    pub fn is_coaxial(&self, other: &Lin, lin_tol: f64, ang_tol: f64) -> bool {
        self.is_parallel(other, ang_tol) && self.distance(other.pos.location) <= lin_tol
    }

    // --- in-place transforms ---

    pub fn reverse(&mut self) {
        self.pos.direction = -self.pos.direction;
    }

    pub fn translate(&mut self, v: Pnt) {
        self.pos.location = self.pos.location + v;
    }

    pub fn rotate(&mut self, ax: Ax1, ang: f64) {
        let t = Trsf::rotation(ax, ang);
        self.pos.location = t.apply_point(self.pos.location);
        self.pos.direction = t.apply_vector(self.pos.direction).normalized();
    }

    pub fn scale(&mut self, p: Pnt, s: f64) {
        self.pos.location = p + (self.pos.location - p) * s;
        if s < 0.0 {
            self.pos.direction = -self.pos.direction;
        }
    }

    pub fn mirror_point(&mut self, p: Pnt) {
        self.pos.location = p * 2.0 - self.pos.location;
        self.pos.direction = -self.pos.direction;
    }

    /// Reflect across a line (Ax1 defines origin + direction of the mirror axis).
    pub fn mirror_axis(&mut self, ax: Ax1) {
        let d = ax.direction;
        let rel = self.pos.location - ax.location;
        self.pos.location = ax.location + d * (2.0 * rel.dot(d)) - rel;
        let dir = self.pos.direction;
        self.pos.direction = (d * (2.0 * dir.dot(d)) - dir).normalized();
    }

    /// Reflect across a plane (Ax3 defines origin + normal z_dir of the mirror plane).
    pub fn mirror_plane(&mut self, plane: Ax3) {
        let n = plane.z_dir.normalized();
        let dist = (self.pos.location - plane.location).dot(n);
        self.pos.location = self.pos.location - n * (2.0 * dist);
        let dir = self.pos.direction;
        self.pos.direction = (dir - n * (2.0 * dir.dot(n))).normalized();
    }

    pub fn transform(&mut self, t: &Trsf) {
        self.pos.location = t.apply_point(self.pos.location);
        self.pos.direction = t.apply_vector(self.pos.direction).normalized();
    }

    // --- functional variants ---

    pub fn reversed(&self) -> Lin {
        let mut c = *self;
        c.reverse();
        c
    }

    pub fn translated(&self, v: Pnt) -> Lin {
        let mut c = *self;
        c.translate(v);
        c
    }

    pub fn rotated(&self, ax: Ax1, ang: f64) -> Lin {
        let mut c = *self;
        c.rotate(ax, ang);
        c
    }

    pub fn scaled(&self, p: Pnt, s: f64) -> Lin {
        let mut c = *self;
        c.scale(p, s);
        c
    }

    pub fn mirrored_point(&self, p: Pnt) -> Lin {
        let mut c = *self;
        c.mirror_point(p);
        c
    }

    pub fn mirrored_axis(&self, ax: Ax1) -> Lin {
        let mut c = *self;
        c.mirror_axis(ax);
        c
    }

    pub fn mirrored_plane(&self, plane: Ax3) -> Lin {
        let mut c = *self;
        c.mirror_plane(plane);
        c
    }

    pub fn transformed(&self, t: &Trsf) -> Lin {
        let mut c = *self;
        c.transform(t);
        c
    }
}

// ---------------------------------------------------------------------------
// Pln  (gp_Pln)
// ---------------------------------------------------------------------------

// Internal helpers for Pln (array-based arithmetic)
#[inline]
fn dot3p(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn sub3p(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn scale3p(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

#[inline]
fn norm3p(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

#[inline]
fn normalize3p(v: [f64; 3]) -> [f64; 3] {
    let n = norm3p(v);
    if n == 0.0 { v } else { scale3p(v, 1.0 / n) }
}

// occt: gp_Pln
/// An infinite plane in 3D space defined by an origin point and a unit normal.
///
/// Mirrors `gp_Pln` from OpenCascade Technology.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pln {
    /// A point on the plane.
    pub origin: [f64; 3],
    /// The unit outward normal.
    pub normal: [f64; 3],
}

impl Pln {
    /// Construct a new plane. `normal` is normalised internally.
    ///
    /// Mirrors `gp_Pln(const gp_Pnt&, const gp_Dir&)`.
    // occt: gp_Pln
    pub fn new(origin: [f64; 3], normal: [f64; 3]) -> Self {
        Self {
            origin,
            normal: normalize3p(normal),
        }
    }

    /// Unsigned perpendicular distance from `pt` to this plane.
    pub fn distance_to_point(&self, pt: [f64; 3]) -> f64 {
        dot3p(sub3p(pt, self.origin), self.normal).abs()
    }

    /// Orthogonal projection of `pt` onto this plane.
    pub fn project_point(&self, pt: [f64; 3]) -> [f64; 3] {
        let signed_dist = dot3p(sub3p(pt, self.origin), self.normal);
        let sp = sub3p(pt, scale3p(self.normal, signed_dist));
        sp
    }

    /// Return `true` when `pt` lies on this plane within `tol`.
    pub fn contains_point(&self, pt: [f64; 3], tol: f64) -> bool {
        self.distance_to_point(pt) <= tol
    }

    /// Intersect this plane with `line`.
    ///
    /// Returns `Some(point)` when the line is not parallel to the plane, or
    /// `None` when parallel.
    pub fn intersect_line(&self, line: &Lin) -> Option<[f64; 3]> {
        let loc = line.location();
        let dir = line.direction();
        let loc_arr = [loc.x, loc.y, loc.z];
        let dir_arr = [dir.x, dir.y, dir.z];
        let denom = dot3p(dir_arr, self.normal);
        if denom.abs() < 1e-12 {
            return None;
        }
        let t = dot3p(sub3p(self.origin, loc_arr), self.normal) / denom;
        let pt = line.point_at(t);
        Some([pt.x, pt.y, pt.z])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, PI};

    const TOL: f64 = 1e-10;

    fn near(a: f64, b: f64) {
        assert!((a - b).abs() <= TOL, "expected {a} ≈ {b}");
    }

    fn x_axis() -> Lin {
        Lin::from_point_dir(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0))
    }

    #[test]
    fn distance_point_on_line_is_zero() {
        let l = x_axis();
        near(l.distance(Pnt::new(5.0, 0.0, 0.0)), 0.0);
        near(l.distance(Pnt::new(-3.0, 0.0, 0.0)), 0.0);
    }

    #[test]
    fn distance_to_off_line_point() {
        let l = x_axis();
        near(l.distance(Pnt::new(0.0, 3.0, 4.0)), 5.0);
        near(l.distance(Pnt::new(7.0, 0.0, 2.0)), 2.0);
    }

    #[test]
    fn square_distance() {
        let l = x_axis();
        near(l.square_distance(Pnt::new(0.0, 3.0, 4.0)), 25.0);
    }

    #[test]
    fn contains() {
        let l = x_axis();
        assert!(l.contains(Pnt::new(100.0, 0.0, 0.0), 1e-9));
        assert!(!l.contains(Pnt::new(0.0, 1.0, 0.0), 1e-9));
    }

    #[test]
    fn angle_perpendicular_lines() {
        let l1 = x_axis();
        let l2 = Lin::from_point_dir(Pnt::origin(), Pnt::new(0.0, 1.0, 0.0));
        near(l1.angle(&l2), FRAC_PI_2);
    }

    #[test]
    fn angle_anti_parallel() {
        let l1 = x_axis();
        let l2 = l1.reversed();
        near(l1.angle(&l2), PI);
    }

    #[test]
    fn reverse() {
        let r = x_axis().reversed();
        near(r.direction().x, -1.0);
        near(r.direction().y, 0.0);
        near(r.direction().z, 0.0);
    }

    #[test]
    fn translate() {
        let l = x_axis().translated(Pnt::new(0.0, 5.0, 3.0));
        near(l.location().x, 0.0);
        near(l.location().y, 5.0);
        near(l.location().z, 3.0);
        near(l.direction().x, 1.0);
    }

    #[test]
    fn rotate_90_about_z() {
        let l = Lin::from_point_dir(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let z_ax = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
        let r = l.rotated(z_ax, FRAC_PI_2);
        near(r.location().x, 0.0);
        near(r.location().y, 1.0);
        near(r.direction().x, 0.0);
        near(r.direction().y, 1.0);
    }

    #[test]
    fn scale_about_origin() {
        let l = Lin::from_point_dir(Pnt::new(2.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let s = l.scaled(Pnt::origin(), 3.0);
        near(s.location().x, 6.0);
        near(s.direction().x, 1.0);
    }

    #[test]
    fn is_parallel_and_normal() {
        let l1 = x_axis();
        let l2 = Lin::from_point_dir(Pnt::new(0.0, 5.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let l3 = Lin::from_point_dir(Pnt::origin(), Pnt::new(0.0, 1.0, 0.0));
        assert!(l1.is_parallel(&l2, 1e-10));
        assert!(l1.is_normal(&l3, 1e-10));
    }

    #[test]
    fn is_coaxial() {
        let l1 = x_axis();
        let l2 = Lin::from_point_dir(Pnt::new(5.0, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0));
        assert!(l1.is_coaxial(&l2, 1e-7, 1e-10));
        let l3 = Lin::from_point_dir(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        assert!(!l1.is_coaxial(&l3, 1e-7, 1e-10));
    }

    #[test]
    fn distance_between_skew_lines() {
        let l1 = Lin::from_point_dir(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let l2 = Lin::from_point_dir(Pnt::new(0.0, 0.0, 1.0), Pnt::new(0.0, 1.0, 0.0));
        near(l1.distance_to_line(&l2), 1.0);
    }

    #[test]
    fn mirror_point() {
        let l = Lin::from_point_dir(Pnt::new(2.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let m = l.mirrored_point(Pnt::origin());
        near(m.location().x, -2.0);
        near(m.direction().x, -1.0);
    }

    #[test]
    fn normal_at_point() {
        let l = x_axis();
        let p = Pnt::new(3.0, 4.0, 0.0);
        let n = l.normal(p);
        near(n.location().x, 3.0);
        near(n.location().y, 4.0);
        near(n.direction().dot(l.direction()), 0.0);
    }

    // ── Pln tests ──

    #[test]
    fn pln_new_normalises_normal() {
        let p = Pln::new([0.0, 0.0, 0.0], [0.0, 0.0, 5.0]);
        assert!((norm3p(p.normal) - 1.0).abs() < TOL);
        assert!((p.normal[2] - 1.0).abs() < TOL);
    }

    #[test]
    fn pln_distance_to_point() {
        let p = Pln::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!((p.distance_to_point([3.0, 4.0, 7.0]) - 7.0).abs() < TOL);
        assert!((p.distance_to_point([0.0, 0.0, 0.0]) - 0.0).abs() < TOL);
    }

    #[test]
    fn pln_project_point() {
        let p = Pln::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let foot = p.project_point([3.0, 4.0, 7.0]);
        assert!((foot[0] - 3.0).abs() < TOL && (foot[1] - 4.0).abs() < TOL && (foot[2] - 0.0).abs() < TOL);
    }

    #[test]
    fn pln_contains_point() {
        let p = Pln::new([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        assert!(p.contains_point([1.0, 2.0, 5.0], 1e-9));
        assert!(!p.contains_point([0.0, 0.0, 6.0], 1e-9));
    }

    #[test]
    fn pln_intersect_line_hits() {
        let p = Pln::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let l = Lin::from_point_dir(Pnt::new(1.0, 2.0, -3.0), Pnt::new(0.0, 0.0, 1.0));
        let hit = p.intersect_line(&l).expect("should intersect");
        assert!((hit[0] - 1.0).abs() < TOL && (hit[1] - 2.0).abs() < TOL && (hit[2] - 0.0).abs() < TOL);
    }

    #[test]
    fn pln_intersect_line_parallel_returns_none() {
        let p = Pln::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let l = Lin::from_point_dir(Pnt::new(0.0, 0.0, 5.0), Pnt::new(1.0, 0.0, 0.0));
        assert!(p.intersect_line(&l).is_none());
    }
}
