// FILE: gp_cone.rs
//! Port of gp_Cone — an infinite cone in 3D space.
//!
//! The cone is defined by an apex point, a local coordinate system (gp_Ax3),
//! and a half-angle (semi-angle) in radians where 0 < angle < PI/2.
//! The apex coincides with the origin of the axis system; the cone opens
//! along the positive Z direction of that system.

use crate::gp::{Pnt, Vec3, Ax1, Ax2, Ax3, Trsf};
use crate::gp_prim::Circ;
use crate::precision::ANGULAR;

/// Represents an infinite cone in 3D space defined by an apex and a half-angle.
// occt: gp_Cone
#[derive(Clone, Debug)]
pub struct GpCone {
    /// The local coordinate system; its origin is the apex of the cone.
    axis: Ax3,
    /// Half-angle (semi-angle) of the cone in radians. Must satisfy 0 < half_angle < PI/2.
    half_angle: f64,
}

impl GpCone {
    /// Constructs a cone with the given axis system and half-angle.
    ///
    /// The apex is located at the origin of `axis`. The cone opens along
    /// the Z direction of `axis`. `half_angle` must be in (0, PI/2).
    pub fn new(axis: Ax3, half_angle: f64) -> Self {
        assert!(
            half_angle > ANGULAR && half_angle < std::f64::consts::FRAC_PI_2 - ANGULAR,
            "gp_Cone: half_angle must satisfy 0 < angle < PI/2"
        );
        Self { axis, half_angle }
    }

    /// Returns the apex of the cone (origin of the axis system).
    pub fn apex(&self) -> Pnt {
        self.axis.location
    }

    /// Returns the local coordinate system (Ax3) of the cone.
    pub fn axis(&self) -> &Ax3 {
        &self.axis
    }

    /// Returns the half-angle (semi-angle) of the cone in radians.
    pub fn semi_angle(&self) -> f64 {
        self.half_angle
    }

    /// Returns the location (apex) of the cone.
    pub fn location(&self) -> Pnt {
        self.axis.location
    }

    /// Sets the apex (location) of the cone.
    pub fn set_location(&mut self, p: Pnt) {
        self.axis.location = p;
    }

    /// Sets the axis system of the cone.
    pub fn set_axis(&mut self, ax: Ax3) {
        self.axis = ax;
    }

    /// Sets the half-angle of the cone.
    pub fn set_semi_angle(&mut self, angle: f64) {
        assert!(
            angle > ANGULAR && angle < std::f64::consts::FRAC_PI_2 - ANGULAR,
            "gp_Cone: half_angle must satisfy 0 < angle < PI/2"
        );
        self.half_angle = angle;
    }

    /// Returns whether the cone's axis system is direct (right-handed).
    pub fn is_direct(&self) -> bool {
        self.axis.x_dir.cross(self.axis.y_dir).dot(self.axis.z_dir) > 0.0
    }

    /// Returns the circle at the given distance `u` along the cone axis from the apex.
    ///
    /// The circle lies in the plane perpendicular to the cone axis at parameter `u`.
    /// For a direct axis, positive `u` moves in the +Z direction.
    /// The radius at parameter `u` is `|u| * tan(half_angle)`.
    pub fn at_angle(&self, u: f64) -> Circ {
        let radius = u.abs() * self.half_angle.tan();
        // Build Ax2 for the circle: origin at apex + u * Z_dir, with same X and Y axes
        let z = self.axis.z_dir; // main axis direction (Z of ax3)
        let loc = self.axis.location;
        let center = Pnt::new(
            loc.x + u * z.x,
            loc.y + u * z.y,
            loc.z + u * z.z,
        );
        let ax2 = Ax2 {
            location: center,
            x_dir: self.axis.x_dir,
            y_dir: self.axis.y_dir,
            z_dir: self.axis.z_dir,
        };
        Circ::new(ax2, radius)
    }

    /// Returns the circle at distance `u` along the axis (alias for `at_angle`).
    pub fn circle_at(&self, u: f64) -> Circ {
        self.at_angle(u)
    }

    /// Returns the radius of the cone at parameter `u` (distance along axis from apex).
    pub fn radius_at(&self, u: f64) -> f64 {
        u.abs() * self.half_angle.tan()
    }

    // -------------------------------------------------------------------------
    // Transformations
    // -------------------------------------------------------------------------

    /// Returns a new cone that is the mirror of this cone through point `pt`.
    pub fn mirrored_through_point(&self, pt: &Pnt) -> Self {
        // Point symmetry: new_loc = 2*pt - loc; direction vectors are negated.
        let loc = self.axis.location;
        let new_loc = Pnt::new(2.0 * pt.x - loc.x, 2.0 * pt.y - loc.y, 2.0 * pt.z - loc.z);
        Self {
            axis: Ax3 {
                location: new_loc,
                x_dir: Pnt::new(-self.axis.x_dir.x, -self.axis.x_dir.y, -self.axis.x_dir.z),
                y_dir: Pnt::new(-self.axis.y_dir.x, -self.axis.y_dir.y, -self.axis.y_dir.z),
                z_dir: Pnt::new(-self.axis.z_dir.x, -self.axis.z_dir.y, -self.axis.z_dir.z),
            },
            half_angle: self.half_angle,
        }
    }

    /// Mirrors this cone in-place through point `pt`.
    pub fn mirror_through_point(&mut self, pt: &Pnt) {
        *self = self.mirrored_through_point(pt);
    }

    /// Returns a new cone that is the mirror of this cone through axis `ax1`.
    pub fn mirrored_through_ax1(&self, ax1: &Ax1) -> Self {
        // Reflection through a line: d' = 2*(d·u)*u - d
        let u = ax1.direction.normalized();
        let reflect_vec = |d: Pnt| {
            let proj = 2.0 * d.dot(u);
            Pnt::new(proj * u.x - d.x, proj * u.y - d.y, proj * u.z - d.z)
        };
        let new_loc = self.axis.location.mirrored_axis(*ax1);
        Self {
            axis: Ax3 {
                location: new_loc,
                x_dir: reflect_vec(self.axis.x_dir),
                y_dir: reflect_vec(self.axis.y_dir),
                z_dir: reflect_vec(self.axis.z_dir),
            },
            half_angle: self.half_angle,
        }
    }

    /// Mirrors this cone in-place through axis `ax1`.
    pub fn mirror_through_ax1(&mut self, ax1: &Ax1) {
        *self = self.mirrored_through_ax1(ax1);
    }

    /// Returns a new cone that is the mirror of this cone through plane `ax2`.
    pub fn mirrored_through_ax2(&self, ax2: &Ax2) -> Self {
        // Reflection through a plane with normal n (z_dir of ax2):
        // d' = d - 2*(d·n)*n
        let n = ax2.z_dir.normalized();
        let reflect_vec = |d: Pnt| {
            let proj = 2.0 * d.dot(n);
            Pnt::new(d.x - proj * n.x, d.y - proj * n.y, d.z - proj * n.z)
        };
        let new_loc = self.axis.location.mirrored_plane(*ax2);
        Self {
            axis: Ax3 {
                location: new_loc,
                x_dir: reflect_vec(self.axis.x_dir),
                y_dir: reflect_vec(self.axis.y_dir),
                z_dir: reflect_vec(self.axis.z_dir),
            },
            half_angle: self.half_angle,
        }
    }

    /// Mirrors this cone in-place through plane `ax2`.
    pub fn mirror_through_ax2(&mut self, ax2: &Ax2) {
        *self = self.mirrored_through_ax2(ax2);
    }

    /// Returns a new cone rotated about `ax1` by `angle` radians.
    pub fn rotated(&self, ax1: &Ax1, angle: f64) -> Self {
        let t = Trsf::rotation(*ax1, angle);
        Self {
            axis: self.axis.transformed(&t),
            half_angle: self.half_angle,
        }
    }

    /// Rotates this cone in-place about `ax1` by `angle` radians.
    pub fn rotate(&mut self, ax1: &Ax1, angle: f64) {
        *self = self.rotated(ax1, angle);
    }

    /// Returns a new cone scaled about point `pt` by factor `s`.
    ///
    /// Scaling preserves the half-angle; only the apex location changes.
    /// If `s` is negative the cone's axis direction may flip, but the
    /// half-angle remains positive.
    pub fn scaled(&self, pt: &Pnt, s: f64) -> Self {
        let loc = self.axis.location;
        let new_loc = Pnt::new(
            pt.x + s * (loc.x - pt.x),
            pt.y + s * (loc.y - pt.y),
            pt.z + s * (loc.z - pt.z),
        );
        // Scaling flips directions if s < 0
        let sign = if s < 0.0 { -1.0 } else { 1.0 };
        Self {
            axis: Ax3 {
                location: new_loc,
                x_dir: Pnt::new(sign * self.axis.x_dir.x, sign * self.axis.x_dir.y, sign * self.axis.x_dir.z),
                y_dir: Pnt::new(sign * self.axis.y_dir.x, sign * self.axis.y_dir.y, sign * self.axis.y_dir.z),
                z_dir: Pnt::new(sign * self.axis.z_dir.x, sign * self.axis.z_dir.y, sign * self.axis.z_dir.z),
            },
            // half_angle is unchanged by uniform scaling
            half_angle: self.half_angle,
        }
    }

    /// Scales this cone in-place about point `pt` by factor `s`.
    pub fn scale(&mut self, pt: &Pnt, s: f64) {
        *self = self.scaled(pt, s);
    }

    /// Returns a new cone translated by vector `v`.
    pub fn translated(&self, v: &Vec3) -> Self {
        Self {
            axis: Ax3 {
                location: Pnt::new(self.axis.location.x + v.x, self.axis.location.y + v.y, self.axis.location.z + v.z),
                x_dir: self.axis.x_dir,
                y_dir: self.axis.y_dir,
                z_dir: self.axis.z_dir,
            },
            half_angle: self.half_angle,
        }
    }

    /// Translates this cone in-place by vector `v`.
    pub fn translate(&mut self, v: &Vec3) {
        self.axis.location = Pnt::new(
            self.axis.location.x + v.x,
            self.axis.location.y + v.y,
            self.axis.location.z + v.z,
        );
    }

    /// Returns a new cone translated from point `p1` to point `p2`.
    pub fn translated_from_to(&self, p1: &Pnt, p2: &Pnt) -> Self {
        let v = Vec3::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        self.translated(&v)
    }

    /// Translates this cone in-place from point `p1` to point `p2`.
    pub fn translate_from_to(&mut self, p1: &Pnt, p2: &Pnt) {
        let v = Vec3::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        self.translate(&v);
    }

    /// Returns a clone of this cone (compatible with a "GpCon" representation).
    pub fn to_gp_con(&self) -> GpCone {
        self.clone()
    }
}

// ---------------------------------------------------------------------------
// Helper — build a default (world-aligned) cone for tests and convenience.
// ---------------------------------------------------------------------------

impl Default for GpCone {
    /// Default cone: apex at origin, axis along +Z, half-angle = PI/4.
    fn default() -> Self {
        Self::new(Ax3::identity(), std::f64::consts::FRAC_PI_4)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Pnt;
    use std::f64::consts::{FRAC_PI_4, FRAC_PI_2};

    // Tolerance for floating-point comparisons.
    const EPS: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    fn pnt_approx_eq(a: &Pnt, b: &Pnt, eps: f64) -> bool {
        approx_eq(a.x, b.x, eps) && approx_eq(a.y, b.y, eps) && approx_eq(a.z, b.z, eps)
    }

    #[test]
    fn test_default_cone_apex_at_origin() {
        let cone = GpCone::default();
        let apex = cone.apex();
        assert!(pnt_approx_eq(&apex, &Pnt::new(0.0, 0.0, 0.0), EPS));
    }

    #[test]
    fn test_default_cone_semi_angle() {
        let cone = GpCone::default();
        assert!(approx_eq(cone.semi_angle(), FRAC_PI_4, EPS));
    }

    #[test]
    fn test_default_cone_is_direct() {
        let cone = GpCone::default();
        assert!(cone.is_direct());
    }

    #[test]
    fn test_at_angle_radius_zero_at_apex() {
        let cone = GpCone::default();
        let circ = cone.at_angle(0.0);
        // At u=0 (the apex) the radius should be 0.
        assert!(approx_eq(circ.radius(), 0.0, EPS));
    }

    #[test]
    fn test_at_angle_radius_at_unit_distance() {
        // half_angle = PI/4 => tan(PI/4) = 1.0, so radius at u=1 is 1.0.
        let cone = GpCone::default();
        let circ = cone.at_angle(1.0);
        assert!(approx_eq(circ.radius(), 1.0, EPS));
    }

    #[test]
    fn test_at_angle_radius_at_distance_2() {
        let cone = GpCone::default();
        let circ = cone.at_angle(2.0);
        assert!(approx_eq(circ.radius(), 2.0 * FRAC_PI_4.tan(), EPS));
    }

    #[test]
    fn test_at_angle_center_position() {
        // Default cone: apex at origin, Z-axis is (0,0,1).
        // At u=3, center should be (0,0,3).
        let cone = GpCone::default();
        let circ = cone.at_angle(3.0);
        let center = circ.location();
        assert!(pnt_approx_eq(&center, &Pnt::new(0.0, 0.0, 3.0), EPS));
    }

    #[test]
    fn test_at_angle_negative_u_abs_radius() {
        // Negative u gives same radius magnitude (symmetric cone).
        let cone = GpCone::default();
        let r_pos = cone.at_angle(2.0).radius();
        let r_neg = cone.at_angle(-2.0).radius();
        assert!(approx_eq(r_pos, r_neg, EPS));
    }

    #[test]
    fn test_radius_at() {
        let cone = GpCone::default();
        // half_angle = PI/4, tan = 1
        assert!(approx_eq(cone.radius_at(5.0), 5.0, EPS));
        assert!(approx_eq(cone.radius_at(0.0), 0.0, EPS));
    }

    #[test]
    fn test_translate() {
        let mut cone = GpCone::default();
        let v = Vec3::new(1.0, 2.0, 3.0);
        cone.translate(&v);
        let apex = cone.apex();
        assert!(pnt_approx_eq(&apex, &Pnt::new(1.0, 2.0, 3.0), EPS));
        // semi-angle unchanged
        assert!(approx_eq(cone.semi_angle(), FRAC_PI_4, EPS));
    }

    #[test]
    fn test_translated_does_not_mutate_original() {
        let cone = GpCone::default();
        let v = Vec3::new(10.0, 0.0, 0.0);
        let cone2 = cone.translated(&v);
        // Original unchanged
        assert!(pnt_approx_eq(&cone.apex(), &Pnt::new(0.0, 0.0, 0.0), EPS));
        // New cone moved
        assert!(pnt_approx_eq(&cone2.apex(), &Pnt::new(10.0, 0.0, 0.0), EPS));
    }

    #[test]
    fn test_translate_from_to() {
        let mut cone = GpCone::default();
        let p1 = Pnt::new(0.0, 0.0, 0.0);
        let p2 = Pnt::new(4.0, 5.0, 6.0);
        cone.translate_from_to(&p1, &p2);
        assert!(pnt_approx_eq(&cone.apex(), &Pnt::new(4.0, 5.0, 6.0), EPS));
    }

    #[test]
    fn test_scale_preserves_half_angle() {
        let mut cone = GpCone::default();
        let pt = Pnt::new(0.0, 0.0, 0.0);
        cone.scale(&pt, 3.0);
        // Apex still at origin (scaled about origin)
        assert!(pnt_approx_eq(&cone.apex(), &Pnt::new(0.0, 0.0, 0.0), EPS));
        // Half-angle unchanged
        assert!(approx_eq(cone.semi_angle(), FRAC_PI_4, EPS));
    }

    #[test]
    fn test_scale_moves_apex_when_center_differs() {
        let cone = GpCone::default(); // apex at (0,0,0)
        let pt = Pnt::new(1.0, 0.0, 0.0);
        let cone2 = cone.scaled(&pt, 2.0);
        // apex (0,0,0) scaled by 2 about (1,0,0): new_apex = pt + 2*(apex - pt) = (1,0,0) + 2*(-1,0,0) = (-1,0,0)
        assert!(pnt_approx_eq(&cone2.apex(), &Pnt::new(-1.0, 0.0, 0.0), EPS));
        assert!(approx_eq(cone2.semi_angle(), FRAC_PI_4, EPS));
    }

    #[test]
    fn test_rotate_apex() {
        // Rotate a cone whose apex is at (1,0,0) by PI/2 about Z-axis.
        let ax3 = Ax3::from_origin_normal(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let cone = GpCone::new(ax3, FRAC_PI_4);
        let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
        let cone2 = cone.rotated(&z_ax1, FRAC_PI_2);
        // Apex rotated from (1,0,0) by PI/2 about Z -> (0,1,0)
        assert!(pnt_approx_eq(&cone2.apex(), &Pnt::new(0.0, 1.0, 0.0), EPS));
        // half_angle unchanged
        assert!(approx_eq(cone2.semi_angle(), FRAC_PI_4, EPS));
    }

    #[test]
    fn test_mirror_through_point() {
        let cone = GpCone::default(); // apex at (0,0,0)
        let pt = Pnt::new(1.0, 0.0, 0.0);
        let cone2 = cone.mirrored_through_point(&pt);
        // apex mirrored: 2*(1,0,0) - (0,0,0) = (2,0,0)
        assert!(pnt_approx_eq(&cone2.apex(), &Pnt::new(2.0, 0.0, 0.0), EPS));
        // half_angle unchanged
        assert!(approx_eq(cone2.semi_angle(), FRAC_PI_4, EPS));
    }

    #[test]
    fn test_set_semi_angle() {
        let mut cone = GpCone::default();
        let new_angle = 0.3;
        cone.set_semi_angle(new_angle);
        assert!(approx_eq(cone.semi_angle(), new_angle, EPS));
    }

    #[test]
    fn test_set_location() {
        let mut cone = GpCone::default();
        let new_loc = Pnt::new(5.0, 6.0, 7.0);
        cone.set_location(new_loc.clone());
        assert!(pnt_approx_eq(&cone.apex(), &new_loc, EPS));
    }

    #[test]
    fn test_custom_cone_non_unit_half_angle() {
        let angle = 0.5_f64; // ~28.6 degrees
        let ax3 = Ax3::from_origin_normal(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let cone = GpCone::new(ax3, angle);
        let r = cone.radius_at(10.0);
        let expected = 10.0 * angle.tan();
        assert!(approx_eq(r, expected, EPS));
    }

    #[test]
    fn test_location_equals_apex() {
        let cone = GpCone::default();
        assert!(pnt_approx_eq(&cone.apex(), &cone.location(), EPS));
    }

    #[test]
    fn test_to_gp_con() {
        let cone = GpCone::default();
        let gp_con = cone.to_gp_con();
        assert!(approx_eq(gp_con.half_angle, FRAC_PI_4, EPS));
    }

    #[test]
    fn test_mirror_through_ax1_apex() {
        // Mirror cone apex (1,0,0) through Y-axis (origin, dir Y).
        let ax3 = Ax3::from_origin_normal(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let cone = GpCone::new(ax3, FRAC_PI_4);
        let y_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
        let cone2 = cone.mirrored_through_ax1(&y_ax1);
        // Mirroring (1,0,0) through the Y-axis gives (-1,0,0)
        assert!(pnt_approx_eq(&cone2.apex(), &Pnt::new(-1.0, 0.0, 0.0), EPS));
        assert!(approx_eq(cone2.semi_angle(), FRAC_PI_4, EPS));
    }

    #[test]
    #[should_panic]
    fn test_half_angle_zero_panics() {
        let ax3 = Ax3::identity();
        let _ = GpCone::new(ax3, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_half_angle_pi_over_2_panics() {
        let ax3 = Ax3::identity();
        let _ = GpCone::new(ax3, FRAC_PI_2);
    }

    #[test]
    fn test_circle_at_normal_direction_matches_cone_axis() {
        // The circle at u=5 should have its normal along the cone axis direction.
        let cone = GpCone::default(); // axis along +Z
        let circ = cone.at_angle(5.0);
        let normal = circ.axis(); // z_dir of the circle's placement (returns Pnt)
        let cone_dir = cone.axis().z_dir;
        // Both should be (0,0,1)
        assert!(approx_eq(normal.x, cone_dir.x, EPS));
        assert!(approx_eq(normal.y, cone_dir.y, EPS));
        assert!(approx_eq(normal.z, cone_dir.z, EPS));
    }
}
