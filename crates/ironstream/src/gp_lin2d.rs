// FILE: gp_lin2d.rs
//! `gp_Lin2d` — non-parameterized infinite 2D line.
// occt: gp_Lin2d

use crate::gp2d::{Ax2d, Pnt2d, Trsf2d};

/// An infinite 2D line defined by a positioning axis.
// occt: gp_Lin2d
#[derive(Clone, Copy, Debug)]
pub struct Lin2d {
    pos: Ax2d,
}

impl Lin2d {
    /// Construct from a positioning axis.
    #[inline]
    pub fn new(a: Ax2d) -> Self {
        Self { pos: a }
    }

    /// Construct from point and direction.
    #[inline]
    pub fn from_point_dir(p: Pnt2d, v: Pnt2d) -> Self {
        Self { pos: Ax2d::new(p, v) }
    }

    // --- setters ---

    #[inline]
    pub fn set_direction(&mut self, v: Pnt2d) {
        self.pos.direction = v.normalized();
    }

    #[inline]
    pub fn set_location(&mut self, p: Pnt2d) {
        self.pos.location = p;
    }

    #[inline]
    pub fn set_position(&mut self, a: Ax2d) {
        self.pos = a;
    }

    // --- getters ---

    #[inline]
    pub fn direction(&self) -> Pnt2d {
        self.pos.direction
    }

    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.pos.location
    }

    #[inline]
    pub fn position(&self) -> Ax2d {
        self.pos
    }

    // --- predicates ---

    /// True when lines are parallel (or anti-parallel) AND distance ≤ linear_tol.
    pub fn is_coaxial(&self, other: &Lin2d, linear_tol: f64, angular_tol: f64) -> bool {
        self.is_parallel(other, angular_tol) && self.distance(other.pos.location) <= linear_tol
    }

    /// True when directions are perpendicular within angular_tol.
    pub fn is_normal(&self, other: &Lin2d, angular_tol: f64) -> bool {
        let cross = self.pos.direction.cross(other.pos.direction).abs();
        (cross - 1.0).abs() <= angular_tol
    }

    /// True when directions point in opposite directions within angular_tol.
    pub fn is_opposite(&self, other: &Lin2d, angular_tol: f64) -> bool {
        let dot = self.pos.direction.dot(other.pos.direction);
        (dot + 1.0).abs() <= angular_tol
    }

    /// True when directions are parallel or anti-parallel within angular_tol.
    pub fn is_parallel(&self, other: &Lin2d, angular_tol: f64) -> bool {
        let cross = self.pos.direction.cross(other.pos.direction).abs();
        cross <= angular_tol
    }

    // --- geometric queries ---

    /// Signed angle in [-π, π] from self to other.
    pub fn angle(&self, other: &Lin2d) -> f64 {
        let d1 = self.pos.direction;
        let d2 = other.pos.direction;
        let cross = d1.x * d2.y - d1.y * d2.x;
        let dot = d1.x * d2.x + d1.y * d2.y;
        cross.atan2(dot)
    }

    /// True if distance from p to line ≤ linear_tol.
    #[inline]
    pub fn contains(&self, p: Pnt2d, linear_tol: f64) -> bool {
        self.distance(p) <= linear_tol
    }

    /// Perpendicular distance from point p to the line.
    pub fn distance(&self, p: Pnt2d) -> f64 {
        let v = p - self.pos.location;
        let d = self.pos.direction;
        (v.x * d.y - v.y * d.x).abs()
    }

    /// Distance between two lines (0 if intersecting, perpendicular distance if parallel).
    pub fn distance_to_line(&self, other: &Lin2d) -> f64 {
        let cross = (self.pos.direction.cross(other.pos.direction)).abs();
        if cross < 1e-12 {
            self.distance(other.pos.location)
        } else {
            0.0
        }
    }

    #[inline]
    pub fn square_distance(&self, p: Pnt2d) -> f64 {
        let d = self.distance(p);
        d * d
    }

    /// Line normal to self passing through p.
    pub fn normal(&self, p: Pnt2d) -> Lin2d {
        let d = self.pos.direction;
        Lin2d::from_point_dir(p, Pnt2d::new(-d.y, d.x))
    }

    // --- in-place transforms ---

    pub fn reverse(&mut self) {
        self.pos.direction = -self.pos.direction;
    }

    pub fn translate(&mut self, v: Pnt2d) {
        self.pos.location = self.pos.location + v;
    }

    pub fn rotate(&mut self, p: Pnt2d, ang: f64) {
        let (s, c) = ang.sin_cos();
        let loc = self.pos.location;
        let dx = loc.x - p.x;
        let dy = loc.y - p.y;
        self.pos.location = Pnt2d::new(p.x + c * dx - s * dy, p.y + s * dx + c * dy);
        let dir = self.pos.direction;
        self.pos.direction = Pnt2d::new(c * dir.x - s * dir.y, s * dir.x + c * dir.y);
    }

    pub fn mirror_point(&mut self, p: Pnt2d) {
        self.pos.location = Pnt2d::new(2.0 * p.x - self.pos.location.x, 2.0 * p.y - self.pos.location.y);
        self.pos.direction = -self.pos.direction;
    }

    /// Reflect across another line.
    pub fn mirror_axis(&mut self, a: &Lin2d) {
        // Reflect location and direction across the line `a`.
        let ax_dir = a.pos.direction;
        // Reflect a vector v across axis direction d: v' = 2(v·d)d - v
        let reflect = |v: Pnt2d| -> Pnt2d {
            let dot = v.dot(ax_dir);
            Pnt2d::new(2.0 * dot * ax_dir.x - v.x, 2.0 * dot * ax_dir.y - v.y)
        };
        // Reflect location: translate to axis origin, reflect, translate back
        let loc_rel = self.pos.location - a.pos.location;
        let loc_ref = reflect(loc_rel);
        self.pos.location = a.pos.location + loc_ref;
        self.pos.direction = reflect(self.pos.direction);
    }

    pub fn scale(&mut self, p: Pnt2d, s: f64) {
        let loc = self.pos.location;
        self.pos.location = Pnt2d::new(p.x + s * (loc.x - p.x), p.y + s * (loc.y - p.y));
        if s < 0.0 {
            self.pos.direction = -self.pos.direction;
        }
    }

    pub fn transform(&mut self, t: &Trsf2d) {
        self.pos.location = t.apply(self.pos.location);
        let d = self.pos.direction;
        self.pos.direction = Pnt2d::new(
            t.m[0][0] * d.x + t.m[0][1] * d.y,
            t.m[1][0] * d.x + t.m[1][1] * d.y,
        ).normalized();
    }

    // --- functional transforms ---

    pub fn reversed(&self) -> Lin2d {
        let mut c = *self;
        c.reverse();
        c
    }

    pub fn translated(&self, v: Pnt2d) -> Lin2d {
        Lin2d { pos: Ax2d { location: self.pos.location + v, direction: self.pos.direction } }
    }

    pub fn rotated(&self, p: Pnt2d, ang: f64) -> Lin2d {
        let mut c = *self;
        c.rotate(p, ang);
        c
    }

    pub fn mirrored_point(&self, p: Pnt2d) -> Lin2d {
        let mut c = *self;
        c.mirror_point(p);
        c
    }

    pub fn mirrored_axis(&self, a: &Lin2d) -> Lin2d {
        let mut c = *self;
        c.mirror_axis(a);
        c
    }

    pub fn scaled(&self, p: Pnt2d, s: f64) -> Lin2d {
        let mut c = *self;
        c.scale(p, s);
        c
    }

    pub fn transformed(&self, t: &Trsf2d) -> Lin2d {
        let mut c = *self;
        c.transform(t);
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

    const TOL: f64 = 1e-10;

    fn x_axis() -> Lin2d {
        Lin2d::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0))
    }

    fn near(a: f64, b: f64) {
        assert!((a - b).abs() <= TOL, "expected {a} ≈ {b}");
    }

    #[test]
    fn constructor_normalises_direction() {
        let l = Lin2d::from_point_dir(Pnt2d::new(1.0, 2.0), Pnt2d::new(3.0, 0.0));
        near(l.direction().x, 1.0);
        near(l.direction().y, 0.0);
        near(l.location().x, 1.0);
        near(l.location().y, 2.0);
    }

    #[test]
    fn distance_to_point_on_line() {
        let l = x_axis();
        near(l.distance(Pnt2d::new(5.0, 0.0)), 0.0);
        near(l.distance(Pnt2d::new(-3.0, 0.0)), 0.0);
    }

    #[test]
    fn distance_to_off_line_point() {
        let l = x_axis();
        near(l.distance(Pnt2d::new(3.0, 4.0)), 4.0);
        near(l.distance(Pnt2d::new(-1.0, -2.0)), 2.0);
    }

    #[test]
    fn square_distance() {
        let l = x_axis();
        near(l.square_distance(Pnt2d::new(0.0, 3.0)), 9.0);
    }

    #[test]
    fn contains() {
        let l = x_axis();
        assert!(l.contains(Pnt2d::new(10.0, 0.0), 1e-9));
        assert!(!l.contains(Pnt2d::new(0.0, 1.0), 1e-9));
    }

    #[test]
    fn normal_at_point() {
        let l = x_axis();
        let n = l.normal(Pnt2d::new(3.0, 0.0));
        near(n.location().x, 3.0);
        near(n.location().y, 0.0);
        near(n.direction().dot(l.direction()), 0.0);
    }

    #[test]
    fn reverse() {
        let r = x_axis().reversed();
        near(r.direction().x, -1.0);
        near(r.direction().y, 0.0);
    }

    #[test]
    fn angle_between_lines() {
        let l1 = x_axis();
        let l2 = Lin2d::from_point_dir(Pnt2d::origin(), Pnt2d::new(0.0, 1.0));
        near(l1.angle(&l2), FRAC_PI_2);
        near(l2.angle(&l1), -FRAC_PI_2);
    }

    #[test]
    fn is_parallel() {
        let l1 = Lin2d::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        let l2 = Lin2d::from_point_dir(Pnt2d::new(0.0, 5.0), Pnt2d::new(2.0, 0.0));
        assert!(l1.is_parallel(&l2, 1e-10));
        assert!(!l1.is_normal(&l2, 1e-10));
    }

    #[test]
    fn is_normal() {
        let l1 = Lin2d::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        let l2 = Lin2d::from_point_dir(Pnt2d::origin(), Pnt2d::new(0.0, 1.0));
        assert!(l1.is_normal(&l2, 1e-10));
    }

    #[test]
    fn translate() {
        let l = x_axis().translated(Pnt2d::new(3.0, 7.0));
        near(l.location().x, 3.0);
        near(l.location().y, 7.0);
        near(l.direction().x, 1.0);
    }

    #[test]
    fn rotate_90() {
        let l = Lin2d::from_point_dir(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0));
        let r = l.rotated(Pnt2d::origin(), FRAC_PI_2);
        near(r.location().x, 0.0);
        near(r.location().y, 1.0);
        near(r.direction().x, 0.0);
        near(r.direction().y, 1.0);
    }

    #[test]
    fn distance_to_parallel_line() {
        let l1 = x_axis();
        let l2 = Lin2d::from_point_dir(Pnt2d::new(0.0, 3.0), Pnt2d::new(1.0, 0.0));
        near(l1.distance_to_line(&l2), 3.0);
    }

    #[test]
    fn mirror_point() {
        let l = Lin2d::from_point_dir(Pnt2d::new(2.0, 0.0), Pnt2d::new(1.0, 0.0));
        let m = l.mirrored_point(Pnt2d::origin());
        near(m.location().x, -2.0);
        near(m.direction().x, -1.0);
    }

    #[test]
    fn set_direction() {
        let mut l = x_axis();
        l.set_direction(Pnt2d::new(0.0, 5.0));
        near(l.direction().x, 0.0);
        near(l.direction().y, 1.0);
    }

    #[test]
    fn scale() {
        let l = Lin2d::from_point_dir(Pnt2d::new(2.0, 0.0), Pnt2d::new(1.0, 0.0));
        let s = l.scaled(Pnt2d::origin(), 3.0);
        near(s.location().x, 6.0);
        near(s.direction().x, 1.0);
    }
}
