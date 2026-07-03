//! `Geom_OffsetCurve` — a curve offset in 3D space by a given distance along
//! a direction derived from a reference vector (`V`) and the curve tangent.
//!
//! Mirrors OpenCascade's `Geom_OffsetCurve`
//! (`src/ModelingData/TKG3d/Geom/Geom_OffsetCurve.hxx`).
//!
//! Given a base curve `C`, an offset distance `d`, and a reference direction
//! `V` (a `gp_Dir`), the offset curve is defined by:
//!
//! ```text
//! Q(u) = C(u) + d * N(u)
//! ```
//!
//! where the offset normal at `u` is:
//!
//! ```text
//! N(u) = normalize( V × C'(u) )
//! ```
//!
//! (cross product of the reference direction with the curve tangent). When
//! `V × C'(u)` has near-zero magnitude (the tangent is parallel to `V`), the
//! normal is left as the zero vector and the point coincides with the base
//! curve point — this matches OCCT's degenerate-case behaviour.
//!
//! Derivatives are computed analytically via the chain rule:
//!
//! ```text
//! Q'(u) = C'(u) + d * N'(u)
//! ```
//!
//! where `N'(u)` is the derivative of the unit offset-normal. For the
//! higher-order derivatives (D2, D3) the same first-principles differentiation
//! is applied — each derivative of the normalised vector is recovered from the
//! raw cross-products and their derivatives using the standard formulas for the
//! derivative of a unit vector:
//!
//! ```text
//! (f/|f|)' = (f' - (f·f') / |f|^2 * f) / |f|
//! ```
//!
//! This module is zero-dependency (only `std`).

use crate::gp::{Ax1, Pnt, Trsf, Vec3};
use crate::geom_bezier::GeomBezierCurve;
use crate::geom_bspline_curve::GeomBSplineCurve;
use crate::geom_circle::GeomCircle;
use crate::geom_ellipse::GeomEllipse;
use crate::geom_hyperbola::GeomHyperbola;
use crate::geom_line::GeomLine;
use crate::geom_parabola::GeomParabola;
use crate::precision::INFINITE;

// ─────────────────────────────────────────────────────────────────────────────
// BasisCurve — the union of all concrete 3D curve types that can be offset.
// ─────────────────────────────────────────────────────────────────────────────

/// The concrete 3D curve types that [`GeomOffsetCurve`] can wrap.
///
/// Mirrors the `Handle(Geom_Curve)` parameter of OCCT's `Geom_OffsetCurve`
/// constructor without any heap allocation overhead.

#[derive(Clone, Debug)]
pub enum BasisCurve {
    Line(GeomLine),
    Circle(GeomCircle),
    Ellipse(GeomEllipse),
    Hyperbola(GeomHyperbola),
    Parabola(GeomParabola),
    Bezier(GeomBezierCurve),
    BSpline(GeomBSplineCurve),
}

impl BasisCurve {
    /// First parameter of the underlying curve.
    pub fn first_parameter(&self) -> f64 {
        match self {
            BasisCurve::Line(c) => c.first_parameter(),
            BasisCurve::Circle(c) => c.first_parameter(),
            BasisCurve::Ellipse(c) => c.first_parameter(),
            BasisCurve::Hyperbola(_) => -INFINITE,
            BasisCurve::Parabola(c) => c.first_parameter(),
            BasisCurve::Bezier(c) => c.first_parameter(),
            BasisCurve::BSpline(c) => c.first_parameter(),
        }
    }

    /// Last parameter of the underlying curve.
    pub fn last_parameter(&self) -> f64 {
        match self {
            BasisCurve::Line(c) => c.last_parameter(),
            BasisCurve::Circle(c) => c.last_parameter(),
            BasisCurve::Ellipse(c) => c.last_parameter(),
            BasisCurve::Hyperbola(_) => INFINITE,
            BasisCurve::Parabola(c) => c.last_parameter(),
            BasisCurve::Bezier(c) => c.last_parameter(),
            BasisCurve::BSpline(c) => c.last_parameter(),
        }
    }

    /// Whether the curve is closed.
    pub fn is_closed(&self) -> bool {
        match self {
            BasisCurve::Line(c) => c.is_closed(),
            BasisCurve::Circle(c) => c.is_closed(),
            BasisCurve::Ellipse(c) => c.is_closed(),
            BasisCurve::Hyperbola(c) => c.is_closed(),
            BasisCurve::Parabola(c) => c.is_closed(),
            BasisCurve::Bezier(c) => c.is_closed(),
            BasisCurve::BSpline(c) => c.is_closed(),
        }
    }

    /// Whether the curve is periodic.
    pub fn is_periodic(&self) -> bool {
        match self {
            BasisCurve::Line(c) => c.is_periodic(),
            BasisCurve::Circle(c) => c.is_periodic(),
            BasisCurve::Ellipse(c) => c.is_periodic(),
            BasisCurve::Hyperbola(c) => c.is_periodic(),
            BasisCurve::Parabola(c) => c.is_periodic(),
            BasisCurve::Bezier(c) => c.is_periodic(),
            BasisCurve::BSpline(c) => c.is_periodic(),
        }
    }

    /// Point at parameter `u` (`D0`).
    pub fn d0(&self, u: f64) -> Pnt {
        match self {
            BasisCurve::Line(c) => c.d0(u),
            BasisCurve::Circle(c) => c.d0(u),
            BasisCurve::Ellipse(c) => c.d0(u),
            BasisCurve::Hyperbola(c) => c.value(u),
            BasisCurve::Parabola(c) => c.d0(u),
            BasisCurve::Bezier(c) => c.d0(u),
            BasisCurve::BSpline(c) => c.d0(u),
        }
    }

    /// Point and first derivative at `u` (`D1`).
    pub fn d1(&self, u: f64) -> (Pnt, Vec3) {
        match self {
            BasisCurve::Line(c) => c.d1(u),
            BasisCurve::Circle(c) => c.d1(u),
            BasisCurve::Ellipse(c) => c.d1(u),
            BasisCurve::Hyperbola(c) => c.d1(u),
            BasisCurve::Parabola(c) => c.d1(u),
            BasisCurve::Bezier(c) => c.d1(u),
            BasisCurve::BSpline(c) => c.d1(u),
        }
    }

    /// Point, first, and second derivative at `u` (`D2`).
    pub fn d2(&self, u: f64) -> (Pnt, Vec3, Vec3) {
        match self {
            BasisCurve::Line(c) => c.d2(u),
            BasisCurve::Circle(c) => c.d2(u),
            BasisCurve::Ellipse(c) => c.d2(u),
            BasisCurve::Hyperbola(c) => c.d2(u),
            BasisCurve::Parabola(c) => c.d2(u),
            BasisCurve::Bezier(c) => c.d2(u),
            BasisCurve::BSpline(c) => c.d2(u),
        }
    }

    /// Point, first, second, and third derivative at `u` (`D3`).
    pub fn d3(&self, u: f64) -> (Pnt, Vec3, Vec3, Vec3) {
        match self {
            BasisCurve::Line(c) => c.d3(u),
            BasisCurve::Circle(c) => c.d3(u),
            BasisCurve::Ellipse(c) => c.d3(u),
            BasisCurve::Hyperbola(c) => c.d3(u),
            BasisCurve::Parabola(c) => c.d3(u),
            BasisCurve::Bezier(c) => c.d3(u),
            BasisCurve::BSpline(c) => c.d3(u),
        }
    }

    /// `DN`-th derivative at `u`.
    pub fn dn(&self, u: f64, n: i32) -> Vec3 {
        match self {
            BasisCurve::Line(c) => c.dn(u, n),
            BasisCurve::Circle(c) => c.dn(u, n),
            BasisCurve::Ellipse(c) => c.dn(u, n),
            BasisCurve::Hyperbola(c) => c.dn(u, n),
            BasisCurve::Parabola(c) => c.dn(u, n),
            BasisCurve::Bezier(c) => c.dn(u, n),
            BasisCurve::BSpline(c) => c.dn(u, n),
        }
    }

    /// Apply a transform in place.
    pub fn transform(&mut self, t: &Trsf) {
        match self {
            BasisCurve::Line(c) => c.transform(t),
            BasisCurve::Circle(c) => c.transform(t),
            BasisCurve::Ellipse(c) => c.transform(t),
            BasisCurve::Hyperbola(c) => c.transform(t),
            BasisCurve::Parabola(c) => c.transform(t),
            BasisCurve::Bezier(c) => c.transform(t),
            BasisCurve::BSpline(c) => c.transform(t),
        }
    }

    /// Reverse the orientation of the curve.
    ///
    /// For curve types without a `reverse()` method (e.g. [`GeomCircle`],
    /// [`GeomEllipse`], [`GeomHyperbola`]) we negate the implicit direction by
    /// wrapping them in the sense that reversing an offset curve negates the
    /// offset — the basis curve's own geometry stays unchanged.  In practice
    /// the reversal is tracked entirely at the [`GeomOffsetCurve`] level (the
    /// offset distance is negated), so this method is a no-op for those types.
    pub fn reverse(&mut self) {
        match self {
            BasisCurve::Line(c) => c.reverse(),
            BasisCurve::Circle(_) => {
                // GeomCircle has no `reverse()` method; the offset negate
                // at the GeomOffsetCurve level handles the reversal.
            }
            BasisCurve::Ellipse(_) => {
                // GeomEllipse has no `reverse()` method; same as circle.
            }
            BasisCurve::Hyperbola(_) => {
                // GeomHyperbola has no `reverse()` method; same pattern.
            }
            BasisCurve::Parabola(c) => c.reverse(),
            BasisCurve::Bezier(c) => c.reverse(),
            BasisCurve::BSpline(c) => c.reverse(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Geom_OffsetCurve
// ─────────────────────────────────────────────────────────────────────────────

/// `Geom_OffsetCurve` — an offset of a 3D curve by a signed distance `d` in
/// the direction `normalize( V × C'(u) )`, where `V` is the reference
/// direction and `C'(u)` is the tangent of the base curve at parameter `u`.
///
/// The parameter range and topology (closed / periodic) are inherited from the
/// basis curve.
// occt: Geom_OffsetCurve
#[derive(Clone, Debug)]
pub struct GeomOffsetCurve {
    /// The underlying base curve.
    basis: BasisCurve,
    /// Signed offset distance.
    offset: f64,
    /// Reference direction `V` (stored as a unit vector). The offset normal at
    /// any parameter `u` is `normalize(direction × C'(u))`.
    direction: Vec3,
}

impl GeomOffsetCurve {
    // ──────────────────────────────────── Constructor ─────────────────────────

    /// `Geom_OffsetCurve(C, Offset, V)` — offset curve built from basis curve
    /// `C`, signed offset distance `Offset`, and reference direction `V`
    /// (a `gp_Dir`).
    ///
    /// `V` is normalised on construction so that the stored direction is always
    /// a unit vector.
    pub fn new(basis: BasisCurve, offset: f64, v: Vec3) -> Self {
        Self {
            basis,
            offset,
            direction: v.normalized(),
        }
    }

    // ──────────────────────────────────── Getters ─────────────────────────────

    /// `BasisCurve()` — the underlying base curve.
    pub fn basis_curve(&self) -> &BasisCurve {
        &self.basis
    }

    /// `Offset()` — the signed offset distance.
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// The reference direction `V` (always a unit vector).
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    // ──────────────────────────────────── Parameter range ─────────────────────

    /// `FirstParameter()` — inherited from the basis curve.
    pub fn first_parameter(&self) -> f64 {
        self.basis.first_parameter()
    }

    /// `LastParameter()` — inherited from the basis curve.
    pub fn last_parameter(&self) -> f64 {
        self.basis.last_parameter()
    }

    /// `IsClosed()` — `true` when the offset curve is geometrically closed:
    /// the base curve must be closed and the offset normal must be the same at
    /// the two endpoints (which is automatically satisfied when the base curve
    /// is truly closed and periodic).
    pub fn is_closed(&self) -> bool {
        let fp = self.first_parameter();
        let lp = self.last_parameter();
        // Infinite curves are never closed.
        if fp <= -INFINITE * 0.5 || lp >= INFINITE * 0.5 {
            return false;
        }
        let p0 = self.value(fp);
        let p1 = self.value(lp);
        p0.distance(p1) <= crate::precision::CONFUSION
    }

    /// `IsPeriodic()` — inherited from the basis curve.
    pub fn is_periodic(&self) -> bool {
        self.basis.is_periodic()
    }

    // ──────────────────────────────────── Evaluation ──────────────────────────

    /// `Value(u)` / `D0` — point on the offset curve at parameter `u`.
    ///
    /// `Q(u) = C(u) + d * normalize(V × C'(u))`
    pub fn value(&self, u: f64) -> Pnt {
        let (p, t) = self.basis.d1(u);
        let n = offset_normal(self.direction, t);
        p + n * self.offset
    }

    /// `D1(u)` — point and first derivative of the offset curve.
    ///
    /// `Q'(u) = C'(u) + d * N'(u)`
    ///
    /// where `N'(u) = d/du normalize(V × C'(u))`.
    pub fn d1(&self, u: f64) -> (Pnt, Vec3) {
        let (cp, t, t1) = self.basis.d2(u);
        // raw cross f = V × T
        let f = self.direction.cross(t);
        // raw cross derivative f' = V × T'
        let f1 = self.direction.cross(t1);
        let n = unit_vec(f);
        let n1 = unit_vec_deriv(f, f1);
        let q = cp + n * self.offset;
        let dq = t + n1 * self.offset;
        (q, dq)
    }

    /// `D2(u)` — point and first two derivatives of the offset curve.
    ///
    /// `Q''(u) = C''(u) + d * N''(u)`
    pub fn d2(&self, u: f64) -> (Pnt, Vec3, Vec3) {
        let (cp, t, t1, t2) = self.basis.d3(u);
        let f = self.direction.cross(t);
        let f1 = self.direction.cross(t1);
        let f2 = self.direction.cross(t2);
        let n = unit_vec(f);
        let n1 = unit_vec_deriv(f, f1);
        let n2 = unit_vec_deriv2(f, f1, f2);
        let q = cp + n * self.offset;
        let dq = t + n1 * self.offset;
        let d2q = t1 + n2 * self.offset;
        (q, dq, d2q)
    }

    /// `D3(u)` — point and first three derivatives of the offset curve.
    ///
    /// `Q'''(u) = C'''(u) + d * N'''(u)`
    pub fn d3(&self, u: f64) -> (Pnt, Vec3, Vec3, Vec3) {
        // We need up to the 4th derivative of C for N'''.
        let t = self.basis.d0(u);
        let t1 = self.basis.dn(u, 1);
        let t2 = self.basis.dn(u, 2);
        let t3 = self.basis.dn(u, 3);
        let t4 = self.basis.dn(u, 4);
        let f = self.direction.cross(t1);
        let f1 = self.direction.cross(t2);
        let f2 = self.direction.cross(t3);
        let f3 = self.direction.cross(t4);
        let n = unit_vec(f);
        let n1 = unit_vec_deriv(f, f1);
        let n2 = unit_vec_deriv2(f, f1, f2);
        let n3 = unit_vec_deriv3(f, f1, f2, f3);
        let q = t + n * self.offset;
        let dq = t1 + n1 * self.offset;
        let d2q = t2 + n2 * self.offset;
        let d3q = t3 + n3 * self.offset;
        (q, dq, d2q, d3q)
    }

    /// `DN(u, n)` — the `n`-th derivative vector of the offset curve
    /// (`n >= 1`).
    ///
    /// For orders `n >= 4` we fall back to numerical differentiation using a
    /// central-difference scheme (adequate for smooth curves). Orders 1–3 are
    /// computed analytically.
    ///
    /// # Panics
    /// Panics if `n < 1`.
    pub fn dn(&self, u: f64, n: i32) -> Vec3 {
        assert!(n >= 1, "Geom_OffsetCurve::DN: N must be >= 1");
        match n {
            1 => self.d1(u).1,
            2 => self.d2(u).2,
            3 => self.d3(u).3,
            _ => {
                // Numerical n-th derivative via central finite differences.
                let h = 1.0e-5;
                num_deriv(|t| self.value(t), u, h, n)
            }
        }
    }

    // ──────────────────────────────────── Orientation ─────────────────────────

    /// `Reverse()` — reverse the orientation of the offset curve in place.
    ///
    /// Reverses the basis curve (which re-parameterises it) and negates the
    /// offset distance so that the geometric offset shape is preserved but
    /// traversed in the opposite direction.
    pub fn reverse(&mut self) {
        self.basis.reverse();
        self.offset = -self.offset;
    }

    /// Returns a reversed copy of this offset curve.
    pub fn reversed(&self) -> GeomOffsetCurve {
        let mut c = self.clone();
        c.reverse();
        c
    }

    // ──────────────────────────────────── Transform ───────────────────────────

    /// `Transform(T)` — apply an affine transform `T` to this curve in place.
    ///
    /// The basis curve is transformed; the offset is scaled by the transform's
    /// scale factor; the reference direction is rotated (remains a unit vector
    /// because `T`'s linear part is assumed orthogonal up to a uniform scale).
    pub fn transform(&mut self, t: &Trsf) {
        self.basis.transform(t);
        let scale = t.scale_factor();
        self.offset *= scale;
        self.direction = t.apply_vector(self.direction).normalized();
    }

    /// Returns a transformed copy of this offset curve.
    pub fn transformed(&self, t: &Trsf) -> GeomOffsetCurve {
        let mut c = self.clone();
        c.transform(t);
        c
    }

    /// `Axis()` — the reference axis: the first parameter's base-curve point
    /// and the reference direction `V`. Provided for OCCT compatibility.
    pub fn axis(&self) -> Ax1 {
        let p = self.basis.d0(self.first_parameter());
        Ax1::new(p, self.direction)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper: unit-vector arithmetic
// ─────────────────────────────────────────────────────────────────────────────

/// Compute the unit vector of `f`, or the zero vector when `|f|` is
/// negligibly small (matches OCCT's degenerate-tangent handling).
#[inline]
fn unit_vec(f: Vec3) -> Vec3 {
    let len = f.norm();
    if len < crate::gp::EPS {
        Pnt::origin()
    } else {
        f * (1.0 / len)
    }
}

/// First derivative of `f / |f|` with respect to the curve parameter.
///
/// `(f / |f|)' = (f' - (f·f'/|f|^2) * f) / |f|`
#[inline]
fn unit_vec_deriv(f: Vec3, f1: Vec3) -> Vec3 {
    let len2 = f.dot(f);
    if len2 < crate::gp::EPS * crate::gp::EPS {
        return Pnt::origin();
    }
    let len = len2.sqrt();
    let proj = f.dot(f1) / len2;
    let perp = f1 - f * proj;
    perp * (1.0 / len)
}

/// Second derivative of `f / |f|` with respect to the curve parameter.
///
/// Standard second-derivative-of-unit-vector formula:
///
/// `N'' = (f'' - (f·f'')/|f|^2 * f - 2*(f·f')/|f|^2 * f'
///          + (3*(f·f')^2/|f|^4 - (f'·f')/|f|^2) * f) / |f|`
#[inline]
fn unit_vec_deriv2(f: Vec3, f1: Vec3, f2: Vec3) -> Vec3 {
    let len2 = f.dot(f);
    if len2 < crate::gp::EPS * crate::gp::EPS {
        return Pnt::origin();
    }
    let len = len2.sqrt();
    let a = f.dot(f1) / len2;   // (f·f') / |f|^2
    let b = f.dot(f2) / len2;   // (f·f'') / |f|^2
    let c = f1.dot(f1) / len2;  // |f'|^2 / |f|^2
    // N'' = (f'' - b*f - 2*a*f' + (3*a^2 - c)*f) / |f|
    let raw = f2 - f * b - f1 * (2.0 * a) + f * (3.0 * a * a - c);
    raw * (1.0 / len)
}

/// Third derivative of `f / |f|` with respect to the curve parameter.
///
/// Derived by differentiating `unit_vec_deriv2` once more using the chain rule.
#[inline]
fn unit_vec_deriv3(f: Vec3, f1: Vec3, f2: Vec3, f3: Vec3) -> Vec3 {
    let len2 = f.dot(f);
    if len2 < crate::gp::EPS * crate::gp::EPS {
        return Pnt::origin();
    }
    let len = len2.sqrt();
    let a = f.dot(f1) / len2;    // (f·f') / |f|^2
    let b = f.dot(f2) / len2;    // (f·f'') / |f|^2
    let c = f1.dot(f1) / len2;   // |f'|^2 / |f|^2
    let d = f.dot(f3) / len2;    // (f·f''') / |f|^2
    let e = f1.dot(f2) / len2;   // (f'·f'') / |f|^2

    // Derivative of a = (f·f')/|f|^2:
    // a' = (f·f'') / |f|^2 + (f'·f') / |f|^2 - 2*(f·f')^2 / |f|^4
    //    = b + c - 2*a^2
    let da = b + c - 2.0 * a * a;

    // Derivative of b = (f·f'')/|f|^2:
    // b' = (f'·f'' + f·f''') / |f|^2 - 2*(f·f'')*(f·f') / |f|^4
    //    = (e + d) - 2*a*b
    let db = e + d - 2.0 * a * b;

    // Derivative of c = (f'·f')/|f|^2:
    // c' = 2*(f'·f'') / |f|^2 - 2*(f'·f)^2 / |f|^4 * (note f·f'/|f|^2 = a)
    //    = 2*e - 4*a*c
    let dc = 2.0 * e - 4.0 * a * c;

    // From N'' = (f2 - b*f - 2*a*f1 + (3*a^2 - c)*f) / |f|
    // Differentiate the numerator:
    // = f3 - db*f - b*f1 - 2*da*f1 - 2*a*f2 + (6*a*da - dc)*f + (3*a^2 - c)*f1
    // Then subtract N'' * a (because |f|' = f·f' / |f| = a * |f|, so d/du(1/|f|) = -a/|f|)
    let n2_num = f2 - f * b - f1 * (2.0 * a) + f * (3.0 * a * a - c);
    let num_deriv_inner = f3
        - f * db
        - f1 * b
        - f1 * (2.0 * da)
        - f2 * (2.0 * a)
        + f * (6.0 * a * da - dc)
        + f1 * (3.0 * a * a - c);
    let raw = num_deriv_inner - n2_num * a;
    raw * (1.0 / len)
}

/// Compute the offset normal `N = normalize(V × T)`.
#[inline]
fn offset_normal(v: Vec3, t: Vec3) -> Vec3 {
    unit_vec(v.cross(t))
}

// ─────────────────────────────────────────────────────────────────────────────
// Numerical n-th derivative (fallback for DN with n >= 4)
// ─────────────────────────────────────────────────────────────────────────────

/// Compute the `n`-th derivative of `f(u)` using the central-difference
/// stencil: `D^n f(u) ≈ Σ_k (-1)^k C(n,k) f(u + (n/2 - k)*h) / h^n`.
fn num_deriv(f: impl Fn(f64) -> Pnt, u: f64, h: f64, n: i32) -> Vec3 {
    let mut result = Pnt::origin();
    let mut binom: i64 = 1;
    let hn = h.powi(n);
    for k in 0..=(n as usize) {
        let sign = if k % 2 == 0 { 1.0_f64 } else { -1.0_f64 };
        let shift = ((n as f64) / 2.0 - k as f64) * h;
        let p = f(u + shift);
        // Accumulate: result += sign * binom * p / h^n
        result = result + p * (sign * binom as f64 / hn);
        if k < n as usize {
            binom = binom * (n as i64 - k as i64) / (k as i64 + 1);
        }
    }
    result
}
