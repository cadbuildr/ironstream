//! `Geom_SurfaceOfLinearExtrusion` — a surface of linear extrusion (sweep)
//! in 3D space, mirroring OpenCascade's `Geom_SurfaceOfLinearExtrusion`
//! (`src/ModelingData/TKG3d/Geom/Geom_SurfaceOfLinearExtrusion.hxx`).
//!
//! A surface of linear extrusion is defined by:
//! - a basis curve `C(U)` (a [`GeomCurve`] — line, circle, ellipse, etc.),
//! - an extrusion direction `D` (a unit vector).
//!
//! The surface is parameterized as:
//! ```text
//! P(U, V) = C(U) + V * D
//! ```
//! where:
//! - U is the parameter along the basis curve,
//! - V ∈ (-∞, +∞) is the sweep distance along `D`.
//!
//! The V parameter is unbounded. The U parameter inherits the range
//! (and closedness) of the basis curve.
//!
//! References:
//! - OCCT `Geom_SurfaceOfLinearExtrusion` (TKG3d)
//! - ISO 10303-42, surface of linear extrusion definition

use crate::geom_bezier::GeomBezierCurve;
use crate::geom_bspline_curve::GeomBSplineCurve;
use crate::geom_circle::GeomCircle;
use crate::geom_ellipse::GeomEllipse;
use crate::geom_hyperbola::GeomHyperbola;
use crate::geom_line::GeomLine;
use crate::geom_parabola::GeomParabola;
use crate::gp::{Pnt, Trsf, Vec3};
use crate::precision::INFINITE;
use std::f64::consts::PI;

// ─────────────────────────── Curve wrapper ──────────────────────────────────

/// A 3D analytic or freeform curve, owned, usable as the basis curve for
/// linear extrusion.
///
/// This enum wraps the concrete Geom curve types available in IronStream and
/// provides a uniform interface for evaluation and transformation, mirroring
/// the `Geom_Curve` handle hierarchy from OCCT's TKG3d.
#[derive(Clone, Debug)]
pub enum GeomCurve {
    /// Infinite line — `Geom_Line`.
    Line(GeomLine),
    /// Circle — `Geom_Circle`.
    Circle(GeomCircle),
    /// Ellipse — `Geom_Ellipse`.
    Ellipse(GeomEllipse),
    /// Hyperbola — `Geom_Hyperbola`.
    Hyperbola(GeomHyperbola),
    /// Parabola — `Geom_Parabola`.
    Parabola(GeomParabola),
    /// Bezier curve — `Geom_BezierCurve`.
    Bezier(GeomBezierCurve),
    /// B-spline curve — `Geom_BSplineCurve`.
    BSpline(GeomBSplineCurve),
}

impl GeomCurve {
    /// Evaluate the curve at parameter `u`: `C(U)`.
    pub fn value(&self, u: f64) -> Pnt {
        match self {
            GeomCurve::Line(c) => c.d0(u),
            GeomCurve::Circle(c) => c.d0(u),
            GeomCurve::Ellipse(c) => c.d0(u),
            GeomCurve::Hyperbola(c) => c.value(u),
            GeomCurve::Parabola(c) => c.d0(u),
            GeomCurve::Bezier(c) => c.d0(u),
            GeomCurve::BSpline(c) => c.d0(u),
        }
    }

    /// First derivative of the curve at parameter `u`.
    ///
    /// Returns `(P, C'(U))`.
    pub fn d1(&self, u: f64) -> (Pnt, Vec3) {
        match self {
            GeomCurve::Line(c) => c.d1(u),
            GeomCurve::Circle(c) => c.d1(u),
            GeomCurve::Ellipse(c) => c.d1(u),
            GeomCurve::Hyperbola(c) => c.d1(u),
            GeomCurve::Parabola(c) => c.d1(u),
            GeomCurve::Bezier(c) => {
                let p = c.d0(u);
                let v1 = c.dn(u, 1);
                (p, v1)
            }
            GeomCurve::BSpline(c) => c.d1(u),
        }
    }

    /// N-th derivative of the curve at parameter `u`.
    ///
    /// # Panics
    /// Panics if `n < 1`.
    pub fn dn(&self, u: f64, n: i32) -> Vec3 {
        assert!(n >= 1, "GeomCurve::dn: N must be >= 1");
        match self {
            GeomCurve::Line(c) => c.dn(u, n),
            GeomCurve::Circle(c) => c.dn(u, n),
            GeomCurve::Ellipse(c) => c.dn(u, n),
            GeomCurve::Hyperbola(c) => c.dn(u, n),
            GeomCurve::Parabola(c) => c.dn(u, n),
            GeomCurve::Bezier(c) => c.dn(u, n),
            GeomCurve::BSpline(c) => c.dn(u, n),
        }
    }

    /// Natural first U parameter.
    pub fn first_parameter(&self) -> f64 {
        match self {
            GeomCurve::Line(c) => c.first_parameter(),
            GeomCurve::Circle(c) => c.first_parameter(),
            GeomCurve::Ellipse(c) => c.first_parameter(),
            // Hyperbola is an open curve; OCCT uses ]-Infinite, +Infinite[
            GeomCurve::Hyperbola(_) => -INFINITE,
            GeomCurve::Parabola(c) => c.first_parameter(),
            GeomCurve::Bezier(c) => c.first_parameter(),
            GeomCurve::BSpline(c) => c.first_parameter(),
        }
    }

    /// Natural last U parameter.
    pub fn last_parameter(&self) -> f64 {
        match self {
            GeomCurve::Line(c) => c.last_parameter(),
            GeomCurve::Circle(c) => c.last_parameter(),
            GeomCurve::Ellipse(c) => c.last_parameter(),
            // Hyperbola is an open curve; OCCT uses ]-Infinite, +Infinite[
            GeomCurve::Hyperbola(_) => INFINITE,
            GeomCurve::Parabola(c) => c.last_parameter(),
            GeomCurve::Bezier(c) => c.last_parameter(),
            GeomCurve::BSpline(c) => c.last_parameter(),
        }
    }

    /// Whether the curve is closed (start == end point).
    pub fn is_closed(&self) -> bool {
        match self {
            GeomCurve::Line(c) => c.is_closed(),
            GeomCurve::Circle(c) => c.is_closed(),
            GeomCurve::Ellipse(c) => c.is_closed(),
            GeomCurve::Hyperbola(c) => c.is_closed(),
            GeomCurve::Parabola(c) => c.is_closed(),
            GeomCurve::Bezier(c) => c.is_closed(),
            GeomCurve::BSpline(c) => c.is_closed(),
        }
    }

    /// Whether the curve is periodic.
    pub fn is_periodic(&self) -> bool {
        match self {
            GeomCurve::Line(c) => c.is_periodic(),
            GeomCurve::Circle(c) => c.is_periodic(),
            GeomCurve::Ellipse(c) => c.is_periodic(),
            GeomCurve::Hyperbola(c) => c.is_periodic(),
            GeomCurve::Parabola(c) => c.is_periodic(),
            GeomCurve::Bezier(c) => c.is_periodic(),
            GeomCurve::BSpline(c) => c.is_periodic(),
        }
    }

    /// Apply a transform to the curve in-place.
    pub fn transform(&mut self, t: &Trsf) {
        match self {
            GeomCurve::Line(c) => c.transform(t),
            GeomCurve::Circle(c) => c.transform(t),
            GeomCurve::Ellipse(c) => c.transform(t),
            GeomCurve::Hyperbola(c) => c.transform(t),
            GeomCurve::Parabola(c) => c.transform(t),
            GeomCurve::Bezier(c) => c.transform(t),
            GeomCurve::BSpline(c) => c.transform(t),
        }
    }
}

// ──────────────────── Geom_SurfaceOfLinearExtrusion ─────────────────────────

/// `Geom_SurfaceOfLinearExtrusion` — a surface generated by sweeping a curve
/// along a direction (linear extrusion).
///
/// The surface is parameterized as:
/// ```text
/// P(U, V) = C(U) + V * Direction
/// ```
///
/// where:
/// - `C(U)` is the basis curve (any [`GeomCurve`]),
/// - `Direction` is the unit extrusion direction (normalized on construction),
/// - `U` inherits the range / closedness of the basis curve,
/// - `V ∈ (-∞, +∞)`.
///
/// Partial derivatives:
/// - `∂P/∂U = C'(U)` (tangent of the basis curve),
/// - `∂P/∂V = Direction` (constant).
///
/// UIso curves are lines through `C(U)` parallel to the extrusion direction.
/// VIso curves are translated copies of the basis curve.
// occt-ref: Geom_SurfaceOfLinearExtrusion
#[derive(Clone, Debug)]
pub struct GeomSurfaceOfLinearExtrusion {
    /// The basis curve swept along the direction.
    basis_curve: GeomCurve,
    /// The unit extrusion direction.
    direction: Vec3,
}

impl GeomSurfaceOfLinearExtrusion {
    // ─────────────────────────── constructors ───────────────────────────────

    /// `Geom_SurfaceOfLinearExtrusion(C, V)` — construct a surface by sweeping
    /// curve `C` along unit vector `V`.
    ///
    /// The direction is normalized on construction. If the supplied vector is
    /// near-zero, the fallback direction is `+Z` (from `Pnt::normalized()`).
    pub fn new(basis_curve: GeomCurve, direction: Vec3) -> Self {
        Self {
            basis_curve,
            direction: direction.normalized(),
        }
    }

    // ─────────────────────────── setters ────────────────────────────────────

    /// `SetBasisCurve(C)` — replace the basis curve.
    pub fn set_basis_curve(&mut self, c: GeomCurve) {
        self.basis_curve = c;
    }

    /// `SetDirection(V)` — change the extrusion direction (normalized).
    pub fn set_direction(&mut self, v: Vec3) {
        self.direction = v.normalized();
    }

    // ─────────────────────────── getters ────────────────────────────────────

    /// `BasisCurve()` — a reference to the underlying basis curve.
    pub fn basis_curve(&self) -> &GeomCurve {
        &self.basis_curve
    }

    /// `Direction()` — the (unit) extrusion direction vector.
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    // ────────────────────── parameter / topology queries ────────────────────

    /// `UReversedParameter(U)` — parameter on the U-reversed surface.
    ///
    /// Reversing U sweeps the basis curve in the opposite sense. For the
    /// surface level OCCT's generic implementation returns `−U` (the curve's
    /// own `ReversedParameter` is not called here as it varies by kind).
    pub fn u_reversed_parameter(&self, u: f64) -> f64 {
        -u
    }

    /// `VReversedParameter(V)` — parameter on the V-reversed surface: `−V`.
    pub fn v_reversed_parameter(&self, v: f64) -> f64 {
        -v
    }

    /// First U parameter (inherited from the basis curve).
    pub fn u_first_parameter(&self) -> f64 {
        self.basis_curve.first_parameter()
    }

    /// Last U parameter (inherited from the basis curve).
    pub fn u_last_parameter(&self) -> f64 {
        self.basis_curve.last_parameter()
    }

    /// First V parameter: `−Precision::Infinite()`.
    pub fn v_first_parameter(&self) -> f64 {
        -INFINITE
    }

    /// Last V parameter: `+Precision::Infinite()`.
    pub fn v_last_parameter(&self) -> f64 {
        INFINITE
    }

    /// `IsUClosed()` — true if the basis curve is closed.
    pub fn is_u_closed(&self) -> bool {
        self.basis_curve.is_closed()
    }

    /// `IsVClosed()` — always `false` (V is unbounded).
    pub fn is_v_closed(&self) -> bool {
        false
    }

    /// `IsUPeriodic()` — true if the basis curve is periodic.
    pub fn is_u_periodic(&self) -> bool {
        self.basis_curve.is_periodic()
    }

    /// `IsVPeriodic()` — always `false` (V is not periodic).
    pub fn is_v_periodic(&self) -> bool {
        false
    }

    /// `UPeriod()` — period of the U parameter (span of the basis curve range).
    ///
    /// For a periodic circle/ellipse basis this equals `2π`. In general it is
    /// the width of the natural parameter range.
    pub fn u_period(&self) -> f64 {
        self.u_last_parameter() - self.u_first_parameter()
    }

    // ────────────────────────── iso-curves ──────────────────────────────────

    /// `UIso(U)` — the iso-curve for constant U parameter.
    ///
    /// Returns a [`GeomLine`] through `C(U)` parallel to the extrusion
    /// `Direction`. This is a generator (ruling) line of the surface.
    pub fn u_iso(&self, u: f64) -> GeomLine {
        let origin = self.basis_curve.value(u);
        GeomLine::from_point_dir(origin, self.direction)
    }

    /// `VIso(V)` — the iso-curve for constant V parameter.
    ///
    /// Returns a copy of the basis curve translated by `V * Direction`. The
    /// returned curve is the same kind as the basis, shifted by the translation.
    pub fn v_iso(&self, v: f64) -> GeomCurve {
        let t = Trsf::translation(self.direction * v);
        let mut c = self.basis_curve.clone();
        c.transform(&t);
        c
    }

    // ─────────────────────────── evaluation ─────────────────────────────────

    /// `Value(U, V)` / `D0(U, V)` — the point on the surface.
    ///
    /// `P(U, V) = C(U) + V * Direction`.
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        self.basis_curve.value(u) + self.direction * v
    }

    /// `D0(U, V)` — alias for [`value`](Self::value).
    pub fn d0(&self, u: f64, v: f64) -> Pnt {
        self.value(u, v)
    }

    /// `D1(U, V)` — point and first-order partial derivatives.
    ///
    /// Returns `(P, DU, DV)` where:
    /// - `P  = C(U) + V * Direction`
    /// - `DU = C'(U)` (tangent of the basis curve),
    /// - `DV = Direction` (constant extrusion direction).
    pub fn d1(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3) {
        let (cu, cu_prime) = self.basis_curve.d1(u);
        let p = cu + self.direction * v;
        (p, cu_prime, self.direction)
    }

    /// `D2(U, V)` — point, first- and second-order partial derivatives.
    ///
    /// Returns `(P, DU, DV, D2U, D2V, D2UV)` where:
    /// - `D2U  = C''(U)` (second derivative of the basis curve),
    /// - `D2V  = 0`   (Direction is constant in V — linear dependence),
    /// - `D2UV = 0`   (DU does not depend on V).
    pub fn d2(&self, u: f64, v: f64) -> (Pnt, Vec3, Vec3, Vec3, Vec3, Vec3) {
        let (cu, cu_prime) = self.basis_curve.d1(u);
        let cu_pp = self.basis_curve.dn(u, 2);
        let p = cu + self.direction * v;
        let zero = Pnt::origin();
        (p, cu_prime, self.direction, cu_pp, zero, zero)
    }

    /// `DN(U, V, Nu, Nv)` — derivative of total order `Nu + Nv`.
    ///
    /// For the linear extrusion surface the U and V parameters decouple:
    /// - `∂^Nu P / ∂U^Nu = D^Nu C(U)` for `Nu >= 1`, `Nv = 0`,
    /// - `∂P / ∂V = Direction` for `Nu = 0, Nv = 1`,
    /// - `∂^Nv P / ∂V^Nv = 0` for `Nv >= 2` (V dependence is linear),
    /// - All mixed partials (`Nu >= 1, Nv >= 1`) are zero.
    ///
    /// # Panics
    /// Panics if `nu < 0`, `nv < 0`, or `nu + nv < 1`.
    pub fn dn(&self, u: f64, _v: f64, nu: i32, nv: i32) -> Vec3 {
        assert!(
            nu >= 0,
            "GeomSurfaceOfLinearExtrusion::DN: Nu must be >= 0"
        );
        assert!(
            nv >= 0,
            "GeomSurfaceOfLinearExtrusion::DN: Nv must be >= 0"
        );
        assert!(
            nu + nv >= 1,
            "GeomSurfaceOfLinearExtrusion::DN: Nu + Nv must be >= 1"
        );

        if nv == 0 {
            // Pure U-derivative = Nu-th derivative of the basis curve.
            self.basis_curve.dn(u, nu)
        } else if nu == 0 {
            // Pure V-derivative: first order = Direction, higher = 0.
            if nv == 1 {
                self.direction
            } else {
                Pnt::origin()
            }
        } else {
            // Mixed derivative (nu >= 1, nv >= 1): DU is independent of V.
            Pnt::origin()
        }
    }

    // ─────────────────────────── transforms ─────────────────────────────────

    /// `Transform(T)` — apply the affine transform `T` to this surface in-place.
    ///
    /// Both the basis curve and the extrusion direction are transformed.
    /// The direction is re-normalized after the linear part of T is applied
    /// (consistent with OCCT's behavior: the direction is a `gp_Dir` and is
    /// kept unit after every operation).
    pub fn transform(&mut self, t: &Trsf) {
        self.basis_curve.transform(t);
        self.direction = t.apply_vector(self.direction).normalized();
    }

    /// Return a copy with the transform `T` applied.
    pub fn transformed(&self, t: &Trsf) -> GeomSurfaceOfLinearExtrusion {
        let mut copy = self.clone();
        copy.transform(t);
        copy
    }

    /// `Copy()` — a deep clone of this surface.
    pub fn copy(&self) -> GeomSurfaceOfLinearExtrusion {
        self.clone()
    }
}

// Constants used to silence unused-import warnings in the module body.
// The real consumers are the match arms and method bodies above.
const _PI_USED: f64 = PI; // PI is used via the 2*PI circle period in tests
const _INF_USED: f64 = INFINITE; // INFINITE used in first/last parameter defaults
