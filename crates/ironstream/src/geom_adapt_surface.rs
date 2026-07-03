// FILE: geom_adapt_surface.rs
//! `BRepAdaptor_Surface` / `BRepAdaptor_Curve` stubs — thin wrappers that
//! present a topological face or edge through the uniform parametric
//! surface/curve interface used by IronStream algorithms.
//!
//! These types are intentionally minimal stubs.  They carry enough state to
//! identify the underlying shape (by name) and its parameter domain, and they
//! provide evaluation methods that return plausible placeholder geometry
//! (identity plane for surfaces, unit X-line for curves).  Replace the stub
//! bodies with real evaluation logic once the topology layer is in place.

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Private pure-math helpers (zero external dependencies)
// ---------------------------------------------------------------------------

#[inline]
fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

#[inline]
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn norm3(a: [f64; 3]) -> f64 {
    dot3(a, a).sqrt()
}

#[inline]
fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn normalize3(a: [f64; 3]) -> [f64; 3] {
    let n = norm3(a);
    if n < 1e-14 {
        [0.0, 0.0, 1.0]
    } else {
        scale3(a, 1.0 / n)
    }
}

// ---------------------------------------------------------------------------
// BRepAdaptorSurface
// ---------------------------------------------------------------------------

/// Adapts a topological face to the standard parametric surface interface.
///
/// Mirrors `BRepAdaptor_Surface` from OpenCascade, exposing point evaluation
/// (`value`), first-derivative evaluation (`d1`), outward unit normal
/// (`normal`), and parametric domain / periodicity queries.
///
/// In this stub the underlying geometry is encoded as a `surface_type` string
/// (e.g. `"Plane"`, `"Cylinder"`, `"Sphere"`) and a rectangular parameter
/// domain.  Evaluation delegates to simple analytic formulae for the known
/// types; unknown types return the origin / zero vectors.
// occt: BRepAdaptor_Surface
#[derive(Debug, Clone)]
pub struct BrepAdaptorSurface {
    /// Name / identifier of the originating topological face.
    pub face: String,
    /// String discriminant for the underlying surface type
    /// (e.g. `"Plane"`, `"Cylinder"`, `"Cone"`, `"Sphere"`, `"Torus"`,
    /// `"BSpline"`, `"Other"`).
    pub surface_type: String,
    /// First parameter in the U direction.
    pub u_first: f64,
    /// Last parameter in the U direction.
    pub u_last: f64,
    /// First parameter in the V direction.
    pub v_first: f64,
    /// Last parameter in the V direction.
    pub v_last: f64,
}

impl BrepAdaptorSurface {
    /// Construct a default adaptor for the face identified by `face`.
    ///
    /// The stub initialises the surface as an infinite plane (`"Plane"`) with
    /// the parameter domain `[-1e6, 1e6] × [-1e6, 1e6]`.  Replace with real
    /// shape-query logic once the topology layer exists.
    // occt: BRepAdaptor_Surface::BRepAdaptor_Surface(const TopoDS_Face&)
    pub fn new(face: &str) -> Self {
        BrepAdaptorSurface {
            face: face.to_owned(),
            surface_type: "Plane".to_owned(),
            u_first: -1e6,
            u_last: 1e6,
            v_first: -1e6,
            v_last: 1e6,
        }
    }

    /// Evaluate the surface point at parameters `(u, v)`.
    ///
    /// For the stub `"Plane"` type this is the identity XY plane:
    /// `P(u,v) = (u, v, 0)`.  All other types return the origin.
    // occt: BRepAdaptor_Surface::Value / D0
    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        match self.surface_type.as_str() {
            "Plane" => [u, v, 0.0],
            "Cylinder" => {
                // Unit cylinder about the Z-axis: P(u,v) = (cos u, sin u, v).
                [u.cos(), u.sin(), v]
            }
            "Sphere" => {
                // Unit sphere: P(u,v) = (cos v · cos u, cos v · sin u, sin v).
                [v.cos() * u.cos(), v.cos() * u.sin(), v.sin()]
            }
            "Torus" => {
                // Unit torus with major radius 2, minor radius 1.
                let major = 2.0_f64;
                let minor = 1.0_f64;
                let cx = (major + minor * v.cos()) * u.cos();
                let cy = (major + minor * v.cos()) * u.sin();
                let cz = minor * v.sin();
                [cx, cy, cz]
            }
            _ => [0.0, 0.0, 0.0],
        }
    }

    /// Evaluate the surface point and first partial derivatives at `(u, v)`.
    ///
    /// Returns `(point, d/du, d/dv)`.
    // occt: BRepAdaptor_Surface::D1
    pub fn d1(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        match self.surface_type.as_str() {
            "Plane" => {
                let pt = [u, v, 0.0];
                let du = [1.0, 0.0, 0.0];
                let dv = [0.0, 1.0, 0.0];
                (pt, du, dv)
            }
            "Cylinder" => {
                let pt = [u.cos(), u.sin(), v];
                let du = [-u.sin(), u.cos(), 0.0];
                let dv = [0.0, 0.0, 1.0];
                (pt, du, dv)
            }
            "Sphere" => {
                let pt = self.value(u, v);
                let du = [-v.cos() * u.sin(), v.cos() * u.cos(), 0.0];
                let dv = [-v.sin() * u.cos(), -v.sin() * u.sin(), v.cos()];
                (pt, du, dv)
            }
            "Torus" => {
                let major = 2.0_f64;
                let minor = 1.0_f64;
                let pt = self.value(u, v);
                let du = [
                    -(major + minor * v.cos()) * u.sin(),
                    (major + minor * v.cos()) * u.cos(),
                    0.0,
                ];
                let dv = [
                    -minor * v.sin() * u.cos(),
                    -minor * v.sin() * u.sin(),
                    minor * v.cos(),
                ];
                (pt, du, dv)
            }
            _ => ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
        }
    }

    /// Outward unit normal at `(u, v)`.
    ///
    /// Computed as `normalize(d/du × d/dv)` from `d1`.
    // occt: BRepAdaptor_Surface normal (conceptually via DN / GeomLProp)
    pub fn normal(&self, u: f64, v: f64) -> [f64; 3] {
        let (_, du, dv) = self.d1(u, v);
        let n = cross3(du, dv);
        normalize3(n)
    }

    /// Returns `true` when the surface is closed (and periodic) in the U
    /// direction.
    // occt: BRepAdaptor_Surface::IsUClosed
    pub fn is_u_closed(&self) -> bool {
        matches!(
            self.surface_type.as_str(),
            "Cylinder" | "Cone" | "Sphere" | "Torus"
        )
    }

    /// Returns `true` when the surface is closed (and periodic) in the V
    /// direction.
    // occt: BRepAdaptor_Surface::IsVClosed
    pub fn is_v_closed(&self) -> bool {
        self.surface_type == "Torus"
    }

    /// Period in the U direction (2π for all analytic solids of revolution).
    ///
    /// # Panics
    /// Panics if the surface is not U-periodic.
    // occt: BRepAdaptor_Surface::UPeriod
    pub fn u_period(&self) -> f64 {
        if self.is_u_closed() {
            2.0 * PI
        } else {
            panic!(
                "BrepAdaptorSurface::u_period called on non-periodic surface ({})",
                self.surface_type
            )
        }
    }

    /// Period in the V direction (2π for the torus).
    ///
    /// # Panics
    /// Panics if the surface is not V-periodic.
    // occt: BRepAdaptor_Surface::VPeriod
    pub fn v_period(&self) -> f64 {
        if self.is_v_closed() {
            2.0 * PI
        } else {
            panic!(
                "BrepAdaptorSurface::v_period called on non-periodic surface ({})",
                self.surface_type
            )
        }
    }
}

// ---------------------------------------------------------------------------
// BrepAdaptorCurve
// ---------------------------------------------------------------------------

/// Adapts a topological edge to the standard parametric 3-D curve interface.
///
/// Mirrors `BRepAdaptor_Curve` from OpenCascade, exposing point evaluation
/// (`value`), first-derivative evaluation (`d1`), and a chord-length
/// approximation of the arc length (`length`).
///
/// In this stub the underlying geometry is an implicit unit X-axis line:
/// `P(t) = (t, 0, 0)`.  Replace the evaluation bodies with real curve
/// dispatch once the topology layer is available.
// occt: BRepAdaptor_Curve
#[derive(Debug, Clone)]
pub struct BrepAdaptorCurve {
    /// Name / identifier of the originating topological edge.
    pub edge: String,
    /// First parameter of the valid curve range.
    pub first: f64,
    /// Last parameter of the valid curve range.
    pub last: f64,
}

impl BrepAdaptorCurve {
    /// Construct a default adaptor for the edge identified by `edge`.
    ///
    /// The stub initialises the parameter range to `[0, 1]` and the
    /// underlying geometry to a unit segment along the X axis.  Replace with
    /// real shape-query logic once the topology layer exists.
    // occt: BRepAdaptor_Curve::BRepAdaptor_Curve(const TopoDS_Edge&)
    pub fn new(edge: &str) -> Self {
        BrepAdaptorCurve {
            edge: edge.to_owned(),
            first: 0.0,
            last: 1.0,
        }
    }

    /// Evaluate the curve point at parameter `t`.
    ///
    /// Stub: identity X-axis line, `P(t) = (t, 0, 0)`.
    // occt: BRepAdaptor_Curve::Value / D0
    pub fn value(&self, t: f64) -> [f64; 3] {
        [t, 0.0, 0.0]
    }

    /// Evaluate the curve point and first derivative at parameter `t`.
    ///
    /// Returns `(point, tangent)`.
    ///
    /// Stub: identity X-axis line — tangent is always `(1, 0, 0)`.
    // occt: BRepAdaptor_Curve::D1
    pub fn d1(&self, t: f64) -> ([f64; 3], [f64; 3]) {
        ([t, 0.0, 0.0], [1.0, 0.0, 0.0])
    }

    /// Approximate arc length of the curve over `[first, last]` using
    /// 5-point Gauss-Legendre quadrature on the speed `|dP/dt|`.
    ///
    /// Stub: for the X-axis line the speed is exactly 1, so the result is
    /// `last - first`.
    // occt: BRepAdaptor_Curve (length via GCPnts_AbscissaPoint or similar)
    pub fn length(&self) -> f64 {
        gauss_legendre_length(self, self.first, self.last)
    }
}

// ---------------------------------------------------------------------------
// Arc-length integration helper (5-point Gauss-Legendre, no external crates)
// ---------------------------------------------------------------------------

/// Gauss-Legendre 5-point nodes on `[-1, 1]`.
const GL5_NODES: [f64; 5] = [
    -0.906_179_845_938_664,
    -0.538_469_310_105_683,
     0.0,
     0.538_469_310_105_683,
     0.906_179_845_938_664,
];

/// Gauss-Legendre 5-point weights on `[-1, 1]`.
const GL5_WEIGHTS: [f64; 5] = [
    0.236_926_885_056_189,
    0.478_628_670_499_366,
    0.568_888_888_888_889,
    0.478_628_670_499_366,
    0.236_926_885_056_189,
];

/// Integrate `|dP/dt|` over `[a, b]` using GL-5 quadrature.
fn gauss_legendre_length(curve: &BrepAdaptorCurve, a: f64, b: f64) -> f64 {
    let mid = 0.5 * (a + b);
    let half = 0.5 * (b - a);
    let mut len = 0.0_f64;
    for (&xi, &wi) in GL5_NODES.iter().zip(GL5_WEIGHTS.iter()) {
        let t = mid + half * xi;
        let (_, d) = curve.d1(t);
        len += wi * norm3(d);
    }
    len * half
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    const EPS: f64 = 1e-9;
    const EPS_APPROX: f64 = 1e-4;

    fn vec_eq(a: [f64; 3], b: [f64; 3], eps: f64) -> bool {
        (a[0] - b[0]).abs() < eps
            && (a[1] - b[1]).abs() < eps
            && (a[2] - b[2]).abs() < eps
    }

    // -----------------------------------------------------------------------
    // BrepAdaptorSurface – construction
    // -----------------------------------------------------------------------

    #[test]
    fn surface_new_fields() {
        let s = BrepAdaptorSurface::new("face_001");
        assert_eq!(s.face, "face_001");
        assert_eq!(s.surface_type, "Plane");
        assert!(s.u_first < s.u_last);
        assert!(s.v_first < s.v_last);
    }

    // -----------------------------------------------------------------------
    // BrepAdaptorSurface – Plane
    // -----------------------------------------------------------------------

    #[test]
    fn plane_value() {
        let s = BrepAdaptorSurface::new("f");
        assert!(vec_eq(s.value(3.0, 4.0), [3.0, 4.0, 0.0], EPS));
    }

    #[test]
    fn plane_d1_derivatives() {
        let s = BrepAdaptorSurface::new("f");
        let (pt, du, dv) = s.d1(1.0, 2.0);
        assert!(vec_eq(pt, [1.0, 2.0, 0.0], EPS));
        assert!(vec_eq(du, [1.0, 0.0, 0.0], EPS));
        assert!(vec_eq(dv, [0.0, 1.0, 0.0], EPS));
    }

    #[test]
    fn plane_normal_is_z() {
        let s = BrepAdaptorSurface::new("f");
        let n = s.normal(0.0, 0.0);
        assert!((n[2].abs() - 1.0).abs() < EPS);
    }

    #[test]
    fn plane_not_closed() {
        let s = BrepAdaptorSurface::new("f");
        assert!(!s.is_u_closed());
        assert!(!s.is_v_closed());
    }

    #[test]
    #[should_panic]
    fn plane_u_period_panics() {
        let s = BrepAdaptorSurface::new("f");
        let _ = s.u_period();
    }

    // -----------------------------------------------------------------------
    // BrepAdaptorSurface – Cylinder
    // -----------------------------------------------------------------------

    #[test]
    fn cylinder_value_at_zero() {
        let mut s = BrepAdaptorSurface::new("f");
        s.surface_type = "Cylinder".to_owned();
        let pt = s.value(0.0, 0.0);
        assert!(vec_eq(pt, [1.0, 0.0, 0.0], EPS));
    }

    #[test]
    fn cylinder_is_u_closed() {
        let mut s = BrepAdaptorSurface::new("f");
        s.surface_type = "Cylinder".to_owned();
        assert!(s.is_u_closed());
        assert!(!s.is_v_closed());
    }

    #[test]
    fn cylinder_u_period_is_two_pi() {
        let mut s = BrepAdaptorSurface::new("f");
        s.surface_type = "Cylinder".to_owned();
        assert!((s.u_period() - 2.0 * PI).abs() < EPS);
    }

    #[test]
    fn cylinder_d1_fd_consistency() {
        let mut s = BrepAdaptorSurface::new("f");
        s.surface_type = "Cylinder".to_owned();
        let u = 0.7_f64;
        let v = 1.5_f64;
        let h = 1e-6_f64;
        let (_, du, dv) = s.d1(u, v);
        // finite-difference du
        let pu = add3(s.value(u + h, v), scale3(s.value(u - h, v), -1.0));
        let fd_du = scale3(pu, 1.0 / (2.0 * h));
        assert!((du[0] - fd_du[0]).abs() < EPS_APPROX);
        assert!((du[1] - fd_du[1]).abs() < EPS_APPROX);
        // finite-difference dv
        let pv = add3(s.value(u, v + h), scale3(s.value(u, v - h), -1.0));
        let fd_dv = scale3(pv, 1.0 / (2.0 * h));
        assert!((dv[2] - fd_dv[2]).abs() < EPS_APPROX);
    }

    // -----------------------------------------------------------------------
    // BrepAdaptorSurface – Sphere
    // -----------------------------------------------------------------------

    #[test]
    fn sphere_points_on_unit_sphere() {
        let mut s = BrepAdaptorSurface::new("f");
        s.surface_type = "Sphere".to_owned();
        for i in 0..8 {
            for j in 0..4 {
                let u = i as f64 * PI / 4.0;
                let v = -PI / 2.0 + j as f64 * PI / 3.0;
                let pt = s.value(u, v);
                let r = norm3(pt);
                assert!((r - 1.0).abs() < EPS_APPROX, "r={} at u={} v={}", r, u, v);
            }
        }
    }

    #[test]
    fn sphere_normal_is_outward() {
        let mut s = BrepAdaptorSurface::new("f");
        s.surface_type = "Sphere".to_owned();
        let pt = s.value(0.3, 0.5);
        let n = s.normal(0.3, 0.5);
        // normal dot position should be 1 for unit sphere
        let d = dot3(pt, n);
        assert!((d - 1.0).abs() < EPS_APPROX, "dot={}", d);
    }

    // -----------------------------------------------------------------------
    // BrepAdaptorSurface – Torus
    // -----------------------------------------------------------------------

    #[test]
    fn torus_is_closed_both_directions() {
        let mut s = BrepAdaptorSurface::new("f");
        s.surface_type = "Torus".to_owned();
        assert!(s.is_u_closed());
        assert!(s.is_v_closed());
        assert!((s.u_period() - 2.0 * PI).abs() < EPS);
        assert!((s.v_period() - 2.0 * PI).abs() < EPS);
    }

    #[test]
    fn torus_value_at_zero() {
        let mut s = BrepAdaptorSurface::new("f");
        s.surface_type = "Torus".to_owned();
        // major=2, minor=1 → at (0,0): (3,0,0)
        let pt = s.value(0.0, 0.0);
        assert!(vec_eq(pt, [3.0, 0.0, 0.0], EPS));
    }

    // -----------------------------------------------------------------------
    // BrepAdaptorCurve – construction
    // -----------------------------------------------------------------------

    #[test]
    fn curve_new_fields() {
        let c = BrepAdaptorCurve::new("edge_001");
        assert_eq!(c.edge, "edge_001");
        assert!(c.first < c.last);
    }

    // -----------------------------------------------------------------------
    // BrepAdaptorCurve – evaluation
    // -----------------------------------------------------------------------

    #[test]
    fn curve_value_is_t_on_x_axis() {
        let c = BrepAdaptorCurve::new("e");
        assert!(vec_eq(c.value(0.0), [0.0, 0.0, 0.0], EPS));
        assert!(vec_eq(c.value(1.0), [1.0, 0.0, 0.0], EPS));
        assert!(vec_eq(c.value(0.5), [0.5, 0.0, 0.0], EPS));
    }

    #[test]
    fn curve_d1_tangent_is_unit_x() {
        let c = BrepAdaptorCurve::new("e");
        let (pt, tan) = c.d1(0.7);
        assert!(vec_eq(pt, [0.7, 0.0, 0.0], EPS));
        assert!(vec_eq(tan, [1.0, 0.0, 0.0], EPS));
    }

    #[test]
    fn curve_length_equals_parameter_range() {
        let mut c = BrepAdaptorCurve::new("e");
        c.first = 0.0;
        c.last = 5.0;
        let l = c.length();
        assert!((l - 5.0).abs() < EPS_APPROX, "length={}", l);
    }

    #[test]
    fn curve_length_unit_interval() {
        let c = BrepAdaptorCurve::new("e");
        assert!((c.length() - 1.0).abs() < EPS_APPROX);
    }
}
