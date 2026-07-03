// FILE: gp_tor.rs
//! Port of `gp_Torus` — an analytic torus in 3D space, mirroring
//! OpenCascade's `gp_Torus` from package `TKMath/gp`.
//!
//! A torus is fully defined by:
//! - A right-handed local coordinate system ([`Ax3`]) whose origin is the
//!   center of the torus and whose Z axis is the revolution axis.
//! - A **major radius** `R > 0`: distance from the torus center to the center
//!   of the tube.
//! - A **minor radius** `r > 0`: radius of the tube (`r <= R` for a ring torus,
//!   `r > R` for a horn/spindle torus).
//!
//! The surface is parameterized as:
//! ```text
//! P(U, V) = O + (R + r·cos(V))·(cos(U)·XDir + sin(U)·YDir) + r·sin(V)·ZDir
//! ```
//! where:
//! - U ∈ [0, 2π) is the angular parameter around the main axis,
//! - V ∈ [0, 2π) is the angular parameter around the tube.
//!
//! Key formulas:
//! - Surface area: `4·π²·R·r`
//! - Volume:       `2·π²·R·r²`

use crate::gp::{Ax1, Ax2, Ax3, Pnt, Trsf, Vec3};
use crate::gp_prim::Circ;
use std::f64::consts::PI;

/// A torus in 3D space defined by a placement and two radii.
// occt: gp_Torus
#[derive(Clone, Debug)]
pub struct GpTorus {
    /// The local coordinate system. Origin = torus center, Z = revolution axis.
    pos: Ax3,
    /// Major radius: distance from torus center to tube center (must be > 0).
    major_radius: f64,
    /// Minor radius: radius of the tube (must be > 0).
    minor_radius: f64,
}

impl GpTorus {
    // ─────────────────────────────── constructors ────────────────────────────

    /// Construct a torus from a placement, a major radius and a minor radius.
    ///
    /// # Panics
    /// Panics if either radius is `<= 0`.
    // occt: gp_Torus::gp_Torus(const gp_Ax3&, Standard_Real, Standard_Real)
    pub fn new(pos: Ax3, major_radius: f64, minor_radius: f64) -> Self {
        assert!(
            major_radius > 0.0,
            "gp_Torus: major_radius must be > 0, got {major_radius}"
        );
        assert!(
            minor_radius > 0.0,
            "gp_Torus: minor_radius must be > 0, got {minor_radius}"
        );
        Self {
            pos,
            major_radius,
            minor_radius,
        }
    }

    // ───────────────────────────────── accessors ─────────────────────────────

    /// The local coordinate system (placement) of the torus.
    // occt: gp_Torus::Position
    pub fn position(&self) -> &Ax3 {
        &self.pos
    }

    /// The center of the torus (origin of the local frame).
    // occt: gp_Torus::Location
    pub fn location(&self) -> Pnt {
        self.pos.location
    }

    /// The main (revolution) axis of the torus as an `Ax1` (Z axis of the frame).
    // occt: gp_Torus::Axis
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.z_dir)
    }

    /// X axis of the local frame as an `Ax1`.
    // occt: gp_Torus::XAxis
    pub fn x_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.x_dir)
    }

    /// Y axis of the local frame as an `Ax1`.
    // occt: gp_Torus::YAxis
    pub fn y_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.y_dir)
    }

    /// Major radius: distance from torus center to tube center.
    // occt: gp_Torus::MajorRadius
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// Minor radius: radius of the tube.
    // occt: gp_Torus::MinorRadius
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    // ──────────────────────────────── setters ────────────────────────────────

    /// Set the major radius (must be > 0).
    // occt: gp_Torus::SetMajorRadius
    pub fn set_major_radius(&mut self, r: f64) {
        assert!(
            r > 0.0,
            "gp_Torus::SetMajorRadius: radius must be > 0, got {r}"
        );
        self.major_radius = r;
    }

    /// Set the minor radius (must be > 0).
    // occt: gp_Torus::SetMinorRadius
    pub fn set_minor_radius(&mut self, r: f64) {
        assert!(
            r > 0.0,
            "gp_Torus::SetMinorRadius: radius must be > 0, got {r}"
        );
        self.minor_radius = r;
    }

    /// Replace the local coordinate system.
    // occt: gp_Torus::SetPosition
    pub fn set_position(&mut self, pos: Ax3) {
        self.pos = pos;
    }

    /// Move the torus center to `p` (keeps axes and radii unchanged).
    // occt: gp_Torus::SetLocation
    pub fn set_location(&mut self, p: Pnt) {
        self.pos.location = p;
    }

    // ─────────────────────────── geometric properties ────────────────────────

    /// Surface area of the torus: `4·π²·R·r`.
    // occt: gp_Torus::Area
    pub fn area(&self) -> f64 {
        4.0 * PI * PI * self.major_radius * self.minor_radius
    }

    /// Volume enclosed by the torus: `2·π²·R·r²`.
    // occt: gp_Torus::Volume
    pub fn volume(&self) -> f64 {
        2.0 * PI * PI * self.major_radius * self.minor_radius * self.minor_radius
    }

    /// Returns `true` when the local frame is direct (right-handed).
    // occt: gp_Torus::Direct
    pub fn is_direct(&self) -> bool {
        self.pos.x_dir.cross(self.pos.y_dir).dot(self.pos.z_dir) > 0.0
    }

    // ───────────────────────────── iso-circles ───────────────────────────────

    /// The "outer equatorial" circle: the circle of maximum radius in the XY
    /// plane of the local frame (V = 0, outer edge).
    ///
    /// Radius = `R + r`.
    // occt: gp_Torus::VIso-related
    pub fn outer_circle(&self) -> Circ {
        Circ::new(self.pos, self.major_radius + self.minor_radius)
    }

    /// The "inner equatorial" circle: the circle of minimum radius in the XY
    /// plane of the local frame.
    ///
    /// Radius = `R - r` (may be 0 for a horn torus).
    // occt: gp_Torus::VIso-related
    pub fn inner_circle(&self) -> Circ {
        let r = (self.major_radius - self.minor_radius).max(0.0);
        Circ::new(self.pos, r)
    }

    /// The meridian circle at longitude `u` (angle around the revolution axis).
    ///
    /// This is the cross-section circle of radius `minor_radius` whose center
    /// lies at distance `major_radius` from the torus center along the radial
    /// direction `cos(u)·XDir + sin(u)·YDir`.
    ///
    /// The circle plane contains ZDir and the radial direction.
    // occt: gp_Torus::UIso
    pub fn u_iso(&self, u: f64) -> Circ {
        let (su, cu) = u.sin_cos();
        // Radial direction in the equatorial plane at angle u.
        let radial = self.pos.x_dir * cu + self.pos.y_dir * su;
        // Center of the tube circle.
        let center = self.pos.location + radial * self.major_radius;
        // The meridian circle lies in the plane spanned by (radial, z_dir).
        // Its normal is perpendicular to that plane = radial × z_dir (normalized).
        let normal = radial.cross(self.pos.z_dir).normalized();
        // x_dir of the circle frame is the radial direction itself.
        let frame = Ax3 {
            location: center,
            x_dir: radial,
            y_dir: self.pos.z_dir,
            z_dir: normal,
        };
        Circ::new(frame, self.minor_radius)
    }

    /// The latitude circle at tube angle `v` (angle around the tube cross-section).
    ///
    /// At angle `v` around the tube, the circle radius is `R + r·cos(v)` and it
    /// lies at height `r·sin(v)` along ZDir.
    ///
    /// Radius = `|R + r·cos(v)|`.
    // occt: gp_Torus::VIso
    pub fn v_iso(&self, v: f64) -> Circ {
        let (sv, cv) = v.sin_cos();
        let lat_radius = self.major_radius + self.minor_radius * cv;
        let height = self.minor_radius * sv;
        let center = self.pos.location + self.pos.z_dir * height;
        let frame = Ax3 {
            location: center,
            x_dir: self.pos.x_dir,
            y_dir: self.pos.y_dir,
            z_dir: self.pos.z_dir,
        };
        Circ::new(frame, lat_radius.abs())
    }

    // ──────────────────────── parametric evaluation ──────────────────────────

    /// Evaluate a point on the torus surface at parameters `(u, v)`.
    ///
    /// ```text
    /// P(U, V) = O + (R + r·cos(V))·(cos(U)·XDir + sin(U)·YDir) + r·sin(V)·ZDir
    /// ```
    // occt: implicit via Geom_ToroidalSurface::Value
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        let (su, cu) = u.sin_cos();
        let (sv, cv) = v.sin_cos();
        let rho = self.major_radius + self.minor_radius * cv;
        self.pos.location
            + self.pos.x_dir * (rho * cu)
            + self.pos.y_dir * (rho * su)
            + self.pos.z_dir * (self.minor_radius * sv)
    }

    /// Returns `true` when `p` lies on the torus surface within tolerance `tol`.
    ///
    /// A point is on the torus when the distance from `p` to the nearest point
    /// of the tube center circle equals `minor_radius`.
    // occt: gp_Torus::Contains (implicit)
    pub fn contains(&self, p: Pnt, tol: f64) -> bool {
        // Project p onto the equatorial plane by stripping the Z component.
        let rel = p - self.pos.location;
        let z_comp = rel.dot(self.pos.z_dir);
        let in_plane = rel - self.pos.z_dir * z_comp;
        let in_plane_dist = in_plane.norm();
        // Distance from p to the nearest point on the tube center circle.
        let dist = ((in_plane_dist - self.major_radius).powi(2) + z_comp.powi(2)).sqrt();
        (dist - self.minor_radius).abs() <= tol
    }

    // ───────────────────────────── transforms ────────────────────────────────

    /// Mirror the torus through a point `pt`.
    // occt: gp_Torus::Mirrored (gp_Pnt overload)
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
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// Mirror the torus in place through point `pt`.
    // occt: gp_Torus::Mirror (gp_Pnt overload)
    pub fn mirror_through_point(&mut self, pt: &Pnt) {
        *self = self.mirrored_through_point(pt);
    }

    /// Mirror the torus through a line (`Ax1`).
    // occt: gp_Torus::Mirrored (gp_Ax1 overload)
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
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// Mirror the torus in place through a line (`Ax1`).
    // occt: gp_Torus::Mirror (gp_Ax1 overload)
    pub fn mirror_through_ax1(&mut self, ax: &Ax1) {
        *self = self.mirrored_through_ax1(ax);
    }

    /// Mirror the torus through a plane (`Ax2` — its XY plane is the mirror plane).
    // occt: gp_Torus::Mirrored (gp_Ax2 overload)
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
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// Mirror the torus in place through a plane (`Ax2`).
    // occt: gp_Torus::Mirror (gp_Ax2 overload)
    pub fn mirror_through_ax2(&mut self, ax: &Ax2) {
        *self = self.mirrored_through_ax2(ax);
    }

    /// Return a new torus rotated about `ax` by `angle` radians.
    // occt: gp_Torus::Rotated
    pub fn rotated(&self, ax: &Ax1, angle: f64) -> Self {
        let t = Trsf::rotation(*ax, angle);
        Self {
            pos: self.pos.transformed(&t),
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// Rotate the torus in place.
    // occt: gp_Torus::Rotate
    pub fn rotate(&mut self, ax: &Ax1, angle: f64) {
        *self = self.rotated(ax, angle);
    }

    /// Return a new torus scaled about point `pt` by factor `s`.
    ///
    /// Both radii scale by `|s|`. If `s < 0` the frame directions are negated.
    // occt: gp_Torus::Scaled
    pub fn scaled(&self, pt: &Pnt, s: f64) -> Self {
        let loc = self.pos.location;
        let new_loc = Pnt::new(
            pt.x + s * (loc.x - pt.x),
            pt.y + s * (loc.y - pt.y),
            pt.z + s * (loc.z - pt.z),
        );
        let sign = if s < 0.0 { -1.0_f64 } else { 1.0_f64 };
        let abs_s = s.abs();
        Self {
            pos: Ax3 {
                location: new_loc,
                x_dir: Pnt::new(
                    sign * self.pos.x_dir.x,
                    sign * self.pos.x_dir.y,
                    sign * self.pos.x_dir.z,
                ),
                y_dir: Pnt::new(
                    sign * self.pos.y_dir.x,
                    sign * self.pos.y_dir.y,
                    sign * self.pos.y_dir.z,
                ),
                z_dir: Pnt::new(
                    sign * self.pos.z_dir.x,
                    sign * self.pos.z_dir.y,
                    sign * self.pos.z_dir.z,
                ),
            },
            major_radius: self.major_radius * abs_s,
            minor_radius: self.minor_radius * abs_s,
        }
    }

    /// Scale the torus in place.
    // occt: gp_Torus::Scale
    pub fn scale(&mut self, pt: &Pnt, s: f64) {
        *self = self.scaled(pt, s);
    }

    /// Return a new torus translated by vector `v`.
    // occt: gp_Torus::Translated (gp_Vec overload)
    pub fn translated(&self, v: &Vec3) -> Self {
        Self {
            pos: Ax3 {
                location: Pnt::new(
                    self.pos.location.x + v.x,
                    self.pos.location.y + v.y,
                    self.pos.location.z + v.z,
                ),
                x_dir: self.pos.x_dir,
                y_dir: self.pos.y_dir,
                z_dir: self.pos.z_dir,
            },
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// Translate the torus in place by vector `v`.
    // occt: gp_Torus::Translate (gp_Vec overload)
    pub fn translate(&mut self, v: &Vec3) {
        self.pos.location = Pnt::new(
            self.pos.location.x + v.x,
            self.pos.location.y + v.y,
            self.pos.location.z + v.z,
        );
    }

    /// Return a new torus translated from point `p1` to point `p2`.
    // occt: gp_Torus::Translated (gp_Pnt, gp_Pnt overload)
    pub fn translated_from_to(&self, p1: &Pnt, p2: &Pnt) -> Self {
        let v = Vec3::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        self.translated(&v)
    }

    /// Translate the torus in place from point `p1` to point `p2`.
    // occt: gp_Torus::Translate (gp_Pnt, gp_Pnt overload)
    pub fn translate_from_to(&mut self, p1: &Pnt, p2: &Pnt) {
        let v = Vec3::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        self.translate(&v);
    }

    /// Apply an arbitrary transformation.
    // occt: gp_Torus::Transformed
    pub fn transformed(&self, t: &Trsf) -> Self {
        let abs_s = t.scale_factor().abs();
        Self {
            pos: self.pos.transformed(t),
            major_radius: self.major_radius * abs_s,
            minor_radius: self.minor_radius * abs_s,
        }
    }
}

// ─────────────────────────────── Default ─────────────────────────────────────

impl Default for GpTorus {
    /// Default torus: center at origin, axis along +Z, major radius 2, minor radius 1.
    fn default() -> Self {
        Self::new(Ax3::identity(), 2.0, 1.0)
    }
}

// ─────────────────────────────── Tests ───────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Pnt;
    use std::f64::consts::{FRAC_PI_2, PI};

    const EPS: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    fn pnt_approx_eq(a: &Pnt, b: &Pnt, eps: f64) -> bool {
        approx_eq(a.x, b.x, eps) && approx_eq(a.y, b.y, eps) && approx_eq(a.z, b.z, eps)
    }

    // ── construction ─────────────────────────────────────────────────────────

    #[test]
    fn test_default_radii() {
        let t = GpTorus::default();
        assert!(approx_eq(t.major_radius(), 2.0, EPS));
        assert!(approx_eq(t.minor_radius(), 1.0, EPS));
    }

    #[test]
    fn test_default_center_at_origin() {
        let t = GpTorus::default();
        assert!(pnt_approx_eq(&t.location(), &Pnt::new(0.0, 0.0, 0.0), EPS));
    }

    #[test]
    fn test_default_is_direct() {
        let t = GpTorus::default();
        assert!(t.is_direct());
    }

    #[test]
    #[should_panic]
    fn test_zero_major_radius_panics() {
        let _ = GpTorus::new(Ax3::identity(), 0.0, 1.0);
    }

    #[test]
    #[should_panic]
    fn test_zero_minor_radius_panics() {
        let _ = GpTorus::new(Ax3::identity(), 2.0, 0.0);
    }

    // ── area and volume ───────────────────────────────────────────────────────

    #[test]
    fn test_area_formula() {
        let t = GpTorus::default(); // R=2, r=1
        let expected = 4.0 * PI * PI * 2.0 * 1.0;
        assert!(approx_eq(t.area(), expected, EPS));
    }

    #[test]
    fn test_volume_formula() {
        let t = GpTorus::default(); // R=2, r=1
        let expected = 2.0 * PI * PI * 2.0 * 1.0 * 1.0;
        assert!(approx_eq(t.volume(), expected, EPS));
    }

    #[test]
    fn test_scaled_radii_scale_area_correctly() {
        let t = GpTorus::default(); // R=2, r=1
        let t2 = t.scaled(&Pnt::new(0.0, 0.0, 0.0), 3.0);
        // scaled: R=6, r=3 → area = 4π²·6·3 = 9 * original area
        let expected_area = 4.0 * PI * PI * 6.0 * 3.0;
        assert!(approx_eq(t2.area(), expected_area, EPS));
    }

    // ── iso-circles ───────────────────────────────────────────────────────────

    #[test]
    fn test_outer_circle_radius() {
        let t = GpTorus::default(); // R=2, r=1 → outer = 3
        assert!(approx_eq(t.outer_circle().radius(), 3.0, EPS));
    }

    #[test]
    fn test_inner_circle_radius() {
        let t = GpTorus::default(); // R=2, r=1 → inner = 1
        assert!(approx_eq(t.inner_circle().radius(), 1.0, EPS));
    }

    #[test]
    fn test_v_iso_at_zero_gives_outer_radius() {
        let t = GpTorus::default();
        // V=0 → cos(0)=1 → radius = R + r = 3
        let circ = t.v_iso(0.0);
        assert!(approx_eq(circ.radius(), 3.0, EPS));
    }

    #[test]
    fn test_v_iso_at_pi_gives_inner_radius() {
        let t = GpTorus::default();
        // V=π → cos(π)=-1 → radius = R - r = 1
        let circ = t.v_iso(PI);
        assert!(approx_eq(circ.radius(), 1.0, EPS));
    }

    #[test]
    fn test_v_iso_height_at_pi_over_2() {
        let t = GpTorus::default(); // r=1
        // V=π/2 → sin(π/2)=1 → height = r = 1
        let circ = t.v_iso(FRAC_PI_2);
        // circle center should be at (0,0,1)
        assert!(approx_eq(circ.location().z, 1.0, EPS));
    }

    #[test]
    fn test_u_iso_center_at_major_radius() {
        let t = GpTorus::default(); // R=2
        // U=0 → radial = (1,0,0) → tube center at (2,0,0)
        let circ = t.u_iso(0.0);
        assert!(approx_eq(circ.location().x, 2.0, EPS));
        assert!(approx_eq(circ.location().y, 0.0, EPS));
        assert!(approx_eq(circ.location().z, 0.0, EPS));
    }

    #[test]
    fn test_u_iso_radius_equals_minor() {
        let t = GpTorus::default(); // r=1
        let circ = t.u_iso(0.0);
        assert!(approx_eq(circ.radius(), 1.0, EPS));
    }

    // ── parametric value ─────────────────────────────────────────────────────

    #[test]
    fn test_value_at_u0_v0_is_outer_point() {
        let t = GpTorus::default(); // R=2, r=1
        // U=0, V=0 → P = (R+r, 0, 0) = (3, 0, 0)
        let p = t.value(0.0, 0.0);
        assert!(pnt_approx_eq(&p, &Pnt::new(3.0, 0.0, 0.0), EPS));
    }

    #[test]
    fn test_value_at_u0_v_pi_is_inner_point() {
        let t = GpTorus::default(); // R=2, r=1
        // U=0, V=π → P = (R-r, 0, 0) = (1, 0, 0)
        let p = t.value(0.0, PI);
        assert!(pnt_approx_eq(&p, &Pnt::new(1.0, 0.0, 0.0), EPS));
    }

    #[test]
    fn test_value_at_u0_v_pi_over_2_is_top_of_tube() {
        let t = GpTorus::default(); // R=2, r=1
        // U=0, V=π/2 → P = (R, 0, r) = (2, 0, 1)
        let p = t.value(0.0, FRAC_PI_2);
        assert!(pnt_approx_eq(&p, &Pnt::new(2.0, 0.0, 1.0), EPS));
    }

    #[test]
    fn test_contains_outer_point() {
        let t = GpTorus::default();
        // Outer equator point (3,0,0) is on the surface
        assert!(t.contains(Pnt::new(3.0, 0.0, 0.0), 1e-9));
    }

    #[test]
    fn test_contains_inner_point() {
        let t = GpTorus::default();
        // Inner equator point (1,0,0) is on the surface
        assert!(t.contains(Pnt::new(1.0, 0.0, 0.0), 1e-9));
    }

    #[test]
    fn test_contains_top_of_tube() {
        let t = GpTorus::default();
        // Top of tube at U=0: (2,0,1)
        assert!(t.contains(Pnt::new(2.0, 0.0, 1.0), 1e-9));
    }

    #[test]
    fn test_origin_not_on_torus_surface() {
        let t = GpTorus::default();
        // The origin is inside the torus, not on the surface
        assert!(!t.contains(Pnt::new(0.0, 0.0, 0.0), 1e-9));
    }

    // ── transforms ────────────────────────────────────────────────────────────

    #[test]
    fn test_translate_moves_center() {
        let mut t = GpTorus::default();
        t.translate(&Vec3::new(1.0, 2.0, 3.0));
        assert!(pnt_approx_eq(&t.location(), &Pnt::new(1.0, 2.0, 3.0), EPS));
    }

    #[test]
    fn test_translated_immutable() {
        let t = GpTorus::default();
        let t2 = t.translated(&Vec3::new(5.0, 0.0, 0.0));
        assert!(pnt_approx_eq(&t.location(), &Pnt::new(0.0, 0.0, 0.0), EPS));
        assert!(pnt_approx_eq(&t2.location(), &Pnt::new(5.0, 0.0, 0.0), EPS));
    }

    #[test]
    fn test_translate_preserves_radii() {
        let mut t = GpTorus::default();
        t.translate(&Vec3::new(100.0, 200.0, 300.0));
        assert!(approx_eq(t.major_radius(), 2.0, EPS));
        assert!(approx_eq(t.minor_radius(), 1.0, EPS));
    }

    #[test]
    fn test_translate_from_to() {
        let mut t = GpTorus::default();
        let p1 = Pnt::new(0.0, 0.0, 0.0);
        let p2 = Pnt::new(3.0, 4.0, 5.0);
        t.translate_from_to(&p1, &p2);
        assert!(pnt_approx_eq(&t.location(), &p2, EPS));
    }

    #[test]
    fn test_scale_about_origin_scales_radii_and_position() {
        let ax3 = Ax3::from_origin_normal(
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let t = GpTorus::new(ax3, 2.0, 1.0);
        let origin = Pnt::new(0.0, 0.0, 0.0);
        let t2 = t.scaled(&origin, 2.0);
        assert!(pnt_approx_eq(&t2.location(), &Pnt::new(4.0, 0.0, 0.0), EPS));
        assert!(approx_eq(t2.major_radius(), 4.0, EPS));
        assert!(approx_eq(t2.minor_radius(), 2.0, EPS));
    }

    #[test]
    fn test_scale_about_center_keeps_location() {
        let t = GpTorus::default();
        let center = Pnt::new(0.0, 0.0, 0.0);
        let t2 = t.scaled(&center, 3.0);
        assert!(pnt_approx_eq(&t2.location(), &Pnt::new(0.0, 0.0, 0.0), EPS));
        assert!(approx_eq(t2.major_radius(), 6.0, EPS));
        assert!(approx_eq(t2.minor_radius(), 3.0, EPS));
    }

    #[test]
    fn test_rotate_moves_center() {
        let ax3 = Ax3::from_origin_normal(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let t = GpTorus::new(ax3, 2.0, 1.0);
        let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
        let t2 = t.rotated(&z_ax1, FRAC_PI_2);
        // center (1,0,0) rotated PI/2 about Z → (0,1,0)
        assert!(pnt_approx_eq(&t2.location(), &Pnt::new(0.0, 1.0, 0.0), 1e-9));
        assert!(approx_eq(t2.major_radius(), 2.0, EPS));
        assert!(approx_eq(t2.minor_radius(), 1.0, EPS));
    }

    #[test]
    fn test_rotate_full_circle_returns_to_start() {
        let ax3 = Ax3::from_origin_normal(
            Pnt::new(3.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let t = GpTorus::new(ax3, 2.0, 0.5);
        let z_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
        let t2 = t.rotated(&z_ax1, 2.0 * PI);
        assert!(pnt_approx_eq(&t2.location(), &t.location(), 1e-9));
    }

    #[test]
    fn test_mirror_through_point() {
        let t = GpTorus::default(); // center at (0,0,0)
        let pt = Pnt::new(2.0, 0.0, 0.0);
        let t2 = t.mirrored_through_point(&pt);
        // 2*(2,0,0) - (0,0,0) = (4,0,0)
        assert!(pnt_approx_eq(&t2.location(), &Pnt::new(4.0, 0.0, 0.0), EPS));
        assert!(approx_eq(t2.major_radius(), 2.0, EPS));
        assert!(approx_eq(t2.minor_radius(), 1.0, EPS));
    }

    #[test]
    fn test_mirror_through_point_twice_returns_original() {
        let t = GpTorus::default();
        let pt = Pnt::new(3.0, 4.0, 5.0);
        let t2 = t
            .mirrored_through_point(&pt)
            .mirrored_through_point(&pt);
        assert!(pnt_approx_eq(&t2.location(), &t.location(), 1e-9));
    }

    #[test]
    fn test_mirror_through_ax1() {
        let ax3 = Ax3::from_origin_normal(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let t = GpTorus::new(ax3, 2.0, 1.0);
        // Mirror through Y-axis → center (1,0,0) → (-1,0,0)
        let y_ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
        let t2 = t.mirrored_through_ax1(&y_ax1);
        assert!(pnt_approx_eq(&t2.location(), &Pnt::new(-1.0, 0.0, 0.0), EPS));
        assert!(approx_eq(t2.major_radius(), 2.0, EPS));
    }

    #[test]
    fn test_mirror_through_ax2_plane() {
        let ax3 = Ax3::from_origin_normal(
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let t = GpTorus::new(ax3, 2.0, 1.0);
        // Mirror through XY-plane (normal Z at origin) → center (0,0,1) → (0,0,-1)
        let xy_plane = Ax3::from_origin_normal(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let t2 = t.mirrored_through_ax2(&xy_plane);
        assert!(pnt_approx_eq(&t2.location(), &Pnt::new(0.0, 0.0, -1.0), EPS));
        assert!(approx_eq(t2.minor_radius(), 1.0, EPS));
    }

    #[test]
    fn test_set_major_radius() {
        let mut t = GpTorus::default();
        t.set_major_radius(5.0);
        assert!(approx_eq(t.major_radius(), 5.0, EPS));
    }

    #[test]
    fn test_set_minor_radius() {
        let mut t = GpTorus::default();
        t.set_minor_radius(0.5);
        assert!(approx_eq(t.minor_radius(), 0.5, EPS));
    }

    #[test]
    fn test_set_location() {
        let mut t = GpTorus::default();
        let new_loc = Pnt::new(10.0, 20.0, 30.0);
        t.set_location(new_loc);
        assert!(pnt_approx_eq(&t.location(), &new_loc, EPS));
    }

    #[test]
    fn test_axis_direction_is_z() {
        let t = GpTorus::default();
        let ax1 = t.axis();
        // Default frame: Z = (0,0,1)
        assert!(approx_eq(ax1.direction.z, 1.0, EPS));
    }

    #[test]
    fn test_inner_circle_equal_radii_horn_torus() {
        // When r = R, inner radius = 0 (horn torus)
        let t = GpTorus::new(Ax3::identity(), 1.0, 1.0);
        assert!(approx_eq(t.inner_circle().radius(), 0.0, EPS));
    }

    #[test]
    fn test_value_parametric_on_surface() {
        let t = GpTorus::default();
        // For various (U,V) the point should satisfy the contains test.
        for &(u, v) in &[(0.0, 0.0), (1.0, 1.0), (2.5, 3.1), (PI, FRAC_PI_2)] {
            let p = t.value(u, v);
            assert!(t.contains(p, 1e-9), "({u}, {v}) not on surface");
        }
    }

    #[test]
    fn test_position_returns_ax3() {
        let t = GpTorus::default();
        let pos = t.position();
        // identity frame has x_dir = (1,0,0)
        assert!(approx_eq(pos.x_dir.x, 1.0, EPS));
    }
}
