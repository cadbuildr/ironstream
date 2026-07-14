// FILE: gp_circ.rs

use crate::gp::{Ax1, Ax3, Pnt, Trsf};
use std::f64::consts::PI;

// occt: gp_Circ
/// Analytic 3D circle: a placement (`gp_Ax2`) and a non-negative radius.
///
/// The origin of `pos` is the circle centre; `z_dir` is the outward normal;
/// `x_dir` and `y_dir` span the circle plane.
#[derive(Clone, Copy, Debug)]
pub struct Circ {
    pos: Ax3,
    radius: f64,
}

impl Circ {
    /// Panics if `radius < 0`.
    pub fn new(pos: Ax3, radius: f64) -> Self {
        assert!(radius >= 0.0, "gp_Circ: radius must be >= 0");
        Self { pos, radius }
    }

    /// Convenience: build from a centre point, a unit normal, and a radius.
    pub fn from_center_normal_radius(center: Pnt, normal: Pnt, radius: f64) -> Self {
        let pos = Ax3::from_origin_normal(center, normal, normal.any_perpendicular());
        Self::new(pos, radius)
    }

    // ----------------------------------------------------------------- getters

    #[inline]
    pub fn position(&self) -> Ax3 {
        self.pos
    }

    #[inline]
    pub fn location(&self) -> Pnt {
        self.pos.location
    }

    /// Normal axis through the centre (Z direction of the placement).
    #[inline]
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.z_dir)
    }

    #[inline]
    pub fn x_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.x_dir)
    }

    #[inline]
    pub fn y_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.y_dir)
    }

    #[inline]
    pub fn radius(&self) -> f64 {
        self.radius
    }

    // --------------------------------------------------------------- geometry

    /// Area enclosed by the circle: `π * r²`.
    #[inline]
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    /// Circumference: `2 * π * r`.
    #[inline]
    pub fn length(&self) -> f64 {
        2.0 * PI * self.radius
    }

    // ----------------------------------------------------------------- queries

    /// Absolute distance from point `p` to the circle: `||center−p| − r|`.
    pub fn distance(&self, p: Pnt) -> f64 {
        (self.pos.location.distance(p) - self.radius).abs()
    }

    pub fn square_distance(&self, p: Pnt) -> f64 {
        let d = self.distance(p);
        d * d
    }

    pub fn contains(&self, p: Pnt, tol: f64) -> bool {
        self.distance(p) <= tol
    }

    // --------------------------------------------------------- in-place transforms

    /// Flip the normal (Z direction) — reverses the circle orientation.
    pub fn reverse(&mut self) {
        self.pos.z_dir = -self.pos.z_dir;
        // Swap X and Y so the frame stays right-handed with the new Z.
        core::mem::swap(&mut self.pos.x_dir, &mut self.pos.y_dir);
    }

    pub fn translate(&mut self, v: Pnt) {
        *self = self.translated(v);
    }

    pub fn rotate(&mut self, ax: Ax1, ang: f64) {
        *self = self.rotated(ax, ang);
    }

    pub fn scale(&mut self, p: Pnt, s: f64) {
        *self = self.scaled(p, s);
    }

    pub fn mirror_point(&mut self, p: Pnt) {
        *self = self.mirrored_point(p);
    }

    pub fn mirror_axis(&mut self, ax: Ax1) {
        *self = self.mirrored_axis(ax);
    }

    pub fn mirror_plane(&mut self, plane: Ax3) {
        *self = self.mirrored_plane(plane);
    }

    pub fn transform(&mut self, t: &Trsf) {
        *self = self.transformed(t);
    }

    // --------------------------------------------------------- functional transforms

    pub fn reversed(&self) -> Circ {
        let mut c = *self;
        c.reverse();
        c
    }

    pub fn translated(&self, v: Pnt) -> Circ {
        Circ {
            pos: self.pos.transformed(&Trsf::translation(v)),
            radius: self.radius,
        }
    }

    pub fn rotated(&self, ax: Ax1, ang: f64) -> Circ {
        Circ {
            pos: self.pos.transformed(&Trsf::rotation(ax, ang)),
            radius: self.radius,
        }
    }

    /// Uniform scale about point `p`.  New centre = p + s*(centre−p); new radius = r*|s|.
    pub fn scaled(&self, p: Pnt, s: f64) -> Circ {
        let mut t = Trsf::identity();
        t.set_scale(p, s);
        self.transformed(&t)
    }

    pub fn mirrored_point(&self, p: Pnt) -> Circ {
        let mut pos = self.pos;
        pos.location = pos.location.mirrored_point(p);
        pos.x_dir = -pos.x_dir;
        pos.y_dir = -pos.y_dir;
        Circ {
            pos,
            radius: self.radius,
        }
    }

    pub fn mirrored_axis(&self, ax: Ax1) -> Circ {
        let a = ax.direction;
        let new_loc = self.pos.location.mirrored_axis(ax);
        let mirror_dir = |d: Pnt| (a * (2.0 * d.dot(a)) - d).normalized();
        let x = mirror_dir(self.pos.x_dir);
        let y = mirror_dir(self.pos.y_dir);
        let z = x.cross(y).normalized();
        Circ {
            pos: Ax3 {
                location: new_loc,
                x_dir: x,
                y_dir: y,
                z_dir: z,
            },
            radius: self.radius,
        }
    }

    pub fn mirrored_plane(&self, plane: Ax3) -> Circ {
        let n = plane.z_dir;
        let mirror_pt = |p: Pnt| {
            let dist = (p - plane.location).dot(n);
            p - n * (2.0 * dist)
        };
        let mirror_dir = |d: Pnt| (d - n * (2.0 * d.dot(n))).normalized();
        let new_loc = mirror_pt(self.pos.location);
        let x = mirror_dir(self.pos.x_dir);
        let y = mirror_dir(self.pos.y_dir);
        let z = x.cross(y).normalized();
        Circ {
            pos: Ax3 {
                location: new_loc,
                x_dir: x,
                y_dir: y,
                z_dir: z,
            },
            radius: self.radius,
        }
    }

    pub fn transformed(&self, t: &Trsf) -> Circ {
        Circ {
            pos: self.pos.transformed(t),
            radius: self.radius * t.scale_factor(),
        }
    }
}

// ---------------------------------------------------------------------------
// Parab  (gp_Parab)
// ---------------------------------------------------------------------------

// occt-ref: gp_Parab
/// Analytic 3-D parabola: a focus, an axis of symmetry direction, and a focal
/// length.
///
/// The **parameter** of the parabola is `p = 2 * focal` (the semi-latus
/// rectum doubled, i.e. the length of the latus rectum chord).
///
/// The **directrix** is at distance `focal` from the vertex on the opposite
/// side from the focus; its distance from the focus is therefore `2 * focal`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Parab {
    /// Focus of the parabola in 3-D space.
    pub focus: [f64; 3],
    /// Unit direction of the axis of symmetry.
    pub axis: [f64; 3],
    /// Focal length: non-negative distance from vertex to focus.
    pub focal: f64,
}

impl Parab {
    /// Construct a parabola from focus, axis direction, and focal length.
    ///
    /// # Panics
    /// Panics if `focal < 0`.
    pub fn new(focus: [f64; 3], axis: [f64; 3], focal: f64) -> Self {
        assert!(focal >= 0.0, "gp_Parab: focal must be >= 0");
        Self { focus, axis, focal }
    }

    /// Return the focal length stored on this parabola.
    #[inline]
    pub fn focal(&self) -> f64 {
        self.focal
    }

    /// The parabola parameter `p = 2 * focal` (length of the latus rectum).
    #[inline]
    pub fn parameter(&self) -> f64 {
        2.0 * self.focal
    }

    /// Distance from the focus to the directrix: `2 * focal`.
    #[inline]
    pub fn directrix_distance(&self) -> f64 {
        2.0 * self.focal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precision::CONFUSION;
    use std::f64::consts::FRAC_PI_2;

    fn unit_circle() -> Circ {
        Circ::new(Ax3::identity(), 1.0)
    }

    #[test]
    fn constructor_radius() {
        let c = unit_circle();
        assert!((c.radius() - 1.0).abs() < CONFUSION);
    }

    #[test]
    fn area_and_length() {
        let c = unit_circle();
        assert!((c.area() - PI).abs() < CONFUSION);
        assert!((c.length() - 2.0 * PI).abs() < CONFUSION);
    }

    #[test]
    fn distance_on_circle_is_zero() {
        let c = Circ::new(Ax3::identity(), 3.0);
        assert!(c.distance(Pnt::new(3.0, 0.0, 0.0)) < CONFUSION);
    }

    #[test]
    fn contains_on_circle() {
        let c = unit_circle();
        assert!(c.contains(Pnt::new(1.0, 0.0, 0.0), CONFUSION));
    }

    #[test]
    fn translate_moves_center() {
        let c = unit_circle();
        let t = c.translated(Pnt::new(5.0, 0.0, 0.0));
        assert!((t.location().x - 5.0).abs() < CONFUSION);
        assert!((t.radius() - 1.0).abs() < CONFUSION);
    }

    #[test]
    fn scale_adjusts_radius_and_center() {
        let c = Circ::new(
            Ax3::from_origin_normal(
                Pnt::new(1.0, 0.0, 0.0),
                Pnt::new(0.0, 0.0, 1.0),
                Pnt::new(1.0, 0.0, 0.0),
            ),
            2.0,
        );
        let s = c.scaled(Pnt::origin(), 3.0);
        assert!((s.radius() - 6.0).abs() < CONFUSION);
        assert!((s.location().x - 3.0).abs() < CONFUSION);
    }

    #[test]
    fn reverse_flips_normal() {
        let c = unit_circle();
        let r = c.reversed();
        assert!((r.axis().direction.z + 1.0).abs() < CONFUSION);
    }

    #[test]
    fn from_center_normal_radius() {
        let c = Circ::from_center_normal_radius(
            Pnt::new(1.0, 2.0, 3.0),
            Pnt::new(0.0, 0.0, 1.0),
            4.0,
        );
        assert!((c.radius() - 4.0).abs() < CONFUSION);
        assert!(c.location().is_equal(Pnt::new(1.0, 2.0, 3.0), CONFUSION));
    }

    #[test]
    fn rotate_moves_center() {
        let c = Circ::new(
            Ax3::from_origin_normal(
                Pnt::new(1.0, 0.0, 0.0),
                Pnt::new(0.0, 0.0, 1.0),
                Pnt::new(1.0, 0.0, 0.0),
            ),
            1.0,
        );
        let r = c.rotated(Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0)), FRAC_PI_2);
        assert!(r.location().is_equal(Pnt::new(0.0, 1.0, 0.0), CONFUSION));
    }

    #[test]
    fn in_place_translate() {
        let mut c = unit_circle();
        c.translate(Pnt::new(0.0, 7.0, 0.0));
        assert!((c.location().y - 7.0).abs() < CONFUSION);
    }

    // Parab tests
    #[test]
    fn parab_parameter_is_twice_focal() {
        let p = Parab::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 3.0);
        assert!((p.parameter() - 6.0).abs() < CONFUSION);
    }

    #[test]
    fn parab_directrix_distance_is_twice_focal() {
        let p = Parab::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 5.0);
        assert!((p.directrix_distance() - 10.0).abs() < CONFUSION);
    }
}
