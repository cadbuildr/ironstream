//! `gp_Elips` / `gp_Hypr` -- analytic 3D ellipse and hyperbola primitives,
//! mirroring OpenCascade's `gp_Elips` and `gp_Hypr` (package `TKMath`).
//!
//! An ellipse is placed in 3D space by a local coordinate system (`gp_Ax2`,
//! aliased here as [`crate::gp::Ax3`]): the origin is the ellipse centre, the
//! "X Direction" is the major axis and the "Y Direction" is the minor axis.
//! The normal to the ellipse plane is the "Z Direction".
//!
//! # Invariants (mirroring OCCT `Standard_ConstructionError`)
//! * `minor_radius >= 0`
//! * `major_radius >= minor_radius`
//!
//! # Geometry
//! ```text
//!   c  = sqrt(a² - b²)          -- focal half-distance
//!   e  = c / a                   -- eccentricity  (0 for circle, <1 for ellipse)
//!   p  = b² / a                  -- semi-latus rectum
//!   Focus1 = Centre + c * XDir
//!   Focus2 = Centre - c * XDir
//! ```

use crate::gp::{Ax1, Ax3, Pnt, Trsf};
use std::f64::consts::PI;

// occt: gp_Elips
/// An analytic 3D ellipse: a placement (`gp_Ax2`) plus major and minor radii.
///
/// The placement's origin is the ellipse centre; `XDir` points along the
/// major axis; `YDir` points along the minor axis; `ZDir` is the plane normal.
#[derive(Clone, Copy, Debug)]
pub struct Elips {
    /// Local coordinate system — OCCT calls this `pos` (a `gp_Ax2`).
    pos: Ax3,
    major_radius: f64,
    minor_radius: f64,
}

impl Elips {
    /// `gp_Elips(const gp_Ax2& A2, Standard_Real MajorRadius,
    ///           Standard_Real MinorRadius)`
    ///
    /// Panics (`Standard_ConstructionError` in OCCT) if
    /// `minor_radius < 0` or `major_radius < minor_radius`.
    pub fn new(ax2: Ax3, major_radius: f64, minor_radius: f64) -> Self {
        assert!(
            minor_radius >= 0.0 && major_radius >= minor_radius,
            "gp_Elips: require MinorRadius >= 0 and MajorRadius >= MinorRadius"
        );
        Self {
            pos: ax2,
            major_radius,
            minor_radius,
        }
    }

    // ------------------------------------------------------------------ radii

    /// `MajorRadius()`.
    #[inline]
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// `MinorRadius()`.
    #[inline]
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    /// `SetMajorRadius(r)` — update the major radius.
    ///
    /// Panics if `major_radius < minor_radius`.
    pub fn set_major_radius(&mut self, major_radius: f64) {
        assert!(
            major_radius >= self.minor_radius,
            "gp_Elips::SetMajorRadius: MajorRadius must be >= MinorRadius"
        );
        self.major_radius = major_radius;
    }

    /// `SetMinorRadius(r)` — update the minor radius.
    ///
    /// Panics if `minor_radius < 0` or `minor_radius > major_radius`.
    pub fn set_minor_radius(&mut self, minor_radius: f64) {
        assert!(
            minor_radius >= 0.0 && self.major_radius >= minor_radius,
            "gp_Elips::SetMinorRadius: require 0 <= MinorRadius <= MajorRadius"
        );
        self.minor_radius = minor_radius;
    }

    // ----------------------------------------------------------------- axes

    /// `Axis()` — the main axis of the ellipse (`gp_Ax1` through the centre
    /// along `ZDir`).
    #[inline]
    pub fn axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.z_dir)
    }

    /// `XAxis()` — the major-axis line (`gp_Ax1` through the centre along
    /// `XDir`).
    #[inline]
    pub fn x_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.x_dir)
    }

    /// `YAxis()` — the minor-axis line (`gp_Ax1` through the centre along
    /// `YDir`).
    #[inline]
    pub fn y_axis(&self) -> Ax1 {
        Ax1::new(self.pos.location, self.pos.y_dir)
    }

    /// `Position()` — the local coordinate system (`gp_Ax2`).
    #[inline]
    pub fn position(&self) -> Ax3 {
        self.pos
    }

    /// `Location()` — the centre of the ellipse.
    #[inline]
    pub fn location(&self) -> Pnt {
        self.pos.location
    }

    // ------------------------------------------------- derived geometry

    /// `Eccentricity()` — `e = sqrt(a² - b²) / a`.
    ///
    /// Returns `0` when `major_radius == 0` (degenerate point).
    #[inline]
    pub fn eccentricity(&self) -> f64 {
        let a = self.major_radius;
        let b = self.minor_radius;
        if a == 0.0 {
            0.0
        } else {
            (a * a - b * b).sqrt() / a
        }
    }

    /// `Focal()` — the distance between the two foci: `2 * sqrt(a² - b²)`.
    #[inline]
    pub fn focal(&self) -> f64 {
        let a = self.major_radius;
        let b = self.minor_radius;
        2.0 * (a * a - b * b).sqrt()
    }

    /// `Focus1()` — the first focus: `Centre + c * XDir`
    /// where `c = sqrt(a² - b²)`.
    pub fn focus1(&self) -> Pnt {
        let c = (self.major_radius * self.major_radius
            - self.minor_radius * self.minor_radius)
            .sqrt();
        self.pos.location + self.pos.x_dir * c
    }

    /// `Focus2()` — the second focus: `Centre - c * XDir`.
    pub fn focus2(&self) -> Pnt {
        let c = (self.major_radius * self.major_radius
            - self.minor_radius * self.minor_radius)
            .sqrt();
        self.pos.location - self.pos.x_dir * c
    }

    /// `Parameter()` — the semi-latus rectum `p = b² / a`.
    ///
    /// Returns `0` when `major_radius == 0`.
    #[inline]
    pub fn parameter(&self) -> f64 {
        let a = self.major_radius;
        let b = self.minor_radius;
        if a == 0.0 {
            0.0
        } else {
            b * b / a
        }
    }

    /// `Area()` — `π * a * b`.
    #[inline]
    pub fn area(&self) -> f64 {
        PI * self.major_radius * self.minor_radius
    }

    // --------------------------------------------------------------- transforms

    /// `Translate(Vec)` — translate by a vector.
    pub fn translated(&self, v: Pnt) -> Elips {
        Elips {
            pos: self.pos.transformed(&Trsf::translation(v)),
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// `Rotate(Ax1, angle)` — rotate about an arbitrary axis.
    pub fn rotated(&self, axis: Ax1, angle: f64) -> Elips {
        Elips {
            pos: self.pos.transformed(&Trsf::rotation(axis, angle)),
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// `Mirror(Pnt)` — point symmetry through `center`.
    pub fn mirrored_point(&self, center: Pnt) -> Elips {
        let mut pos = self.pos;
        pos.location = pos.location.mirrored_point(center);
        pos.x_dir = -pos.x_dir;
        pos.y_dir = -pos.y_dir;
        Elips {
            pos,
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// `Mirror(Ax1)` — reflection across the line `axis`.
    pub fn mirrored_axis1(&self, axis: Ax1) -> Elips {
        let a = axis.direction;
        let new_loc = self.pos.location.mirrored_axis(axis);
        let mirror_dir = |d: Pnt| a * (2.0 * d.dot(a)) - d;
        let x = mirror_dir(self.pos.x_dir).normalized();
        let y = mirror_dir(self.pos.y_dir).normalized();
        let z = x.cross(y).normalized();
        Elips {
            pos: Ax3 {
                location: new_loc,
                x_dir: x,
                y_dir: y,
                z_dir: z,
            },
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// `Mirror(Ax2)` — reflection across the plane defined by `mirror`
    /// (the XY plane of the given `Ax3`).
    pub fn mirrored_plane(&self, mirror: Ax3) -> Elips {
        let n = mirror.z_dir;
        let mirror_pt = |p: Pnt| {
            let dist = (p - mirror.location).dot(n);
            p - n * (2.0 * dist)
        };
        let mirror_dir = |d: Pnt| (d - n * (2.0 * d.dot(n))).normalized();

        let new_loc = mirror_pt(self.pos.location);
        let x = mirror_dir(self.pos.x_dir);
        let y = mirror_dir(self.pos.y_dir);
        let z = x.cross(y).normalized();
        Elips {
            pos: Ax3 {
                location: new_loc,
                x_dir: x,
                y_dir: y,
                z_dir: z,
            },
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// `Scale(center, factor)` — uniform scale about `center`.
    pub fn scaled(&self, center: Pnt, factor: f64) -> Elips {
        let mut t = Trsf::identity();
        t.set_scale(center, factor);
        self.transformed(&t)
    }

    /// `Transform(Trsf)` — apply an affine transform.
    pub fn transformed(&self, t: &Trsf) -> Elips {
        let scale = t.scale_factor();
        Elips {
            pos: self.pos.transformed(t),
            major_radius: self.major_radius * scale,
            minor_radius: self.minor_radius * scale,
        }
    }
}

// ---------------------------------------------------------------------------
// Hypr  (gp_Hypr)
// ---------------------------------------------------------------------------

// Internal helpers
#[inline]
fn vec_add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn vec_sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn vec_scale3(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

#[inline]
fn vec_norm3(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

#[inline]
fn vec_normalize3(v: [f64; 3]) -> [f64; 3] {
    let n = vec_norm3(v);
    if n == 0.0 { v } else { vec_scale3(v, 1.0 / n) }
}

// occt: gp_Hypr
/// An analytic 3-D hyperbola, mirroring `gp_Hypr`.
///
/// Uses raw `[f64; 3]` arrays for its centre and axis.
#[derive(Clone, Copy, Debug)]
pub struct Hypr {
    /// Centre of the hyperbola.
    pub center: [f64; 3],
    /// Semi-major radius `a`.
    pub major_radius: f64,
    /// Semi-minor radius `b`.
    pub minor_radius: f64,
}

impl Hypr {
    /// Construct a hyperbola with centre, semi-major, and semi-minor radii.
    pub fn new(center: [f64; 3], major: f64, minor: f64) -> Self {
        Self { center, major_radius: major, minor_radius: minor }
    }

    /// `Focal()` — distance between the two foci: `2 * sqrt(a² + b²)`.
    #[inline]
    pub fn focal(&self) -> f64 {
        let a = self.major_radius;
        let b = self.minor_radius;
        2.0 * (a * a + b * b).sqrt()
    }

    /// `Focus1()` — `centre + c * [1,0,0]`, `c = sqrt(a²+b²)`.
    pub fn focus1(&self) -> [f64; 3] {
        let a = self.major_radius;
        let b = self.minor_radius;
        let c = (a * a + b * b).sqrt();
        vec_add3(self.center, [c, 0.0, 0.0])
    }

    /// `Focus2()` — `centre - c * [1,0,0]`.
    pub fn focus2(&self) -> [f64; 3] {
        let a = self.major_radius;
        let b = self.minor_radius;
        let c = (a * a + b * b).sqrt();
        vec_sub3(self.center, [c, 0.0, 0.0])
    }

    /// `Eccentricity()` — `sqrt(1 + (b/a)²)`.
    #[inline]
    pub fn eccentricity(&self) -> f64 {
        let a = self.major_radius;
        let b = self.minor_radius;
        if a == 0.0 { 0.0 } else { (1.0 + (b / a) * (b / a)).sqrt() }
    }

    /// Asymptote 1 direction: `normalise(a * [1,0,0] + b * [0,1,0])`.
    pub fn asymptote1_dir(&self) -> [f64; 3] {
        let a = self.major_radius;
        let b = self.minor_radius;
        vec_normalize3([a, b, 0.0])
    }

    /// Asymptote 2 direction: `normalise(a * [1,0,0] - b * [0,1,0])`.
    pub fn asymptote2_dir(&self) -> [f64; 3] {
        let a = self.major_radius;
        let b = self.minor_radius;
        vec_normalize3([a, -b, 0.0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precision::{ANGULAR, CONFUSION};
    use std::f64::consts::FRAC_PI_2;

    fn default_ax2() -> Ax3 {
        Ax3::identity()
    }

    fn unit_ellipse() -> Elips {
        Elips::new(default_ax2(), 5.0, 3.0)
    }

    #[test]
    fn constructor_radii() {
        let e = unit_ellipse();
        assert!((e.major_radius() - 5.0).abs() < CONFUSION);
        assert!((e.minor_radius() - 3.0).abs() < CONFUSION);
    }

    #[test]
    fn eccentricity() {
        let e = unit_ellipse();
        assert!((e.eccentricity() - 0.8).abs() < CONFUSION);
    }

    #[test]
    fn focal_and_foci() {
        let e = unit_ellipse();
        assert!((e.focal() - 8.0).abs() < CONFUSION);
        let f1 = e.focus1();
        let f2 = e.focus2();
        assert!(f1.is_equal(Pnt::new(4.0, 0.0, 0.0), CONFUSION));
        assert!(f2.is_equal(Pnt::new(-4.0, 0.0, 0.0), CONFUSION));
    }

    #[test]
    fn parameter() {
        let e = unit_ellipse();
        assert!((e.parameter() - 1.8).abs() < CONFUSION);
    }

    #[test]
    fn translate() {
        let e = unit_ellipse();
        let t = e.translated(Pnt::new(10.0, 20.0, 30.0));
        assert!(t.location().is_equal(Pnt::new(10.0, 20.0, 30.0), CONFUSION));
        assert!((t.major_radius() - 5.0).abs() < CONFUSION);
    }

    #[test]
    fn rotate() {
        let e = Elips::new(
            Ax3::from_origin_normal(
                Pnt::new(1.0, 0.0, 0.0),
                Pnt::new(0.0, 0.0, 1.0),
                Pnt::new(1.0, 0.0, 0.0),
            ),
            5.0,
            3.0,
        );
        let r = e.rotated(Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0)), FRAC_PI_2);
        assert!(r.location().is_equal(Pnt::new(0.0, 1.0, 0.0), CONFUSION));
        assert!((r.major_radius() - 5.0).abs() < CONFUSION);
    }

    #[test]
    fn scale_radii() {
        let e = unit_ellipse();
        let s = e.scaled(Pnt::origin(), 3.0);
        assert!((s.major_radius() - 15.0).abs() < CONFUSION);
        assert!((s.minor_radius() - 9.0).abs() < CONFUSION);
    }

    #[test]
    fn axis_directions() {
        let e = unit_ellipse();
        let xa = e.x_axis();
        let ya = e.y_axis();
        let a = e.axis();
        assert!(xa.direction.is_equal_dir(Pnt::new(1.0, 0.0, 0.0), ANGULAR));
        assert!(ya.direction.is_equal_dir(Pnt::new(0.0, 1.0, 0.0), ANGULAR));
        assert!(a.direction.is_equal_dir(Pnt::new(0.0, 0.0, 1.0), ANGULAR));
    }

    #[test]
    fn circle_eccentricity_zero() {
        let e = Elips::new(default_ax2(), 4.0, 4.0);
        assert!(e.eccentricity().abs() < CONFUSION);
        assert!(e.focal().abs() < CONFUSION);
    }

    // ── Hypr tests ───────────────────────────────────────────────────────────

    #[test]
    fn hypr_new_stores_fields() {
        let h = Hypr::new([1.0, 0.0, 0.0], 3.0, 4.0);
        assert_eq!(h.center, [1.0, 0.0, 0.0]);
        assert_eq!(h.major_radius, 3.0);
        assert_eq!(h.minor_radius, 4.0);
    }

    #[test]
    fn hypr_focal() {
        // c = sqrt(3²+4²) = 5, focal = 10
        let h = Hypr::new([0.0; 3], 3.0, 4.0);
        assert!((h.focal() - 10.0).abs() < 1e-12);
    }

    #[test]
    fn hypr_eccentricity() {
        // e = sqrt(1+(4/3)²) = sqrt(1+16/9) = sqrt(25/9) = 5/3
        let h = Hypr::new([0.0; 3], 3.0, 4.0);
        assert!((h.eccentricity() - 5.0 / 3.0).abs() < 1e-12);
    }
}
