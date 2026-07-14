// FILE: geom_lprop.rs
//! `GeomLProp` — local properties of curves and surfaces in 3D space.
//!
//! Faithful port of OpenCascade's `GeomLProp` package:
//! - `GeomLProp_CLProps`    — local properties of a 3D curve (curvature, torsion,
//!                            tangent, normal, bi-normal).
//! - `GeomLProp_SLProps`    — local properties of a parametric surface (normal,
//!                            principal curvatures, mean curvature, Gaussian curvature).
//! - `GeomLProp_CurveTool`  — adaptor used by `CLProps` to evaluate the curve and
//!                            its derivatives.
//! - `GeomLProp_SurfaceTool`— adaptor used by `SLProps` to evaluate the surface and
//!                            its derivatives.

use crate::gp::{Pnt, Vec3};
use crate::geom_bspline_curve::GeomBSplineCurve;
use crate::geom_bspline_surface::GeomBSplineSurface;
use crate::geom_circle::GeomCircle;
use crate::geom_line::GeomLine;

// ---------------------------------------------------------------------------
// GeomLProp_CurveTool
// ---------------------------------------------------------------------------

/// Tool providing the evaluation interface for 3D curves used by `CLProps`.
///
/// Mirrors OCCT's `GeomLProp_CurveTool` — a purely static adaptor that
/// dispatches D0/D1/D2/D3 and parameter-range queries to the concrete curve
/// type through the `Curve` enum.
// occt-ref: GeomLProp_CurveTool
#[derive(Clone, Debug)]
pub struct CurveTool;

/// The concrete curve kinds that `CurveTool` and `CLProps` can operate on.
// occt-note: Handle(Geom_Curve) variants
#[derive(Clone, Debug)]
pub enum Curve {
    /// An infinite straight line.
    Line(GeomLine),
    /// A circle.
    Circle(GeomCircle),
    /// A B-spline / NURBS curve.
    BSpline(GeomBSplineCurve),
}

impl CurveTool {
    /// `GeomLProp_CurveTool::FirstParameter(C)`.
    pub fn first_parameter(c: &Curve) -> f64 {
        match c {
            Curve::Line(_) => f64::NEG_INFINITY,
            Curve::Circle(_) => 0.0,
            Curve::BSpline(b) => b.first_parameter(),
        }
    }

    /// `GeomLProp_CurveTool::LastParameter(C)`.
    pub fn last_parameter(c: &Curve) -> f64 {
        match c {
            Curve::Line(_) => f64::INFINITY,
            Curve::Circle(_) => std::f64::consts::TAU,
            Curve::BSpline(b) => b.last_parameter(),
        }
    }

    /// `GeomLProp_CurveTool::Continuity(C)` — returns the order of continuity
    /// (0 = C0, 1 = C1, 2 = C2, ...).
    pub fn continuity(c: &Curve) -> i32 {
        match c {
            Curve::Line(_) => i32::MAX,   // C-infinity
            Curve::Circle(_) => i32::MAX, // C-infinity
            Curve::BSpline(b) => {
                use crate::geom_bspline_curve::Shape;
                match b.continuity() {
                    Shape::CN => 3,
                    Shape::C3 => 3,
                    Shape::C2 => 2,
                    Shape::C1 => 1,
                    Shape::C0 | Shape::G1 | Shape::G2 => 0,
                }
            }
        }
    }

    /// `GeomLProp_CurveTool::Value(C, U, P)` — point at parameter `u`.
    pub fn value(c: &Curve, u: f64) -> Pnt {
        match c {
            Curve::Line(l) => l.d0(u),
            Curve::Circle(ci) => ci.d0(u),
            Curve::BSpline(b) => b.value(u),
        }
    }

    /// `GeomLProp_CurveTool::D1(C, U, P, V1)`.
    pub fn d1(c: &Curve, u: f64) -> (Pnt, Vec3) {
        match c {
            Curve::Line(l) => l.d1(u),
            Curve::Circle(ci) => ci.d1(u),
            Curve::BSpline(b) => b.d1(u),
        }
    }

    /// `GeomLProp_CurveTool::D2(C, U, P, V1, V2)`.
    pub fn d2(c: &Curve, u: f64) -> (Pnt, Vec3, Vec3) {
        match c {
            Curve::Line(l) => l.d2(u),
            Curve::Circle(ci) => ci.d2(u),
            Curve::BSpline(b) => b.d2(u),
        }
    }

    /// `GeomLProp_CurveTool::D3(C, U, P, V1, V2, V3)`.
    pub fn d3(c: &Curve, u: f64) -> (Pnt, Vec3, Vec3, Vec3) {
        match c {
            Curve::Line(l) => l.d3(u),
            Curve::Circle(ci) => ci.d3(u),
            Curve::BSpline(b) => b.d3(u),
        }
    }
}

// ---------------------------------------------------------------------------
// GeomLProp_SurfaceTool
// ---------------------------------------------------------------------------

/// The concrete surface kinds that `SurfaceTool` and `SLProps` can operate on.
// occt-note: Handle(Geom_Surface) variants
#[derive(Clone, Debug)]
pub enum Surface {
    /// A B-spline / NURBS surface.
    BSpline(GeomBSplineSurface),
    /// A plane (infinite flat surface).
    Plane {
        /// Origin of the plane.
        origin: Pnt,
        /// Unit X direction (first parametric axis).
        x_dir: Pnt,
        /// Unit Y direction (second parametric axis).
        y_dir: Pnt,
        /// Unit normal (Z = X × Y).
        normal: Pnt,
    },
    /// A sphere of given radius centered at `center` with axis frame.
    Sphere {
        center: Pnt,
        radius: f64,
        /// Unit Z direction (polar axis).
        z_dir: Pnt,
        /// Unit X direction.
        x_dir: Pnt,
        /// Unit Y direction.
        y_dir: Pnt,
    },
    /// A right circular cylinder.
    Cylinder {
        origin: Pnt,
        axis_dir: Pnt,
        x_dir: Pnt,
        y_dir: Pnt,
        radius: f64,
    },
}

/// Tool providing the evaluation interface for parametric surfaces used by
/// `SLProps`.
///
/// Mirrors OCCT's `GeomLProp_SurfaceTool` — a static adaptor that dispatches
/// D1/D2/DN queries to the concrete surface.
// occt-ref: GeomLProp_SurfaceTool
#[derive(Clone, Debug)]
pub struct SurfaceTool;

impl SurfaceTool {
    /// `GeomLProp_SurfaceTool::Bounds(S, U1, U2, V1, V2)`.
    pub fn bounds(s: &Surface) -> (f64, f64, f64, f64) {
        match s {
            Surface::BSpline(b) => {
                let (u1, u2, v1, v2) = b.bounds();
                (u1, u2, v1, v2)
            }
            Surface::Plane { .. } => (
                f64::NEG_INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
            ),
            Surface::Sphere { .. } => (
                -std::f64::consts::FRAC_PI_2,
                std::f64::consts::FRAC_PI_2,
                0.0,
                std::f64::consts::TAU,
            ),
            Surface::Cylinder { .. } => (
                f64::NEG_INFINITY,
                f64::INFINITY,
                0.0,
                std::f64::consts::TAU,
            ),
        }
    }

    /// `GeomLProp_SurfaceTool::Continuity(S)` — order of continuity.
    pub fn continuity(s: &Surface) -> i32 {
        match s {
            Surface::BSpline(_) => 2, // typically at least C2
            Surface::Plane { .. } | Surface::Sphere { .. } | Surface::Cylinder { .. } => {
                i32::MAX
            }
        }
    }

    /// `GeomLProp_SurfaceTool::Value(S, U, V)` — point on surface.
    pub fn value(s: &Surface, u: f64, v: f64) -> Pnt {
        let (p, _, _) = Self::d1(s, u, v);
        p
    }

    /// `GeomLProp_SurfaceTool::D1(S, U, V, P, D1U, D1V)`.
    ///
    /// Returns `(point, d1u, d1v)`.
    pub fn d1(s: &Surface, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        match s {
            Surface::BSpline(b) => {
                let (p, d1u, d1v) = b.d1(u, v);
                (p, d1u, d1v)
            }
            Surface::Plane { origin, x_dir, y_dir, .. } => {
                let p = *origin + *x_dir * u + *y_dir * v;
                (p, *x_dir, *y_dir)
            }
            Surface::Sphere { center, radius, z_dir, x_dir, y_dir } => {
                // Parameterization: u = latitude (−π/2..π/2), v = longitude (0..2π)
                // P(u,v) = center + r*(cos(u)*cos(v)*x + cos(u)*sin(v)*y + sin(u)*z)
                let r = *radius;
                let (su, cu) = u.sin_cos();
                let (sv, cv) = v.sin_cos();
                let p = *center
                    + *x_dir * (r * cu * cv)
                    + *y_dir * (r * cu * sv)
                    + *z_dir * (r * su);
                let d1u = *x_dir * (-r * su * cv)
                    + *y_dir * (-r * su * sv)
                    + *z_dir * (r * cu);
                let d1v = *x_dir * (-r * cu * sv)
                    + *y_dir * (r * cu * cv);
                (p, d1u, d1v)
            }
            Surface::Cylinder { origin, axis_dir, x_dir, y_dir, radius } => {
                // P(u,v) = origin + radius*(cos(v)*x + sin(v)*y) + u*axis
                // u is along axis, v is angle
                let r = *radius;
                let (sv, cv) = v.sin_cos();
                let p = *origin
                    + *axis_dir * u
                    + *x_dir * (r * cv)
                    + *y_dir * (r * sv);
                let d1u = *axis_dir;
                let d1v = *x_dir * (-r * sv) + *y_dir * (r * cv);
                (p, d1u, d1v)
            }
        }
    }

    /// `GeomLProp_SurfaceTool::D2(S, U, V, P, D1U, D1V, D2U, D2V, D2UV)`.
    ///
    /// Returns `(point, d1u, d1v, d2u, d2v, d2uv)`.
    pub fn d2(
        s: &Surface,
        u: f64,
        v: f64,
    ) -> (Pnt, Vec3, Vec3, Vec3, Vec3, Vec3) {
        match s {
            Surface::BSpline(b) => b.d2(u, v),
            Surface::Plane { origin, x_dir, y_dir, .. } => {
                let p = *origin + *x_dir * u + *y_dir * v;
                (p, *x_dir, *y_dir, Pnt::origin(), Pnt::origin(), Pnt::origin())
            }
            Surface::Sphere { center, radius, z_dir, x_dir, y_dir } => {
                let r = *radius;
                let (su, cu) = u.sin_cos();
                let (sv, cv) = v.sin_cos();
                let p = *center
                    + *x_dir * (r * cu * cv)
                    + *y_dir * (r * cu * sv)
                    + *z_dir * (r * su);
                let d1u = *x_dir * (-r * su * cv)
                    + *y_dir * (-r * su * sv)
                    + *z_dir * (r * cu);
                let d1v = *x_dir * (-r * cu * sv)
                    + *y_dir * (r * cu * cv);
                let d2u = *x_dir * (-r * cu * cv)
                    + *y_dir * (-r * cu * sv)
                    + *z_dir * (-r * su);
                let d2v = *x_dir * (-r * cu * cv)
                    + *y_dir * (-r * cu * sv);
                let d2uv = *x_dir * (r * su * sv)
                    + *y_dir * (-r * su * cv);
                (p, d1u, d1v, d2u, d2v, d2uv)
            }
            Surface::Cylinder { origin, axis_dir, x_dir, y_dir, radius } => {
                let r = *radius;
                let (sv, cv) = v.sin_cos();
                let p = *origin
                    + *axis_dir * u
                    + *x_dir * (r * cv)
                    + *y_dir * (r * sv);
                let d1u = *axis_dir;
                let d1v = *x_dir * (-r * sv) + *y_dir * (r * cv);
                let d2u = Pnt::origin();
                let d2v = *x_dir * (-r * cv) + *y_dir * (-r * sv);
                let d2uv = Pnt::origin();
                (p, d1u, d1v, d2u, d2v, d2uv)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// GeomLProp_CLProps
// ---------------------------------------------------------------------------

/// Local properties of a 3D curve at a given parameter value.
///
/// Mirrors OCCT's `GeomLProp_CLProps`:
/// - `Value()` / `D1()` / `D2()` / `D3()` — set the parameter and (lazily)
///   cache the evaluated position and derivatives.
/// - `IsNormalDefined()` / `Normal()` — the principal normal to the curve.
/// - `IsTangentDefined()` / `Tangent()` / `Curvature()` — curvature properties.
/// - `CentreOfCurvature()` — the osculating circle's centre.
/// - `Torsion()` — the torsion (second curvature) of a space curve.
// occt: GeomLProp_CLProps
#[derive(Clone, Debug)]
pub struct CLProps {
    curve: Curve,
    order: i32,
    resolution: f64,
    /// Current parameter value.
    u: f64,
    /// Cached position.
    p: Option<Pnt>,
    /// Cached first derivative.
    d1: Option<Vec3>,
    /// Cached second derivative.
    d2: Option<Vec3>,
    /// Cached third derivative.
    d3: Option<Vec3>,
}

impl CLProps {
    /// `GeomLProp_CLProps(C, N, Resolution)` — construct from a curve, derivative
    /// order `n` (0..3) and a linear resolution used for zero-length tests.
    pub fn new(curve: Curve, order: i32, resolution: f64) -> Self {
        assert!((0..=3).contains(&order), "CLProps: order must be 0..=3");
        CLProps {
            curve,
            order,
            resolution: resolution.max(1.0e-15),
            u: f64::NAN,
            p: None,
            d1: None,
            d2: None,
            d3: None,
        }
    }

    /// `GeomLProp_CLProps(C, U, N, Resolution)` — construct and set parameter.
    pub fn new_at(curve: Curve, u: f64, order: i32, resolution: f64) -> Self {
        let mut c = Self::new(curve, order, resolution);
        c.set_parameter(u);
        c
    }

    /// `void SetParameter(U)` — set the parameter and invalidate cached values.
    pub fn set_parameter(&mut self, u: f64) {
        self.u = u;
        // Evaluate the requested derivatives eagerly (OCCT evaluates up to order).
        match self.order {
            0 => {
                self.p = Some(CurveTool::value(&self.curve, u));
                self.d1 = None;
                self.d2 = None;
                self.d3 = None;
            }
            1 => {
                let (p, d1) = CurveTool::d1(&self.curve, u);
                self.p = Some(p);
                self.d1 = Some(d1);
                self.d2 = None;
                self.d3 = None;
            }
            2 => {
                let (p, d1, d2) = CurveTool::d2(&self.curve, u);
                self.p = Some(p);
                self.d1 = Some(d1);
                self.d2 = Some(d2);
                self.d3 = None;
            }
            _ => {
                let (p, d1, d2, d3) = CurveTool::d3(&self.curve, u);
                self.p = Some(p);
                self.d1 = Some(d1);
                self.d2 = Some(d2);
                self.d3 = Some(d3);
            }
        }
    }

    /// `void SetCurve(C)` — replace the underlying curve.
    pub fn set_curve(&mut self, c: Curve) {
        self.curve = c;
        self.p = None;
        self.d1 = None;
        self.d2 = None;
        self.d3 = None;
    }

    /// `gp_Pnt Value()` — the point on the curve.
    pub fn value(&self) -> Pnt {
        self.p.expect("CLProps: call SetParameter before Value()")
    }

    /// `gp_Vec D1()` — the first derivative.
    pub fn d1(&self) -> Vec3 {
        self.d1.expect("CLProps: order must be >= 1 to call D1()")
    }

    /// `gp_Vec D2()` — the second derivative.
    pub fn d2(&self) -> Vec3 {
        self.d2.expect("CLProps: order must be >= 2 to call D2()")
    }

    /// `gp_Vec D3()` — the third derivative.
    pub fn d3(&self) -> Vec3 {
        self.d3.expect("CLProps: order must be >= 3 to call D3()")
    }

    /// `Standard_Boolean IsTangentDefined()` — returns true if the first
    /// derivative is non-zero (i.e., the tangent is defined).
    pub fn is_tangent_defined(&self) -> bool {
        if let Some(d1) = self.d1 {
            d1.norm() > self.resolution
        } else {
            false
        }
    }

    /// `void Tangent(V)` — returns the unit tangent vector.
    ///
    /// # Panics
    /// Panics if the tangent is not defined (`is_tangent_defined() == false`).
    pub fn tangent(&self) -> Vec3 {
        let d1 = self.d1.expect("CLProps: order >= 1 required for Tangent()");
        let n = d1.norm();
        assert!(n > self.resolution, "CLProps: Tangent not defined (zero D1)");
        d1 * (1.0 / n)
    }

    /// `Standard_Real Curvature()` — the curvature κ = |T'| / |C'|  =
    /// |D1 × D2| / |D1|³.
    ///
    /// Returns `0.0` for a straight line (zero curvature), `+∞` when `|D1| ≈ 0`.
    pub fn curvature(&self) -> f64 {
        let d1 = self.d1.expect("CLProps: order >= 2 required for Curvature()");
        let d2 = self.d2.expect("CLProps: order >= 2 required for Curvature()");
        let n1 = d1.norm();
        if n1 <= self.resolution {
            return f64::INFINITY;
        }
        let cross = d1.cross(d2);
        cross.norm() / (n1 * n1 * n1)
    }

    /// `Standard_Boolean IsNormalDefined()` — true when the curvature is
    /// non-zero (the principal normal is well-defined).
    pub fn is_normal_defined(&self) -> bool {
        self.curvature() > self.resolution
    }

    /// `void Normal(V)` — the unit principal normal: the unit vector in the
    /// direction of `D2 - (D2·T̂)·T̂` (the component of `D2` orthogonal to
    /// the tangent), for non-degenerate curves.
    ///
    /// Uses the Frenet–Serret formula: N = (D1 × D2) × D1 / |…|.
    ///
    /// # Panics
    /// Panics if the normal is not defined.
    pub fn normal(&self) -> Vec3 {
        let d1 = self.d1.expect("CLProps: order >= 2 required for Normal()");
        let d2 = self.d2.expect("CLProps: order >= 2 required for Normal()");
        let n1 = d1.norm();
        assert!(n1 > self.resolution, "CLProps: Normal not defined (zero D1)");
        // Frenet normal: project D2 onto the plane perpendicular to D1.
        let t = d1 * (1.0 / n1);
        let proj = d2 - t * d2.dot(t);
        let np = proj.norm();
        assert!(np > self.resolution, "CLProps: Normal not defined (zero curvature)");
        proj * (1.0 / np)
    }

    /// `Standard_Boolean IsBiNormalDefined()` — true if both tangent and normal
    /// are defined.
    pub fn is_bi_normal_defined(&self) -> bool {
        self.is_normal_defined()
    }

    /// `void BiNormal(V)` — the unit bi-normal B = T × N.
    ///
    /// # Panics
    /// Panics if the bi-normal is not defined.
    pub fn bi_normal(&self) -> Vec3 {
        self.tangent().cross(self.normal()).normalized()
    }

    /// `void CentreOfCurvature(P)` — the centre of the osculating circle:
    /// `C_osc = P + (1/κ) · N`.
    ///
    /// # Panics
    /// Panics if the normal is not defined.
    pub fn centre_of_curvature(&self) -> Pnt {
        let kappa = self.curvature();
        assert!(kappa > self.resolution, "CLProps: CentreOfCurvature undefined (zero curvature)");
        self.value() + self.normal() * (1.0 / kappa)
    }

    /// `Standard_Real Torsion()` — the torsion τ of the curve at the current
    /// parameter.
    ///
    /// Computed from Frenet–Serret: τ = (D1 × D2) · D3 / |D1 × D2|².
    ///
    /// Returns `0.0` when the curve is planar (zero torsion numerator) or when
    /// the curvature is zero.
    pub fn torsion(&self) -> f64 {
        let d1 = self.d1.expect("CLProps: order >= 3 required for Torsion()");
        let d2 = self.d2.expect("CLProps: order >= 3 required for Torsion()");
        let d3 = self.d3.expect("CLProps: order >= 3 required for Torsion()");
        let cross = d1.cross(d2);
        let denom = cross.dot(cross); // |D1 × D2|²
        if denom <= self.resolution * self.resolution {
            return 0.0;
        }
        // τ = (D1 × D2) · D3 / |D1 × D2|²
        cross.dot(d3) / denom
    }
}

// ---------------------------------------------------------------------------
// GeomLProp_SLProps
// ---------------------------------------------------------------------------

/// Local properties of a parametric surface at a given (U, V) parameter pair.
///
/// Mirrors OCCT's `GeomLProp_SLProps`:
/// - Position, first and second partial derivatives.
/// - Surface normal.
/// - Principal curvatures, mean and Gaussian curvature.
/// - Curvature direction vectors (principal directions).
// occt: GeomLProp_SLProps
#[derive(Clone, Debug)]
pub struct SLProps {
    surface: Surface,
    order: i32,
    resolution: f64,
    u: f64,
    v: f64,
    /// Point on the surface.
    p: Option<Pnt>,
    d1u: Option<Vec3>,
    d1v: Option<Vec3>,
    d2u: Option<Vec3>,
    d2v: Option<Vec3>,
    d2uv: Option<Vec3>,
}

impl SLProps {
    /// `GeomLProp_SLProps(S, N, Resolution)`.
    pub fn new(surface: Surface, order: i32, resolution: f64) -> Self {
        assert!((0..=2).contains(&order), "SLProps: order must be 0..=2");
        SLProps {
            surface,
            order,
            resolution: resolution.max(1.0e-15),
            u: f64::NAN,
            v: f64::NAN,
            p: None,
            d1u: None,
            d1v: None,
            d2u: None,
            d2v: None,
            d2uv: None,
        }
    }

    /// `GeomLProp_SLProps(S, U, V, N, Resolution)`.
    pub fn new_at(surface: Surface, u: f64, v: f64, order: i32, resolution: f64) -> Self {
        let mut s = Self::new(surface, order, resolution);
        s.set_parameters(u, v);
        s
    }

    /// `void SetParameters(U, V)`.
    pub fn set_parameters(&mut self, u: f64, v: f64) {
        self.u = u;
        self.v = v;
        match self.order {
            0 => {
                self.p = Some(SurfaceTool::value(&self.surface, u, v));
                self.d1u = None;
                self.d1v = None;
                self.d2u = None;
                self.d2v = None;
                self.d2uv = None;
            }
            1 => {
                let (p, d1u, d1v) = SurfaceTool::d1(&self.surface, u, v);
                self.p = Some(p);
                self.d1u = Some(d1u);
                self.d1v = Some(d1v);
                self.d2u = None;
                self.d2v = None;
                self.d2uv = None;
            }
            _ => {
                let (p, d1u, d1v, d2u, d2v, d2uv) =
                    SurfaceTool::d2(&self.surface, u, v);
                self.p = Some(p);
                self.d1u = Some(d1u);
                self.d1v = Some(d1v);
                self.d2u = Some(d2u);
                self.d2v = Some(d2v);
                self.d2uv = Some(d2uv);
            }
        }
    }

    /// `void SetSurface(S)`.
    pub fn set_surface(&mut self, s: Surface) {
        self.surface = s;
        self.p = None;
        self.d1u = None;
        self.d1v = None;
        self.d2u = None;
        self.d2v = None;
        self.d2uv = None;
    }

    /// `gp_Pnt Value()`.
    pub fn value(&self) -> Pnt {
        self.p.expect("SLProps: call SetParameters before Value()")
    }

    /// `gp_Vec D1U()`.
    pub fn d1u(&self) -> Vec3 {
        self.d1u.expect("SLProps: order >= 1 required for D1U()")
    }

    /// `gp_Vec D1V()`.
    pub fn d1v(&self) -> Vec3 {
        self.d1v.expect("SLProps: order >= 1 required for D1V()")
    }

    /// `Standard_Boolean IsNormalDefined()` — true when D1U × D1V is non-zero.
    pub fn is_normal_defined(&self) -> bool {
        if let (Some(du), Some(dv)) = (self.d1u, self.d1v) {
            du.cross(dv).norm() > self.resolution
        } else {
            false
        }
    }

    /// `void Normal(V)` — the unit surface normal: N = (D1U × D1V) / |D1U × D1V|.
    ///
    /// # Panics
    /// Panics if the normal is not defined.
    pub fn normal(&self) -> Vec3 {
        let du = self.d1u.expect("SLProps: order >= 1 required for Normal()");
        let dv = self.d1v.expect("SLProps: order >= 1 required for Normal()");
        let cross = du.cross(dv);
        let n = cross.norm();
        assert!(n > self.resolution, "SLProps: Normal not defined (degenerate patch)");
        cross * (1.0 / n)
    }

    // ---- Second fundamental form (shape operator) -------------------------

    /// Returns the coefficients of the first fundamental form:
    /// E = D1U·D1U, F = D1U·D1V, G = D1V·D1V.
    pub fn first_fundamental_form(&self) -> (f64, f64, f64) {
        let du = self.d1u.expect("SLProps: order >= 1 required");
        let dv = self.d1v.expect("SLProps: order >= 1 required");
        let e = du.dot(du);
        let f = du.dot(dv);
        let g = dv.dot(dv);
        (e, f, g)
    }

    /// Returns the coefficients of the second fundamental form:
    /// L = N·D2U, M = N·D2UV, N_coeff = N·D2V,
    /// where N is the unit surface normal.
    pub fn second_fundamental_form(&self) -> (f64, f64, f64) {
        let n = self.normal();
        let d2u = self.d2u.expect("SLProps: order >= 2 required");
        let d2uv = self.d2uv.expect("SLProps: order >= 2 required");
        let d2v = self.d2v.expect("SLProps: order >= 2 required");
        let l = n.dot(d2u);
        let m = n.dot(d2uv);
        let nn = n.dot(d2v);
        (l, m, nn)
    }

    /// `Standard_Real GaussianCurvature()` — K = (LN − M²) / (EG − F²).
    pub fn gaussian_curvature(&self) -> f64 {
        let (e, f, g) = self.first_fundamental_form();
        let (l, m, nn) = self.second_fundamental_form();
        let denom = e * g - f * f;
        if denom.abs() <= self.resolution * self.resolution {
            return 0.0;
        }
        (l * nn - m * m) / denom
    }

    /// `Standard_Real MeanCurvature()` — H = (EN − 2FM + GL) / (2*(EG − F²)).
    pub fn mean_curvature(&self) -> f64 {
        let (e, f, g) = self.first_fundamental_form();
        let (l, m, nn) = self.second_fundamental_form();
        let denom = e * g - f * f;
        if denom.abs() <= self.resolution * self.resolution {
            return 0.0;
        }
        (e * nn - 2.0 * f * m + g * l) / (2.0 * denom)
    }

    /// `Standard_Integer MaxPrincipalCurvature(kmax, Dmax)` — the maximum
    /// principal curvature κ₁ = H + √(H² − K).
    ///
    /// Returns `(κ₁, κ₂)` — both principal curvatures.
    pub fn principal_curvatures(&self) -> (f64, f64) {
        let h = self.mean_curvature();
        let k = self.gaussian_curvature();
        let disc = (h * h - k).max(0.0);
        let sqrt_disc = disc.sqrt();
        (h + sqrt_disc, h - sqrt_disc)
    }

    /// `Standard_Real MaxPrincipalCurvature()` — κ₁ (the larger of the two).
    pub fn max_principal_curvature(&self) -> f64 {
        self.principal_curvatures().0
    }

    /// `Standard_Real MinPrincipalCurvature()` — κ₂ (the smaller of the two).
    pub fn min_principal_curvature(&self) -> f64 {
        self.principal_curvatures().1
    }

    /// `Standard_Boolean IsCurvatureDefined()` — true when order >= 2 and the
    /// surface has a well-defined normal.
    pub fn is_curvature_defined(&self) -> bool {
        self.order >= 2 && self.is_normal_defined() && self.d2u.is_some()
    }

    /// `Standard_Boolean IsUmbilic()` — true when the two principal curvatures
    /// are equal within a reasonable tolerance (the point is an umbilical point).
    ///
    /// Uses a relative tolerance: the difference |κ₁ − κ₂| must be less than
    /// `sqrt_resolution` times the maximum of |κ₁|, |κ₂|, to handle both small
    /// and large curvatures robustly. Falls back to the absolute `resolution` for
    /// nearly-flat surfaces.
    pub fn is_umbilic(&self) -> bool {
        let (k1, k2) = self.principal_curvatures();
        let diff = (k1 - k2).abs();
        let max_kappa = k1.abs().max(k2.abs());
        let tol = if max_kappa > 1.0 {
            self.resolution.sqrt() * max_kappa
        } else {
            self.resolution.sqrt()
        };
        diff <= tol
    }

    /// `Standard_Integer MaxPrincipalCurvature(kmax, Dmax)` — the principal
    /// direction corresponding to the maximum principal curvature.
    ///
    /// Uses the shape-operator eigenvector formula. Returns the unit vector in
    /// the surface tangent plane aligned with the max-curvature direction.
    pub fn max_principal_direction(&self) -> Vec3 {
        self.principal_direction(true)
    }

    /// The principal direction corresponding to the minimum principal curvature.
    pub fn min_principal_direction(&self) -> Vec3 {
        self.principal_direction(false)
    }

    /// Compute a principal direction by solving the shape-operator eigenvalue
    /// problem in the tangent plane.
    fn principal_direction(&self, max_dir: bool) -> Vec3 {
        let (e, f, g) = self.first_fundamental_form();
        let (l, m, nn) = self.second_fundamental_form();
        let kappa = if max_dir {
            self.max_principal_curvature()
        } else {
            self.min_principal_curvature()
        };
        // Shape operator: solve (II − κ·I)·v = 0 in the (D1U, D1V) basis.
        // [L−κE  M−κF] [a]   [0]
        // [M−κF  N−κG] [b] = [0]
        let a11 = l - kappa * e;
        let a12 = m - kappa * f;
        let a21 = m - kappa * f;
        let a22 = nn - kappa * g;
        let du = self.d1u.unwrap();
        let dv = self.d1v.unwrap();
        // Find the null-space vector (a, b) such that a11*a + a12*b = 0 (or the second row).
        let (a, b) = if a11.abs() > a21.abs() {
            if a12.abs() > a11.abs().max(a21.abs()) {
                (a12, -a11)
            } else {
                (-a12, a11)
            }
        } else {
            if a22.abs() > a21.abs() {
                (a22, -a21)
            } else {
                (-a22, a21)
            }
        };
        let dir = du * a + dv * b;
        let n = dir.norm();
        if n <= self.resolution {
            // Degenerate: return D1U normalized.
            return du.normalized();
        }
        dir * (1.0 / n)
    }
}

// ---------------------------------------------------------------------------
// Internal tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::{Ax1, Ax3, Pnt};
    use crate::geom_circle::GeomCircle;
    use crate::geom_line::GeomLine;
    use std::f64::consts::{FRAC_PI_2, PI};

    fn near(a: f64, b: f64, tol: f64, msg: &str) {
        assert!((a - b).abs() <= tol, "{msg}: expected {a} ≈ {b} (tol {tol})");
    }

    fn make_unit_circle() -> Curve {
        let ax = Ax3::identity();
        let c = GeomCircle::from_ax3(ax, 1.0);
        Curve::Circle(c)
    }

    fn make_x_line() -> Curve {
        let ax1 = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
        Curve::Line(GeomLine::new(ax1))
    }

    // -----------------------------------------------------------------------
    // CurveTool tests
    // -----------------------------------------------------------------------

    #[test]
    fn curve_tool_circle_first_last() {
        let c = make_unit_circle();
        near(CurveTool::first_parameter(&c), 0.0, 1e-12, "first");
        near(CurveTool::last_parameter(&c), std::f64::consts::TAU, 1e-12, "last");
    }

    #[test]
    fn curve_tool_circle_value_at_zero() {
        let c = make_unit_circle();
        let p = CurveTool::value(&c, 0.0);
        near(p.x, 1.0, 1e-10, "x");
        near(p.y, 0.0, 1e-10, "y");
        near(p.z, 0.0, 1e-10, "z");
    }

    #[test]
    fn curve_tool_line_d1() {
        let c = make_x_line();
        let (p, d1) = CurveTool::d1(&c, 3.0);
        near(p.x, 3.0, 1e-10, "px");
        near(d1.x, 1.0, 1e-10, "d1x");
        near(d1.y, 0.0, 1e-10, "d1y");
    }

    // -----------------------------------------------------------------------
    // CLProps — circle curvature
    // -----------------------------------------------------------------------

    #[test]
    fn clprops_circle_curvature() {
        // A unit circle has constant curvature = 1/r = 1.
        let c = make_unit_circle();
        let mut props = CLProps::new(c, 2, 1e-10);
        props.set_parameter(0.0);
        near(props.curvature(), 1.0, 1e-8, "curvature");
    }

    #[test]
    fn clprops_circle_tangent_perpendicular_to_radius() {
        let c = make_unit_circle();
        let mut props = CLProps::new(c, 2, 1e-10);
        props.set_parameter(0.0);
        let t = props.tangent();
        let p = props.value();
        // At u=0 the point is (1,0,0) and the tangent should be (0,1,0).
        near(t.x, 0.0, 1e-8, "tx");
        near(t.y, 1.0, 1e-8, "ty");
        near(t.dot(p), 0.0, 1e-8, "tangent·radius = 0");
    }

    #[test]
    fn clprops_circle_normal_points_to_center() {
        let c = make_unit_circle();
        let mut props = CLProps::new(c, 2, 1e-10);
        props.set_parameter(0.0);
        let n = props.normal();
        // For a unit circle at u=0, the normal points toward the center (−X).
        near(n.x, -1.0, 1e-8, "nx");
        near(n.y, 0.0, 1e-8, "ny");
    }

    #[test]
    fn clprops_line_zero_curvature() {
        let c = make_x_line();
        let mut props = CLProps::new(c, 2, 1e-10);
        props.set_parameter(0.0);
        near(props.curvature(), 0.0, 1e-12, "curvature");
        assert!(!props.is_normal_defined());
    }

    #[test]
    fn clprops_circle_centre_of_curvature_is_origin() {
        let c = make_unit_circle();
        let mut props = CLProps::new(c, 2, 1e-10);
        props.set_parameter(0.0);
        let coc = props.centre_of_curvature();
        near(coc.x, 0.0, 1e-8, "cx");
        near(coc.y, 0.0, 1e-8, "cy");
        near(coc.z, 0.0, 1e-8, "cz");
    }

    #[test]
    fn clprops_circle_torsion_is_zero() {
        // A planar circle has zero torsion.
        let c = make_unit_circle();
        let mut props = CLProps::new(c, 3, 1e-10);
        props.set_parameter(0.5);
        near(props.torsion(), 0.0, 1e-8, "torsion");
    }

    // -----------------------------------------------------------------------
    // SurfaceTool / SLProps — plane
    // -----------------------------------------------------------------------

    fn make_xy_plane() -> Surface {
        Surface::Plane {
            origin: Pnt::origin(),
            x_dir: Pnt::new(1.0, 0.0, 0.0),
            y_dir: Pnt::new(0.0, 1.0, 0.0),
            normal: Pnt::new(0.0, 0.0, 1.0),
        }
    }

    #[test]
    fn slprops_plane_normal_is_z() {
        let s = make_xy_plane();
        let mut props = SLProps::new(s, 2, 1e-10);
        props.set_parameters(1.0, 2.0);
        let n = props.normal();
        near(n.x, 0.0, 1e-10, "nx");
        near(n.y, 0.0, 1e-10, "ny");
        near(n.z, 1.0, 1e-10, "nz");
    }

    #[test]
    fn slprops_plane_zero_curvature() {
        let s = make_xy_plane();
        let mut props = SLProps::new(s, 2, 1e-10);
        props.set_parameters(0.0, 0.0);
        near(props.gaussian_curvature(), 0.0, 1e-12, "K");
        near(props.mean_curvature(), 0.0, 1e-12, "H");
    }

    // -----------------------------------------------------------------------
    // SLProps — sphere
    // -----------------------------------------------------------------------

    fn make_unit_sphere() -> Surface {
        Surface::Sphere {
            center: Pnt::origin(),
            radius: 1.0,
            z_dir: Pnt::new(0.0, 0.0, 1.0),
            x_dir: Pnt::new(1.0, 0.0, 0.0),
            y_dir: Pnt::new(0.0, 1.0, 0.0),
        }
    }

    #[test]
    fn slprops_sphere_gaussian_curvature() {
        // For a unit sphere K = 1/r² = 1.
        let s = make_unit_sphere();
        let mut props = SLProps::new(s, 2, 1e-10);
        props.set_parameters(0.0, 0.0);
        near(props.gaussian_curvature(), 1.0, 1e-8, "K sphere r=1");
    }

    #[test]
    fn slprops_sphere_mean_curvature() {
        // For a unit sphere H = 1/r = 1 (both principal curvatures = 1).
        let s = make_unit_sphere();
        let mut props = SLProps::new(s, 2, 1e-10);
        props.set_parameters(0.0, 0.0);
        near(props.mean_curvature(), 1.0, 1e-8, "H sphere r=1");
    }

    #[test]
    fn slprops_sphere_is_umbilic() {
        let s = make_unit_sphere();
        let mut props = SLProps::new(s, 2, 1e-10);
        props.set_parameters(0.0, 0.0);
        assert!(props.is_umbilic(), "sphere should be umbilic");
    }

    // -----------------------------------------------------------------------
    // SLProps — cylinder
    // -----------------------------------------------------------------------

    fn make_unit_cylinder() -> Surface {
        Surface::Cylinder {
            origin: Pnt::origin(),
            axis_dir: Pnt::new(0.0, 0.0, 1.0),
            x_dir: Pnt::new(1.0, 0.0, 0.0),
            y_dir: Pnt::new(0.0, 1.0, 0.0),
            radius: 1.0,
        }
    }

    #[test]
    fn slprops_cylinder_gaussian_curvature_zero() {
        // K = 0 for a cylinder (one principal curvature is zero).
        let s = make_unit_cylinder();
        let mut props = SLProps::new(s, 2, 1e-10);
        props.set_parameters(0.0, 0.0);
        near(props.gaussian_curvature(), 0.0, 1e-8, "K cylinder");
    }

    #[test]
    fn slprops_cylinder_mean_curvature() {
        // H = (κ₁ + κ₂)/2 = (1/r + 0)/2 = 0.5 for r=1.
        let s = make_unit_cylinder();
        let mut props = SLProps::new(s, 2, 1e-10);
        props.set_parameters(0.0, 0.0);
        near(props.mean_curvature().abs(), 0.5, 1e-8, "H cylinder r=1");
    }
}
