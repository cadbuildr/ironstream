// FILE: gce2d_make.rs
//! `GCE2d` — 2D construction algorithms mirroring OpenCascade's
//! `GCE2d` package (`src/ModelingAlgorithms/TKGeomBase/GCE2d`).
//!
//! Each `GCE2d_MakeXxx` class builds a `Geom2d_Xxx` object from simple 2D
//! geometric inputs (points, radii, angles, axes). Construction may fail if
//! the inputs are geometrically degenerate; errors are reported through
//! [`Gce2dError`] rather than panics.
//!
//! # Covered classes
//! - [`Gce2dMakeArcOfCircle`]  — `GCE2d_MakeArcOfCircle`
//! - [`Gce2dMakeCircle`]       — `GCE2d_MakeCircle`
//! - [`Gce2dMakeLine`]         — `GCE2d_MakeLine`
//! - [`Gce2dMakeSegment`]      — `GCE2d_MakeSegment`
//! - [`Gce2dMakeEllipse`]      — `GCE2d_MakeEllipse`

use crate::geom2d_circle::{Ax22d2, Circ2d, Geom2dCircle};
use crate::geom2d_ellipse::{Elips2d, Geom2dEllipse};
use crate::geom2d_line::{Geom2dLine, Lin2d};
use crate::geom2d_trimmed_curve::{BasisCurve2d, Geom2dTrimmedCurve};
use crate::gp2d::{Ax2d, Pnt2d};
use crate::precision::CONFUSION;
use std::f64::consts::PI;

// ─────────────────────────────── Error type ──────────────────────────────────

/// Errors that a `GCE2d_MakeXxx` construction may return.
// occt: GCE2d_Root // (status codes)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gce2dError {
    /// The two end points are coincident and do not define a direction.
    CoincidentPoints,
    /// Three or more of the input points are collinear (cannot define a circle).
    CollinearPoints,
    /// The radius value is negative or too small to form the requested curve.
    NegativeRadius,
    /// The specified angle range is degenerate (start == end).
    BadAngles,
    /// The major radius is smaller than the minor radius.
    InvertedRadii,
    /// A direction or vector provided has zero magnitude.
    NullVector,
}

impl std::fmt::Display for Gce2dError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gce2dError::CoincidentPoints => write!(f, "GCE2d: coincident points"),
            Gce2dError::CollinearPoints  => write!(f, "GCE2d: collinear points"),
            Gce2dError::NegativeRadius   => write!(f, "GCE2d: negative or zero radius"),
            Gce2dError::BadAngles        => write!(f, "GCE2d: degenerate angle range"),
            Gce2dError::InvertedRadii    => write!(f, "GCE2d: major radius < minor radius"),
            Gce2dError::NullVector       => write!(f, "GCE2d: null direction vector"),
        }
    }
}

// ─────────────────────────────── internal helpers ────────────────────────────

/// Signed angle of a 2D point relative to a circle frame, in `[−π, π]`.
fn angle_on_circle_2d(center: Pnt2d, x_dir: Pnt2d, y_dir: Pnt2d, p: Pnt2d) -> f64 {
    let rel = p - center;
    let u = rel.x * x_dir.x + rel.y * x_dir.y;
    let v = rel.x * y_dir.x + rel.y * y_dir.y;
    v.atan2(u)
}

/// Circumscribed circle of three 2D points.
/// Returns `(center, radius)` or an error when points are collinear/coincident.
fn circumcircle_2d(p1: Pnt2d, p2: Pnt2d, p3: Pnt2d) -> Result<(Pnt2d, f64), Gce2dError> {
    if p1.distance(p2) < CONFUSION
        || p2.distance(p3) < CONFUSION
        || p1.distance(p3) < CONFUSION
    {
        return Err(Gce2dError::CoincidentPoints);
    }

    // Perpendicular bisector of P1P2: through mid12 in direction perp(P2-P1)
    let mid12 = (p1 + p2) * 0.5;
    let mid13 = (p1 + p3) * 0.5;
    // Perpendicular directions (rotated +90°)
    let d12 = (p2 - p1).perp(); // perp to P1P2
    let d13 = (p3 - p1).perp(); // perp to P1P3

    // Solve: mid12 + s * d12 = mid13 + t * d13
    // => s * d12.x - t * d13.x = (mid13 - mid12).x
    //    s * d12.y - t * d13.y = (mid13 - mid12).y
    let rhs = mid13 - mid12;
    let a11 = d12.x;
    let a12 = -d13.x;
    let a21 = d12.y;
    let a22 = -d13.y;
    let det = a11 * a22 - a12 * a21;
    if det.abs() < 1e-15 {
        return Err(Gce2dError::CollinearPoints);
    }
    let s = (rhs.x * a22 - rhs.y * a12) / det;
    let center = mid12 + d12 * s;
    let radius = center.distance(p1);
    Ok((center, radius))
}

// ─────────────────────────── Gce2dMakeArcOfCircle ────────────────────────────

/// `GCE2d_MakeArcOfCircle` — constructs an arc of a 2D circle as a
/// [`Geom2dTrimmedCurve`] wrapping a [`Geom2dCircle`].
///
/// Three construction modes are provided:
/// 1. Three points on the arc (arc runs from `p1` to `p3` through `p2`).
/// 2. A 2D circle primitive + two parameter angles (with sense flag).
/// 3. A 2D circle primitive + two points (with sense flag).
// occt: GCE2d_MakeArcOfCircle
pub struct Gce2dMakeArcOfCircle {
    result: Result<Geom2dTrimmedCurve, Gce2dError>,
}

impl Gce2dMakeArcOfCircle {
    /// Construct the arc passing through three distinct 2D points `p1`, `p2`, `p3`.
    /// The arc runs from `p1` to `p3` through `p2`.
    ///
    /// Mirrors `GCE2d_MakeArcOfCircle(P1, P2, P3)`.
    pub fn from_three_points(p1: Pnt2d, p2: Pnt2d, p3: Pnt2d) -> Self {
        Self {
            result: Self::build_three_points(p1, p2, p3),
        }
    }

    fn build_three_points(
        p1: Pnt2d,
        p2: Pnt2d,
        p3: Pnt2d,
    ) -> Result<Geom2dTrimmedCurve, Gce2dError> {
        let (center, radius) = circumcircle_2d(p1, p2, p3)?;
        // X axis points toward p1; direct (CCW) frame.
        let x_dir = (p1 - center).normalized();
        let pos = Ax22d2::from_x_axis(center, x_dir, true);
        let circle = Geom2dCircle::from_axis(pos, radius);

        // Compute angles on the circle frame.
        let x_d = pos.x_direction();
        let y_d = pos.y_direction();
        let u1 = angle_on_circle_2d(center, x_d, y_d, p1);
        let u2 = angle_on_circle_2d(center, x_d, y_d, p2);
        let u3 = angle_on_circle_2d(center, x_d, y_d, p3);

        // Choose the range [u_start, u_end] that spans from p1 to p3 through p2.
        let two_pi = 2.0 * PI;
        let u_start = u1;
        let mut u_end = u3;
        let mut u_mid = u2;
        while u_end < u_start {
            u_end += two_pi;
        }
        while u_mid < u_start {
            u_mid += two_pi;
        }
        // If mid overshoots end, the arc is going the wrong way — flip direction.
        if u_mid > u_end {
            u_end -= two_pi;
        }
        let (trim1, trim2) = if u_end >= u_start {
            (u_start, u_end)
        } else {
            (u_end, u_start)
        };
        if (trim2 - trim1).abs() < CONFUSION {
            return Err(Gce2dError::BadAngles);
        }
        let basis = BasisCurve2d::Circle(circle);
        Ok(Geom2dTrimmedCurve::new(basis, trim1, trim2))
    }

    /// Construct an arc of the given 2D circle between two parameter angles.
    ///
    /// When `sense = true` the arc runs counter-clockwise from `alpha1` to `alpha2`.
    ///
    /// Mirrors `GCE2d_MakeArcOfCircle(C, Alpha1, Alpha2, Sense)`.
    pub fn from_circ_angles(circ: Circ2d, alpha1: f64, alpha2: f64, sense: bool) -> Self {
        Self {
            result: Self::build_angles(circ, alpha1, alpha2, sense),
        }
    }

    fn build_angles(
        circ: Circ2d,
        alpha1: f64,
        alpha2: f64,
        sense: bool,
    ) -> Result<Geom2dTrimmedCurve, Gce2dError> {
        let two_pi = 2.0 * PI;
        let a1 = alpha1.rem_euclid(two_pi);
        let mut a2 = alpha2.rem_euclid(two_pi);
        if (a1 - a2).abs() < CONFUSION {
            return Err(Gce2dError::BadAngles);
        }
        if sense {
            if a2 < a1 {
                a2 += two_pi;
            }
        } else if a2 > a1 {
            a2 -= two_pi;
        }
        let (u1, u2) = if a1 <= a2 { (a1, a2) } else { (a2, a1) };
        let circle = Geom2dCircle::from_circ2d(circ);
        let basis = BasisCurve2d::Circle(circle);
        Ok(Geom2dTrimmedCurve::new(basis, u1, u2))
    }

    /// Construct an arc of the given 2D circle from point `p1` to point `p2`.
    ///
    /// When `sense = true` the arc runs counter-clockwise from `p1` to `p2`.
    ///
    /// Mirrors `GCE2d_MakeArcOfCircle(C, P1, P2, Sense)`.
    pub fn from_circ_two_points(circ: Circ2d, p1: Pnt2d, p2: Pnt2d, sense: bool) -> Self {
        let pos = circ.position();
        let alpha1 = angle_on_circle_2d(pos.location(), pos.x_direction(), pos.y_direction(), p1);
        let alpha2 = angle_on_circle_2d(pos.location(), pos.x_direction(), pos.y_direction(), p2);
        Self {
            result: Self::build_angles(circ, alpha1, alpha2, sense),
        }
    }

    /// Returns the constructed arc, or the construction error.
    pub fn result(self) -> Result<Geom2dTrimmedCurve, Gce2dError> {
        self.result
    }

    /// Returns `true` if the construction succeeded.
    pub fn is_done(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns the arc (panics if construction failed).
    pub fn value(&self) -> &Geom2dTrimmedCurve {
        self.result
            .as_ref()
            .expect("Gce2dMakeArcOfCircle: construction failed")
    }
}

// ────────────────────────────── Gce2dMakeCircle ──────────────────────────────

/// `GCE2d_MakeCircle` — constructs a [`Geom2dCircle`] from various 2D inputs.
///
/// Construction modes:
/// 1. A `gp_Circ2d` value.
/// 2. An `Ax22d` placement and a radius.
/// 3. Three 2D points on the circle.
/// 4. A center point, X-axis direction, and radius (with sense).
/// 5. A center point and a point on the circle (direct frame).
// occt: GCE2d_MakeCircle
pub struct Gce2dMakeCircle {
    result: Result<Geom2dCircle, Gce2dError>,
}

impl Gce2dMakeCircle {
    /// Construct from a `gp_Circ2d`.
    ///
    /// Mirrors `GCE2d_MakeCircle(const gp_Circ2d& C)`.
    pub fn from_circ2d(circ: Circ2d) -> Self {
        Self {
            result: Ok(Geom2dCircle::from_circ2d(circ)),
        }
    }

    /// Construct from a full 2D axis placement and a radius.
    ///
    /// Mirrors `GCE2d_MakeCircle(const gp_Ax22d& A, Standard_Real Radius)`.
    pub fn from_ax22d_radius(ax: Ax22d2, radius: f64) -> Self {
        Self {
            result: if radius < 0.0 {
                Err(Gce2dError::NegativeRadius)
            } else {
                Ok(Geom2dCircle::from_axis(ax, radius))
            },
        }
    }

    /// Construct from a single X axis (origin + direction), a radius and a sense.
    ///
    /// Mirrors `GCE2d_MakeCircle(const gp_Ax2d& A, Radius, Sense)`.
    pub fn from_ax2d_radius(ax: Ax2d, radius: f64, sense: bool) -> Self {
        Self {
            result: if radius < 0.0 {
                Err(Gce2dError::NegativeRadius)
            } else {
                Ok(Geom2dCircle::from_x_axis(ax.location, ax.direction, radius, sense))
            },
        }
    }

    /// Construct the circle passing through three distinct 2D points.
    ///
    /// Mirrors `GCE2d_MakeCircle(P1, P2, P3)`.
    pub fn from_three_points(p1: Pnt2d, p2: Pnt2d, p3: Pnt2d) -> Self {
        Self {
            result: Self::build_three_points(p1, p2, p3),
        }
    }

    fn build_three_points(p1: Pnt2d, p2: Pnt2d, p3: Pnt2d) -> Result<Geom2dCircle, Gce2dError> {
        let (center, radius) = circumcircle_2d(p1, p2, p3)?;
        let x_dir = (p1 - center).normalized();
        let pos = Ax22d2::from_x_axis(center, x_dir, true);
        Ok(Geom2dCircle::from_axis(pos, radius))
    }

    /// Construct a circle with given center, X-axis direction, and radius.
    ///
    /// Mirrors `GCE2d_MakeCircle(Center, XAxis, Radius, Sense)`.
    pub fn from_center_dir_radius(center: Pnt2d, x_dir: Pnt2d, radius: f64, sense: bool) -> Self {
        if x_dir.norm() < CONFUSION {
            return Self { result: Err(Gce2dError::NullVector) };
        }
        Self {
            result: if radius < 0.0 {
                Err(Gce2dError::NegativeRadius)
            } else {
                Ok(Geom2dCircle::from_x_axis(center, x_dir, radius, sense))
            },
        }
    }

    /// Construct the circle with a given center passing through a point.
    ///
    /// The frame X axis points from `center` to `pt`. Direct (CCW) orientation.
    ///
    /// Mirrors `GCE2d_MakeCircle(Center, Pt)`.
    pub fn from_center_and_point(center: Pnt2d, pt: Pnt2d) -> Self {
        let radius = center.distance(pt);
        if radius < CONFUSION {
            return Self { result: Err(Gce2dError::CoincidentPoints) };
        }
        let x_dir = (pt - center).normalized();
        Self {
            result: Ok(Geom2dCircle::from_x_axis(center, x_dir, radius, true)),
        }
    }

    /// Returns the constructed circle, or the error.
    pub fn result(self) -> Result<Geom2dCircle, Gce2dError> {
        self.result
    }

    /// Returns `true` if the construction succeeded.
    pub fn is_done(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns the circle (panics if construction failed).
    pub fn value(&self) -> &Geom2dCircle {
        self.result
            .as_ref()
            .expect("Gce2dMakeCircle: construction failed")
    }
}

// ─────────────────────────────── Gce2dMakeLine ───────────────────────────────

/// `GCE2d_MakeLine` — constructs a [`Geom2dLine`] from various 2D inputs.
///
/// Construction modes:
/// 1. A `gp_Lin2d` value.
/// 2. An `Ax2d` (origin + direction).
/// 3. A point and a direction vector.
/// 4. Two distinct points (the line passes through both).
/// 5. A point and a distance to a parallel reference line.
// occt: GCE2d_MakeLine
pub struct Gce2dMakeLine {
    result: Result<Geom2dLine, Gce2dError>,
}

impl Gce2dMakeLine {
    /// Construct from a `gp_Lin2d`.
    ///
    /// Mirrors `GCE2d_MakeLine(const gp_Lin2d& L)`.
    pub fn from_lin2d(lin: Lin2d) -> Self {
        Self {
            result: Ok(Geom2dLine::from_lin2d(lin)),
        }
    }

    /// Construct from an `Ax2d` positioning axis.
    ///
    /// Mirrors `GCE2d_MakeLine(const gp_Ax2d& A)`.
    pub fn from_ax2d(ax: Ax2d) -> Self {
        Self {
            result: Ok(Geom2dLine::new(ax)),
        }
    }

    /// Construct from a point and a direction.
    ///
    /// Mirrors `GCE2d_MakeLine(const gp_Pnt2d& P, const gp_Dir2d& V)`.
    pub fn from_point_dir(p: Pnt2d, v: Pnt2d) -> Self {
        Self {
            result: if v.norm() < CONFUSION {
                Err(Gce2dError::NullVector)
            } else {
                Ok(Geom2dLine::from_point_dir(p, v))
            },
        }
    }

    /// Construct from two distinct points.
    ///
    /// The line runs from `p1` through `p2`.
    ///
    /// Mirrors `GCE2d_MakeLine(const gp_Pnt2d& P1, const gp_Pnt2d& P2)`.
    pub fn from_two_points(p1: Pnt2d, p2: Pnt2d) -> Self {
        let dir = p2 - p1;
        Self {
            result: if dir.norm() < CONFUSION {
                Err(Gce2dError::CoincidentPoints)
            } else {
                Ok(Geom2dLine::from_point_dir(p1, dir))
            },
        }
    }

    /// Construct the line through `p` parallel to `lin` at a signed
    /// perpendicular distance `dist`.
    ///
    /// A positive distance offsets to the left (i.e. in the direction of the
    /// frame's Y axis, which is `lin.direction()` rotated +90°).
    ///
    /// Mirrors `GCE2d_MakeLine(const gp_Lin2d& Lin, const gp_Pnt2d& P)`.
    pub fn from_parallel_offset(lin: Lin2d, dist: f64) -> Self {
        // Offset = direction rotated +90° scaled by dist.
        let d = lin.direction().perp() * dist;
        let new_loc = lin.location() + d;
        Self {
            result: Ok(Geom2dLine::new(Ax2d::new(new_loc, lin.direction()))),
        }
    }

    /// Returns the constructed line, or the error.
    pub fn result(self) -> Result<Geom2dLine, Gce2dError> {
        self.result
    }

    /// Returns `true` if the construction succeeded.
    pub fn is_done(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns the line (panics if construction failed).
    pub fn value(&self) -> &Geom2dLine {
        self.result
            .as_ref()
            .expect("Gce2dMakeLine: construction failed")
    }
}

// ────────────────────────────── Gce2dMakeSegment ─────────────────────────────

/// `GCE2d_MakeSegment` — constructs a bounded straight-line segment as a
/// [`Geom2dTrimmedCurve`] wrapping a [`Geom2dLine`].
///
/// Construction modes:
/// 1. Two distinct 2D points.
/// 2. A `Lin2d` + two parameter values.
/// 3. A `Lin2d` + a start point + an end parameter.
/// 4. A `Lin2d` + a start parameter + an end point.
// occt: GCE2d_MakeSegment
pub struct Gce2dMakeSegment {
    result: Result<Geom2dTrimmedCurve, Gce2dError>,
}

impl Gce2dMakeSegment {
    /// Construct the segment from `p1` to `p2`.
    ///
    /// Mirrors `GCE2d_MakeSegment(const gp_Pnt2d& P1, const gp_Pnt2d& P2)`.
    pub fn from_two_points(p1: Pnt2d, p2: Pnt2d) -> Self {
        let dir = p2 - p1;
        let len = dir.norm();
        Self {
            result: if len < CONFUSION {
                Err(Gce2dError::CoincidentPoints)
            } else {
                let line = Geom2dLine::from_point_dir(p1, dir);
                // Parameter is arc-length because line direction is normalised inside
                // Geom2dLine.  The unit direction is `dir / len`, so parameter at p2
                // = dot((p2-p1), unit_dir) = len.
                let basis = BasisCurve2d::Line(line);
                Ok(Geom2dTrimmedCurve::new(basis, 0.0, len))
            },
        }
    }

    /// Construct a segment on the given 2D line between parameter values `u1`
    /// and `u2`.
    ///
    /// Mirrors `GCE2d_MakeSegment(const gp_Lin2d& L, Real U1, Real U2)`.
    pub fn from_lin_params(lin: Lin2d, u1: f64, u2: f64) -> Self {
        Self {
            result: if (u2 - u1).abs() < CONFUSION {
                Err(Gce2dError::CoincidentPoints)
            } else {
                let line = Geom2dLine::from_lin2d(lin);
                let basis = BasisCurve2d::Line(line);
                let (a, b) = if u1 <= u2 { (u1, u2) } else { (u2, u1) };
                Ok(Geom2dTrimmedCurve::new(basis, a, b))
            },
        }
    }

    /// Construct a segment on the given 2D line from a start point to an end
    /// parameter.
    ///
    /// Mirrors `GCE2d_MakeSegment(const gp_Lin2d& L, const gp_Pnt2d& P, Real Ulast)`.
    pub fn from_lin_point_param(lin: Lin2d, start_pt: Pnt2d, u_last: f64) -> Self {
        // Project start_pt onto the line.
        let rel = start_pt - lin.location();
        let u_first = rel.x * lin.direction().x + rel.y * lin.direction().y;
        Self::from_lin_params(lin, u_first, u_last)
    }

    /// Construct a segment on the given 2D line from a start parameter to an
    /// end point.
    ///
    /// Mirrors `GCE2d_MakeSegment(const gp_Lin2d& L, Real Ufirst, const gp_Pnt2d& P)`.
    pub fn from_lin_param_point(lin: Lin2d, u_first: f64, end_pt: Pnt2d) -> Self {
        let rel = end_pt - lin.location();
        let u_last = rel.x * lin.direction().x + rel.y * lin.direction().y;
        Self::from_lin_params(lin, u_first, u_last)
    }

    /// Returns the constructed segment, or the error.
    pub fn result(self) -> Result<Geom2dTrimmedCurve, Gce2dError> {
        self.result
    }

    /// Returns `true` if the construction succeeded.
    pub fn is_done(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns the segment (panics if construction failed).
    pub fn value(&self) -> &Geom2dTrimmedCurve {
        self.result
            .as_ref()
            .expect("Gce2dMakeSegment: construction failed")
    }
}

// ────────────────────────────── Gce2dMakeEllipse ─────────────────────────────

/// `GCE2d_MakeEllipse` — constructs a [`Geom2dEllipse`] from various 2D inputs.
///
/// Construction modes:
/// 1. A `gp_Elips2d` value.
/// 2. A `gp_Ax2d` (major-axis direction) + major and minor radii (with sense).
/// 3. A `gp_Ax22d` + major and minor radii.
/// 4. Two foci and a point on the ellipse.
/// 5. Center, X-axis direction, major radius, and minor radius (with sense).
// occt: GCE2d_MakeEllipse
pub struct Gce2dMakeEllipse {
    result: Result<Geom2dEllipse, Gce2dError>,
}

impl Gce2dMakeEllipse {
    /// Construct from a `gp_Elips2d`.
    ///
    /// Mirrors `GCE2d_MakeEllipse(const gp_Elips2d& E)`.
    pub fn from_elips2d(elips: Elips2d) -> Self {
        Self {
            result: Ok(Geom2dEllipse::from_elips2d(elips)),
        }
    }

    /// Construct from a major-axis `Ax2d`, major radius, minor radius, and
    /// orientation sense.
    ///
    /// Mirrors `GCE2d_MakeEllipse(const gp_Ax2d& MajorAxis, MajorRadius,
    /// MinorRadius, Sense)`.
    pub fn from_major_axis(
        major_axis: Ax2d,
        major_radius: f64,
        minor_radius: f64,
        sense: bool,
    ) -> Self {
        Self {
            result: Self::validate_and_build_from_ax2d(major_axis, major_radius, minor_radius, sense),
        }
    }

    fn validate_and_build_from_ax2d(
        major_axis: Ax2d,
        major_radius: f64,
        minor_radius: f64,
        sense: bool,
    ) -> Result<Geom2dEllipse, Gce2dError> {
        if minor_radius < 0.0 {
            return Err(Gce2dError::NegativeRadius);
        }
        if major_radius < minor_radius {
            return Err(Gce2dError::InvertedRadii);
        }
        Ok(Geom2dEllipse::from_major_axis(major_axis, major_radius, minor_radius, sense))
    }

    /// Construct from a full 2D axis placement (`Ax22d`), major radius, and
    /// minor radius.
    ///
    /// Mirrors `GCE2d_MakeEllipse(const gp_Ax22d& Axis, MajorRadius,
    /// MinorRadius)`.
    pub fn from_ax22d(ax: Ax22d2, major_radius: f64, minor_radius: f64) -> Self {
        Self {
            result: if minor_radius < 0.0 {
                Err(Gce2dError::NegativeRadius)
            } else if major_radius < minor_radius {
                Err(Gce2dError::InvertedRadii)
            } else {
                Ok(Geom2dEllipse::from_axis(ax, major_radius, minor_radius))
            },
        }
    }

    /// Construct an ellipse with given center, X-axis direction, major and
    /// minor radii, and orientation sense.
    ///
    /// Mirrors `GCE2d_MakeEllipse(Center, XDir, MajorRadius, MinorRadius, Sense)`.
    pub fn from_center_dir_radii(
        center: Pnt2d,
        x_dir: Pnt2d,
        major_radius: f64,
        minor_radius: f64,
        sense: bool,
    ) -> Self {
        if x_dir.norm() < CONFUSION {
            return Self { result: Err(Gce2dError::NullVector) };
        }
        let major_axis = Ax2d::new(center, x_dir);
        Self {
            result: Self::validate_and_build_from_ax2d(major_axis, major_radius, minor_radius, sense),
        }
    }

    /// Construct an ellipse from two foci `f1`, `f2` and a point `p` on the
    /// ellipse.
    ///
    /// - The major axis lies along the line `F1 F2`; the X direction points
    ///   from the midpoint toward `F1`.
    /// - The sum `|PF1| + |PF2|` equals `2 * major_radius`.
    ///
    /// Mirrors `GCE2d_MakeEllipse(F1, F2, P)`.
    pub fn from_foci_and_point(f1: Pnt2d, f2: Pnt2d, p: Pnt2d) -> Self {
        Self {
            result: Self::build_from_foci(f1, f2, p),
        }
    }

    fn build_from_foci(
        f1: Pnt2d,
        f2: Pnt2d,
        p: Pnt2d,
    ) -> Result<Geom2dEllipse, Gce2dError> {
        let center = (f1 + f2) * 0.5;
        let focal_dist = f1.distance(f2); // = 2 * c

        // Sum of distances from p to both foci = 2 * major_radius
        let sum = p.distance(f1) + p.distance(f2);
        let major_radius = sum * 0.5;

        // half focal distance c
        let c = focal_dist * 0.5;

        if major_radius < c - CONFUSION {
            // Geometrically impossible: the point is closer to the line F1F2
            // than the minor vertex, so the ellipse would be degenerate.
            return Err(Gce2dError::CollinearPoints);
        }

        let minor_sq = major_radius * major_radius - c * c;
        let minor_radius = if minor_sq > 0.0 { minor_sq.sqrt() } else { 0.0 };

        // X axis: from center toward f1.
        let x_dir = if focal_dist < CONFUSION {
            Pnt2d::new(1.0, 0.0) // coincident foci → circle; arbitrary axis
        } else {
            (f1 - center).normalized()
        };

        let major_axis = Ax2d::new(center, x_dir);
        Ok(Geom2dEllipse::from_major_axis(major_axis, major_radius, minor_radius, true))
    }

    /// Returns the constructed ellipse, or the error.
    pub fn result(self) -> Result<Geom2dEllipse, Gce2dError> {
        self.result
    }

    /// Returns `true` if the construction succeeded.
    pub fn is_done(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns the ellipse (panics if construction failed).
    pub fn value(&self) -> &Geom2dEllipse {
        self.result
            .as_ref()
            .expect("Gce2dMakeEllipse: construction failed")
    }
}

// ─────────────────────────────────── tests ───────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom2d_circle::Ax22d2;
    use std::f64::consts::{FRAC_PI_2, PI};

    const TOL: f64 = 1e-9;

    // ── Gce2dMakeLine ────────────────────────────────────────────────────────

    #[test]
    fn make_line_from_two_points() {
        let p1 = Pnt2d::new(0.0, 0.0);
        let p2 = Pnt2d::new(1.0, 0.0);
        let maker = Gce2dMakeLine::from_two_points(p1, p2);
        assert!(maker.is_done());
        let line = maker.value();
        // At u=0 the point is the origin.
        let at0 = line.eval_d0(0.0);
        assert!((at0.x - p1.x).abs() < TOL && (at0.y - p1.y).abs() < TOL);
        // At u=1 the point is on the line.
        let at1 = line.eval_d0(1.0);
        assert!((at1.x - 1.0).abs() < TOL);
    }

    #[test]
    fn make_line_coincident_fails() {
        let p = Pnt2d::new(3.0, 4.0);
        assert!(matches!(
            Gce2dMakeLine::from_two_points(p, p).result(),
            Err(Gce2dError::CoincidentPoints)
        ));
    }

    #[test]
    fn make_line_from_ax2d() {
        let ax = Ax2d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(0.0, 1.0));
        let maker = Gce2dMakeLine::from_ax2d(ax);
        assert!(maker.is_done());
        let line = maker.value();
        // At u=3 the point should be (1, 5).
        let pt = line.eval_d0(3.0);
        assert!((pt.x - 1.0).abs() < TOL && (pt.y - 5.0).abs() < TOL);
    }

    #[test]
    fn make_line_null_dir_fails() {
        assert!(matches!(
            Gce2dMakeLine::from_point_dir(Pnt2d::origin(), Pnt2d::new(0.0, 0.0)).result(),
            Err(Gce2dError::NullVector)
        ));
    }

    #[test]
    fn make_line_parallel_offset() {
        // X-axis line, offset by 2 in the +Y direction.
        let lin = Lin2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        let maker = Gce2dMakeLine::from_parallel_offset(lin, 2.0);
        assert!(maker.is_done());
        let line = maker.value();
        // The origin of the offset line should be at (0, 2).
        assert!((line.location().y - 2.0).abs() < TOL);
        // Direction is still (1, 0).
        assert!((line.direction().x - 1.0).abs() < TOL);
    }

    // ── Gce2dMakeSegment ─────────────────────────────────────────────────────

    #[test]
    fn make_segment_from_two_points() {
        let p1 = Pnt2d::new(0.0, 0.0);
        let p2 = Pnt2d::new(3.0, 4.0);
        let maker = Gce2dMakeSegment::from_two_points(p1, p2);
        assert!(maker.is_done());
        let seg = maker.value();
        // Length should be 5.
        let len = seg.last_parameter() - seg.first_parameter();
        assert!((len - 5.0).abs() < TOL);
        // Start and end points.
        let start = seg.value(seg.first_parameter());
        let end = seg.value(seg.last_parameter());
        assert!(start.distance(p1) < 1e-7);
        assert!(end.distance(p2) < 1e-7);
    }

    #[test]
    fn make_segment_coincident_fails() {
        let p = Pnt2d::new(1.0, 1.0);
        assert!(matches!(
            Gce2dMakeSegment::from_two_points(p, p).result(),
            Err(Gce2dError::CoincidentPoints)
        ));
    }

    #[test]
    fn make_segment_from_lin_params() {
        let lin = Lin2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        let maker = Gce2dMakeSegment::from_lin_params(lin, 2.0, 7.0);
        assert!(maker.is_done());
        let seg = maker.value();
        assert!((seg.first_parameter() - 2.0).abs() < TOL);
        assert!((seg.last_parameter() - 7.0).abs() < TOL);
    }

    #[test]
    fn make_segment_from_lin_point_param() {
        let lin = Lin2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        let start_pt = Pnt2d::new(3.0, 0.0); // projects to u=3 on the X line
        let maker = Gce2dMakeSegment::from_lin_point_param(lin, start_pt, 8.0);
        assert!(maker.is_done());
        let seg = maker.value();
        assert!((seg.first_parameter() - 3.0).abs() < TOL);
        assert!((seg.last_parameter() - 8.0).abs() < TOL);
    }

    // ── Gce2dMakeCircle ──────────────────────────────────────────────────────

    #[test]
    fn make_circle_from_three_points() {
        // Three points on a unit circle centred at origin.
        let p1 = Pnt2d::new(1.0, 0.0);
        let p2 = Pnt2d::new(0.0, 1.0);
        let p3 = Pnt2d::new(-1.0, 0.0);
        let maker = Gce2dMakeCircle::from_three_points(p1, p2, p3);
        assert!(maker.is_done());
        let circ = maker.value();
        assert!((circ.radius() - 1.0).abs() < 1e-7);
        assert!(circ.location().distance(Pnt2d::origin()) < 1e-7);
    }

    #[test]
    fn make_circle_from_ax2d_radius() {
        let ax = Ax2d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0));
        let maker = Gce2dMakeCircle::from_ax2d_radius(ax, 3.0, true);
        assert!(maker.is_done());
        let circ = maker.value();
        assert!((circ.radius() - 3.0).abs() < TOL);
        assert!(circ.location().distance(Pnt2d::new(1.0, 2.0)) < TOL);
    }

    #[test]
    fn make_circle_negative_radius_fails() {
        let ax = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        assert!(matches!(
            Gce2dMakeCircle::from_ax2d_radius(ax, -1.0, true).result(),
            Err(Gce2dError::NegativeRadius)
        ));
    }

    #[test]
    fn make_circle_center_and_point() {
        let center = Pnt2d::new(1.0, 1.0);
        let pt = Pnt2d::new(4.0, 1.0); // radius = 3
        let maker = Gce2dMakeCircle::from_center_and_point(center, pt);
        assert!(maker.is_done());
        let circ = maker.value();
        assert!((circ.radius() - 3.0).abs() < TOL);
    }

    // ── Gce2dMakeArcOfCircle ─────────────────────────────────────────────────

    #[test]
    fn make_arc_from_three_points() {
        let p1 = Pnt2d::new(1.0, 0.0);
        let p2 = Pnt2d::new(0.0, 1.0);
        let p3 = Pnt2d::new(-1.0, 0.0);
        let maker = Gce2dMakeArcOfCircle::from_three_points(p1, p2, p3);
        assert!(maker.is_done());
        let arc = maker.value();
        // The endpoints of the arc should be close to p1 and p3.
        let start = arc.value(arc.first_parameter());
        let end = arc.value(arc.last_parameter());
        let ok1 = start.distance(p1) < 1e-6 && end.distance(p3) < 1e-6;
        let ok2 = start.distance(p3) < 1e-6 && end.distance(p1) < 1e-6;
        assert!(ok1 || ok2, "arc endpoints: start={start:?}, end={end:?}");
    }

    #[test]
    fn make_arc_from_circ_angles() {
        // Quarter-circle arc on a unit circle.
        let circ = Circ2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 1.0, true);
        let maker = Gce2dMakeArcOfCircle::from_circ_angles(circ, 0.0, FRAC_PI_2, true);
        assert!(maker.is_done());
        let arc = maker.value();
        // Start at angle 0: (1, 0)
        let start = arc.value(arc.first_parameter());
        assert!((start.x - 1.0).abs() < 1e-7 && start.y.abs() < 1e-7);
        // End at PI/2: (0, 1)
        let end = arc.value(arc.last_parameter());
        assert!(end.x.abs() < 1e-7 && (end.y - 1.0).abs() < 1e-7);
    }

    #[test]
    fn make_arc_same_angles_fails() {
        let circ = Circ2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 1.0, true);
        assert!(matches!(
            Gce2dMakeArcOfCircle::from_circ_angles(circ, 1.0, 1.0, true).result(),
            Err(Gce2dError::BadAngles)
        ));
    }

    // ── Gce2dMakeEllipse ─────────────────────────────────────────────────────

    #[test]
    fn make_ellipse_from_major_axis() {
        let ax = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        let maker = Gce2dMakeEllipse::from_major_axis(ax, 5.0, 3.0, true);
        assert!(maker.is_done());
        let ell = maker.value();
        assert!((ell.major_radius() - 5.0).abs() < TOL);
        assert!((ell.minor_radius() - 3.0).abs() < TOL);
        // At u=0: point is (major, 0).
        let p = ell.eval_d0(0.0);
        assert!((p.x - 5.0).abs() < 1e-7 && p.y.abs() < 1e-7);
        // At u=PI/2: point is (0, minor).
        let p2 = ell.eval_d0(FRAC_PI_2);
        assert!(p2.x.abs() < 1e-7 && (p2.y - 3.0).abs() < 1e-7);
    }

    #[test]
    fn make_ellipse_inverted_radii_fails() {
        let ax = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        assert!(matches!(
            Gce2dMakeEllipse::from_major_axis(ax, 2.0, 5.0, true).result(),
            Err(Gce2dError::InvertedRadii)
        ));
    }

    #[test]
    fn make_ellipse_negative_minor_radius_fails() {
        let ax = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
        assert!(matches!(
            Gce2dMakeEllipse::from_major_axis(ax, 5.0, -1.0, true).result(),
            Err(Gce2dError::NegativeRadius)
        ));
    }

    #[test]
    fn make_ellipse_from_foci_and_point() {
        // Foci at (-3, 0) and (3, 0); point at (0, 4).
        // |PF1| = sqrt(9+16)=5, |PF2|=5 → 2a=10, a=5.
        // c=3, b=sqrt(25-9)=4.
        let f1 = Pnt2d::new(-3.0, 0.0);
        let f2 = Pnt2d::new(3.0, 0.0);
        let p = Pnt2d::new(0.0, 4.0);
        let maker = Gce2dMakeEllipse::from_foci_and_point(f1, f2, p);
        assert!(maker.is_done());
        let ell = maker.value();
        assert!((ell.major_radius() - 5.0).abs() < 1e-7);
        assert!((ell.minor_radius() - 4.0).abs() < 1e-7);
    }

    #[test]
    fn make_ellipse_from_ax22d() {
        let pos = Ax22d2::from_x_axis(Pnt2d::new(1.0, 1.0), Pnt2d::new(1.0, 0.0), true);
        let maker = Gce2dMakeEllipse::from_ax22d(pos, 4.0, 2.0);
        assert!(maker.is_done());
        let ell = maker.value();
        assert!((ell.major_radius() - 4.0).abs() < TOL);
        assert!((ell.minor_radius() - 2.0).abs() < TOL);
        assert!(ell.location().distance(Pnt2d::new(1.0, 1.0)) < TOL);
    }
}
