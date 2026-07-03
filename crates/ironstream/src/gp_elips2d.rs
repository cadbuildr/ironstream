//! `gp_Elips2d` -- analytic 2D ellipse primitive, mirroring OpenCascade's
//! `gp_Elips2d` (package `TKMath`).
//!
//! A 2D ellipse is positioned in the plane by a local coordinate system
//! (`gp_Ax22d`): an origin (the ellipse centre) plus an "X Direction" (the
//! major axis) and a "Y Direction" (the minor axis). The normal to the ellipse
//! plane is the "Z Direction" (always `+Z` for a 2D ellipse).
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
//!   Directrix1: line at distance a/e from centre along +XDir, parallel to YDir
//!   Directrix2: line at distance a/e from centre along -XDir, parallel to YDir
//! ```

use crate::geom2d_circle::{Ax22d2, Dir2d};
use crate::gp2d::{Ax2d, Pnt2d, Trsf2d};

// occt: gp_Elips2d
/// An analytic 2D ellipse: a placement (`gp_Ax22d`) plus major and minor radii.
///
/// The placement's origin is the ellipse centre; `XDir` points along the major
/// axis; `YDir` points along the minor axis.
#[derive(Clone, Copy, Debug)]
pub struct Elips2d {
    /// Local coordinate system — OCCT calls this `pos` (a `gp_Ax22d`).
    pos: Ax22d2,
    major_radius: f64,
    minor_radius: f64,
}

impl Elips2d {
    /// `gp_Elips2d(const gp_Ax2d& MajorAxis, Standard_Real MajorRadius,
    ///             Standard_Real MinorRadius, Standard_Boolean Sense = Standard_True)`
    ///
    /// Constructs an ellipse from its major-axis line (which provides the centre
    /// location and the "X Direction") and the two radii. The frame is direct
    /// (counter-clockwise) if `sense` is `true`, otherwise indirect.
    ///
    /// Panics (`Standard_ConstructionError` in OCCT) if
    /// `minor_radius < 0` or `major_radius < minor_radius`.
    pub fn new(major_axis: Ax2d, major_radius: f64, minor_radius: f64) -> Self {
        assert!(
            minor_radius >= 0.0 && major_radius >= minor_radius,
            "gp_Elips2d: require MinorRadius >= 0 and MajorRadius >= MinorRadius"
        );
        Self {
            pos: Ax22d2::from_x_axis(major_axis.location, major_axis.direction, true),
            major_radius,
            minor_radius,
        }
    }

    /// `gp_Elips2d(const gp_Ax2d& MajorAxis, Standard_Real MajorRadius,
    ///             Standard_Real MinorRadius, Standard_Boolean Sense)`
    ///
    /// Constructs with explicit `sense` flag controlling frame handedness.
    pub fn new_with_sense(
        major_axis: Ax2d,
        major_radius: f64,
        minor_radius: f64,
        sense: bool,
    ) -> Self {
        assert!(
            minor_radius >= 0.0 && major_radius >= minor_radius,
            "gp_Elips2d: require MinorRadius >= 0 and MajorRadius >= MinorRadius"
        );
        Self {
            pos: Ax22d2::from_x_axis(major_axis.location, major_axis.direction, sense),
            major_radius,
            minor_radius,
        }
    }

    /// `gp_Elips2d(const gp_Ax22d& A, Standard_Real MajorRadius,
    ///             Standard_Real MinorRadius)`
    ///
    /// Constructs an ellipse from a full local frame; `A`'s origin is the
    /// centre, "X Direction" the major axis, "Y Direction" the minor axis.
    ///
    /// Panics if `minor_radius < 0` or `major_radius < minor_radius`.
    pub fn from_axis(position: Ax22d2, major_radius: f64, minor_radius: f64) -> Self {
        assert!(
            minor_radius >= 0.0 && major_radius >= minor_radius,
            "gp_Elips2d: require MinorRadius >= 0 and MajorRadius >= MinorRadius"
        );
        Self {
            pos: position,
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
            "gp_Elips2d::SetMajorRadius: MajorRadius must be >= MinorRadius"
        );
        self.major_radius = major_radius;
    }

    /// `SetMinorRadius(r)` — update the minor radius.
    ///
    /// Panics if `minor_radius < 0` or `minor_radius > major_radius`.
    pub fn set_minor_radius(&mut self, minor_radius: f64) {
        assert!(
            minor_radius >= 0.0 && self.major_radius >= minor_radius,
            "gp_Elips2d::SetMinorRadius: require 0 <= MinorRadius <= MajorRadius"
        );
        self.minor_radius = minor_radius;
    }

    // ----------------------------------------------------------------- axes

    /// `XAxis()` — the major-axis line (`gp_Ax2d` through the centre along
    /// "X Direction").
    #[inline]
    pub fn x_axis(&self) -> Ax2d {
        Ax2d::new(self.pos.location(), self.pos.x_direction())
    }

    /// `YAxis()` — the minor-axis line (`gp_Ax2d` through the centre along
    /// "Y Direction").
    #[inline]
    pub fn y_axis(&self) -> Ax2d {
        Ax2d::new(self.pos.location(), self.pos.y_direction())
    }

    /// `Position()` — the local coordinate system (`gp_Ax22d`).
    #[inline]
    pub fn position(&self) -> Ax22d2 {
        self.pos
    }

    /// `Location()` — the centre of the ellipse (origin of the local frame).
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.pos.location()
    }

    /// `IsDirect()` — `true` if the local frame is direct (counter-clockwise).
    ///
    /// The frame is direct when the cross product of X and Y directions is
    /// positive (i.e. Y = X rotated +90°).
    #[inline]
    pub fn is_direct(&self) -> bool {
        self.pos.x_direction().cross(self.pos.y_direction()) > 0.0
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
    pub fn focus1(&self) -> Pnt2d {
        let c = (self.major_radius * self.major_radius
            - self.minor_radius * self.minor_radius)
            .sqrt();
        self.pos.location() + self.pos.x_direction() * c
    }

    /// `Focus2()` — the second focus: `Centre - c * XDir`.
    pub fn focus2(&self) -> Pnt2d {
        let c = (self.major_radius * self.major_radius
            - self.minor_radius * self.minor_radius)
            .sqrt();
        self.pos.location() - self.pos.x_direction() * c
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

    /// `Directrix1()` — the first directrix: a line normal to the "X Axis" at
    /// distance `d = MajorRadius / Eccentricity` from the centre on the
    /// positive side, parallel to the "Y Axis".
    ///
    /// Panics (`Standard_ConstructionError` in OCCT) if the eccentricity is `0`
    /// (the ellipse degenerates into a circle).
    pub fn directrix1(&self) -> Ax2d {
        let e = self.eccentricity();
        assert!(
            e > crate::precision::ANGULAR,
            "gp_Elips2d::Directrix1: eccentricity must be non-zero (not a circle)"
        );
        let d = self.major_radius / e;
        let loc = self.pos.location() + self.pos.x_direction() * d;
        Ax2d::new(loc, self.pos.y_direction())
    }

    /// `Directrix2()` — the second directrix: the symmetric of `Directrix1`
    /// about the "Y Axis" (on the negative side of the "X Axis").
    ///
    /// Panics if the eccentricity is `0`.
    pub fn directrix2(&self) -> Ax2d {
        let e = self.eccentricity();
        assert!(
            e > crate::precision::ANGULAR,
            "gp_Elips2d::Directrix2: eccentricity must be non-zero (not a circle)"
        );
        let d = self.major_radius / e;
        let loc = self.pos.location() - self.pos.x_direction() * d;
        Ax2d::new(loc, self.pos.y_direction())
    }

    // --------------------------------------------------------------- transforms

    /// `Translate(Vec2d)` — translate by a 2D vector, returning a new ellipse.
    pub fn translated(&self, v: Pnt2d) -> Elips2d {
        let t = Trsf2d::translation(v);
        self.transformed(&t)
    }

    /// `Rotate(Pnt2d, angle)` — rotate about a 2D point by `angle` radians,
    /// returning a new ellipse.
    pub fn rotated(&self, center: Pnt2d, angle: f64) -> Elips2d {
        let t = Trsf2d::rotation(center, angle);
        self.transformed(&t)
    }

    /// `Mirror(Pnt2d)` — point symmetry through `center`, returning a new
    /// ellipse.
    ///
    /// The centre is reflected; the X and Y directions are negated (which
    /// reverses the frame handedness, matching OCCT behaviour for point mirror).
    pub fn mirrored_point(&self, center: Pnt2d) -> Elips2d {
        let new_loc = center * 2.0 - self.pos.location();
        let new_x = -self.pos.x_direction();
        let new_y = -self.pos.y_direction();
        Elips2d {
            pos: Ax22d2::new(new_loc, new_x, new_y),
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// `Mirror(Ax2d)` — reflection across a 2D axis (a line), returning a new
    /// ellipse.
    ///
    /// Both the centre and the frame directions are reflected across `axis`.
    pub fn mirrored_axis(&self, axis: Ax2d) -> Elips2d {
        let d = axis.direction.normalized();
        // Reflect a 2D point across the line through `axis.location` with
        // direction `d`: p' = 2*(proj of (p - loc) onto d)*d - (p - loc) + loc.
        let mirror_pt = |p: Pnt2d| {
            let rel = p - axis.location;
            let proj = d * (rel.dot(d) * 2.0) - rel;
            axis.location + proj
        };
        // Reflect a 2D direction vector.
        let mirror_dir = |v: Dir2d| {
            let proj = d * (v.dot(d) * 2.0) - v;
            proj.normalized()
        };

        let new_loc = mirror_pt(self.pos.location());
        let new_x = mirror_dir(self.pos.x_direction());
        let new_y = mirror_dir(self.pos.y_direction());
        Elips2d {
            pos: Ax22d2::new(new_loc, new_x, new_y),
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// `Scale(Pnt2d, factor)` — uniform scale about `center`, returning a new
    /// ellipse.
    ///
    /// A negative factor reverses the directions (mirror-like effect), just as
    /// OCCT does.
    pub fn scaled(&self, center: Pnt2d, factor: f64) -> Elips2d {
        let new_loc = center + (self.pos.location() - center) * factor;
        let abs_f = factor.abs();
        // For negative factor the directions flip: use the sign to reorient.
        let (new_x, new_y) = if factor >= 0.0 {
            (self.pos.x_direction(), self.pos.y_direction())
        } else {
            (-self.pos.x_direction(), -self.pos.y_direction())
        };
        Elips2d {
            pos: Ax22d2::new(new_loc, new_x, new_y),
            major_radius: self.major_radius * abs_f,
            minor_radius: self.minor_radius * abs_f,
        }
    }

    /// `Transform(Trsf2d)` — apply a 2D affine transform, returning a new
    /// ellipse.
    ///
    /// The radii scale by `|ScaleFactor|`; the directions are updated via the
    /// linear part of `t`.
    pub fn transformed(&self, t: &Trsf2d) -> Elips2d {
        let scale = trsf2d_scale_factor(t);
        let new_loc = t.apply(self.pos.location());
        let new_x = apply_vector_2d(t, self.pos.x_direction());
        let new_y = apply_vector_2d(t, self.pos.y_direction());
        Elips2d {
            pos: Ax22d2::new(new_loc, new_x, new_y),
            major_radius: self.major_radius * scale.abs(),
            minor_radius: self.minor_radius * scale.abs(),
        }
    }
}

/// Applies the linear part of a 2D transform to a vector (ignoring translation).
#[inline]
fn apply_vector_2d(t: &Trsf2d, v: Pnt2d) -> Pnt2d {
    Pnt2d::new(
        t.m[0][0] * v.x + t.m[0][1] * v.y,
        t.m[1][0] * v.x + t.m[1][1] * v.y,
    )
}

/// The uniform scale factor of a 2D transform: `sqrt(|det(linear)|)`.
#[inline]
fn trsf2d_scale_factor(t: &Trsf2d) -> f64 {
    let det = t.m[0][0] * t.m[1][1] - t.m[0][1] * t.m[1][0];
    det.abs().sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precision::{ANGULAR, CONFUSION};
    use std::f64::consts::FRAC_PI_2;

    fn unit_ellipse() -> Elips2d {
        Elips2d::new(
            Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
            5.0,
            3.0,
        )
    }

    #[test]
    fn constructor_stores_radii() {
        let e = unit_ellipse();
        assert!((e.major_radius() - 5.0).abs() < CONFUSION);
        assert!((e.minor_radius() - 3.0).abs() < CONFUSION);
    }

    #[test]
    fn eccentricity() {
        // e = sqrt(25 - 9) / 5 = 4/5 = 0.8
        let e = unit_ellipse();
        assert!((e.eccentricity() - 0.8).abs() < CONFUSION);
    }

    #[test]
    fn focal_and_foci() {
        // c = 4, focal = 8
        let e = unit_ellipse();
        assert!((e.focal() - 8.0).abs() < CONFUSION);
        let f1 = e.focus1();
        let f2 = e.focus2();
        assert!((f1.x - 4.0).abs() < CONFUSION && f1.y.abs() < CONFUSION);
        assert!((f2.x + 4.0).abs() < CONFUSION && f2.y.abs() < CONFUSION);
    }

    #[test]
    fn parameter_semi_latus_rectum() {
        // p = b²/a = 9/5 = 1.8
        let e = unit_ellipse();
        assert!((e.parameter() - 1.8).abs() < CONFUSION);
    }

    #[test]
    fn axes() {
        let e = unit_ellipse();
        let xa = e.x_axis();
        let ya = e.y_axis();
        let x_dir = xa.direction.normalized();
        let y_dir = ya.direction.normalized();
        assert!((x_dir.x - 1.0).abs() < ANGULAR && x_dir.y.abs() < ANGULAR);
        assert!(y_dir.x.abs() < ANGULAR && (y_dir.y - 1.0).abs() < ANGULAR);
    }

    #[test]
    fn translate() {
        let e = unit_ellipse();
        let t = e.translated(Pnt2d::new(10.0, 20.0));
        assert!((t.location().x - 10.0).abs() < CONFUSION);
        assert!((t.location().y - 20.0).abs() < CONFUSION);
        assert!((t.major_radius() - 5.0).abs() < CONFUSION);
    }

    #[test]
    fn rotate_centre() {
        // Ellipse at (1, 0); rotate 90° about origin → (0, 1).
        let e = Elips2d::new(
            Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0)),
            5.0,
            3.0,
        );
        let r = e.rotated(Pnt2d::origin(), FRAC_PI_2);
        assert!((r.location().x).abs() < CONFUSION);
        assert!((r.location().y - 1.0).abs() < CONFUSION);
        assert!((r.major_radius() - 5.0).abs() < CONFUSION);
    }

    #[test]
    fn scale_radii_and_location() {
        let e = Elips2d::new(
            Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0)),
            5.0,
            3.0,
        );
        let s = e.scaled(Pnt2d::origin(), 2.0);
        assert!((s.major_radius() - 10.0).abs() < CONFUSION);
        assert!((s.minor_radius() - 6.0).abs() < CONFUSION);
        assert!((s.location().x - 2.0).abs() < CONFUSION);
    }

    #[test]
    fn circle_eccentricity_zero() {
        let e = Elips2d::new(
            Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
            4.0,
            4.0,
        );
        assert!(e.eccentricity().abs() < CONFUSION);
        assert!(e.focal().abs() < CONFUSION);
    }

    #[test]
    fn is_direct_default_frame() {
        let e = unit_ellipse();
        assert!(e.is_direct());
    }

    #[test]
    fn is_direct_indirect_frame() {
        let e = Elips2d::new_with_sense(
            Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
            5.0,
            3.0,
            false,
        );
        assert!(!e.is_direct());
    }
}
