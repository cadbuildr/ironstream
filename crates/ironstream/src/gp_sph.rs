// FILE: gp_sph.rs
//! Port of `gp_Sphere` — an analytic sphere in 3D space.
//!
//! A sphere is fully defined by:
//! - A right-handed local coordinate system ([`Ax3`]) whose origin is the
//!   center of the sphere.
//! - A radius `R > 0`.
//!
//! The local frame's Z axis is the "north pole" axis.  The equatorial
//! plane is the XY plane of the frame.
//!
//! Key formulas used in this module:
//! - Surface area: `4·π·R²`
//! - Volume:       `(4/3)·π·R³`
//! - A point is on the sphere when `|P - C| == R`.

use crate::gp::{Ax1, Ax2, Ax3, Pnt, Trsf, Vec3};
use crate::gp_prim::Circ;
use std::f64::consts::PI;

/// A sphere in 3D space defined by a placement and a radius.
// occt: gp_Sphere
#[derive(Clone, Debug)]
pub struct GpSphere {
    /// The local coordinate system. Its origin is the sphere center.
    /// Z axis points toward the "north pole".
    pos: Ax3,
    /// Sphere radius (must be > 0).
    radius: f64,
}

impl GpSphere {
    // ──────────────────────────── constructors ──────────────────────────────

    /// Construct a sphere from a placement and a positive radius.
    ///
    /// # Panics
    /// Panics if `radius <= 0`.
    pub fn new(pos: Ax3, radius: f64) -> Self {
        assert!(radius > 0.0, "gp_Sphere: radius must be > 0, got {radius}");
        Self { pos, radius }
    }

    // ───────────────────────────── accessors ────────────────────────────────

    /// Center of the sphere (origin of the local frame).
    // occt: gp_Sphere::Location
    pub fn location(&self) -> Pnt {
        self.pos.location
    }

    /// The full local coordinate system (`gp_Ax3`).
    // occt: gp_Sphere::Position
    pub fn position(&self) -> &Ax3 {
        &self.pos
    }

    /// The sphere radius.
    // occt: gp_Sphere::Radius
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// The main axis of the sphere (Z axis of the local frame), as an `Ax1`.
    // occt: gp_Sphere::Axis
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.z_dir)
    }

    /// X axis of the local frame as an `Ax1`.
    // occt: gp_Sphere::XAxis
    pub fn x_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.x_dir)
    }

    /// Y axis of the local frame as an `Ax1`.
    // occt: gp_Sphere::YAxis
    pub fn y_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.y_dir)
    }

    // ─────────────────────────── setters ────────────────────────────────────

    /// Set the radius (must be > 0).
    // occt: gp_Sphere::SetRadius
    pub fn set_radius(&mut self, r: f64) {
        assert!(r > 0.0, "gp_Sphere::SetRadius: radius must be > 0, got {r}");
        self.radius = r;
    }

    /// Replace the local coordinate system.
    // occt: gp_Sphere::SetPosition
    pub fn set_position(&mut self, pos: Ax3) {
        self.pos = pos;
    }

    /// Move the sphere center to `p` (keeps axes and radius unchanged).
    // occt: gp_Sphere::SetLocation
    pub fn set_location(&mut self, p: Pnt) {
        self.pos.location = p;
    }

    // ──────────────────────── geometric properties ──────────────────────────

    /// Surface area of the sphere: `4·π·R²`.
    // occt: gp_Sphere::Area
    pub fn area(&self) -> f64 {
        4.0 * PI * self.radius * self.radius
    }

    /// Volume enclosed by the sphere: `(4/3)·π·R³`.
    // occt: gp_Sphere::Volume
    pub fn volume(&self) -> f64 {
        (4.0 / 3.0) * PI * self.radius.powi(3)
    }

    /// Returns `true` when the local frame is direct (right-handed).
    // occt: gp_Sphere::Direct
    pub fn is_direct(&self) -> bool {
        self.pos.x_dir.cross(self.pos.y_dir).dot(self.pos.z_dir) > 0.0
    }

    // ──────────────────────────── iso-circles ───────────────────────────────

    /// The "equatorial" circle in the XY plane of the local frame
    /// (the circle at latitude V = 0).
    // occt: gp_Sphere::Equator
    pub fn equator(&self) -> Circ {
        Circ::new(self.pos, self.radius)
    }

    /// The circle at latitude angle `v` (measured from the equatorial plane,
    /// positive toward the north pole).
    ///
    /// The circle lies in a plane parallel to the equatorial plane, at height
    /// `R·sin(v)` along the Z axis.  Its radius is `R·cos(v)`.
    // occt: gp_Sphere::VIso (analogue)
    pub fn circle_at_v(&self, v: f64) -> Circ {
        let (sv, cv) = v.sin_cos();
        let center = self.pos.location + self.pos.z_dir * (self.radius * sv);
        let frame = Ax3 {
            location: center,
            x_dir: self.pos.x_dir,
            y_dir: self.pos.y_dir,
            z_dir: self.pos.z_dir,
        };
        let r = (self.radius * cv).abs();
        Circ::new(frame, r)
    }

    /// The great circle (meridian) at longitude `u` — passes through both
    /// poles.  Its plane is spanned by the radial direction at angle `u`
    /// in the XY plane and the Z axis.
    // occt: gp_Sphere::UIso (analogue)
    pub fn circle_at_u(&self, u: f64) -> Circ {
        let (su, cu) = u.sin_cos();
        // Radial direction in the equatorial plane.
        let x_radial = self.pos.x_dir * cu + self.pos.y_dir * su;
        // Normal to the meridian plane (perpendicular in equatorial plane).
        let z_normal = self.pos.x_dir * (-su) + self.pos.y_dir * cu;
        let frame = Ax3 {
            location: self.pos.location,
            x_dir: x_radial,
            y_dir: self.pos.z_dir,
            z_dir: z_normal,
        };
        Circ::new(frame, self.radius)
    }

    // ──────────────────────── point-on-sphere test ───────────────────────────

    /// Returns the Cartesian point on the sphere at parametric coordinates
    /// `(u, v)`: longitude `u` ∈ [0, 2π) and latitude `v` ∈ [-π/2, π/2].
    ///
    /// ```text
    /// P = C + R·cos(v)·cos(u)·X + R·cos(v)·sin(u)·Y + R·sin(v)·Z
    /// ```
    // occt: gp_Sphere::Value (implicit via Geom surface)
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        let (su, cu) = u.sin_cos();
        let (sv, cv) = v.sin_cos();
        self.pos.location
            + self.pos.x_dir * (self.radius * cv * cu)
            + self.pos.y_dir * (self.radius * cv * su)
            + self.pos.z_dir * (self.radius * sv)
    }

    /// Returns `true` when `p` lies on the sphere surface within tolerance `tol`.
    // occt: gp_Sphere::Contains
    pub fn contains(&self, p: Pnt, tol: f64) -> bool {
        let d = (p - self.pos.location).norm();
        (d - self.radius).abs() <= tol
    }

    // ─────────────────────────── transforms ─────────────────────────────────

    /// Mirror the sphere through a point `pt` — center maps to `2·pt - C`.
    // occt: gp_Sphere::Mirrored (gp_Pnt overload)
    pub fn mirrored_through_point(&self, pt: &Pnt) -> Self {
        let loc = self.pos.location;
        let new_loc = Pnt::new(
            2.0 * pt.x - loc.x,
            2.0 * pt.y - loc.y,
            2.0 * pt.z - loc.z,
        );
        Self {
            pos: Ax3 {
                location: new_loc,
                x_dir: Pnt::new(-self.pos.x_dir.x, -self.pos.x_dir.y, -self.pos.x_dir.z),
                y_dir: Pnt::new(-self.pos.y_dir.x, -self.pos.y_dir.y, -self.pos.y_dir.z),
                z_dir: Pnt::new(-self.pos.z_dir.x, -self.pos.z_dir.y, -self.pos.z_dir.z),
            },
            radius: self.radius,
        }
    }

    /// Mirror the sphere through a line (`Ax1`) — center is reflected over
    /// the line, axis directions are reflected.
    // occt: gp_Sphere::Mirrored (gp_Ax1 overload)
    pub fn mirrored_through_ax1(&self, ax: &Ax1) -> Self {
        let u = ax.direction.normalized();
        let reflect_vec = |d: Pnt| {
            let proj = 2.0 * d.dot(u);
            Pnt::new(proj * u.x - d.x, proj * u.y - d.y, proj * u.z - d.z)
        };
        Self {
            pos: Ax3 {
                location: self.pos.location.mirrored_axis(*ax),
                x_dir: reflect_vec(self.pos.x_dir),
                y_dir: reflect_vec(self.pos.y_dir),
                z_dir: reflect_vec(self.pos.z_dir),
            },
            radius: self.radius,
        }
    }

    /// Mirror the sphere through a plane (`Ax2` — its XY plane is the mirror
    /// plane).
    // occt: gp_Sphere::Mirrored (gp_Ax2 overload)
    pub fn mirrored_through_ax2(&self, ax: &Ax2) -> Self {
        let n = ax.z_dir.normalized();
        let reflect_vec = |d: Pnt| {
            let proj = 2.0 * d.dot(n);
            Pnt::new(d.x - proj * n.x, d.y - proj * n.y, d.z - proj * n.z)
        };
        Self {
            pos: Ax3 {
                location: self.pos.location.mirrored_plane(*ax),
                x_dir: reflect_vec(self.pos.x_dir),
                y_dir: reflect_vec(self.pos.y_dir),
                z_dir: reflect_vec(self.pos.z_dir),
            },
            radius: self.radius,
        }
    }

    /// Return a new sphere rotated about `ax` by `angle` radians.
    // occt: gp_Sphere::Rotated
    pub fn rotated(&self, ax: &Ax1, angle: f64) -> Self {
        let t = Trsf::rotation(*ax, angle);
        Self {
            pos: self.pos.transformed(&t),
            radius: self.radius,
        }
    }

    /// Rotate the sphere in place.
    // occt: gp_Sphere::Rotate
    pub fn rotate(&mut self, ax: &Ax1, angle: f64) {
        *self = self.rotated(ax, angle);
    }

    /// Return a new sphere scaled about point `pt` by factor `s`.
    ///
    /// The radius scales by `|s|`.  If `s < 0` the frame directions are
    /// negated (as OCCT specifies).
    // occt: gp_Sphere::Scaled
    pub fn scaled(&self, pt: &Pnt, s: f64) -> Self {
        let loc = self.pos.location;
        let new_loc = Pnt::new(
            pt.x + s * (loc.x - pt.x),
            pt.y + s * (loc.y - pt.y),
            pt.z + s * (loc.z - pt.z),
        );
        let sign = if s < 0.0 { -1.0_f64 } else { 1.0 };
        Self {
            pos: Ax3 {
                location: new_loc,
                x_dir: self.pos.x_dir * sign,
                y_dir: self.pos.y_dir * sign,
                z_dir: self.pos.z_dir * sign,
            },
            radius: self.radius * s.abs(),
        }
    }

    /// Scale the sphere in place.
    // occt: gp_Sphere::Scale
    pub fn scale(&mut self, pt: &Pnt, s: f64) {
        *self = self.scaled(pt, s);
    }

    /// Return a new sphere translated by vector `v`.
    // occt: gp_Sphere::Translated (gp_Vec overload)
    pub fn translated(&self, v: &Vec3) -> Self {
        Self {
            pos: Ax3 {
                location: self.pos.location + *v,
                x_dir: self.pos.x_dir,
                y_dir: self.pos.y_dir,
                z_dir: self.pos.z_dir,
            },
            radius: self.radius,
        }
    }

    /// Translate the sphere in place.
    // occt: gp_Sphere::Translate (gp_Vec overload)
    pub fn translate(&mut self, v: &Vec3) {
        self.pos.location = self.pos.location + *v;
    }

    /// Return a new sphere translated from `p1` to `p2`.
    // occt: gp_Sphere::Translated (gp_Pnt, gp_Pnt overload)
    pub fn translated_points(&self, p1: &Pnt, p2: &Pnt) -> Self {
        let v = Vec3::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        self.translated(&v)
    }

    /// Apply an arbitrary affine transform `t`.
    // occt: gp_Sphere::Transformed
    pub fn transformed(&self, t: &Trsf) -> Self {
        Self {
            pos: self.pos.transformed(t),
            radius: (self.radius * t.scale_factor()).abs(),
        }
    }
}

// ─────────────────────────── Default ────────────────────────────────────────

impl Default for GpSphere {
    /// Unit sphere centered at the origin with the standard XYZ frame.
    fn default() -> Self {
        Self::new(Ax3::identity(), 1.0)
    }
}

// ─────────────────────────────── tests ──────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::{Ax1, Ax3, Pnt, Vec3};
    use std::f64::consts::{FRAC_PI_2, PI};

    const EPS: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    fn pnt_eq(a: Pnt, b: Pnt) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
    }

    #[test]
    fn test_default_sphere_center_at_origin() {
        let s = GpSphere::default();
        assert!(pnt_eq(s.location(), Pnt::new(0.0, 0.0, 0.0)));
    }

    #[test]
    fn test_default_sphere_radius_one() {
        let s = GpSphere::default();
        assert!(approx_eq(s.radius(), 1.0));
    }

    #[test]
    fn test_area_unit_sphere() {
        let s = GpSphere::default();
        assert!(approx_eq(s.area(), 4.0 * PI));
    }

    #[test]
    fn test_volume_unit_sphere() {
        let s = GpSphere::default();
        let expected = (4.0 / 3.0) * PI;
        assert!(approx_eq(s.volume(), expected));
    }

    #[test]
    fn test_area_radius_2() {
        let s = GpSphere::new(Ax3::identity(), 2.0);
        assert!(approx_eq(s.area(), 16.0 * PI));
    }

    #[test]
    fn test_volume_radius_3() {
        let s = GpSphere::new(Ax3::identity(), 3.0);
        let expected = 4.0 * PI * 9.0; // (4/3)*PI*27 = 36*PI
        assert!(approx_eq(s.volume(), expected));
    }

    #[test]
    fn test_equator_radius_matches_sphere_radius() {
        let s = GpSphere::default();
        let eq = s.equator();
        assert!(approx_eq(eq.radius(), 1.0));
    }

    #[test]
    fn test_equator_center_is_sphere_center() {
        let center = Pnt::new(3.0, 2.0, 1.0);
        let ax3 = Ax3::from_origin_normal(center, Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0));
        let s = GpSphere::new(ax3, 5.0);
        assert!(pnt_eq(s.equator().location(), center));
    }

    #[test]
    fn test_circle_at_v_north_pole_zero_radius() {
        // At v = PI/2 (north pole), circle radius = R*cos(PI/2) = 0.
        let s = GpSphere::default();
        let c = s.circle_at_v(FRAC_PI_2);
        assert!(c.radius() < EPS);
    }

    #[test]
    fn test_circle_at_v_south_pole_zero_radius() {
        let s = GpSphere::default();
        let c = s.circle_at_v(-FRAC_PI_2);
        assert!(c.radius() < EPS);
    }

    #[test]
    fn test_circle_at_v_equator_matches_equator() {
        let s = GpSphere::new(Ax3::identity(), 4.0);
        let c = s.circle_at_v(0.0);
        assert!(approx_eq(c.radius(), 4.0));
    }

    #[test]
    fn test_circle_at_v_center_offset() {
        // At v = PI/6, sin(v) = 0.5, so center offset = R * 0.5 along Z.
        let r = 3.0_f64;
        let s = GpSphere::new(Ax3::identity(), r);
        let v = PI / 6.0;
        let c = s.circle_at_v(v);
        let expected_z = r * (PI / 6.0_f64).sin();
        assert!(approx_eq(c.location().x, 0.0));
        assert!(approx_eq(c.location().y, 0.0));
        assert!(approx_eq(c.location().z, expected_z));
    }

    #[test]
    fn test_value_north_pole() {
        let s = GpSphere::default();
        // At u=0, v=PI/2: P = C + R*Z = (0,0,1)
        let p = s.value(0.0, FRAC_PI_2);
        assert!(pnt_eq(p, Pnt::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn test_value_equator_x() {
        let s = GpSphere::default();
        // At u=0, v=0: P = C + R*X = (1,0,0)
        let p = s.value(0.0, 0.0);
        assert!(pnt_eq(p, Pnt::new(1.0, 0.0, 0.0)));
    }

    #[test]
    fn test_value_equator_y() {
        let s = GpSphere::default();
        // At u=PI/2, v=0: P = (0,1,0)
        let p = s.value(FRAC_PI_2, 0.0);
        assert!(approx_eq(p.x, 0.0));
        assert!(approx_eq(p.y, 1.0));
        assert!(approx_eq(p.z, 0.0));
    }

    #[test]
    fn test_value_all_points_at_radius_distance() {
        let r = 7.0;
        let s = GpSphere::new(Ax3::identity(), r);
        for &u in &[0.0, 1.0, 2.5, PI] {
            for &v in &[-FRAC_PI_2, -0.5, 0.0, 0.7, FRAC_PI_2] {
                let p = s.value(u, v);
                let dist = (p - s.location()).norm();
                assert!((dist - r).abs() < EPS, "u={u} v={v} dist={dist}");
            }
        }
    }

    #[test]
    fn test_contains_surface_point() {
        let s = GpSphere::default();
        let p = s.value(1.2, 0.4);
        assert!(s.contains(p, 1e-9));
    }

    #[test]
    fn test_contains_interior_false() {
        let s = GpSphere::default();
        assert!(!s.contains(Pnt::new(0.0, 0.0, 0.0), 1e-9));
    }

    #[test]
    fn test_is_direct_identity_frame() {
        let s = GpSphere::default();
        assert!(s.is_direct());
    }

    #[test]
    fn test_set_radius() {
        let mut s = GpSphere::default();
        s.set_radius(5.0);
        assert!(approx_eq(s.radius(), 5.0));
    }

    #[test]
    fn test_set_location() {
        let mut s = GpSphere::default();
        s.set_location(Pnt::new(1.0, 2.0, 3.0));
        assert!(pnt_eq(s.location(), Pnt::new(1.0, 2.0, 3.0)));
    }

    #[test]
    fn test_translate_moves_center() {
        let mut s = GpSphere::default();
        s.translate(&Vec3::new(1.0, 2.0, 3.0));
        assert!(pnt_eq(s.location(), Pnt::new(1.0, 2.0, 3.0)));
        assert!(approx_eq(s.radius(), 1.0));
    }

    #[test]
    fn test_translated_does_not_mutate_original() {
        let s = GpSphere::default();
        let s2 = s.translated(&Vec3::new(5.0, 0.0, 0.0));
        assert!(pnt_eq(s.location(), Pnt::new(0.0, 0.0, 0.0)));
        assert!(pnt_eq(s2.location(), Pnt::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn test_translated_points() {
        let s = GpSphere::default();
        let p1 = Pnt::new(0.0, 0.0, 0.0);
        let p2 = Pnt::new(3.0, 4.0, 5.0);
        let s2 = s.translated_points(&p1, &p2);
        assert!(pnt_eq(s2.location(), p2));
    }

    #[test]
    fn test_scale_about_origin_moves_offcenter_sphere() {
        let ax3 = Ax3::from_origin_normal(
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let s = GpSphere::new(ax3, 1.0);
        let origin = Pnt::new(0.0, 0.0, 0.0);
        let s2 = s.scaled(&origin, 3.0);
        assert!(pnt_eq(s2.location(), Pnt::new(6.0, 0.0, 0.0)));
        assert!(approx_eq(s2.radius(), 3.0));
    }

    #[test]
    fn test_scale_radius_absolute_value_with_negative_factor() {
        let s = GpSphere::default();
        let origin = Pnt::new(0.0, 0.0, 0.0);
        let s2 = s.scaled(&origin, -2.0);
        assert!(approx_eq(s2.radius(), 2.0));
    }

    #[test]
    fn test_rotate_about_z_axis() {
        let ax3 = Ax3::from_origin_normal(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let s = GpSphere::new(ax3, 1.0);
        let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
        let s2 = s.rotated(&z_ax1, FRAC_PI_2);
        // Center (1,0,0) rotated PI/2 about Z → (0,1,0)
        assert!(pnt_eq(s2.location(), Pnt::new(0.0, 1.0, 0.0)));
        assert!(approx_eq(s2.radius(), 1.0));
    }

    #[test]
    fn test_rotate_full_circle_identity() {
        let s = GpSphere::new(
            Ax3::from_origin_normal(Pnt::new(3.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0)),
            2.0,
        );
        let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
        let s2 = s.rotated(&z_ax1, 2.0 * PI);
        assert!(pnt_eq(s2.location(), s.location()));
    }

    #[test]
    fn test_mirror_through_point() {
        let s = GpSphere::default(); // center (0,0,0)
        let pt = Pnt::new(1.0, 0.0, 0.0);
        let s2 = s.mirrored_through_point(&pt);
        // 2*(1,0,0) - (0,0,0) = (2,0,0)
        assert!(pnt_eq(s2.location(), Pnt::new(2.0, 0.0, 0.0)));
        assert!(approx_eq(s2.radius(), 1.0));
    }

    #[test]
    fn test_mirror_through_point_twice_identity() {
        let s = GpSphere::new(
            Ax3::from_origin_normal(Pnt::new(1.0, 2.0, 3.0), Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0)),
            4.0,
        );
        let pt = Pnt::new(5.0, 5.0, 5.0);
        let s2 = s.mirrored_through_point(&pt).mirrored_through_point(&pt);
        assert!(pnt_eq(s2.location(), s.location()));
        assert!(approx_eq(s2.radius(), s.radius()));
    }

    #[test]
    fn test_mirror_through_ax1() {
        let ax3 = Ax3::from_origin_normal(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let s = GpSphere::new(ax3, 2.0);
        let y_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
        let s2 = s.mirrored_through_ax1(&y_ax1);
        // (1,0,0) mirrored through Y-axis → (-1,0,0)
        assert!(pnt_eq(s2.location(), Pnt::new(-1.0, 0.0, 0.0)));
        assert!(approx_eq(s2.radius(), 2.0));
    }

    #[test]
    fn test_mirror_through_ax2_plane() {
        let ax3 = Ax3::from_origin_normal(
            Pnt::new(0.0, 0.0, 3.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let s = GpSphere::new(ax3, 1.0);
        // Mirror through XY-plane (z=0)
        let xy_plane = Ax3::from_origin_normal(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let s2 = s.mirrored_through_ax2(&xy_plane);
        // (0,0,3) reflected through z=0 → (0,0,-3)
        assert!(pnt_eq(s2.location(), Pnt::new(0.0, 0.0, -3.0)));
        assert!(approx_eq(s2.radius(), 1.0));
    }

    #[test]
    fn test_axis_direction_is_z_dir() {
        let s = GpSphere::default();
        let ax = s.axis();
        assert!(approx_eq(ax.direction.x, 0.0));
        assert!(approx_eq(ax.direction.y, 0.0));
        assert!(approx_eq(ax.direction.z, 1.0));
    }

    #[test]
    fn test_x_axis_direction() {
        let s = GpSphere::default();
        let xax = s.x_axis();
        assert!(approx_eq(xax.direction.x, 1.0));
        assert!(approx_eq(xax.direction.y, 0.0));
        assert!(approx_eq(xax.direction.z, 0.0));
    }

    #[test]
    fn test_y_axis_direction() {
        let s = GpSphere::default();
        let yax = s.y_axis();
        assert!(approx_eq(yax.direction.x, 0.0));
        assert!(approx_eq(yax.direction.y, 1.0));
        assert!(approx_eq(yax.direction.z, 0.0));
    }

    #[test]
    #[should_panic]
    fn test_zero_radius_panics() {
        let _ = GpSphere::new(Ax3::identity(), 0.0);
    }

    #[test]
    #[should_panic]
    fn test_negative_radius_panics() {
        let _ = GpSphere::new(Ax3::identity(), -1.0);
    }

    #[test]
    fn test_circle_at_u_is_great_circle() {
        // A meridian circle must have radius == sphere radius.
        let s = GpSphere::new(Ax3::identity(), 5.0);
        for &u in &[0.0, 0.7, PI, 1.5 * PI] {
            let c = s.circle_at_u(u);
            assert!(approx_eq(c.radius(), 5.0), "u={u} r={}", c.radius());
        }
    }
}
