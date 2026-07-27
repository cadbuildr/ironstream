// FILE: geom_adaptor_curve.rs
// occt: GeomAdaptor_Curve

use crate::geom_circle::GeomCircle;
use crate::geom_ellipse::GeomEllipse;
use crate::geom_line::GeomLine;
use crate::geom_bspline_curve::GeomBSplineCurve;
use crate::gp_prim::Circ;
use crate::geom_abs::CurveType;
#[allow(non_camel_case_types)]
type GeomAbs_CurveType = CurveType;

/// Parameters for a trimmed adaptor curve.
// occt: GeomAdaptor_Curve // (trimmed variant)
#[derive(Debug)]
pub struct TrimmedParams {
    pub inner: Box<GeomAdaptorCurveKind>,
    pub u1: f64,
    pub u2: f64,
}

/// The kind of underlying curve being adapted.
// occt: GeomAdaptor_Curve
#[derive(Debug)]
pub enum GeomAdaptorCurveKind {
    Line(GeomLine),
    Circle(GeomCircle),
    Ellipse(GeomEllipse),
    BSpline(GeomBSplineCurve),
    Trimmed(TrimmedParams),
    /// Raw line data produced by surface iso-curve extraction.
    RawLine { origin: [f64; 3], direction: [f64; 3] },
    /// Raw circle data produced by surface iso-curve extraction.
    RawCircle { center: [f64; 3], x_axis: [f64; 3], y_axis: [f64; 3], radius: f64 },
    /// Raw BSpline curve from surface iso extraction (bsplines::BSplineCurve<Pnt>).
    RawBSpline(crate::bsplines::BSplineCurve<crate::gp::Pnt>),
}

/// Adapts a 3D parametric Geom_Curve to a uniform interface.
// occt: GeomAdaptor_Curve
pub struct GeomAdaptorCurve {
    pub kind: GeomAdaptorCurveKind,
    pub first: f64,
    pub last: f64,
}

impl GeomAdaptorCurve {
    /// Adapt a line over the parameter interval [u1, u2].
    pub fn from_line(line: GeomLine, u1: f64, u2: f64) -> Self {
        GeomAdaptorCurve {
            kind: GeomAdaptorCurveKind::Line(line),
            first: u1,
            last: u2,
        }
    }

    /// Adapt a circle over the parameter interval [u1, u2].
    pub fn from_circle(c: GeomCircle, u1: f64, u2: f64) -> Self {
        GeomAdaptorCurve {
            kind: GeomAdaptorCurveKind::Circle(c),
            first: u1,
            last: u2,
        }
    }

    /// Adapt an ellipse over the parameter interval [u1, u2].
    pub fn from_ellipse(e: GeomEllipse, u1: f64, u2: f64) -> Self {
        GeomAdaptorCurve {
            kind: GeomAdaptorCurveKind::Ellipse(e),
            first: u1,
            last: u2,
        }
    }

    /// Adapt a BSpline curve (uses the curve's own parameter range).
    pub fn from_bspline(b: GeomBSplineCurve) -> Self {
        let first = b.first_parameter();
        let last = b.last_parameter();
        GeomAdaptorCurve {
            kind: GeomAdaptorCurveKind::BSpline(b),
            first,
            last,
        }
    }

    /// Wrap an existing adaptor as a trimmed curve over [u1, u2].
    pub fn from_trimmed(inner_adaptor: GeomAdaptorCurve, u1: f64, u2: f64) -> Self {
        GeomAdaptorCurve {
            kind: GeomAdaptorCurveKind::Trimmed(TrimmedParams {
                inner: Box::new(inner_adaptor.kind),
                u1,
                u2,
            }),
            first: u1,
            last: u2,
        }
    }

    /// Returns the abstract curve type tag.
    pub fn curve_type(&self) -> GeomAbs_CurveType {
        kind_curve_type(&self.kind)
    }

    /// First parameter of the valid range.
    pub fn first_parameter(&self) -> f64 {
        self.first
    }

    /// Last parameter of the valid range.
    pub fn last_parameter(&self) -> f64 {
        self.last
    }

    /// Evaluate position at parameter u.
    pub fn value(&self, u: f64) -> [f64; 3] {
        kind_value(&self.kind, u)
    }

    /// Evaluate position and first derivative at parameter u.
    pub fn d1(&self, u: f64) -> ([f64; 3], [f64; 3]) {
        kind_d1(&self.kind, u)
    }

    /// Evaluate position, first and second derivatives at parameter u.
    pub fn d2(&self, u: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        kind_d2(&self.kind, u)
    }

    /// Returns true if the curve is closed (start and end points coincide).
    pub fn is_closed(&self) -> bool {
        kind_is_closed(&self.kind)
    }

    /// Returns true if the curve is periodic.
    pub fn is_periodic(&self) -> bool {
        kind_is_periodic(&self.kind)
    }

    /// Returns the period of the curve. Panics if not periodic.
    pub fn period(&self) -> f64 {
        kind_period(&self.kind)
    }

    /// Returns the underlying Circ if the curve is a circle.
    pub fn circle(&self) -> Option<Circ> {
        match &self.kind {
            GeomAdaptorCurveKind::Circle(c) => Some(c.circ()),
            GeomAdaptorCurveKind::Trimmed(t) => match t.inner.as_ref() {
                GeomAdaptorCurveKind::Circle(c) => Some(c.circ()),
                _ => None,
            },
            _ => None,
        }
    }

    /// Returns a reference to the underlying GeomEllipse if the curve is an ellipse.
    pub fn ellipse(&self) -> Option<&GeomEllipse> {
        match &self.kind {
            GeomAdaptorCurveKind::Ellipse(e) => Some(e),
            GeomAdaptorCurveKind::Trimmed(t) => match t.inner.as_ref() {
                GeomAdaptorCurveKind::Ellipse(e) => Some(e),
                _ => None,
            },
            _ => None,
        }
    }

    /// Returns a reference to the underlying GeomLine if the curve is a line.
    pub fn line(&self) -> Option<&GeomLine> {
        match &self.kind {
            GeomAdaptorCurveKind::Line(l) => Some(l),
            GeomAdaptorCurveKind::Trimmed(t) => match t.inner.as_ref() {
                GeomAdaptorCurveKind::Line(l) => Some(l),
                _ => None,
            },
            _ => None,
        }
    }

    /// Returns a reference to the underlying GeomBSplineCurve if the curve is a BSpline.
    pub fn bspline(&self) -> Option<&GeomBSplineCurve> {
        match &self.kind {
            GeomAdaptorCurveKind::BSpline(b) => Some(b),
            GeomAdaptorCurveKind::Trimmed(t) => match t.inner.as_ref() {
                GeomAdaptorCurveKind::BSpline(b) => Some(b),
                _ => None,
            },
            _ => None,
        }
    }
}

// ---- helpers operating on GeomAdaptorCurveKind --------------------------------

fn kind_curve_type(kind: &GeomAdaptorCurveKind) -> GeomAbs_CurveType {
    match kind {
        GeomAdaptorCurveKind::Line(_) => GeomAbs_CurveType::Line,
        GeomAdaptorCurveKind::Circle(_) => GeomAbs_CurveType::Circle,
        GeomAdaptorCurveKind::Ellipse(_) => GeomAbs_CurveType::Ellipse,
        GeomAdaptorCurveKind::BSpline(_) | GeomAdaptorCurveKind::RawBSpline(_) => GeomAbs_CurveType::BSplineCurve,
        GeomAdaptorCurveKind::Trimmed(t) => kind_curve_type(&t.inner),
        GeomAdaptorCurveKind::RawLine { .. } => GeomAbs_CurveType::Line,
        GeomAdaptorCurveKind::RawCircle { .. } => GeomAbs_CurveType::Circle,
    }
}

fn kind_is_periodic(kind: &GeomAdaptorCurveKind) -> bool {
    match kind {
        GeomAdaptorCurveKind::Line(_) => false,
        GeomAdaptorCurveKind::Circle(_) => true,
        GeomAdaptorCurveKind::Ellipse(_) => true,
        GeomAdaptorCurveKind::BSpline(b) => b.is_periodic(),
        GeomAdaptorCurveKind::RawBSpline(_) => false,
        GeomAdaptorCurveKind::Trimmed(_) => false,
        GeomAdaptorCurveKind::RawLine { .. } => false,
        GeomAdaptorCurveKind::RawCircle { .. } => true,
    }
}

fn kind_period(kind: &GeomAdaptorCurveKind) -> f64 {
    use core::f64::consts::PI;
    match kind {
        GeomAdaptorCurveKind::Circle(_) => 2.0 * PI,
        GeomAdaptorCurveKind::Ellipse(_) => 2.0 * PI,
        GeomAdaptorCurveKind::RawCircle { .. } => 2.0 * PI,
        _ => panic!("GeomAdaptorCurve::period called on non-periodic curve"),
    }
}

fn kind_is_closed(kind: &GeomAdaptorCurveKind) -> bool {
    match kind {
        GeomAdaptorCurveKind::Circle(_) => true,
        GeomAdaptorCurveKind::Ellipse(_) => true,
        GeomAdaptorCurveKind::BSpline(b) => b.is_closed(),
        GeomAdaptorCurveKind::RawBSpline(_) => false,
        GeomAdaptorCurveKind::Line(_) => false,
        GeomAdaptorCurveKind::RawLine { .. } => false,
        GeomAdaptorCurveKind::RawCircle { .. } => true,
        GeomAdaptorCurveKind::Trimmed(t) => {
            // closed only if inner curve is periodic and trimmed interval spans a full period
            if kind_is_periodic(&t.inner) {
                let p = kind_period(&t.inner);
                (t.u2 - t.u1 - p).abs() < 1e-7
            } else {
                false
            }
        }
    }
}

fn kind_value(kind: &GeomAdaptorCurveKind, u: f64) -> [f64; 3] {
    match kind {
        GeomAdaptorCurveKind::Line(l) => line_value(l, u),
        GeomAdaptorCurveKind::Circle(c) => circle_value(c, u),
        GeomAdaptorCurveKind::Ellipse(e) => ellipse_value(e, u),
        GeomAdaptorCurveKind::BSpline(b) => {
            let p = b.value(u);
            [p.x, p.y, p.z]
        }
        GeomAdaptorCurveKind::RawBSpline(b) => {
            let p = b.value(u);
            [p.x, p.y, p.z]
        }
        GeomAdaptorCurveKind::Trimmed(t) => kind_value(&t.inner, u),
        GeomAdaptorCurveKind::RawLine { origin, direction } => raw_line_value(*origin, *direction, u),
        GeomAdaptorCurveKind::RawCircle { center, x_axis, y_axis, radius } => {
            raw_circle_value(*center, *x_axis, *y_axis, *radius, u)
        }
    }
}

fn kind_d1(kind: &GeomAdaptorCurveKind, u: f64) -> ([f64; 3], [f64; 3]) {
    match kind {
        GeomAdaptorCurveKind::Line(l) => line_d1(l, u),
        GeomAdaptorCurveKind::Circle(c) => circle_d1(c, u),
        GeomAdaptorCurveKind::Ellipse(e) => ellipse_d1(e, u),
        GeomAdaptorCurveKind::BSpline(b) => {
            let (p, v) = b.d1(u);
            ([p.x, p.y, p.z], [v.x, v.y, v.z])
        }
        GeomAdaptorCurveKind::RawBSpline(b) => {
            let p = b.value(u);
            let v = b.d1(u);
            ([p.x, p.y, p.z], [v.x, v.y, v.z])
        }
        GeomAdaptorCurveKind::Trimmed(t) => kind_d1(&t.inner, u),
        GeomAdaptorCurveKind::RawLine { origin, direction } => {
            let p = raw_line_value(*origin, *direction, u);
            (p, *direction)
        }
        GeomAdaptorCurveKind::RawCircle { center, x_axis, y_axis, radius } => {
            raw_circle_d1(*center, *x_axis, *y_axis, *radius, u)
        }
    }
}

fn kind_d2(kind: &GeomAdaptorCurveKind, u: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
    match kind {
        GeomAdaptorCurveKind::Line(l) => line_d2(l, u),
        GeomAdaptorCurveKind::Circle(c) => circle_d2(c, u),
        GeomAdaptorCurveKind::Ellipse(e) => ellipse_d2(e, u),
        GeomAdaptorCurveKind::BSpline(b) => {
            let (p, v1, v2) = b.d2(u);
            ([p.x, p.y, p.z], [v1.x, v1.y, v1.z], [v2.x, v2.y, v2.z])
        }
        GeomAdaptorCurveKind::RawBSpline(b) => {
            // BSplineCurve<Pnt>::d1 only gives the first derivative; no d2 available.
            let p = b.value(u);
            let v1 = b.d1(u);
            ([p.x, p.y, p.z], [v1.x, v1.y, v1.z], [0.0, 0.0, 0.0])
        }
        GeomAdaptorCurveKind::Trimmed(t) => kind_d2(&t.inner, u),
        GeomAdaptorCurveKind::RawLine { origin, direction } => {
            let p = raw_line_value(*origin, *direction, u);
            (p, *direction, [0.0, 0.0, 0.0])
        }
        GeomAdaptorCurveKind::RawCircle { center, x_axis, y_axis, radius } => {
            raw_circle_d2(*center, *x_axis, *y_axis, *radius, u)
        }
    }
}

// ---- Line evaluation ----------------------------------------------------------
// A line is P(u) = Location + u * Direction

fn line_value(l: &GeomLine, u: f64) -> [f64; 3] {
    let pos = l.position(); // Ax1: location + direction fields
    let loc = pos.location;
    let dir = pos.direction;
    [
        loc.x + u * dir.x,
        loc.y + u * dir.y,
        loc.z + u * dir.z,
    ]
}

fn line_d1(l: &GeomLine, u: f64) -> ([f64; 3], [f64; 3]) {
    let p = line_value(l, u);
    let dir = l.position().direction;
    (p, [dir.x, dir.y, dir.z])
}

fn line_d2(l: &GeomLine, u: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let (p, d1) = line_d1(l, u);
    (p, d1, [0.0, 0.0, 0.0])
}

fn raw_line_value(origin: [f64; 3], direction: [f64; 3], u: f64) -> [f64; 3] {
    [
        origin[0] + u * direction[0],
        origin[1] + u * direction[1],
        origin[2] + u * direction[2],
    ]
}

// ---- Circle evaluation --------------------------------------------------------
// Circle: P(u) = Center + R*(cos(u)*XDir + sin(u)*YDir)

fn circle_value(c: &GeomCircle, u: f64) -> [f64; 3] {
    let circ = c.circ();
    let center = circ.location(); // returns Pnt
    let ax = circ.position(); // Ax2/Ax3 — use field access
    let xdir = ax.x_dir;
    let ydir = ax.y_dir;
    let r = circ.radius();
    let cu = u.cos();
    let su = u.sin();
    [
        center.x + r * (cu * xdir.x + su * ydir.x),
        center.y + r * (cu * xdir.y + su * ydir.y),
        center.z + r * (cu * xdir.z + su * ydir.z),
    ]
}

fn circle_d1(c: &GeomCircle, u: f64) -> ([f64; 3], [f64; 3]) {
    let p = circle_value(c, u);
    let circ = c.circ();
    let ax = circ.position();
    let xdir = ax.x_dir;
    let ydir = ax.y_dir;
    let r = circ.radius();
    let cu = u.cos();
    let su = u.sin();
    // dP/du = R*(-sin(u)*XDir + cos(u)*YDir)
    let d1 = [
        r * (-su * xdir.x + cu * ydir.x),
        r * (-su * xdir.y + cu * ydir.y),
        r * (-su * xdir.z + cu * ydir.z),
    ];
    (p, d1)
}

fn circle_d2(c: &GeomCircle, u: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let (p, d1) = circle_d1(c, u);
    let circ = c.circ();
    let ax = circ.position();
    let xdir = ax.x_dir;
    let ydir = ax.y_dir;
    let r = circ.radius();
    let cu = u.cos();
    let su = u.sin();
    // d²P/du² = R*(-cos(u)*XDir - sin(u)*YDir)
    let d2 = [
        r * (-cu * xdir.x - su * ydir.x),
        r * (-cu * xdir.y - su * ydir.y),
        r * (-cu * xdir.z - su * ydir.z),
    ];
    (p, d1, d2)
}

fn raw_circle_value(center: [f64; 3], x_axis: [f64; 3], y_axis: [f64; 3], radius: f64, u: f64) -> [f64; 3] {
    let cu = u.cos();
    let su = u.sin();
    [
        center[0] + radius * (cu * x_axis[0] + su * y_axis[0]),
        center[1] + radius * (cu * x_axis[1] + su * y_axis[1]),
        center[2] + radius * (cu * x_axis[2] + su * y_axis[2]),
    ]
}

fn raw_circle_d1(center: [f64; 3], x_axis: [f64; 3], y_axis: [f64; 3], radius: f64, u: f64) -> ([f64; 3], [f64; 3]) {
    let p = raw_circle_value(center, x_axis, y_axis, radius, u);
    let cu = u.cos();
    let su = u.sin();
    let d1 = [
        radius * (-su * x_axis[0] + cu * y_axis[0]),
        radius * (-su * x_axis[1] + cu * y_axis[1]),
        radius * (-su * x_axis[2] + cu * y_axis[2]),
    ];
    (p, d1)
}

fn raw_circle_d2(center: [f64; 3], x_axis: [f64; 3], y_axis: [f64; 3], radius: f64, u: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let (p, d1) = raw_circle_d1(center, x_axis, y_axis, radius, u);
    let cu = u.cos();
    let su = u.sin();
    let d2 = [
        radius * (-cu * x_axis[0] - su * y_axis[0]),
        radius * (-cu * x_axis[1] - su * y_axis[1]),
        radius * (-cu * x_axis[2] - su * y_axis[2]),
    ];
    (p, d1, d2)
}

// ---- Ellipse evaluation -------------------------------------------------------
// Ellipse: P(u) = Center + a*cos(u)*XDir + b*sin(u)*YDir

fn ellipse_value(e: &GeomEllipse, u: f64) -> [f64; 3] {
    let a = e.major_radius();
    let b = e.minor_radius();
    let ax = e.position(); // Ax3 — use field access
    let center = ax.location;
    let xdir = ax.x_dir;
    let ydir = ax.y_dir;
    let cu = u.cos();
    let su = u.sin();
    [
        center.x + a * cu * xdir.x + b * su * ydir.x,
        center.y + a * cu * xdir.y + b * su * ydir.y,
        center.z + a * cu * xdir.z + b * su * ydir.z,
    ]
}

fn ellipse_d1(e: &GeomEllipse, u: f64) -> ([f64; 3], [f64; 3]) {
    let p = ellipse_value(e, u);
    let a = e.major_radius();
    let b = e.minor_radius();
    let ax = e.position();
    let xdir = ax.x_dir;
    let ydir = ax.y_dir;
    let cu = u.cos();
    let su = u.sin();
    let d1 = [
        -a * su * xdir.x + b * cu * ydir.x,
        -a * su * xdir.y + b * cu * ydir.y,
        -a * su * xdir.z + b * cu * ydir.z,
    ];
    (p, d1)
}

fn ellipse_d2(e: &GeomEllipse, u: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let (p, d1) = ellipse_d1(e, u);
    let a = e.major_radius();
    let b = e.minor_radius();
    let ax = e.position();
    let xdir = ax.x_dir;
    let ydir = ax.y_dir;
    let cu = u.cos();
    let su = u.sin();
    let d2 = [
        -a * cu * xdir.x - b * su * ydir.x,
        -a * cu * xdir.y - b * su * ydir.y,
        -a * cu * xdir.z - b * su * ydir.z,
    ];
    (p, d1, d2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::PI;
    use crate::geom_circle::GeomCircle;
    use crate::geom_ellipse::GeomEllipse;
    use crate::geom_line::GeomLine;
    use crate::gp_prim::Circ;
    use crate::gp::{Ax1, Ax2, Ax3, Pnt};
    use crate::geom_abs::CurveType as GeomAbs_CurveType;

    fn make_xy_circle(radius: f64) -> GeomCircle {
        // Circle in the XY plane centred at origin
        let ax2 = Ax2::from_origin_normal(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        GeomCircle::from_ax3(ax2, radius)
    }

    fn make_xy_ellipse(a: f64, b: f64) -> GeomEllipse {
        let ax2 = Ax2::from_origin_normal(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        GeomEllipse::from_ax3(ax2, a, b)
    }

    fn make_line_along_x() -> GeomLine {
        // Line through origin along +X
        let ax1 = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        GeomLine::new(ax1)
    }

    // ---- circle tests ---------------------------------------------------------

    #[test]
    fn circle_curve_type() {
        let c = make_xy_circle(3.0);
        let adaptor = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        assert!(matches!(adaptor.curve_type(), GeomAbs_CurveType::Circle));
    }

    #[test]
    fn circle_value_at_zero() {
        let r = 5.0;
        let c = make_xy_circle(r);
        let adaptor = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let p = adaptor.value(0.0);
        // At u=0: (r, 0, 0)
        assert!((p[0] - r).abs() < 1e-12, "x={}", p[0]);
        assert!(p[1].abs() < 1e-12, "y={}", p[1]);
        assert!(p[2].abs() < 1e-12, "z={}", p[2]);
    }

    #[test]
    fn circle_value_at_pi() {
        let r = 5.0;
        let c = make_xy_circle(r);
        let adaptor = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let p = adaptor.value(PI);
        // At u=PI: (-r, 0, 0)
        assert!((p[0] + r).abs() < 1e-12, "x={}", p[0]);
        assert!(p[1].abs() < 1e-12, "y={}", p[1]);
        assert!(p[2].abs() < 1e-12, "z={}", p[2]);
    }

    #[test]
    fn circle_value_at_half_pi() {
        let r = 4.0;
        let c = make_xy_circle(r);
        let adaptor = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let p = adaptor.value(PI / 2.0);
        // At u=PI/2: (0, r, 0)
        assert!(p[0].abs() < 1e-12, "x={}", p[0]);
        assert!((p[1] - r).abs() < 1e-12, "y={}", p[1]);
        assert!(p[2].abs() < 1e-12, "z={}", p[2]);
    }

    #[test]
    fn circle_d1_at_zero() {
        let r = 3.0;
        let c = make_xy_circle(r);
        let adaptor = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let (p, d1) = adaptor.d1(0.0);
        // P(0) = (r,0,0), P'(0) = (0,r,0)
        assert!((p[0] - r).abs() < 1e-12);
        assert!(p[1].abs() < 1e-12);
        assert!(d1[0].abs() < 1e-12, "d1x={}", d1[0]);
        assert!((d1[1] - r).abs() < 1e-12, "d1y={}", d1[1]);
        assert!(d1[2].abs() < 1e-12);
    }

    #[test]
    fn circle_d2_at_zero() {
        let r = 3.0;
        let c = make_xy_circle(r);
        let adaptor = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let (_, _, d2) = adaptor.d2(0.0);
        // P''(0) = (-r, 0, 0)
        assert!((d2[0] + r).abs() < 1e-12, "d2x={}", d2[0]);
        assert!(d2[1].abs() < 1e-12);
        assert!(d2[2].abs() < 1e-12);
    }

    #[test]
    fn circle_is_periodic_and_closed() {
        let c = make_xy_circle(1.0);
        let adaptor = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        assert!(adaptor.is_periodic());
        assert!(adaptor.is_closed());
    }

    #[test]
    fn circle_period() {
        let c = make_xy_circle(1.0);
        let adaptor = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        assert!((adaptor.period() - 2.0 * PI).abs() < 1e-12);
    }

    #[test]
    fn circle_accessor() {
        let r = 7.0;
        let c = make_xy_circle(r);
        let adaptor = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let circ = adaptor.circle().expect("should return circle");
        assert!((circ.radius() - r).abs() < 1e-12);
    }

    #[test]
    fn circle_parameter_range() {
        let c = make_xy_circle(1.0);
        let adaptor = GeomAdaptorCurve::from_circle(c, 0.5, 3.0);
        assert!((adaptor.first_parameter() - 0.5).abs() < 1e-15);
        assert!((adaptor.last_parameter() - 3.0).abs() < 1e-15);
    }

    // ---- ellipse tests --------------------------------------------------------

    #[test]
    fn ellipse_curve_type() {
        let e = make_xy_ellipse(5.0, 3.0);
        let adaptor = GeomAdaptorCurve::from_ellipse(e, 0.0, 2.0 * PI);
        assert!(matches!(adaptor.curve_type(), GeomAbs_CurveType::Ellipse));
    }

    #[test]
    fn ellipse_value_at_zero() {
        let a = 5.0;
        let b = 3.0;
        let e = make_xy_ellipse(a, b);
        let adaptor = GeomAdaptorCurve::from_ellipse(e, 0.0, 2.0 * PI);
        let p = adaptor.value(0.0);
        // At u=0: (a, 0, 0)
        assert!((p[0] - a).abs() < 1e-12, "x={}", p[0]);
        assert!(p[1].abs() < 1e-12);
        assert!(p[2].abs() < 1e-12);
    }

    #[test]
    fn ellipse_value_at_half_pi() {
        let a = 5.0;
        let b = 3.0;
        let e = make_xy_ellipse(a, b);
        let adaptor = GeomAdaptorCurve::from_ellipse(e, 0.0, 2.0 * PI);
        let p = adaptor.value(PI / 2.0);
        // At u=PI/2: (0, b, 0)
        assert!(p[0].abs() < 1e-12, "x={}", p[0]);
        assert!((p[1] - b).abs() < 1e-12, "y={}", p[1]);
        assert!(p[2].abs() < 1e-12);
    }

    #[test]
    fn ellipse_d1_at_zero() {
        let a = 5.0;
        let b = 3.0;
        let e = make_xy_ellipse(a, b);
        let adaptor = GeomAdaptorCurve::from_ellipse(e, 0.0, 2.0 * PI);
        let (_, d1) = adaptor.d1(0.0);
        // P'(0) = (0, b, 0)
        assert!(d1[0].abs() < 1e-12);
        assert!((d1[1] - b).abs() < 1e-12, "d1y={}", d1[1]);
        assert!(d1[2].abs() < 1e-12);
    }

    #[test]
    fn ellipse_d2_at_zero() {
        let a = 5.0;
        let b = 3.0;
        let e = make_xy_ellipse(a, b);
        let adaptor = GeomAdaptorCurve::from_ellipse(e, 0.0, 2.0 * PI);
        let (_, _, d2) = adaptor.d2(0.0);
        // P''(0) = (-a, 0, 0)
        assert!((d2[0] + a).abs() < 1e-12, "d2x={}", d2[0]);
        assert!(d2[1].abs() < 1e-12);
        assert!(d2[2].abs() < 1e-12);
    }

    #[test]
    fn ellipse_is_periodic() {
        let e = make_xy_ellipse(5.0, 3.0);
        let adaptor = GeomAdaptorCurve::from_ellipse(e, 0.0, 2.0 * PI);
        assert!(adaptor.is_periodic());
        assert!((adaptor.period() - 2.0 * PI).abs() < 1e-12);
    }

    #[test]
    fn ellipse_accessor() {
        let a = 6.0;
        let b = 2.0;
        let e = make_xy_ellipse(a, b);
        let adaptor = GeomAdaptorCurve::from_ellipse(e, 0.0, 2.0 * PI);
        let ell = adaptor.ellipse().expect("should return ellipse");
        assert!((ell.major_radius() - a).abs() < 1e-12);
        assert!((ell.minor_radius() - b).abs() < 1e-12);
    }

    // ---- line tests -----------------------------------------------------------

    #[test]
    fn line_curve_type() {
        let l = make_line_along_x();
        let adaptor = GeomAdaptorCurve::from_line(l, 0.0, 10.0);
        assert!(matches!(adaptor.curve_type(), GeomAbs_CurveType::Line));
    }

    #[test]
    fn line_value() {
        let l = make_line_along_x();
        let adaptor = GeomAdaptorCurve::from_line(l, 0.0, 10.0);
        let p = adaptor.value(3.5);
        assert!((p[0] - 3.5).abs() < 1e-12);
        assert!(p[1].abs() < 1e-12);
        assert!(p[2].abs() < 1e-12);
    }

    #[test]
    fn line_d1() {
        let l = make_line_along_x();
        let adaptor = GeomAdaptorCurve::from_line(l, 0.0, 10.0);
        let (p, d1) = adaptor.d1(2.0);
        assert!((p[0] - 2.0).abs() < 1e-12);
        // tangent is (1,0,0)
        assert!((d1[0] - 1.0).abs() < 1e-12);
        assert!(d1[1].abs() < 1e-12);
        assert!(d1[2].abs() < 1e-12);
    }

    #[test]
    fn line_d2_is_zero() {
        let l = make_line_along_x();
        let adaptor = GeomAdaptorCurve::from_line(l, 0.0, 10.0);
        let (_, _, d2) = adaptor.d2(5.0);
        assert!(d2[0].abs() < 1e-15);
        assert!(d2[1].abs() < 1e-15);
        assert!(d2[2].abs() < 1e-15);
    }

    #[test]
    fn line_not_closed_not_periodic() {
        let l = make_line_along_x();
        let adaptor = GeomAdaptorCurve::from_line(l, 0.0, 10.0);
        assert!(!adaptor.is_closed());
        assert!(!adaptor.is_periodic());
    }

    #[test]
    fn line_accessor() {
        let l = make_line_along_x();
        let adaptor = GeomAdaptorCurve::from_line(l, 0.0, 5.0);
        let line_ref = adaptor.line().expect("should return line");
        let dir = line_ref.position().direction; // field access, not method
        assert!((dir.x - 1.0).abs() < 1e-12);
        assert!(dir.y.abs() < 1e-12);
    }

    // ---- trimmed tests --------------------------------------------------------

    #[test]
    fn trimmed_circle_curve_type() {
        let c = make_xy_circle(2.0);
        let inner = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let trimmed = GeomAdaptorCurve::from_trimmed(inner, 0.0, PI);
        // Trimmed delegates to inner type
        assert!(matches!(trimmed.curve_type(), GeomAbs_CurveType::Circle));
    }

    #[test]
    fn trimmed_circle_parameter_range() {
        let c = make_xy_circle(2.0);
        let inner = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let trimmed = GeomAdaptorCurve::from_trimmed(inner, 0.0, PI);
        assert!((trimmed.first_parameter() - 0.0).abs() < 1e-15);
        assert!((trimmed.last_parameter() - PI).abs() < 1e-15);
    }

    #[test]
    fn trimmed_circle_not_closed_partial() {
        let c = make_xy_circle(2.0);
        let inner = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let trimmed = GeomAdaptorCurve::from_trimmed(inner, 0.0, PI);
        // Only half circle — not closed
        assert!(!trimmed.is_closed());
    }

    #[test]
    fn trimmed_circle_not_periodic() {
        let c = make_xy_circle(2.0);
        let inner = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let trimmed = GeomAdaptorCurve::from_trimmed(inner, 0.0, PI);
        assert!(!trimmed.is_periodic());
    }

    #[test]
    fn trimmed_circle_value_at_pi() {
        let r = 2.0;
        let c = make_xy_circle(r);
        let inner = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let trimmed = GeomAdaptorCurve::from_trimmed(inner, 0.0, PI);
        let p = trimmed.value(PI);
        assert!((p[0] + r).abs() < 1e-12);
        assert!(p[1].abs() < 1e-12);
    }

    #[test]
    fn trimmed_circle_accessor() {
        let r = 9.0;
        let c = make_xy_circle(r);
        let inner = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let trimmed = GeomAdaptorCurve::from_trimmed(inner, 0.0, PI);
        let circ = trimmed.circle().expect("should return circle from trimmed");
        assert!((circ.radius() - r).abs() < 1e-12);
    }

    // ---- numerical consistency checks -----------------------------------------

    #[test]
    fn circle_d1_consistent_with_finite_difference() {
        let r = 3.0;
        let c = make_xy_circle(r);
        let adaptor = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let u = 1.1;
        let h = 1e-6;
        let (_, d1) = adaptor.d1(u);
        let p_fwd = adaptor.value(u + h);
        let p_bwd = adaptor.value(u - h);
        let fd = [
            (p_fwd[0] - p_bwd[0]) / (2.0 * h),
            (p_fwd[1] - p_bwd[1]) / (2.0 * h),
            (p_fwd[2] - p_bwd[2]) / (2.0 * h),
        ];
        assert!((d1[0] - fd[0]).abs() < 1e-9, "dx err={}", (d1[0]-fd[0]).abs());
        assert!((d1[1] - fd[1]).abs() < 1e-9, "dy err={}", (d1[1]-fd[1]).abs());
    }

    #[test]
    fn ellipse_d2_consistent_with_finite_difference() {
        let a = 4.0;
        let b = 2.0;
        let e = make_xy_ellipse(a, b);
        let adaptor = GeomAdaptorCurve::from_ellipse(e, 0.0, 2.0 * PI);
        let u = 0.8;
        let h = 1e-5;
        let (_, _, d2) = adaptor.d2(u);
        let (_, d1_fwd) = adaptor.d1(u + h);
        let (_, d1_bwd) = adaptor.d1(u - h);
        let fd2 = [
            (d1_fwd[0] - d1_bwd[0]) / (2.0 * h),
            (d1_fwd[1] - d1_bwd[1]) / (2.0 * h),
            (d1_fwd[2] - d1_bwd[2]) / (2.0 * h),
        ];
        assert!((d2[0] - fd2[0]).abs() < 1e-8, "dx2 err={}", (d2[0]-fd2[0]).abs());
        assert!((d2[1] - fd2[1]).abs() < 1e-8, "dy2 err={}", (d2[1]-fd2[1]).abs());
    }

    #[test]
    fn circle_with_offset_center() {
        // Circle centred at (1, 2, 3)
        let ax2 = Ax2::from_origin_normal(
            Pnt::new(1.0, 2.0, 3.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let c = GeomCircle::from_ax3(ax2, 5.0);
        let adaptor = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        let p = adaptor.value(0.0);
        assert!((p[0] - 6.0).abs() < 1e-12, "x={}", p[0]);
        assert!((p[1] - 2.0).abs() < 1e-12, "y={}", p[1]);
        assert!((p[2] - 3.0).abs() < 1e-12, "z={}", p[2]);
    }

    #[test]
    fn wrong_accessor_returns_none() {
        let c = make_xy_circle(1.0);
        let adaptor = GeomAdaptorCurve::from_circle(c, 0.0, 2.0 * PI);
        assert!(adaptor.line().is_none());
        assert!(adaptor.ellipse().is_none());
        assert!(adaptor.bspline().is_none());
    }
}
