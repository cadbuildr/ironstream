// FILE: brep_adaptor.rs
//! `BRepAdaptor` — edge/face/comp-curve adapters that wrap topological shapes
//! into the uniform parametric curve/surface interface.
//!
//! These types mirror OpenCascade's `BRepAdaptor_Curve`,
//! `BRepAdaptor_Surface`, and `BRepAdaptor_CompCurve` and expose the same
//! evaluation methods (`value`, `d1`, `d2`, `is_closed`, `is_periodic`, …) as
//! the `GeomAdaptor*` family, but work directly on topological data rather than
//! on raw `Geom_*` objects.
//!
//! Because IronStream's `topods` module stores geometry as tessellated meshes
//! (not exact analytic data), the adapters here reconstruct an analytic
//! parametric representation from the shape data and delegate evaluation to
//! internal helpers that mirror OCCT's algorithms exactly.

use std::f64::consts::PI;

use crate::gp::{Ax1, Ax3, Pnt, Vec3};
use crate::gp_prim::Circ;
use crate::geom_abs::{CurveType, SurfaceType, Shape as ContinuityShape};
use crate::geom_bspline_curve::GeomBSplineCurve;
use crate::geom_bspline_surface::GeomBSplineSurface;
use crate::bnd_box::BndBox;

// ---------------------------------------------------------------------------
// Small local helpers (avoid pulling in geom_adaptor_surface helpers)
// ---------------------------------------------------------------------------

#[inline]
fn pnt_add(a: Pnt, b: Pnt) -> Pnt { a + b }
#[inline]
fn pnt_scale(a: Pnt, s: f64) -> Pnt { a * s }
#[inline]
fn pnt_dot(a: Pnt, b: Pnt) -> f64 { a.dot(b) }
#[inline]
fn pnt_norm(a: Pnt) -> f64 { a.norm() }
#[inline]
fn pnt_normalized(a: Pnt) -> Pnt { a.normalized() }

// ---------------------------------------------------------------------------
// BRepAdaptor_Curve — adapts a TopoDS_Edge to a parametric 3-D curve
// ---------------------------------------------------------------------------

/// The kind of analytic curve underlying a `BRepAdaptorCurve`.
// occt: BRepAdaptor_Curve (curve kind discriminant)
#[derive(Debug, Clone)]
pub enum BRepCurveKind {
    /// A line: defined by an `Ax1` (origin + unit direction).
    Line(Ax1),
    /// A full circle (or arc): placement `Ax3` + radius.
    Circle { placement: Ax3, radius: f64 },
    /// An ellipse: placement `Ax3`, semi-major `a`, semi-minor `b`.
    Ellipse { placement: Ax3, a: f64, b: f64 },
    /// A BSpline curve.
    BSpline(GeomBSplineCurve),
    /// A generic polyline stored as an ordered list of 3-D points.  Used when
    /// the exact analytic type is not available; evaluation uses linear
    /// interpolation between sample points.
    Polyline(Vec<Pnt>),
}

/// `BRepAdaptor_Curve` — adapts a topological edge to the standard parametric
/// curve interface used by algorithms such as discretisers, intersectors and
/// curve length integrators.
///
/// The parametric range is `[first, last]` and may be a trimmed sub-range of
/// the underlying curve's natural domain.
// occt: BRepAdaptor_Curve
#[derive(Debug, Clone)]
pub struct BRepAdaptorCurve {
    kind: BRepCurveKind,
    /// First parameter of the valid range.
    pub first: f64,
    /// Last parameter of the valid range.
    pub last: f64,
    /// Positional tolerance inherited from the edge.
    pub tolerance: f64,
}

impl BRepAdaptorCurve {
    // ------------------------------------------------------------------
    // Constructors
    // ------------------------------------------------------------------

    /// Construct from a line (infinite or trimmed to `[first, last]`).
    pub fn from_line(axis: Ax1, first: f64, last: f64) -> Self {
        BRepAdaptorCurve {
            kind: BRepCurveKind::Line(axis),
            first,
            last,
            tolerance: 1e-7,
        }
    }

    /// Construct from a circle arc.  `first` and `last` are angular parameters
    /// in radians; a full circle uses `[0, 2π]`.
    pub fn from_circle(placement: Ax3, radius: f64, first: f64, last: f64) -> Self {
        BRepAdaptorCurve {
            kind: BRepCurveKind::Circle { placement, radius },
            first,
            last,
            tolerance: 1e-7,
        }
    }

    /// Construct from an ellipse arc.  `a` is the semi-major radius, `b` the
    /// semi-minor radius.  The angular parameter range is `[first, last]`.
    pub fn from_ellipse(placement: Ax3, a: f64, b: f64, first: f64, last: f64) -> Self {
        BRepAdaptorCurve {
            kind: BRepCurveKind::Ellipse { placement, a, b },
            first,
            last,
            tolerance: 1e-7,
        }
    }

    /// Construct from a B-spline curve.  The parameter range is taken from the
    /// curve's own domain if `first > last`; otherwise the supplied values are
    /// used as a trimmed sub-range.
    pub fn from_bspline(curve: GeomBSplineCurve, first: f64, last: f64) -> Self {
        let (f, l) = if first <= last {
            (first, last)
        } else {
            (curve.first_parameter(), curve.last_parameter())
        };
        BRepAdaptorCurve {
            kind: BRepCurveKind::BSpline(curve),
            first: f,
            last: l,
            tolerance: 1e-7,
        }
    }

    /// Construct from a polyline (e.g. from a tessellated edge).  The parameter
    /// is the cumulative chord-length, which is computed from the point sequence.
    pub fn from_polyline(pts: Vec<Pnt>) -> Self {
        let total = polyline_length(&pts);
        BRepAdaptorCurve {
            kind: BRepCurveKind::Polyline(pts),
            first: 0.0,
            last: total,
            tolerance: 1e-7,
        }
    }

    // ------------------------------------------------------------------
    // Accessors
    // ------------------------------------------------------------------

    /// Returns the abstract curve type.
    // occt: BRepAdaptor_Curve::GetType()
    pub fn get_type(&self) -> CurveType {
        match &self.kind {
            BRepCurveKind::Line(_) => CurveType::Line,
            BRepCurveKind::Circle { .. } => CurveType::Circle,
            BRepCurveKind::Ellipse { .. } => CurveType::Ellipse,
            BRepCurveKind::BSpline(_) => CurveType::BSplineCurve,
            BRepCurveKind::Polyline(_) => CurveType::OtherCurve,
        }
    }

    /// First parameter of the valid range.
    pub fn first_parameter(&self) -> f64 { self.first }

    /// Last parameter of the valid range.
    pub fn last_parameter(&self) -> f64 { self.last }

    /// Geometric continuity of the underlying curve.
    pub fn continuity(&self) -> ContinuityShape {
        match &self.kind {
            BRepCurveKind::Line(_) => ContinuityShape::CN,
            BRepCurveKind::Circle { .. } => ContinuityShape::CN,
            BRepCurveKind::Ellipse { .. } => ContinuityShape::CN,
            BRepCurveKind::BSpline(_) => ContinuityShape::C1,
            BRepCurveKind::Polyline(_) => ContinuityShape::C0,
        }
    }

    /// Returns `true` if the curve is closed (start and end points coincide).
    pub fn is_closed(&self) -> bool {
        match &self.kind {
            BRepCurveKind::Circle { .. } => {
                (self.last - self.first - 2.0 * PI).abs() < 1e-9
            }
            BRepCurveKind::Ellipse { .. } => {
                (self.last - self.first - 2.0 * PI).abs() < 1e-9
            }
            BRepCurveKind::BSpline(b) => b.is_closed(),
            BRepCurveKind::Polyline(pts) => {
                pts.len() >= 2 && pts[0].distance(*pts.last().unwrap()) < 1e-7
            }
            BRepCurveKind::Line(_) => false,
        }
    }

    /// Returns `true` if the curve is periodic.
    pub fn is_periodic(&self) -> bool {
        match &self.kind {
            BRepCurveKind::Circle { .. } => true,
            BRepCurveKind::Ellipse { .. } => true,
            BRepCurveKind::BSpline(b) => b.is_periodic(),
            _ => false,
        }
    }

    /// Returns the period (panics if not periodic).
    pub fn period(&self) -> f64 {
        match &self.kind {
            BRepCurveKind::Circle { .. } | BRepCurveKind::Ellipse { .. } => 2.0 * PI,
            BRepCurveKind::BSpline(b) if b.is_periodic() => {
                b.last_parameter() - b.first_parameter()
            }
            _ => panic!("BRepAdaptorCurve::period called on non-periodic curve"),
        }
    }

    // ------------------------------------------------------------------
    // Evaluation
    // ------------------------------------------------------------------

    /// `D0` — evaluate point at parameter `u`.
    // occt: BRepAdaptor_Curve::D0
    pub fn value(&self, u: f64) -> Pnt {
        match &self.kind {
            BRepCurveKind::Line(ax) => {
                ax.location + ax.direction * u
            }
            BRepCurveKind::Circle { placement, radius } => {
                circle_point(placement, *radius, u)
            }
            BRepCurveKind::Ellipse { placement, a, b } => {
                ellipse_point(placement, *a, *b, u)
            }
            BRepCurveKind::BSpline(bs) => bs.value(u),
            BRepCurveKind::Polyline(pts) => polyline_value(pts, u),
        }
    }

    /// `D1` — evaluate point and first derivative at parameter `u`.
    // occt: BRepAdaptor_Curve::D1
    pub fn d1(&self, u: f64) -> (Pnt, Vec3) {
        match &self.kind {
            BRepCurveKind::Line(ax) => {
                let p = ax.location + ax.direction * u;
                (p, ax.direction)
            }
            BRepCurveKind::Circle { placement, radius } => {
                let p = circle_point(placement, *radius, u);
                let d = circle_d1(placement, *radius, u);
                (p, d)
            }
            BRepCurveKind::Ellipse { placement, a, b } => {
                let p = ellipse_point(placement, *a, *b, u);
                let d = ellipse_d1(placement, *a, *b, u);
                (p, d)
            }
            BRepCurveKind::BSpline(bs) => bs.d1(u),
            BRepCurveKind::Polyline(pts) => {
                let p = polyline_value(pts, u);
                let d = polyline_d1(pts, u);
                (p, d)
            }
        }
    }

    /// `D2` — evaluate point, first and second derivatives at parameter `u`.
    // occt: BRepAdaptor_Curve::D2
    pub fn d2(&self, u: f64) -> (Pnt, Vec3, Vec3) {
        match &self.kind {
            BRepCurveKind::Line(ax) => {
                let p = ax.location + ax.direction * u;
                (p, ax.direction, Pnt::origin())
            }
            BRepCurveKind::Circle { placement, radius } => {
                let p = circle_point(placement, *radius, u);
                let d1 = circle_d1(placement, *radius, u);
                let d2 = circle_d2(placement, *radius, u);
                (p, d1, d2)
            }
            BRepCurveKind::Ellipse { placement, a, b } => {
                let p = ellipse_point(placement, *a, *b, u);
                let d1 = ellipse_d1(placement, *a, *b, u);
                let d2 = ellipse_d2(placement, *a, *b, u);
                (p, d1, d2)
            }
            BRepCurveKind::BSpline(bs) => bs.d2(u),
            BRepCurveKind::Polyline(pts) => {
                let p = polyline_value(pts, u);
                let d1 = polyline_d1(pts, u);
                (p, d1, Pnt::origin())
            }
        }
    }

    /// Approximate arc length by Gauss-Legendre 5-point quadrature.
    pub fn length(&self) -> f64 {
        curve_length(self, self.first, self.last)
    }

    /// Approximate arc length on the sub-range `[u1, u2]`.
    pub fn length_range(&self, u1: f64, u2: f64) -> f64 {
        curve_length(self, u1, u2)
    }

    /// Compute an axis-aligned bounding box by sampling the curve.
    pub fn bounding_box(&self) -> BndBox {
        let n = 64usize;
        let mut bb = BndBox::new();
        for i in 0..=n {
            let t = self.first + (self.last - self.first) * (i as f64 / n as f64);
            bb.add_point(self.value(t));
        }
        bb
    }

    /// Access underlying `Circ` if this is a circle curve.
    pub fn circle(&self) -> Option<Circ> {
        match &self.kind {
            BRepCurveKind::Circle { placement, radius } => {
                Some(Circ::new(*placement, *radius))
            }
            _ => None,
        }
    }

    /// Access underlying line axis if this is a line curve.
    pub fn line(&self) -> Option<Ax1> {
        match &self.kind {
            BRepCurveKind::Line(ax) => Some(*ax),
            _ => None,
        }
    }

    /// Access underlying BSpline if present.
    pub fn bspline(&self) -> Option<&GeomBSplineCurve> {
        match &self.kind {
            BRepCurveKind::BSpline(b) => Some(b),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// BRepAdaptor_Surface — adapts a TopoDS_Face to a parametric 3-D surface
// ---------------------------------------------------------------------------

/// The kind of analytic surface underlying a `BRepAdaptorSurface`.
// occt: BRepAdaptor_Surface (surface kind discriminant)
#[derive(Debug, Clone)]
pub enum BRepSurfaceKind {
    /// An infinite plane: normal (`z_dir`) and local frame.
    Plane(Ax3),
    /// A cylinder: axis frame + radius.  U is the angular parameter [0, 2π),
    /// V is the axial parameter.
    Cylinder { frame: Ax3, radius: f64 },
    /// A cone: axis frame + base radius + half-angle (radians).
    Cone { frame: Ax3, radius: f64, half_angle: f64 },
    /// A sphere: center + axis frame + radius.
    Sphere { frame: Ax3, radius: f64 },
    /// A torus: center + axis frame + major radius + minor radius.
    Torus { frame: Ax3, major_radius: f64, minor_radius: f64 },
    /// A BSpline surface.
    BSpline(GeomBSplineSurface),
    /// A generic triangulated surface.  Not present in OCCT's list but needed
    /// for fully-tessellated inputs.
    Other,
}

/// `BRepAdaptor_Surface` — adapts a topological face to the standard
/// parametric surface interface.
///
/// The parametric bounds `[u_min, u_max] × [v_min, v_max]` are the face's
/// trimmed domain and may be smaller than the underlying surface's natural
/// domain.
// occt: BRepAdaptor_Surface
#[derive(Debug, Clone)]
pub struct BRepAdaptorSurface {
    kind: BRepSurfaceKind,
    pub u_min: f64,
    pub u_max: f64,
    pub v_min: f64,
    pub v_max: f64,
    pub tolerance: f64,
}

impl BRepAdaptorSurface {
    // ------------------------------------------------------------------
    // Constructors
    // ------------------------------------------------------------------

    /// Construct from a plane.
    pub fn from_plane(ax3: Ax3, u_min: f64, u_max: f64, v_min: f64, v_max: f64) -> Self {
        BRepAdaptorSurface {
            kind: BRepSurfaceKind::Plane(ax3),
            u_min, u_max, v_min, v_max,
            tolerance: 1e-7,
        }
    }

    /// Construct from a cylinder.
    pub fn from_cylinder(
        frame: Ax3,
        radius: f64,
        u_min: f64, u_max: f64,
        v_min: f64, v_max: f64,
    ) -> Self {
        BRepAdaptorSurface {
            kind: BRepSurfaceKind::Cylinder { frame, radius },
            u_min, u_max, v_min, v_max,
            tolerance: 1e-7,
        }
    }

    /// Construct from a cone.
    pub fn from_cone(
        frame: Ax3,
        radius: f64,
        half_angle: f64,
        u_min: f64, u_max: f64,
        v_min: f64, v_max: f64,
    ) -> Self {
        BRepAdaptorSurface {
            kind: BRepSurfaceKind::Cone { frame, radius, half_angle },
            u_min, u_max, v_min, v_max,
            tolerance: 1e-7,
        }
    }

    /// Construct from a sphere.
    pub fn from_sphere(
        frame: Ax3,
        radius: f64,
        u_min: f64, u_max: f64,
        v_min: f64, v_max: f64,
    ) -> Self {
        BRepAdaptorSurface {
            kind: BRepSurfaceKind::Sphere { frame, radius },
            u_min, u_max, v_min, v_max,
            tolerance: 1e-7,
        }
    }

    /// Construct from a torus.
    pub fn from_torus(
        frame: Ax3,
        major_radius: f64,
        minor_radius: f64,
        u_min: f64, u_max: f64,
        v_min: f64, v_max: f64,
    ) -> Self {
        BRepAdaptorSurface {
            kind: BRepSurfaceKind::Torus { frame, major_radius, minor_radius },
            u_min, u_max, v_min, v_max,
            tolerance: 1e-7,
        }
    }

    /// Construct from a BSpline surface.
    pub fn from_bspline(surface: GeomBSplineSurface) -> Self {
        let u_min = surface.u_knot(surface.first_u_knot_index());
        let u_max = surface.u_knot(surface.last_u_knot_index());
        let v_min = surface.v_knot(surface.first_v_knot_index());
        let v_max = surface.v_knot(surface.last_v_knot_index());
        BRepAdaptorSurface {
            kind: BRepSurfaceKind::BSpline(surface),
            u_min, u_max, v_min, v_max,
            tolerance: 1e-7,
        }
    }

    // ------------------------------------------------------------------
    // Accessors
    // ------------------------------------------------------------------

    /// Returns the abstract surface type.
    // occt: BRepAdaptor_Surface::GetType()
    pub fn get_type(&self) -> SurfaceType {
        match &self.kind {
            BRepSurfaceKind::Plane(_) => SurfaceType::Plane,
            BRepSurfaceKind::Cylinder { .. } => SurfaceType::Cylinder,
            BRepSurfaceKind::Cone { .. } => SurfaceType::Cone,
            BRepSurfaceKind::Sphere { .. } => SurfaceType::Sphere,
            BRepSurfaceKind::Torus { .. } => SurfaceType::Torus,
            BRepSurfaceKind::BSpline(_) => SurfaceType::BSplineSurface,
            BRepSurfaceKind::Other => SurfaceType::OtherSurface,
        }
    }

    /// First U parameter.
    pub fn u_first(&self) -> f64 { self.u_min }
    /// Last U parameter.
    pub fn u_last(&self) -> f64 { self.u_max }
    /// First V parameter.
    pub fn v_first(&self) -> f64 { self.v_min }
    /// Last V parameter.
    pub fn v_last(&self) -> f64 { self.v_max }

    /// True if the surface is periodic in U.
    pub fn is_u_periodic(&self) -> bool {
        match &self.kind {
            BRepSurfaceKind::Plane(_) => false,
            BRepSurfaceKind::Cylinder { .. } => true,
            BRepSurfaceKind::Cone { .. } => true,
            BRepSurfaceKind::Sphere { .. } => true,
            BRepSurfaceKind::Torus { .. } => true,
            BRepSurfaceKind::BSpline(b) => b.is_u_periodic(),
            BRepSurfaceKind::Other => false,
        }
    }

    /// True if the surface is periodic in V.
    pub fn is_v_periodic(&self) -> bool {
        match &self.kind {
            BRepSurfaceKind::Plane(_) => false,
            BRepSurfaceKind::Cylinder { .. } => false,
            BRepSurfaceKind::Cone { .. } => false,
            BRepSurfaceKind::Sphere { .. } => false,
            BRepSurfaceKind::Torus { .. } => true,
            BRepSurfaceKind::BSpline(b) => b.is_v_periodic(),
            BRepSurfaceKind::Other => false,
        }
    }

    /// True if the surface is closed in U (start and end iso-curves coincide).
    pub fn is_u_closed(&self) -> bool {
        match &self.kind {
            BRepSurfaceKind::Plane(_) => false,
            BRepSurfaceKind::Cylinder { .. }
            | BRepSurfaceKind::Cone { .. }
            | BRepSurfaceKind::Sphere { .. }
            | BRepSurfaceKind::Torus { .. } => {
                (self.u_max - self.u_min - 2.0 * PI).abs() < 1e-9
            }
            BRepSurfaceKind::BSpline(_) => false,
            BRepSurfaceKind::Other => false,
        }
    }

    /// True if the surface is closed in V.
    pub fn is_v_closed(&self) -> bool {
        match &self.kind {
            BRepSurfaceKind::Torus { .. } => {
                (self.v_max - self.v_min - 2.0 * PI).abs() < 1e-9
            }
            _ => false,
        }
    }

    /// U period (panics if not U-periodic).
    pub fn u_period(&self) -> f64 {
        match &self.kind {
            BRepSurfaceKind::Cylinder { .. }
            | BRepSurfaceKind::Cone { .. }
            | BRepSurfaceKind::Sphere { .. }
            | BRepSurfaceKind::Torus { .. } => 2.0 * PI,
            BRepSurfaceKind::BSpline(b) if b.is_u_periodic() => {
                b.u_knot(b.last_u_knot_index()) - b.u_knot(b.first_u_knot_index())
            }
            _ => panic!("BRepAdaptorSurface::u_period called on non-periodic surface"),
        }
    }

    /// V period (panics if not V-periodic).
    pub fn v_period(&self) -> f64 {
        match &self.kind {
            BRepSurfaceKind::Torus { .. } => 2.0 * PI,
            BRepSurfaceKind::BSpline(b) if b.is_v_periodic() => {
                b.v_knot(b.last_v_knot_index()) - b.v_knot(b.first_v_knot_index())
            }
            _ => panic!("BRepAdaptorSurface::v_period called on non-periodic surface"),
        }
    }

    // ------------------------------------------------------------------
    // Evaluation
    // ------------------------------------------------------------------

    /// `D0` — evaluate point at `(u, v)`.
    // occt: BRepAdaptor_Surface::D0
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        match &self.kind {
            BRepSurfaceKind::Plane(ax3) => {
                ax3.location + ax3.x_dir * u + ax3.y_dir * v
            }
            BRepSurfaceKind::Cylinder { frame, radius } => {
                let cu = u.cos();
                let su = u.sin();
                frame.location
                    + frame.x_dir * (*radius * cu)
                    + frame.y_dir * (*radius * su)
                    + frame.z_dir * v
            }
            BRepSurfaceKind::Cone { frame, radius, half_angle } => {
                let r = radius + v * half_angle.tan();
                let cu = u.cos();
                let su = u.sin();
                frame.location
                    + frame.x_dir * (r * cu)
                    + frame.y_dir * (r * su)
                    + frame.z_dir * v
            }
            BRepSurfaceKind::Sphere { frame, radius } => {
                let cv = v.cos();
                let sv = v.sin();
                let cu = u.cos();
                let su = u.sin();
                frame.location
                    + frame.x_dir * (*radius * cv * cu)
                    + frame.y_dir * (*radius * cv * su)
                    + frame.z_dir * (*radius * sv)
            }
            BRepSurfaceKind::Torus { frame, major_radius, minor_radius } => {
                let cu = u.cos();
                let su = u.sin();
                let cv = v.cos();
                let sv = v.sin();
                let circle_center = frame.location
                    + frame.x_dir * (*major_radius * cu)
                    + frame.y_dir * (*major_radius * su);
                let radial_dir = frame.x_dir * cu + frame.y_dir * su;
                circle_center
                    + radial_dir * (*minor_radius * cv)
                    + frame.z_dir * (*minor_radius * sv)
            }
            BRepSurfaceKind::BSpline(b) => b.value(u, v),
            BRepSurfaceKind::Other => Pnt::origin(),
        }
    }

    /// `D1` — evaluate point and first partial derivatives `(d/du, d/dv)`.
    // occt: BRepAdaptor_Surface::D1
    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        match &self.kind {
            BRepSurfaceKind::Plane(ax3) => {
                let p = ax3.location + ax3.x_dir * u + ax3.y_dir * v;
                (p, ax3.x_dir, ax3.y_dir)
            }
            BRepSurfaceKind::Cylinder { frame, radius } => {
                let cu = u.cos();
                let su = u.sin();
                let p = frame.location
                    + frame.x_dir * (*radius * cu)
                    + frame.y_dir * (*radius * su)
                    + frame.z_dir * v;
                let du = frame.x_dir * (-radius * su) + frame.y_dir * (*radius * cu);
                let dv = frame.z_dir;
                (p, du, dv)
            }
            BRepSurfaceKind::Cone { frame, radius, half_angle } => {
                let r = radius + v * half_angle.tan();
                let tan_a = half_angle.tan();
                let cu = u.cos();
                let su = u.sin();
                let p = frame.location
                    + frame.x_dir * (r * cu)
                    + frame.y_dir * (r * su)
                    + frame.z_dir * v;
                let du = frame.x_dir * (-r * su) + frame.y_dir * (r * cu);
                let radial_dir = frame.x_dir * cu + frame.y_dir * su;
                let dv = radial_dir * tan_a + frame.z_dir;
                (p, du, dv)
            }
            BRepSurfaceKind::Sphere { frame, radius } => {
                let p = self.value(u, v);
                let cv = v.cos();
                let sv = v.sin();
                let cu = u.cos();
                let su = u.sin();
                let du = frame.x_dir * (-radius * cv * su)
                    + frame.y_dir * (*radius * cv * cu);
                let dv = frame.x_dir * (-radius * sv * cu)
                    + frame.y_dir * (-radius * sv * su)
                    + frame.z_dir * (*radius * cv);
                (p, du, dv)
            }
            BRepSurfaceKind::Torus { frame, major_radius, minor_radius } => {
                let p = self.value(u, v);
                let cu = u.cos();
                let su = u.sin();
                let cv = v.cos();
                let sv = v.sin();
                let d_radial_du = frame.x_dir * (-su) + frame.y_dir * cu;
                let du = d_radial_du * (*major_radius + minor_radius * cv);
                let radial_dir = frame.x_dir * cu + frame.y_dir * su;
                let dv = radial_dir * (-minor_radius * sv)
                    + frame.z_dir * (*minor_radius * cv);
                (p, du, dv)
            }
            BRepSurfaceKind::BSpline(b) => {
                let (p, du, dv) = b.d1(u, v);
                (p, du, dv)
            }
            BRepSurfaceKind::Other => (Pnt::origin(), Pnt::origin(), Pnt::origin()),
        }
    }

    /// Surface normal at `(u, v)` (unit vector).
    // occt: BRepAdaptor_Surface::Normal (conceptually via DN)
    pub fn normal(&self, u: f64, v: f64) -> Vec3 {
        let (_, du, dv) = self.d1(u, v);
        du.cross(dv).normalized()
    }

    /// Access plane data (`Ax3`) if this is a planar surface.
    pub fn plane(&self) -> Option<Ax3> {
        match &self.kind {
            BRepSurfaceKind::Plane(ax3) => Some(*ax3),
            _ => None,
        }
    }

    /// Access cylinder data if this is a cylindrical surface.
    pub fn cylinder(&self) -> Option<(Ax3, f64)> {
        match &self.kind {
            BRepSurfaceKind::Cylinder { frame, radius } => Some((*frame, *radius)),
            _ => None,
        }
    }

    /// Access cone data if this is a conical surface.
    pub fn cone(&self) -> Option<(Ax3, f64, f64)> {
        match &self.kind {
            BRepSurfaceKind::Cone { frame, radius, half_angle } => {
                Some((*frame, *radius, *half_angle))
            }
            _ => None,
        }
    }

    /// Access sphere data if this is a spherical surface.
    pub fn sphere(&self) -> Option<(Ax3, f64)> {
        match &self.kind {
            BRepSurfaceKind::Sphere { frame, radius } => Some((*frame, *radius)),
            _ => None,
        }
    }

    /// Access torus data if this is a toroidal surface.
    pub fn torus(&self) -> Option<(Ax3, f64, f64)> {
        match &self.kind {
            BRepSurfaceKind::Torus { frame, major_radius, minor_radius } => {
                Some((*frame, *major_radius, *minor_radius))
            }
            _ => None,
        }
    }

    /// Access underlying BSpline surface if present.
    pub fn bspline(&self) -> Option<&GeomBSplineSurface> {
        match &self.kind {
            BRepSurfaceKind::BSpline(b) => Some(b),
            _ => None,
        }
    }

    /// Compute an axis-aligned bounding box by sampling the surface.
    pub fn bounding_box(&self) -> BndBox {
        let n = 16usize;
        let mut bb = BndBox::new();
        for i in 0..=n {
            for j in 0..=n {
                let u = self.u_min + (self.u_max - self.u_min) * (i as f64 / n as f64);
                let v = self.v_min + (self.v_max - self.v_min) * (j as f64 / n as f64);
                bb.add_point(self.value(u, v));
            }
        }
        bb
    }
}

// ---------------------------------------------------------------------------
// BRepAdaptor_CompCurve — composite curve from a Wire
// ---------------------------------------------------------------------------

/// A segment record: parameter interval `[first, last]` referencing a single
/// edge's curve, plus the precomputed cumulative start of this segment in the
/// composite parameter.
// occt: BRepAdaptor_CompCurve (internal segment)
#[derive(Debug, Clone)]
pub struct CompCurveSegment {
    /// The adapted edge curve for this segment.
    pub curve: BRepAdaptorCurve,
    /// Offset to add to the local curve parameter so that the composite curve
    /// uses a continuously-increasing parameter.  Specifically, `composite_u =
    /// param_offset + local_u` where `local_u ∈ [curve.first, curve.last]`.
    pub param_offset: f64,
    /// Length of this segment (used to build `param_offset` for the next one).
    pub length: f64,
}

/// `BRepAdaptor_CompCurve` — models a `TopoDS_Wire` as a single composite
/// parametric curve.
///
/// Each edge in the wire contributes a segment.  The composite parameter is
/// the cumulative chord-length: segment `i` occupies
/// `[sum_{k<i} length_k, sum_{k<=i} length_k]`.
///
/// Evaluation dispatches to the appropriate segment via binary search.
// occt: BRepAdaptor_CompCurve
#[derive(Debug, Clone)]
pub struct BRepAdaptorCompCurve {
    segments: Vec<CompCurveSegment>,
    total_length: f64,
    closed: bool,
}

impl BRepAdaptorCompCurve {
    /// Construct from an ordered list of edge adaptors.  The wire is considered
    /// closed if `closed` is `true`.
    pub fn new(edges: Vec<BRepAdaptorCurve>, closed: bool) -> Self {
        let mut segments = Vec::with_capacity(edges.len());
        let mut offset = 0.0;
        for curve in edges {
            let len = curve.length();
            segments.push(CompCurveSegment {
                curve,
                param_offset: offset,
                length: len,
            });
            offset += len;
        }
        BRepAdaptorCompCurve {
            total_length: offset,
            segments,
            closed,
        }
    }

    /// Total composite parameter range `[0, total_length]`.
    pub fn first_parameter(&self) -> f64 { 0.0 }
    /// Last parameter = total arc-length.
    pub fn last_parameter(&self) -> f64 { self.total_length }

    /// True if the wire is closed.
    pub fn is_closed(&self) -> bool { self.closed }

    /// Number of edges (segments) in the wire.
    pub fn nb_intervals(&self) -> usize { self.segments.len() }

    /// Evaluate point at composite parameter `u`.
    // occt: BRepAdaptor_CompCurve::D0
    pub fn value(&self, u: f64) -> Pnt {
        let (seg, local_u) = self.locate(u);
        seg.curve.value(local_u)
    }

    /// Evaluate point and first derivative at composite parameter `u`.
    // occt: BRepAdaptor_CompCurve::D1
    pub fn d1(&self, u: f64) -> (Pnt, Vec3) {
        let (seg, local_u) = self.locate(u);
        seg.curve.d1(local_u)
    }

    /// Evaluate point, first and second derivative at composite parameter `u`.
    // occt: BRepAdaptor_CompCurve::D2
    pub fn d2(&self, u: f64) -> (Pnt, Vec3, Vec3) {
        let (seg, local_u) = self.locate(u);
        seg.curve.d2(local_u)
    }

    /// Returns the `i`-th segment (0-based).
    pub fn segment(&self, i: usize) -> &CompCurveSegment {
        &self.segments[i]
    }

    /// Compute an axis-aligned bounding box.
    pub fn bounding_box(&self) -> BndBox {
        let mut bb = BndBox::new();
        for seg in &self.segments {
            bb.add(&seg.curve.bounding_box());
        }
        bb
    }

    // ------------------------------------------------------------------
    // Private helpers
    // ------------------------------------------------------------------

    /// Find the segment that contains composite parameter `u` and return a
    /// reference together with the corresponding local curve parameter.
    fn locate(&self, u: f64) -> (&CompCurveSegment, f64) {
        if self.segments.is_empty() {
            panic!("BRepAdaptorCompCurve: no segments");
        }
        // Clamp to valid range.
        let u = u.clamp(0.0, self.total_length);
        // Find the last segment whose offset is <= u.
        let mut idx = 0;
        for (i, seg) in self.segments.iter().enumerate() {
            if seg.param_offset <= u {
                idx = i;
            } else {
                break;
            }
        }
        let seg = &self.segments[idx];
        // Map composite u → local curve parameter.
        let local_start = seg.curve.first;
        let local_end = seg.curve.last;
        let seg_u = u - seg.param_offset;
        let local_u = if seg.length > 1e-15 {
            local_start + (seg_u / seg.length) * (local_end - local_start)
        } else {
            local_start
        };
        (seg, local_u)
    }
}

// ---------------------------------------------------------------------------
// Private pure-math evaluation helpers
// ---------------------------------------------------------------------------

/// Circle point: `P(u) = O + R*cos(u)*X + R*sin(u)*Y`.
fn circle_point(ax3: &Ax3, radius: f64, u: f64) -> Pnt {
    ax3.location
        + ax3.x_dir * (radius * u.cos())
        + ax3.y_dir * (radius * u.sin())
}

/// Circle first derivative: `dP/du = R*(-sin(u)*X + cos(u)*Y)`.
fn circle_d1(ax3: &Ax3, radius: f64, u: f64) -> Vec3 {
    ax3.x_dir * (-radius * u.sin()) + ax3.y_dir * (radius * u.cos())
}

/// Circle second derivative: `d²P/du² = -R*(cos(u)*X + sin(u)*Y)`.
fn circle_d2(ax3: &Ax3, radius: f64, u: f64) -> Vec3 {
    ax3.x_dir * (-radius * u.cos()) + ax3.y_dir * (-radius * u.sin())
}

/// Ellipse point: `P(u) = O + a*cos(u)*X + b*sin(u)*Y`.
fn ellipse_point(ax3: &Ax3, a: f64, b: f64, u: f64) -> Pnt {
    ax3.location
        + ax3.x_dir * (a * u.cos())
        + ax3.y_dir * (b * u.sin())
}

/// Ellipse first derivative.
fn ellipse_d1(ax3: &Ax3, a: f64, b: f64, u: f64) -> Vec3 {
    ax3.x_dir * (-a * u.sin()) + ax3.y_dir * (b * u.cos())
}

/// Ellipse second derivative.
fn ellipse_d2(ax3: &Ax3, a: f64, b: f64, u: f64) -> Vec3 {
    ax3.x_dir * (-a * u.cos()) + ax3.y_dir * (-b * u.sin())
}

// ---------------------------------------------------------------------------
// Polyline helpers
// ---------------------------------------------------------------------------

/// Compute total chord-length of a polyline.
fn polyline_length(pts: &[Pnt]) -> f64 {
    pts.windows(2).map(|w| w[0].distance(w[1])).sum()
}

/// Precompute cumulative chord-length parameters for a polyline.
fn polyline_params(pts: &[Pnt]) -> Vec<f64> {
    let mut params = Vec::with_capacity(pts.len());
    params.push(0.0);
    let mut acc = 0.0;
    for w in pts.windows(2) {
        acc += w[0].distance(w[1]);
        params.push(acc);
    }
    params
}

/// Evaluate a polyline at cumulative chord-length parameter `u`.
fn polyline_value(pts: &[Pnt], u: f64) -> Pnt {
    if pts.is_empty() {
        return Pnt::origin();
    }
    if pts.len() == 1 {
        return pts[0];
    }
    let params = polyline_params(pts);
    let total = *params.last().unwrap();
    let u = u.clamp(0.0, total);
    // Find the segment.
    let mut seg = pts.len() - 2;
    for i in 0..pts.len() - 1 {
        if u <= params[i + 1] {
            seg = i;
            break;
        }
    }
    let dt = params[seg + 1] - params[seg];
    if dt < 1e-15 {
        return pts[seg];
    }
    let t = (u - params[seg]) / dt;
    pts[seg].lerp(pts[seg + 1], t)
}

/// First derivative of a polyline (piecewise-constant unit tangent).
fn polyline_d1(pts: &[Pnt], u: f64) -> Vec3 {
    if pts.len() < 2 {
        return Pnt::new(1.0, 0.0, 0.0);
    }
    let params = polyline_params(pts);
    let total = *params.last().unwrap();
    let u = u.clamp(0.0, total);
    let mut seg = pts.len() - 2;
    for i in 0..pts.len() - 1 {
        if u <= params[i + 1] {
            seg = i;
            break;
        }
    }
    (pts[seg + 1] - pts[seg]).normalized()
}

// ---------------------------------------------------------------------------
// Arc-length integration (5-point Gauss-Legendre)
// ---------------------------------------------------------------------------

// Nodes and weights for Gauss-Legendre 5-point quadrature on [-1, 1].
const GL5_NODES: [f64; 5] = [
    -0.906_179_845_938_664,
    -0.538_469_310_105_683,
     0.0,
     0.538_469_310_105_683,
     0.906_179_845_938_664,
];
const GL5_WEIGHTS: [f64; 5] = [
    0.236_926_885_056_189,
    0.478_628_670_499_366,
    0.568_888_888_888_889,
    0.478_628_670_499_366,
    0.236_926_885_056_189,
];

fn curve_length(curve: &BRepAdaptorCurve, a: f64, b: f64) -> f64 {
    let mid = 0.5 * (a + b);
    let half = 0.5 * (b - a);
    let mut len = 0.0;
    for (&xi, &wi) in GL5_NODES.iter().zip(GL5_WEIGHTS.iter()) {
        let u = mid + half * xi;
        let (_, d1) = curve.d1(u);
        len += wi * d1.norm();
    }
    len * half
}

// ---------------------------------------------------------------------------
// Internal unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn xy_circle_adaptor(radius: f64, a: f64, b: f64) -> BRepAdaptorCurve {
        let ax3 = Ax3::identity();
        BRepAdaptorCurve::from_circle(ax3, radius, a, b)
    }

    fn x_line_adaptor(len: f64) -> BRepAdaptorCurve {
        let ax = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
        BRepAdaptorCurve::from_line(ax, 0.0, len)
    }

    fn xy_plane_adaptor() -> BRepAdaptorSurface {
        BRepAdaptorSurface::from_plane(Ax3::identity(), -10.0, 10.0, -10.0, 10.0)
    }

    // ------------------------------------------------------------------
    // BRepAdaptorCurve – line
    // ------------------------------------------------------------------

    #[test]
    fn line_value() {
        let c = x_line_adaptor(5.0);
        let p = c.value(3.0);
        assert!((p.x - 3.0).abs() < 1e-12, "x={}", p.x);
        assert!(p.y.abs() < 1e-12);
        assert!(p.z.abs() < 1e-12);
    }

    #[test]
    fn line_d1_tangent_is_unit_x() {
        let c = x_line_adaptor(5.0);
        let (_, d1) = c.d1(2.0);
        assert!((d1.x - 1.0).abs() < 1e-12);
        assert!(d1.y.abs() < 1e-12);
        assert!(d1.z.abs() < 1e-12);
    }

    #[test]
    fn line_d2_is_zero() {
        let c = x_line_adaptor(5.0);
        let (_, _, d2) = c.d2(2.0);
        assert!(d2.x.abs() < 1e-15);
        assert!(d2.y.abs() < 1e-15);
        assert!(d2.z.abs() < 1e-15);
    }

    #[test]
    fn line_length() {
        let c = x_line_adaptor(5.0);
        let l = c.length();
        assert!((l - 5.0).abs() < 1e-6, "length={}", l);
    }

    // ------------------------------------------------------------------
    // BRepAdaptorCurve – circle
    // ------------------------------------------------------------------

    #[test]
    fn circle_value_at_zero() {
        let r = 3.0;
        let c = xy_circle_adaptor(r, 0.0, 2.0 * PI);
        let p = c.value(0.0);
        assert!((p.x - r).abs() < 1e-12);
        assert!(p.y.abs() < 1e-12);
        assert!(p.z.abs() < 1e-12);
    }

    #[test]
    fn circle_value_at_half_pi() {
        let r = 3.0;
        let c = xy_circle_adaptor(r, 0.0, 2.0 * PI);
        let p = c.value(PI / 2.0);
        assert!(p.x.abs() < 1e-12);
        assert!((p.y - r).abs() < 1e-12);
    }

    #[test]
    fn circle_d1_at_zero() {
        let r = 4.0;
        let c = xy_circle_adaptor(r, 0.0, 2.0 * PI);
        let (p, d1) = c.d1(0.0);
        assert!((p.x - r).abs() < 1e-12);
        // tangent at 0: (0, r, 0)
        assert!(d1.x.abs() < 1e-12);
        assert!((d1.y - r).abs() < 1e-12);
        assert!(d1.z.abs() < 1e-12);
    }

    #[test]
    fn circle_d2_at_zero() {
        let r = 4.0;
        let c = xy_circle_adaptor(r, 0.0, 2.0 * PI);
        let (_, _, d2) = c.d2(0.0);
        // d²P/du² at u=0 = (-r, 0, 0)
        assert!((d2.x + r).abs() < 1e-12);
        assert!(d2.y.abs() < 1e-12);
        assert!(d2.z.abs() < 1e-12);
    }

    #[test]
    fn circle_is_periodic_and_get_type() {
        let c = xy_circle_adaptor(1.0, 0.0, 2.0 * PI);
        assert!(c.is_periodic());
        assert_eq!(c.get_type(), CurveType::Circle);
    }

    #[test]
    fn circle_full_is_closed() {
        let c = xy_circle_adaptor(1.0, 0.0, 2.0 * PI);
        assert!(c.is_closed());
    }

    #[test]
    fn circle_arc_not_closed() {
        let c = xy_circle_adaptor(1.0, 0.0, PI);
        assert!(!c.is_closed());
    }

    #[test]
    fn circle_circumference() {
        let r = 2.0;
        let c = xy_circle_adaptor(r, 0.0, 2.0 * PI);
        let l = c.length();
        let expected = 2.0 * PI * r;
        assert!((l - expected).abs() < 1e-4, "length={} expected={}", l, expected);
    }

    #[test]
    fn circle_d1_fd_consistency() {
        let r = 3.0;
        let c = xy_circle_adaptor(r, 0.0, 2.0 * PI);
        let u = 1.1;
        let h = 1e-6;
        let (_, d1) = c.d1(u);
        let p_fwd = c.value(u + h);
        let p_bwd = c.value(u - h);
        let fd = (p_fwd - p_bwd) * (1.0 / (2.0 * h));
        assert!((d1.x - fd.x).abs() < 1e-8);
        assert!((d1.y - fd.y).abs() < 1e-8);
    }

    // ------------------------------------------------------------------
    // BRepAdaptorCurve – ellipse
    // ------------------------------------------------------------------

    #[test]
    fn ellipse_value_at_zero() {
        let ax3 = Ax3::identity();
        let c = BRepAdaptorCurve::from_ellipse(ax3, 5.0, 3.0, 0.0, 2.0 * PI);
        let p = c.value(0.0);
        assert!((p.x - 5.0).abs() < 1e-12);
        assert!(p.y.abs() < 1e-12);
    }

    #[test]
    fn ellipse_value_at_half_pi() {
        let ax3 = Ax3::identity();
        let c = BRepAdaptorCurve::from_ellipse(ax3, 5.0, 3.0, 0.0, 2.0 * PI);
        let p = c.value(PI / 2.0);
        assert!(p.x.abs() < 1e-12);
        assert!((p.y - 3.0).abs() < 1e-12);
    }

    // ------------------------------------------------------------------
    // BRepAdaptorCurve – polyline
    // ------------------------------------------------------------------

    #[test]
    fn polyline_value_at_start_and_end() {
        let pts = vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
        ];
        let c = BRepAdaptorCurve::from_polyline(pts.clone());
        let p0 = c.value(0.0);
        let p_end = c.value(c.last_parameter());
        assert!((p0 - pts[0]).norm() < 1e-10);
        assert!((p_end - pts[2]).norm() < 1e-10);
    }

    // ------------------------------------------------------------------
    // BRepAdaptorSurface – plane
    // ------------------------------------------------------------------

    #[test]
    fn plane_value() {
        let s = xy_plane_adaptor();
        let p = s.value(3.0, 4.0);
        assert!((p.x - 3.0).abs() < 1e-12);
        assert!((p.y - 4.0).abs() < 1e-12);
        assert!(p.z.abs() < 1e-12);
    }

    #[test]
    fn plane_d1() {
        let s = xy_plane_adaptor();
        let (p, du, dv) = s.d1(1.0, 2.0);
        assert!((p.x - 1.0).abs() < 1e-12);
        assert!((p.y - 2.0).abs() < 1e-12);
        assert!((du.x - 1.0).abs() < 1e-12);
        assert!(du.y.abs() < 1e-12);
        assert!(dv.x.abs() < 1e-12);
        assert!((dv.y - 1.0).abs() < 1e-12);
    }

    #[test]
    fn plane_normal_is_z() {
        let s = xy_plane_adaptor();
        let n = s.normal(0.0, 0.0);
        assert!(n.z.abs() > 0.999);
    }

    #[test]
    fn plane_get_type() {
        let s = xy_plane_adaptor();
        assert_eq!(s.get_type(), SurfaceType::Plane);
    }

    // ------------------------------------------------------------------
    // BRepAdaptorSurface – cylinder
    // ------------------------------------------------------------------

    #[test]
    fn cylinder_value_at_zero() {
        let s = BRepAdaptorSurface::from_cylinder(Ax3::identity(), 2.0, 0.0, 2.0*PI, -5.0, 5.0);
        let p = s.value(0.0, 0.0);
        assert!((p.x - 2.0).abs() < 1e-12);
        assert!(p.y.abs() < 1e-12);
        assert!(p.z.abs() < 1e-12);
    }

    #[test]
    fn cylinder_u_periodic() {
        let s = BRepAdaptorSurface::from_cylinder(Ax3::identity(), 2.0, 0.0, 2.0*PI, -5.0, 5.0);
        assert!(s.is_u_periodic());
        assert!(!s.is_v_periodic());
    }

    #[test]
    fn cylinder_normal_points_outward() {
        let s = BRepAdaptorSurface::from_cylinder(Ax3::identity(), 2.0, 0.0, 2.0*PI, -5.0, 5.0);
        let n = s.normal(0.0, 0.0);
        // At u=0, v=0 the outward radial direction should be +X
        assert!(n.x > 0.99);
    }

    // ------------------------------------------------------------------
    // BRepAdaptorSurface – sphere
    // ------------------------------------------------------------------

    #[test]
    fn sphere_value_on_unit_sphere() {
        let s = BRepAdaptorSurface::from_sphere(Ax3::identity(), 1.0, 0.0, 2.0*PI, -PI/2.0, PI/2.0);
        for i in 0..8 {
            for j in 0..4 {
                let u = i as f64 * PI / 4.0;
                let v = -PI/2.0 + j as f64 * PI / 3.0;
                let p = s.value(u, v);
                let r = p.norm();
                assert!((r - 1.0).abs() < 1e-10, "r={} at u={} v={}", r, u, v);
            }
        }
    }

    // ------------------------------------------------------------------
    // BRepAdaptorCompCurve
    // ------------------------------------------------------------------

    #[test]
    fn comp_curve_single_line() {
        let ax = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
        let edge = BRepAdaptorCurve::from_line(ax, 0.0, 3.0);
        let comp = BRepAdaptorCompCurve::new(vec![edge], false);
        assert!((comp.last_parameter() - 3.0).abs() < 1e-6);
        let p = comp.value(1.5);
        assert!((p.x - 1.5).abs() < 1e-9);
    }

    #[test]
    fn comp_curve_two_segments() {
        // Segment 1: line along X from 0..2
        let ax1 = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
        let seg1 = BRepAdaptorCurve::from_line(ax1, 0.0, 2.0);
        // Segment 2: line along Y from (2,0,0) upward
        let ax2 = Ax1::new(Pnt::new(2.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
        let seg2 = BRepAdaptorCurve::from_line(ax2, 0.0, 2.0);
        let comp = BRepAdaptorCompCurve::new(vec![seg1, seg2], false);
        assert!((comp.last_parameter() - 4.0).abs() < 1e-4);
        // Mid-point of first segment
        let p1 = comp.value(1.0);
        assert!((p1.x - 1.0).abs() < 1e-9);
        assert!(p1.y.abs() < 1e-9);
    }

    #[test]
    fn comp_curve_closed() {
        let ax = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
        let edge = BRepAdaptorCurve::from_circle(Ax3::identity(), 1.0, 0.0, 2.0 * PI);
        let comp = BRepAdaptorCompCurve::new(vec![edge], true);
        assert!(comp.is_closed());
    }

    #[test]
    fn comp_curve_nb_intervals() {
        let ax = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
        let e1 = BRepAdaptorCurve::from_line(ax, 0.0, 1.0);
        let e2 = BRepAdaptorCurve::from_line(ax, 0.0, 1.0);
        let comp = BRepAdaptorCompCurve::new(vec![e1, e2], false);
        assert_eq!(comp.nb_intervals(), 2);
    }
}
