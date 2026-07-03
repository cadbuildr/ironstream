//! `Geom2d_Ellipse` — an analytic 2D ellipse, faithfully reproducing
//! OpenCascade's `Geom2d_Ellipse` (a `Geom2d_Conic`).
//!
//! An ellipse is positioned in the plane by a local coordinate system
//! (`gp_Ax22d`): an origin (the ellipse centre) plus an "X Direction" (the
//! major axis) and a "Y Direction" (the minor axis). It is parameterised by an
//! angle `U`:
//!
//! ```text
//! P(U) = O + MajorRadius*cos(U)*XDir + MinorRadius*sin(U)*YDir
//! ```
//!
//! where `O`, `XDir`, `YDir` are the origin and the two directions of the
//! local coordinate system. The "X Axis" defines the parameter origin. The
//! curve is closed and periodic with period `2*PI` and a parameter range of
//! `[0, 2*PI[`.
//!
//! This module is self-contained: it builds on the local-frame (`Ax22d2`) and
//! 2D direction (`Dir2d`) types introduced by `geom2d_circle`, plus
//! `gp2d::{Pnt2d, Vec2d, Trsf2d}` and `precision`. It introduces the
//! elementary 2D ellipse value type that OCCT spells `gp_Elips2d`.

use crate::geom2d_circle::{Ax22d2, Dir2d};
use crate::gp2d::{Ax2d, Pnt2d, Trsf2d, Vec2d};
use std::f64::consts::PI;

/// `2*PI` — the period and parameter span of a full ellipse.
const TWO_PI: f64 = 2.0 * PI;

/// An elementary, non-parameterised 2D ellipse (`gp_Elips2d`): a local frame
/// (`gp_Ax22d`) whose origin is the centre, plus a major and a minor radius.
#[derive(Clone, Copy, Debug)]
// occt: gp_Elips2d
pub struct Elips2d {
    position: Ax22d2,
    major_radius: f64,
    minor_radius: f64,
}

impl Elips2d {
    /// `gp_Elips2d(const gp_Ax2d& MajorAxis, MajorRadius, MinorRadius, Sense)`
    /// — ellipse from its major-axis line and the two radii. `major_axis`
    /// locates the centre (its origin) and defines the "X Direction"; the frame
    /// is direct if `sense` is true.
    ///
    /// Panics (OCCT raises `Standard_ConstructionError`) if
    /// `minor_radius < 0` or `major_radius < minor_radius`.
    pub fn new(major_axis: Ax2d, major_radius: f64, minor_radius: f64, sense: bool) -> Self {
        assert!(
            minor_radius >= 0.0 && major_radius >= minor_radius,
            "gp_Elips2d: require MinorRadius >= 0 and MajorRadius >= MinorRadius"
        );
        Self {
            position: Ax22d2::from_x_axis(major_axis.location, major_axis.direction, sense),
            major_radius,
            minor_radius,
        }
    }

    /// `gp_Elips2d(const gp_Ax22d& A, MajorRadius, MinorRadius)` — ellipse from
    /// a full local frame; `A`'s origin is the centre, "X Direction" the major
    /// axis, "Y Direction" the minor axis.
    ///
    /// Panics if `minor_radius < 0` or `major_radius < minor_radius`.
    pub fn from_axis(position: Ax22d2, major_radius: f64, minor_radius: f64) -> Self {
        assert!(
            minor_radius >= 0.0 && major_radius >= minor_radius,
            "gp_Elips2d: require MinorRadius >= 0 and MajorRadius >= MinorRadius"
        );
        Self {
            position,
            major_radius,
            minor_radius,
        }
    }

    #[inline]
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }
    #[inline]
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.position.location()
    }
    #[inline]
    pub fn position(&self) -> Ax22d2 {
        self.position
    }
}

/// `Geom2d_Ellipse` — a parameterised analytic 2D ellipse.
///
/// Stores its local coordinate system (`Ax22d2`, the OCCT `gp_Ax22d pos` of
/// `Geom2d_Conic`) and its major/minor radii. The parametrisation is
/// `P(U) = O + MajorRadius*cos(U)*XDir + MinorRadius*sin(U)*YDir`.
// occt: Geom2d_Ellipse
#[derive(Clone, Copy, Debug)]
pub struct Geom2dEllipse {
    pos: Ax22d2,
    major_radius: f64,
    minor_radius: f64,
}

impl Geom2dEllipse {
    /// `Geom2d_Ellipse(const gp_Elips2d& E)` — by conversion of an elementary
    /// ellipse.
    pub fn from_elips2d(e: Elips2d) -> Self {
        Self {
            pos: e.position(),
            major_radius: e.major_radius(),
            minor_radius: e.minor_radius(),
        }
    }

    /// `Geom2d_Ellipse(const gp_Ax2d& MajorAxis, MajorRadius, MinorRadius,
    /// Sense)` — `MajorAxis` is the "X Axis" of the local frame; the frame is
    /// direct if `sense` is true.
    ///
    /// Panics (OCCT raises `Standard_ConstructionError`) if
    /// `minor_radius < 0` or `major_radius < minor_radius`.
    pub fn from_major_axis(
        major_axis: Ax2d,
        major_radius: f64,
        minor_radius: f64,
        sense: bool,
    ) -> Self {
        assert!(
            minor_radius >= 0.0 && major_radius >= minor_radius,
            "Geom2d_Ellipse: require MinorRadius >= 0 and MajorRadius >= MinorRadius"
        );
        Self {
            pos: Ax22d2::from_x_axis(major_axis.location, major_axis.direction, sense),
            major_radius,
            minor_radius,
        }
    }

    /// `Geom2d_Ellipse(const gp_Ax22d& Axis, MajorRadius, MinorRadius)` — `Axis`
    /// locates and orients the ellipse; its origin is the centre, "X Direction"
    /// the major axis, "Y Direction" the minor axis.
    ///
    /// Panics if `minor_radius < 0` or `major_radius < minor_radius`.
    pub fn from_axis(axis: Ax22d2, major_radius: f64, minor_radius: f64) -> Self {
        assert!(
            minor_radius >= 0.0 && major_radius >= minor_radius,
            "Geom2d_Ellipse: require MinorRadius >= 0 and MajorRadius >= MinorRadius"
        );
        Self {
            pos: axis,
            major_radius,
            minor_radius,
        }
    }

    /// `SetElips2d(const gp_Elips2d& E)` — convert an elementary ellipse into
    /// this one.
    pub fn set_elips2d(&mut self, e: Elips2d) {
        self.pos = e.position();
        self.major_radius = e.major_radius();
        self.minor_radius = e.minor_radius();
    }

    /// `SetMajorRadius(const Standard_Real MajorRadius)`.
    ///
    /// Panics (OCCT raises `Standard_ConstructionError`) if the new major
    /// radius becomes less than the minor radius.
    pub fn set_major_radius(&mut self, major_radius: f64) {
        assert!(
            major_radius >= self.minor_radius,
            "Geom2d_Ellipse::SetMajorRadius: MajorRadius must be >= MinorRadius"
        );
        self.major_radius = major_radius;
    }

    /// `SetMinorRadius(const Standard_Real MinorRadius)`.
    ///
    /// Panics if `minor_radius < 0` or the major radius would become less than
    /// the minor radius.
    pub fn set_minor_radius(&mut self, minor_radius: f64) {
        assert!(
            minor_radius >= 0.0 && self.major_radius >= minor_radius,
            "Geom2d_Ellipse::SetMinorRadius: require MinorRadius >= 0 and MajorRadius >= MinorRadius"
        );
        self.minor_radius = minor_radius;
    }

    /// `Elips2d()` — the equivalent non-persistent `gp_Elips2d`.
    pub fn elips2d(&self) -> Elips2d {
        Elips2d::from_axis(self.pos, self.major_radius, self.minor_radius)
    }

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

    /// `Location()` — the centre of the ellipse (origin of the local frame).
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.pos.location()
    }

    /// The local coordinate system (`gp_Ax22d Position()`).
    #[inline]
    pub fn position(&self) -> Ax22d2 {
        self.pos
    }

    /// The "X Axis" of the local frame (`gp_Ax2d XAxis()`): centre + major
    /// direction.
    #[inline]
    pub fn x_axis(&self) -> Ax2d {
        Ax2d::new(self.pos.location(), self.pos.x_direction())
    }

    /// The "Y Axis" of the local frame (`gp_Ax2d YAxis()`): centre + minor
    /// direction.
    #[inline]
    pub fn y_axis(&self) -> Ax2d {
        Ax2d::new(self.pos.location(), self.pos.y_direction())
    }

    /// The "X Direction" of the local frame (major axis direction).
    #[inline]
    pub fn x_direction(&self) -> Dir2d {
        self.pos.x_direction()
    }

    /// The "Y Direction" of the local frame (minor axis direction).
    #[inline]
    pub fn y_direction(&self) -> Dir2d {
        self.pos.y_direction()
    }

    /// `ReversedParameter(U)` — for an ellipse, `2*PI - U`.
    #[inline]
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        TWO_PI - u
    }

    /// `Eccentricity()` — `e = f / MajorRadius` where `f = sqrt(a^2 - b^2)` is
    /// the focal distance from the centre. Returns `0` if `MajorRadius == 0`.
    #[inline]
    pub fn eccentricity(&self) -> f64 {
        if self.major_radius == 0.0 {
            0.0
        } else {
            (self.major_radius * self.major_radius - self.minor_radius * self.minor_radius).sqrt()
                / self.major_radius
        }
    }

    /// `Focal()` — the focal distance: the distance between the two foci,
    /// `2*sqrt(a^2 - b^2)`.
    #[inline]
    pub fn focal(&self) -> f64 {
        2.0 * (self.major_radius * self.major_radius - self.minor_radius * self.minor_radius).sqrt()
    }

    /// `Parameter()` — the semi-latus rectum `p = (1 - e^2) * MajorRadius`,
    /// equivalently `b^2 / a`. Returns `0` if `MajorRadius == 0`.
    #[inline]
    pub fn parameter(&self) -> f64 {
        if self.major_radius == 0.0 {
            0.0
        } else {
            let e = self.eccentricity();
            (1.0 - e * e) * self.major_radius
        }
    }

    /// `Focus1()` — the first focus, on the positive side of the "X Axis":
    /// `C + sqrt(a^2 - b^2) * XDir`.
    pub fn focus1(&self) -> Pnt2d {
        let c =
            (self.major_radius * self.major_radius - self.minor_radius * self.minor_radius).sqrt();
        self.pos.location() + self.pos.x_direction() * c
    }

    /// `Focus2()` — the second focus, on the negative side of the "X Axis":
    /// `C - sqrt(a^2 - b^2) * XDir`.
    pub fn focus2(&self) -> Pnt2d {
        let c =
            (self.major_radius * self.major_radius - self.minor_radius * self.minor_radius).sqrt();
        self.pos.location() - self.pos.x_direction() * c
    }

    /// `Directrix1()` — the first directrix: the line normal to the "X Axis" at
    /// distance `d = MajorRadius / e` from the centre, on the positive side,
    /// parallel to the "Y Axis".
    ///
    /// Panics (OCCT raises `Standard_ConstructionError`) if the eccentricity is
    /// `0` (the ellipse degenerates into a circle).
    pub fn directrix1(&self) -> Ax2d {
        let e = self.eccentricity();
        assert!(
            e > crate::precision::ANGULAR,
            "Geom2d_Ellipse::Directrix1: eccentricity must be non-zero"
        );
        let d = self.major_radius / e;
        let loc = self.pos.location() + self.pos.x_direction() * d;
        Ax2d::new(loc, self.pos.y_direction())
    }

    /// `Directrix2()` — the symmetric of `Directrix1` about the "Y Axis": the
    /// line on the negative side of the "X Axis".
    ///
    /// Panics if the eccentricity is `0`.
    pub fn directrix2(&self) -> Ax2d {
        let e = self.eccentricity();
        assert!(
            e > crate::precision::ANGULAR,
            "Geom2d_Ellipse::Directrix2: eccentricity must be non-zero"
        );
        let d = self.major_radius / e;
        let loc = self.pos.location() - self.pos.x_direction() * d;
        Ax2d::new(loc, self.pos.y_direction())
    }

    /// `FirstParameter()` — `0.0`.
    #[inline]
    pub fn first_parameter(&self) -> f64 {
        0.0
    }

    /// `LastParameter()` — `2*PI`.
    #[inline]
    pub fn last_parameter(&self) -> f64 {
        TWO_PI
    }

    /// `IsClosed()` — always true.
    #[inline]
    pub fn is_closed(&self) -> bool {
        true
    }

    /// `IsPeriodic()` — always true; the period is `2*PI`.
    #[inline]
    pub fn is_periodic(&self) -> bool {
        true
    }

    /// The period, `2*PI`.
    #[inline]
    pub fn period(&self) -> f64 {
        TWO_PI
    }

    /// `EvalD0(U)` — the point of parameter `U`:
    /// `P = C + MajorRadius*cos(U)*XDir + MinorRadius*sin(U)*YDir`.
    pub fn eval_d0(&self, u: f64) -> Pnt2d {
        let (s, c) = u.sin_cos();
        self.pos.location()
            + self.pos.x_direction() * (self.major_radius * c)
            + self.pos.y_direction() * (self.minor_radius * s)
    }

    /// `EvalD1(U)` — point and first derivative.
    ///
    /// `V1 = -MajorRadius*sin(U)*XDir + MinorRadius*cos(U)*YDir`.
    pub fn eval_d1(&self, u: f64) -> (Pnt2d, Vec2d) {
        let (s, c) = u.sin_cos();
        let xd = self.pos.x_direction();
        let yd = self.pos.y_direction();
        let o = self.pos.location();
        let p = o + xd * (self.major_radius * c) + yd * (self.minor_radius * s);
        let v1 = xd * (-self.major_radius * s) + yd * (self.minor_radius * c);
        (p, v1)
    }

    /// `EvalD2(U)` — point, first and second derivatives.
    ///
    /// `V2 = -MajorRadius*cos(U)*XDir - MinorRadius*sin(U)*YDir`.
    pub fn eval_d2(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d) {
        let (s, c) = u.sin_cos();
        let xd = self.pos.x_direction();
        let yd = self.pos.y_direction();
        let o = self.pos.location();
        let p = o + xd * (self.major_radius * c) + yd * (self.minor_radius * s);
        let v1 = xd * (-self.major_radius * s) + yd * (self.minor_radius * c);
        let v2 = xd * (-self.major_radius * c) + yd * (-self.minor_radius * s);
        (p, v1, v2)
    }

    /// `EvalD3(U)` — point, first, second and third derivatives.
    ///
    /// `V3 = MajorRadius*sin(U)*XDir - MinorRadius*cos(U)*YDir`.
    pub fn eval_d3(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d, Vec2d) {
        let (s, c) = u.sin_cos();
        let xd = self.pos.x_direction();
        let yd = self.pos.y_direction();
        let o = self.pos.location();
        let p = o + xd * (self.major_radius * c) + yd * (self.minor_radius * s);
        let v1 = xd * (-self.major_radius * s) + yd * (self.minor_radius * c);
        let v2 = xd * (-self.major_radius * c) + yd * (-self.minor_radius * s);
        let v3 = xd * (self.major_radius * s) + yd * (-self.minor_radius * c);
        (p, v1, v2, v3)
    }

    /// `EvalDN(U, N)` — the `N`-th derivative vector (`N >= 1`).
    ///
    /// Differentiating `MajorRadius*cos(U)*XDir + MinorRadius*sin(U)*YDir` `N`
    /// times rotates each trigonometric argument by `N*PI/2`:
    /// `DN = MajorRadius*cos(U + N*PI/2)*XDir + MinorRadius*sin(U + N*PI/2)*YDir`.
    ///
    /// Panics (OCCT raises `Standard_RangeError`) if `n < 1`.
    pub fn eval_dn(&self, u: f64, n: i32) -> Vec2d {
        assert!(n >= 1, "Geom2d_Ellipse::EvalDN: N must be >= 1");
        let arg = u + (n as f64) * (PI / 2.0);
        let (s, c) = arg.sin_cos();
        self.pos.x_direction() * (self.major_radius * c)
            + self.pos.y_direction() * (self.minor_radius * s)
    }

    /// `Transform(const gp_Trsf2d& T)` — applies a 2D transform.
    ///
    /// The centre and directions are mapped through `T`; the radii are scaled
    /// by `Abs(T.ScaleFactor())`. The local frame is re-orthonormalised, just
    /// as transforming a `gp_Ax22d` does.
    pub fn transform(&mut self, t: &Trsf2d) {
        let scale = trsf2d_scale_factor(t);
        let new_loc = t.apply(self.pos.location());
        let xd = self.pos.x_direction();
        let yd = self.pos.y_direction();
        let new_x = apply_vector_2d(t, xd);
        let new_y_hint = apply_vector_2d(t, yd);
        self.pos = Ax22d2::new(new_loc, new_x, new_y_hint);
        self.major_radius *= scale.abs();
        self.minor_radius *= scale.abs();
    }

    /// `Transformed(T)` — non-mutating variant returning a transformed copy.
    pub fn transformed(&self, t: &Trsf2d) -> Geom2dEllipse {
        let mut e = *self;
        e.transform(t);
        e
    }

    /// `Copy()` — a deep, independent copy of this ellipse.
    pub fn copy(&self) -> Geom2dEllipse {
        *self
    }
}

/// Applies the linear part of a 2D transform to a vector (ignoring the
/// translation), as `gp_Trsf2d` does for a `gp_Vec2d`.
#[inline]
fn apply_vector_2d(t: &Trsf2d, v: Pnt2d) -> Pnt2d {
    Pnt2d::new(
        t.m[0][0] * v.x + t.m[0][1] * v.y,
        t.m[1][0] * v.x + t.m[1][1] * v.y,
    )
}

/// The uniform scale factor of a 2D transform (`gp_Trsf2d::ScaleFactor()`):
/// the square root of the absolute value of the 2x2 linear determinant.
#[inline]
fn trsf2d_scale_factor(t: &Trsf2d) -> f64 {
    let det = t.m[0][0] * t.m[1][1] - t.m[0][1] * t.m[1][0];
    det.abs().sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn x_axis_ellipse() -> Geom2dEllipse {
        Geom2dEllipse::from_major_axis(
            Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
            10.0,
            5.0,
            true,
        )
    }

    #[test]
    fn major_vertex_at_zero() {
        let e = x_axis_ellipse();
        let p = e.eval_d0(0.0);
        assert!((p.x - 10.0).abs() < 1e-12 && p.y.abs() < 1e-12, "{p:?}");
    }

    #[test]
    fn minor_vertex_at_half_pi() {
        let e = x_axis_ellipse();
        let p = e.eval_d0(PI / 2.0);
        assert!(p.x.abs() < 1e-12 && (p.y - 5.0).abs() < 1e-12, "{p:?}");
    }

    #[test]
    fn periodicity_and_bounds() {
        let e = x_axis_ellipse();
        assert_eq!(e.first_parameter(), 0.0);
        assert!((e.last_parameter() - TWO_PI).abs() < 1e-15);
        assert!(e.is_closed() && e.is_periodic());
    }
}
