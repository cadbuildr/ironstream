// FILE: gc_make.rs
//! `GC` — construction algorithms for analytic curves and surfaces, mirroring
//! OpenCascade's `GC` package (`src/ModelingAlgorithms/TKGeomBase/GC`).
//!
//! Each `GC_MakeXxx` class builds a `Geom_Xxx` object from simple geometric
//! inputs (points, radii, angles, axes). Construction may fail if the inputs are
//! geometrically degenerate; errors are reported through [`GcError`] rather than
//! panics.
//!
//! # Covered classes
//! - [`GcMakeArcOfCircle`]  — `GC_MakeArcOfCircle`
//! - [`GcMakeCircle`]       — `GC_MakeCircle`
//! - [`GcMakeLine`]         — `GC_MakeLine`
//! - [`GcMakePlane`]        — `GC_MakePlane`
//! - [`GcMakeSegment`]      — `GC_MakeSegment`
//! - [`GcMakeArcOfEllipse`] — `GC_MakeArcOfEllipse`

use crate::geom_circle::GeomCircle;
use crate::geom_ellipse::GeomEllipse;
use crate::geom_line::GeomLine;
use crate::geom_plane::GeomPlane;
use crate::geom_trimmed_curve::{BasisCurve, GeomTrimmedCurve};
use crate::gp::{Ax1, Ax3, Pnt};
use crate::gp_prim::{Circ, Lin, Pln};
use crate::precision::CONFUSION;
use std::f64::consts::PI;

// ─────────────────────────────── Error type ──────────────────────────────────

/// Errors that a `GC_MakeXxx` construction may return.
// occt: GC_Root (status codes)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GcError {
    /// The two end points are coincident and do not define a direction.
    CoincidentPoints,
    /// Three or more of the input points are collinear (cannot define a circle/plane).
    CollinearPoints,
    /// The radius value is negative or too small to form the requested curve.
    NegativeRadius,
    /// The two vectors/axes are parallel (cannot define a unique plane/circle).
    ParallelVectors,
    /// The specified angle range is degenerate (start == end).
    BadAngles,
    /// An input point lies outside the curve's natural domain.
    PointNotOnCurve,
}

impl std::fmt::Display for GcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GcError::CoincidentPoints => write!(f, "GC: coincident points"),
            GcError::CollinearPoints => write!(f, "GC: collinear / coplanar points"),
            GcError::NegativeRadius => write!(f, "GC: negative or zero radius"),
            GcError::ParallelVectors => write!(f, "GC: parallel vectors / axes"),
            GcError::BadAngles => write!(f, "GC: degenerate angle range"),
            GcError::PointNotOnCurve => write!(f, "GC: point not on curve"),
        }
    }
}

// ─────────────────────────── internal helpers ────────────────────────────────

/// Signed angle of `p` on the circle defined by `ax3`, measured from the X axis
/// in `[−π, π]`.
fn angle_on_circle(center: Pnt, x_dir: Pnt, y_dir: Pnt, p: Pnt) -> f64 {
    let rel = p - center;
    let u = rel.dot(x_dir);
    let v = rel.dot(y_dir);
    v.atan2(u)
}

/// Find the circumscribed circle of three non-collinear points.
/// Returns `(center, radius)` or an error if the points are degenerate.
fn circumcircle(p1: Pnt, p2: Pnt, p3: Pnt) -> Result<(Pnt, f64), GcError> {
    // Normal of the plane containing the three points.
    let v12 = p2 - p1;
    let v13 = p3 - p1;
    let normal = v12.cross(v13);
    let n_len = normal.norm();
    if n_len < CONFUSION * CONFUSION {
        return Err(GcError::CollinearPoints);
    }
    let n = normal * (1.0 / n_len);

    // The circumcenter lies at the intersection of the perpendicular bisectors
    // of the chords P1P2 and P1P3, restricted to the plane.
    //
    // Midpoints of the two edges:
    let mid12 = (p1 + p2) * 0.5;
    let mid13 = (p1 + p3) * 0.5;

    // Bisector directions (perpendicular to the edge, in the plane):
    let d12 = n.cross(v12).normalized(); // perp to v12 in plane
    let d13 = n.cross(v13).normalized(); // perp to v13 in plane

    // Solve: mid12 + s * d12 == mid13 + t * d13
    // => s * d12 - t * d13 == mid13 - mid12
    let rhs = mid13 - mid12;

    // Least-squares / exact solve in 2-D projection onto the plane
    // Use the matrix [d12 | -d13] and solve for [s, t]:
    //   d12.dot(d12)*s - d13.dot(d12)*t = rhs.dot(d12)
    //   d12.dot(d13)*s - d13.dot(d13)*t = rhs.dot(d13)
    let a11 = d12.dot(d12);
    let a12 = -d13.dot(d12);
    let a21 = d12.dot(d13);
    let a22 = -d13.dot(d13);
    let b1 = rhs.dot(d12);
    let b2 = rhs.dot(d13);
    let det = a11 * a22 - a12 * a21;
    if det.abs() < 1e-15 {
        return Err(GcError::CollinearPoints);
    }
    let s = (b1 * a22 - b2 * a12) / det;
    let center = mid12 + d12 * s;
    let radius = center.distance(p1);
    Ok((center, radius))
}

// ─────────────────────────── GcMakeArcOfCircle ───────────────────────────────

/// `GC_MakeArcOfCircle` — constructs an arc of a circle as a
/// [`GeomTrimmedCurve`] wrapping a [`GeomCircle`].
///
/// Three construction modes are provided:
/// 1. Three points on the arc.
/// 2. A circle + two parameter angles.
/// 3. A circle + two points (with a sense flag).
// occt: GC_MakeArcOfCircle
pub struct GcMakeArcOfCircle {
    result: Result<GeomTrimmedCurve, GcError>,
}

impl GcMakeArcOfCircle {
    /// Construct the arc passing through three distinct points `p1`, `p2`, `p3`
    /// (the arc runs from `p1` to `p3` through `p2`).
    ///
    /// Mirrors `GC_MakeArcOfCircle(P1, P2, P3)`.
    pub fn from_three_points(p1: Pnt, p2: Pnt, p3: Pnt) -> Self {
        Self {
            result: Self::build_three_points(p1, p2, p3),
        }
    }

    fn build_three_points(p1: Pnt, p2: Pnt, p3: Pnt) -> Result<GeomTrimmedCurve, GcError> {
        if p1.distance(p2) < CONFUSION || p2.distance(p3) < CONFUSION || p1.distance(p3) < CONFUSION {
            return Err(GcError::CoincidentPoints);
        }
        let (center, radius) = circumcircle(p1, p2, p3)?;
        let normal = (p2 - p1).cross(p3 - p1).normalized();
        // x_hint: toward p1 from center
        let x_hint = (p1 - center).normalized();
        let ax3 = Ax3::from_origin_normal(center, normal, x_hint);
        let circ = GeomCircle::from_ax3(ax3, radius);

        // Angles measured from X axis of the circle frame.
        let u1 = angle_on_circle(center, ax3.x_dir, ax3.y_dir, p1);
        let u2 = angle_on_circle(center, ax3.x_dir, ax3.y_dir, p2);
        let u3 = angle_on_circle(center, ax3.x_dir, ax3.y_dir, p3);

        // Choose an angular range [u_start, u_end] such that u_mid is inside
        // and the arc is less than 2π.
        let u_start = u1;
        let mut u_end = u3;
        let mut u_mid = u2;

        // Normalise all angles into [u_start, u_start + 2π)
        while u_end < u_start {
            u_end += 2.0 * PI;
        }
        while u_mid < u_start {
            u_mid += 2.0 * PI;
        }
        // If mid is past end, we went the wrong way — reverse by subtracting 2π
        // from end (making it negative) and re-normalising.
        if u_mid > u_end {
            u_end -= 2.0 * PI;
        }

        let (trim_start, trim_end) = if u_end >= u_start {
            (u_start, u_end)
        } else {
            (u_end, u_start)
        };

        if (trim_end - trim_start).abs() < CONFUSION {
            return Err(GcError::BadAngles);
        }

        let basis = BasisCurve::Circle(circ);
        Ok(GeomTrimmedCurve::new(basis, trim_start, trim_end))
    }

    /// Construct an arc of the given circle between two parameter angles.
    ///
    /// Mirrors `GC_MakeArcOfCircle(Circ, Alpha1, Alpha2, Sense)`.
    /// When `sense = true` the arc runs from `alpha1` to `alpha2` in the
    /// positive (counter-clockwise) direction.
    pub fn from_circle_angles(circ: &Circ, alpha1: f64, alpha2: f64, sense: bool) -> Self {
        Self {
            result: Self::build_circle_angles(circ, alpha1, alpha2, sense),
        }
    }

    fn build_circle_angles(
        circ: &Circ,
        alpha1: f64,
        alpha2: f64,
        sense: bool,
    ) -> Result<GeomTrimmedCurve, GcError> {
        let mut a1 = alpha1;
        let mut a2 = alpha2;
        // Normalise into [0, 2π)
        let two_pi = 2.0 * PI;
        a1 = a1.rem_euclid(two_pi);
        a2 = a2.rem_euclid(two_pi);

        if (a1 - a2).abs() < CONFUSION {
            return Err(GcError::BadAngles);
        }

        // Preserve the full placement (X/Y directions) from the gp_Circ so that
        // parametric angles are measured from the correct X axis.
        let geom_circ = GeomCircle::from_ax3(circ.position(), circ.radius());
        let basis = BasisCurve::Circle(geom_circ);

        // Adjust so a2 > a1 when sense=true, a2 < a1 when sense=false
        if sense {
            if a2 < a1 {
                a2 += two_pi;
            }
        } else if a2 > a1 {
            a2 -= two_pi;
        }

        let (u1, u2) = if a1 <= a2 { (a1, a2) } else { (a2, a1) };
        Ok(GeomTrimmedCurve::new(basis, u1, u2))
    }

    /// Construct an arc of `circ` from point `p1` to point `p2`, with `sense` flag.
    ///
    /// When `sense = true` the arc runs CCW from `p1` to `p2`.
    ///
    /// Mirrors `GC_MakeArcOfCircle(Circ, P1, P2, Sense)`.
    pub fn from_circle_two_points(circ: &Circ, p1: Pnt, p2: Pnt, sense: bool) -> Self {
        let pos = circ.position();
        let alpha1 = angle_on_circle(pos.location, pos.x_dir, pos.y_dir, p1);
        let alpha2 = angle_on_circle(pos.location, pos.x_dir, pos.y_dir, p2);
        Self {
            result: Self::build_circle_angles(circ, alpha1, alpha2, sense),
        }
    }

    /// Returns the constructed arc, or the construction error.
    pub fn result(self) -> Result<GeomTrimmedCurve, GcError> {
        self.result
    }

    /// Returns `true` if the construction succeeded.
    pub fn is_done(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns the arc (panics if construction failed).
    pub fn value(&self) -> &GeomTrimmedCurve {
        self.result.as_ref().expect("GcMakeArcOfCircle: construction failed")
    }
}

// ─────────────────────────────── GcMakeCircle ────────────────────────────────

/// `GC_MakeCircle` — constructs a [`GeomCircle`] from various inputs.
///
/// Construction modes:
/// 1. A `gp_Circ` value.
/// 2. An `Ax2` placement and a radius.
/// 3. Three points on the circle.
/// 4. A center point, normal direction, and radius.
/// 5. A center point and a point on the circle.
// occt: GC_MakeCircle
pub struct GcMakeCircle {
    result: Result<GeomCircle, GcError>,
}

impl GcMakeCircle {
    /// Construct from a `gp_Circ`.
    ///
    /// Mirrors `GC_MakeCircle(const gp_Circ& C)`.
    pub fn from_circ(circ: &Circ) -> Self {
        Self {
            result: Ok(GeomCircle::from_circ(circ)),
        }
    }

    /// Construct from an axis placement and a radius.
    ///
    /// Mirrors `GC_MakeCircle(const gp_Ax2& A2, Standard_Real Radius)`.
    pub fn from_ax2_radius(ax2: Ax3, radius: f64) -> Self {
        Self {
            result: if radius < 0.0 {
                Err(GcError::NegativeRadius)
            } else {
                Ok(GeomCircle::from_ax3(ax2, radius))
            },
        }
    }

    /// Construct the circle passing through three distinct points.
    ///
    /// Mirrors `GC_MakeCircle(P1, P2, P3)`.
    pub fn from_three_points(p1: Pnt, p2: Pnt, p3: Pnt) -> Self {
        Self {
            result: Self::build_three_points(p1, p2, p3),
        }
    }

    fn build_three_points(p1: Pnt, p2: Pnt, p3: Pnt) -> Result<GeomCircle, GcError> {
        if p1.distance(p2) < CONFUSION || p2.distance(p3) < CONFUSION || p1.distance(p3) < CONFUSION {
            return Err(GcError::CoincidentPoints);
        }
        let (center, radius) = circumcircle(p1, p2, p3)?;
        let normal = (p2 - p1).cross(p3 - p1).normalized();
        let x_hint = (p1 - center).normalized();
        let ax3 = Ax3::from_origin_normal(center, normal, x_hint);
        Ok(GeomCircle::from_ax3(ax3, radius))
    }

    /// Construct the circle with given center, normal direction, and radius.
    ///
    /// Mirrors `GC_MakeCircle(const gp_Pnt& Center, const gp_Dir& Normal, Standard_Real Radius)`.
    pub fn from_center_normal_radius(center: Pnt, normal: Pnt, radius: f64) -> Self {
        Self {
            result: if radius < 0.0 {
                Err(GcError::NegativeRadius)
            } else {
                let x_hint = normal.any_perpendicular();
                let ax3 = Ax3::from_origin_normal(center, normal, x_hint);
                Ok(GeomCircle::from_ax3(ax3, radius))
            },
        }
    }

    /// Construct the circle with given center passing through a point.
    ///
    /// The circle plane is chosen with Z = Z-axis (XY plane through `center`),
    /// unless the vector `center→point` is along Z — then the plane normal is X.
    ///
    /// Mirrors `GC_MakeCircle(const gp_Pnt& Center, const gp_Pnt& Ptaxis, const gp_Pnt& Pt)`.
    pub fn from_center_and_point(center: Pnt, pt: Pnt) -> Self {
        let radius = center.distance(pt);
        if radius < CONFUSION {
            return Self { result: Err(GcError::CoincidentPoints) };
        }
        // Choose the normal as the perpendicular to the radius in the XZ or XY plane
        let radial = (pt - center).normalized();
        // Normal perpendicular to radial direction — pick a canonical plane
        let normal = radial.any_perpendicular();
        let ax3 = Ax3::from_origin_normal(center, normal, radial);
        Self {
            result: Ok(GeomCircle::from_ax3(ax3, radius)),
        }
    }

    /// Returns the constructed circle, or the error.
    pub fn result(self) -> Result<GeomCircle, GcError> {
        self.result
    }

    /// Returns `true` if the construction succeeded.
    pub fn is_done(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns the circle (panics if construction failed).
    pub fn value(&self) -> &GeomCircle {
        self.result.as_ref().expect("GcMakeCircle: construction failed")
    }
}

// ──────────────────────────────── GcMakeLine ─────────────────────────────────

/// `GC_MakeLine` — constructs a [`GeomLine`] from various inputs.
///
/// Construction modes:
/// 1. A `gp_Lin` value.
/// 2. An `Ax1` (origin + direction).
/// 3. A point and a direction.
/// 4. Two distinct points (the line passes through them).
// occt: GC_MakeLine
pub struct GcMakeLine {
    result: Result<GeomLine, GcError>,
}

impl GcMakeLine {
    /// Construct from a `gp_Lin`.
    ///
    /// Mirrors `GC_MakeLine(const gp_Lin& L)`.
    pub fn from_lin(lin: Lin) -> Self {
        Self {
            result: Ok(GeomLine::from_lin(lin)),
        }
    }

    /// Construct from an axis.
    ///
    /// Mirrors `GC_MakeLine(const gp_Ax1& A1)`.
    pub fn from_ax1(ax1: Ax1) -> Self {
        Self {
            result: Ok(GeomLine::new(ax1)),
        }
    }

    /// Construct from a point and a direction.
    ///
    /// Mirrors `GC_MakeLine(const gp_Pnt& P, const gp_Dir& V)`.
    pub fn from_point_dir(p: Pnt, dir: Pnt) -> Self {
        let n = dir.norm();
        Self {
            result: if n < CONFUSION {
                Err(GcError::CollinearPoints)
            } else {
                Ok(GeomLine::from_point_dir(p, dir))
            },
        }
    }

    /// Construct from two distinct points.
    ///
    /// Mirrors `GC_MakeLine(const gp_Pnt& P1, const gp_Pnt& P2)`.
    pub fn from_two_points(p1: Pnt, p2: Pnt) -> Self {
        let dir = p2 - p1;
        Self {
            result: if dir.norm() < CONFUSION {
                Err(GcError::CoincidentPoints)
            } else {
                Ok(GeomLine::from_point_dir(p1, dir))
            },
        }
    }

    /// Returns the constructed line, or the error.
    pub fn result(self) -> Result<GeomLine, GcError> {
        self.result
    }

    /// Returns `true` if the construction succeeded.
    pub fn is_done(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns the line (panics if construction failed).
    pub fn value(&self) -> &GeomLine {
        self.result.as_ref().expect("GcMakeLine: construction failed")
    }
}

// ──────────────────────────────── GcMakePlane ────────────────────────────────

/// `GC_MakePlane` — constructs a [`GeomPlane`] from various inputs.
///
/// Construction modes:
/// 1. A `gp_Pln` value.
/// 2. A point and a normal direction.
/// 3. Implicit-form coefficients `A*x + B*y + C*z + D = 0`.
/// 4. Three distinct non-collinear points.
/// 5. Two lines (defines a plane containing both if they are coplanar).
// occt: GC_MakePlane
pub struct GcMakePlane {
    result: Result<GeomPlane, GcError>,
}

impl GcMakePlane {
    /// Construct from a `gp_Pln`.
    ///
    /// Mirrors `GC_MakePlane(const gp_Pln& Pl)`.
    pub fn from_pln(pln: Pln) -> Self {
        Self {
            result: Ok(GeomPlane::new(pln)),
        }
    }

    /// Construct from a point and a normal.
    ///
    /// Mirrors `GC_MakePlane(const gp_Pnt& P, const gp_Dir& V)`.
    pub fn from_point_normal(p: Pnt, normal: Pnt) -> Self {
        Self {
            result: if normal.norm() < CONFUSION {
                Err(GcError::CollinearPoints)
            } else {
                Ok(GeomPlane::from_point_normal(p, normal))
            },
        }
    }

    /// Construct from implicit-form coefficients `A*x + B*y + C*z + D = 0`.
    ///
    /// Mirrors `GC_MakePlane(Standard_Real A, B, C, D)`.
    pub fn from_coefficients(a: f64, b: f64, c: f64, d: f64) -> Self {
        let n = Pnt::new(a, b, c);
        Self {
            result: if n.norm() < CONFUSION {
                Err(GcError::CollinearPoints)
            } else {
                Ok(GeomPlane::from_coefficients(a, b, c, d))
            },
        }
    }

    /// Construct from three distinct non-collinear points.
    ///
    /// Mirrors `GC_MakePlane(const gp_Pnt& P1, P2, P3)`.
    pub fn from_three_points(p1: Pnt, p2: Pnt, p3: Pnt) -> Self {
        Self {
            result: Self::build_three_points(p1, p2, p3),
        }
    }

    fn build_three_points(p1: Pnt, p2: Pnt, p3: Pnt) -> Result<GeomPlane, GcError> {
        if p1.distance(p2) < CONFUSION || p1.distance(p3) < CONFUSION || p2.distance(p3) < CONFUSION {
            return Err(GcError::CoincidentPoints);
        }
        let v12 = p2 - p1;
        let v13 = p3 - p1;
        let normal = v12.cross(v13);
        if normal.norm() < CONFUSION * CONFUSION {
            return Err(GcError::CollinearPoints);
        }
        Ok(GeomPlane::from_point_normal(p1, normal))
    }

    /// Construct from an `Ax3` local coordinate system.
    ///
    /// Mirrors `GC_MakePlane(const gp_Ax3& A3)`.
    pub fn from_ax3(ax3: Ax3) -> Self {
        Self {
            result: Ok(GeomPlane::from_ax3(ax3)),
        }
    }

    /// Returns the constructed plane, or the error.
    pub fn result(self) -> Result<GeomPlane, GcError> {
        self.result
    }

    /// Returns `true` if the construction succeeded.
    pub fn is_done(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns the plane (panics if construction failed).
    pub fn value(&self) -> &GeomPlane {
        self.result.as_ref().expect("GcMakePlane: construction failed")
    }
}

// ─────────────────────────────── GcMakeSegment ───────────────────────────────

/// `GC_MakeSegment` — constructs a bounded straight-line segment as a
/// [`GeomTrimmedCurve`] wrapping a [`GeomLine`].
///
/// Construction modes:
/// 1. Two distinct points.
/// 2. A line + two parameter values.
/// 3. A line + a start point + an end parameter.
/// 4. A line + a start parameter + an end point.
// occt: GC_MakeSegment
pub struct GcMakeSegment {
    result: Result<GeomTrimmedCurve, GcError>,
}

impl GcMakeSegment {
    /// Construct the segment from `p1` to `p2`.
    ///
    /// Mirrors `GC_MakeSegment(const gp_Pnt& P1, const gp_Pnt& P2)`.
    pub fn from_two_points(p1: Pnt, p2: Pnt) -> Self {
        let dir = p2 - p1;
        let len = dir.norm();
        Self {
            result: if len < CONFUSION {
                Err(GcError::CoincidentPoints)
            } else {
                let line = GeomLine::from_point_dir(p1, dir);
                let basis = BasisCurve::Line(line);
                // Parameters: 0 to arc-length. Direction is normalised inside
                // GeomLine, so parameter == arc-length.
                Ok(GeomTrimmedCurve::new(basis, 0.0, len))
            },
        }
    }

    /// Construct a segment on the given line between parameters `u1` and `u2`.
    ///
    /// Mirrors `GC_MakeSegment(const gp_Lin& L, Standard_Real U1, Standard_Real U2)`.
    pub fn from_line_params(lin: Lin, u1: f64, u2: f64) -> Self {
        Self {
            result: if (u2 - u1).abs() < CONFUSION {
                Err(GcError::CoincidentPoints)
            } else {
                let line = GeomLine::from_lin(lin);
                let basis = BasisCurve::Line(line);
                let (a, b) = if u1 <= u2 { (u1, u2) } else { (u2, u1) };
                Ok(GeomTrimmedCurve::new(basis, a, b))
            },
        }
    }

    /// Construct a segment on the given line from a point to a parameter value.
    ///
    /// Mirrors `GC_MakeSegment(const gp_Lin& L, const gp_Pnt& Point, Standard_Real Ulast)`.
    pub fn from_line_point_param(lin: Lin, start_pt: Pnt, u_last: f64) -> Self {
        // Project start_pt onto the line to get u_first.
        let dir = lin.direction();
        let rel = start_pt - lin.location();
        let u_first = rel.dot(dir);
        Self::from_line_params(lin, u_first, u_last)
    }

    /// Construct a segment on the given line from a parameter value to an end point.
    ///
    /// Mirrors `GC_MakeSegment(const gp_Lin& L, Standard_Real Ufirst, const gp_Pnt& Point)`.
    pub fn from_line_param_point(lin: Lin, u_first: f64, end_pt: Pnt) -> Self {
        let dir = lin.direction();
        let rel = end_pt - lin.location();
        let u_last = rel.dot(dir);
        Self::from_line_params(lin, u_first, u_last)
    }

    /// Returns the constructed segment, or the error.
    pub fn result(self) -> Result<GeomTrimmedCurve, GcError> {
        self.result
    }

    /// Returns `true` if the construction succeeded.
    pub fn is_done(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns the segment (panics if construction failed).
    pub fn value(&self) -> &GeomTrimmedCurve {
        self.result.as_ref().expect("GcMakeSegment: construction failed")
    }
}

// ─────────────────────────── GcMakeArcOfEllipse ──────────────────────────────

/// `GC_MakeArcOfEllipse` — constructs an arc of an ellipse as a
/// [`GeomTrimmedCurve`] wrapping a [`GeomEllipse`].
///
/// Construction modes:
/// 1. An ellipse + two parameter angles (with sense flag).
/// 2. An ellipse + two points on the ellipse (with sense flag).
/// 3. An ellipse + a start point + an end angle.
/// 4. An ellipse + a start angle + an end point.
// occt: GC_MakeArcOfEllipse
pub struct GcMakeArcOfEllipse {
    result: Result<GeomTrimmedCurve, GcError>,
}

impl GcMakeArcOfEllipse {
    /// Construct an arc of `elips` between parameter angles `alpha1` and `alpha2`.
    ///
    /// When `sense = true` the arc runs from `alpha1` to `alpha2` in the positive
    /// (CCW) direction.
    ///
    /// Mirrors `GC_MakeArcOfEllipse(Elips, Alpha1, Alpha2, Sense)`.
    pub fn from_ellipse_angles(
        elips: &GeomEllipse,
        alpha1: f64,
        alpha2: f64,
        sense: bool,
    ) -> Self {
        Self {
            result: Self::build_from_angles(elips, alpha1, alpha2, sense),
        }
    }

    fn build_from_angles(
        elips: &GeomEllipse,
        alpha1: f64,
        alpha2: f64,
        sense: bool,
    ) -> Result<GeomTrimmedCurve, GcError> {
        let two_pi = 2.0 * PI;
        let mut a1 = alpha1.rem_euclid(two_pi);
        let mut a2 = alpha2.rem_euclid(two_pi);

        if (a1 - a2).abs() < CONFUSION {
            return Err(GcError::BadAngles);
        }

        if sense {
            if a2 < a1 {
                a2 += two_pi;
            }
        } else if a2 > a1 {
            a2 -= two_pi;
        }

        let (u1, u2) = if a1 <= a2 { (a1, a2) } else { (a2, a1) };
        if (u2 - u1).abs() < CONFUSION {
            return Err(GcError::BadAngles);
        }

        let basis = BasisCurve::Ellipse(*elips);
        Ok(GeomTrimmedCurve::new(basis, u1, u2))
    }

    /// Construct an arc of `elips` from point `p1` to point `p2`.
    ///
    /// `sense = true` means the shorter CCW arc.
    ///
    /// Mirrors `GC_MakeArcOfEllipse(Elips, P1, P2, Sense)`.
    pub fn from_ellipse_points(elips: &GeomEllipse, p1: Pnt, p2: Pnt, sense: bool) -> Self {
        let pos = elips.position();
        let alpha1 = angle_on_circle(pos.location, pos.x_dir, pos.y_dir, p1);
        let alpha2 = angle_on_circle(pos.location, pos.x_dir, pos.y_dir, p2);
        Self {
            result: Self::build_from_angles(elips, alpha1, alpha2, sense),
        }
    }

    /// Construct an arc from a start point to an end angle.
    ///
    /// Mirrors `GC_MakeArcOfEllipse(Elips, P1, Alpha2, Sense)`.
    pub fn from_ellipse_point_angle(elips: &GeomEllipse, p1: Pnt, alpha2: f64, sense: bool) -> Self {
        let pos = elips.position();
        let alpha1 = angle_on_circle(pos.location, pos.x_dir, pos.y_dir, p1);
        Self {
            result: Self::build_from_angles(elips, alpha1, alpha2, sense),
        }
    }

    /// Construct an arc from a start angle to an end point.
    ///
    /// Mirrors `GC_MakeArcOfEllipse(Elips, Alpha1, P2, Sense)`.
    pub fn from_ellipse_angle_point(elips: &GeomEllipse, alpha1: f64, p2: Pnt, sense: bool) -> Self {
        let pos = elips.position();
        let alpha2 = angle_on_circle(pos.location, pos.x_dir, pos.y_dir, p2);
        Self {
            result: Self::build_from_angles(elips, alpha1, alpha2, sense),
        }
    }

    /// Returns the constructed arc, or the error.
    pub fn result(self) -> Result<GeomTrimmedCurve, GcError> {
        self.result
    }

    /// Returns `true` if the construction succeeded.
    pub fn is_done(&self) -> bool {
        self.result.is_ok()
    }

    /// Returns the arc (panics if construction failed).
    pub fn value(&self) -> &GeomTrimmedCurve {
        self.result.as_ref().expect("GcMakeArcOfEllipse: construction failed")
    }
}

// ─────────────────────────────────── tests ───────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    const TOL: f64 = 1e-9;

    // ── GcMakeLine ──

    #[test]
    fn make_line_from_two_points() {
        let p1 = Pnt::new(0.0, 0.0, 0.0);
        let p2 = Pnt::new(1.0, 0.0, 0.0);
        let maker = GcMakeLine::from_two_points(p1, p2);
        assert!(maker.is_done());
        let line = maker.value();
        // The line should pass through p1 and p2.
        let at0 = line.d0(0.0);
        assert!((at0.x - p1.x).abs() < TOL);
        assert!((at0.y - p1.y).abs() < TOL);
        let at1 = line.d0(1.0);
        assert!((at1.x - 1.0).abs() < TOL);
    }

    #[test]
    fn make_line_coincident_fails() {
        let p = Pnt::new(1.0, 2.0, 3.0);
        let r = GcMakeLine::from_two_points(p, p).result();
        assert!(matches!(r, Err(GcError::CoincidentPoints)));
    }

    #[test]
    fn make_line_from_ax1() {
        let ax = Ax1::new(Pnt::new(0.0, 1.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
        let maker = GcMakeLine::from_ax1(ax);
        assert!(maker.is_done());
        let line = maker.value();
        // At u=2 the point is origin + 2*Z_dir
        let pt = line.d0(2.0);
        assert!((pt.z - 2.0).abs() < TOL);
        assert!((pt.y - 1.0).abs() < TOL);
    }

    // ── GcMakeSegment ──

    #[test]
    fn make_segment_from_two_points() {
        let p1 = Pnt::new(0.0, 0.0, 0.0);
        let p2 = Pnt::new(3.0, 4.0, 0.0);
        let maker = GcMakeSegment::from_two_points(p1, p2);
        assert!(maker.is_done());
        let seg = maker.value();
        // Check the endpoints match.
        let start = seg.value(seg.first_parameter());
        let end = seg.value(seg.last_parameter());
        assert!(start.distance(p1) < 1e-7);
        assert!(end.distance(p2) < 1e-7);
        // Length = 5
        let expected_len = 5.0;
        let len = seg.last_parameter() - seg.first_parameter();
        assert!((len - expected_len).abs() < TOL);
    }

    #[test]
    fn make_segment_coincident_fails() {
        let p = Pnt::new(0.0, 0.0, 0.0);
        let r = GcMakeSegment::from_two_points(p, p).result();
        assert!(matches!(r, Err(GcError::CoincidentPoints)));
    }

    #[test]
    fn make_segment_from_line_params() {
        let lin = Lin::new_pd(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
        let maker = GcMakeSegment::from_line_params(lin, 2.0, 5.0);
        assert!(maker.is_done());
        let seg = maker.value();
        assert!((seg.first_parameter() - 2.0).abs() < TOL);
        assert!((seg.last_parameter() - 5.0).abs() < TOL);
    }

    // ── GcMakeCircle ──

    #[test]
    fn make_circle_from_three_points() {
        // Three points on the unit circle in the XY plane.
        let p1 = Pnt::new(1.0, 0.0, 0.0);
        let p2 = Pnt::new(0.0, 1.0, 0.0);
        let p3 = Pnt::new(-1.0, 0.0, 0.0);
        let maker = GcMakeCircle::from_three_points(p1, p2, p3);
        assert!(maker.is_done());
        let circ = maker.value();
        assert!((circ.radius() - 1.0).abs() < 1e-7);
        assert!(circ.location().distance(Pnt::origin()) < 1e-7);
    }

    #[test]
    fn make_circle_from_ax2_radius() {
        let ax3 = Ax3::identity();
        let maker = GcMakeCircle::from_ax2_radius(ax3, 5.0);
        assert!(maker.is_done());
        assert!((maker.value().radius() - 5.0).abs() < TOL);
    }

    #[test]
    fn make_circle_negative_radius_fails() {
        let ax3 = Ax3::identity();
        let r = GcMakeCircle::from_ax2_radius(ax3, -1.0).result();
        assert!(matches!(r, Err(GcError::NegativeRadius)));
    }

    // ── GcMakePlane ──

    #[test]
    fn make_plane_from_point_normal() {
        let p = Pnt::new(0.0, 0.0, 3.0);
        let n = Pnt::new(0.0, 0.0, 1.0);
        let maker = GcMakePlane::from_point_normal(p, n);
        assert!(maker.is_done());
        let plane = maker.value();
        // The plane should contain the origin point.
        assert!(plane.pln().contains(p, 1e-7));
        // Normal should be (0,0,1).
        let (a, b, c, _d) = plane.coefficients();
        assert!(a.abs() < TOL);
        assert!(b.abs() < TOL);
        assert!((c - 1.0).abs() < TOL);
    }

    #[test]
    fn make_plane_from_three_points() {
        let p1 = Pnt::new(0.0, 0.0, 0.0);
        let p2 = Pnt::new(1.0, 0.0, 0.0);
        let p3 = Pnt::new(0.0, 1.0, 0.0);
        let maker = GcMakePlane::from_three_points(p1, p2, p3);
        assert!(maker.is_done());
        let plane = maker.value();
        // All three points lie on the plane.
        assert!(plane.pln().contains(p1, 1e-7));
        assert!(plane.pln().contains(p2, 1e-7));
        assert!(plane.pln().contains(p3, 1e-7));
    }

    // ── GcMakeArcOfCircle ──

    #[test]
    fn make_arc_from_three_points() {
        // Arc on unit circle through three points.
        let p1 = Pnt::new(1.0, 0.0, 0.0);
        let p2 = Pnt::new(0.0, 1.0, 0.0);
        let p3 = Pnt::new(-1.0, 0.0, 0.0);
        let maker = GcMakeArcOfCircle::from_three_points(p1, p2, p3);
        assert!(maker.is_done());
        let arc = maker.value();
        // End points of the arc should be close to p1 and p3.
        let start = arc.value(arc.first_parameter());
        let end = arc.value(arc.last_parameter());
        // Either start≈p1/end≈p3 or reversed.
        let ok1 = start.distance(p1) < 1e-6 && end.distance(p3) < 1e-6;
        let ok2 = start.distance(p3) < 1e-6 && end.distance(p1) < 1e-6;
        assert!(ok1 || ok2, "arc endpoints mismatch: start={:?}, end={:?}", start, end);
    }

    #[test]
    fn make_arc_from_circle_angles() {
        let ax3 = Ax3::identity();
        let circ_prim = Circ::new(ax3, 2.0);
        let maker = GcMakeArcOfCircle::from_circle_angles(&circ_prim, 0.0, FRAC_PI_2, true);
        assert!(maker.is_done());
        let arc = maker.value();
        // Start at angle 0 on radius-2 circle: (2,0,0)
        let start = arc.value(arc.first_parameter());
        assert!((start.x - 2.0).abs() < 1e-7);
        assert!(start.y.abs() < 1e-7);
        // End at angle π/2: (0,2,0)
        let end = arc.value(arc.last_parameter());
        assert!(end.x.abs() < 1e-7);
        assert!((end.y - 2.0).abs() < 1e-7);
    }

    // ── GcMakeArcOfEllipse ──

    #[test]
    fn make_arc_of_ellipse_angles() {
        let ax3 = Ax3::identity();
        let ellipse = GeomEllipse::from_ax3(ax3, 3.0, 2.0);
        let maker = GcMakeArcOfEllipse::from_ellipse_angles(&ellipse, 0.0, FRAC_PI_2, true);
        assert!(maker.is_done());
        let arc = maker.value();
        // At angle 0: P = (3, 0, 0)
        let start = arc.value(arc.first_parameter());
        assert!((start.x - 3.0).abs() < 1e-7);
        assert!(start.y.abs() < 1e-7);
        // At angle π/2: P = (0, 2, 0)
        let end = arc.value(arc.last_parameter());
        assert!(end.x.abs() < 1e-7);
        assert!((end.y - 2.0).abs() < 1e-7);
    }

    #[test]
    fn make_arc_of_ellipse_same_angles_fails() {
        let ax3 = Ax3::identity();
        let ellipse = GeomEllipse::from_ax3(ax3, 3.0, 2.0);
        let r = GcMakeArcOfEllipse::from_ellipse_angles(&ellipse, 1.0, 1.0, true).result();
        assert!(matches!(r, Err(GcError::BadAngles)));
    }
}
