//! `Geom2d_OffsetCurve` — a 2D curve offset in the plane by a given distance
//! along the direction normal to the curve tangent.
//!
//! Mirrors OpenCascade's `Geom2d_OffsetCurve`
//! (`src/ModelingData/TKG2d/Geom2d/Geom2d_OffsetCurve.hxx`).
//!
//! Given a base curve `C` and an offset distance `d`, the offset curve is:
//!
//! ```text
//! Q(u) = C(u) + d * N(u)
//! ```
//!
//! where the offset normal at `u` is the unit vector obtained by rotating
//! `C'(u)` by `+90°` (CCW):
//!
//! ```text
//! N(u) = normalize( perp(C'(u)) ) = normalize( (-C'y(u), C'x(u)) )
//! ```
//!
//! This is the standard 2D offset formula used by OCCT.
//!
//! Derivatives are computed analytically via the chain rule:
//!
//! ```text
//! Q'(u) = C'(u) + d * N'(u)
//! ```
//!
//! where `N'(u)` is the derivative of the unit offset-normal.  For higher
//! orders (D2, D3) the same first-principles differentiation is applied — the
//! derivative of the normalised vector is recovered from the raw perpendicular
//! vectors and their derivatives using:
//!
//! ```text
//! (f/|f|)' = (f' - (f·f'/|f|^2) * f) / |f|
//! ```
//!
//! This module is zero-dependency (only `std`).

use crate::geom2d_bezier::Geom2dBezierCurve;
use crate::geom2d_bspline::Geom2dBSplineCurve;
use crate::geom2d_circle::Geom2dCircle;
use crate::geom2d_ellipse::Geom2dEllipse;
use crate::geom2d_hyperbola::Geom2dHyperbola;
use crate::geom2d_line::Geom2dLine;
use crate::geom2d_parabola::Geom2dParabola;
use crate::gp2d::{Pnt2d, Trsf2d, Vec2d};
use crate::precision::INFINITE;

// ─────────────────────────────────────────────────────────────────────────────
// BasisCurve2d — the union of all concrete 2D curve types that can be offset.
// ─────────────────────────────────────────────────────────────────────────────

/// The concrete 2D curve types that [`Geom2dOffsetCurve`] can wrap.
///
/// Mirrors the `Handle(Geom2d_Curve)` parameter of OCCT's
/// `Geom2d_OffsetCurve` constructor without any heap allocation overhead.

#[derive(Clone, Debug)]
pub enum BasisCurve2d {
    Line(Geom2dLine),
    Circle(Geom2dCircle),
    Ellipse(Geom2dEllipse),
    Hyperbola(Geom2dHyperbola),
    Parabola(Geom2dParabola),
    Bezier(Geom2dBezierCurve),
    BSpline(Geom2dBSplineCurve),
}

impl BasisCurve2d {
    /// First parameter of the underlying curve.
    pub fn first_parameter(&self) -> f64 {
        match self {
            BasisCurve2d::Line(c) => c.first_parameter(),
            BasisCurve2d::Circle(c) => c.first_parameter(),
            BasisCurve2d::Ellipse(c) => c.first_parameter(),
            BasisCurve2d::Hyperbola(c) => c.first_parameter(),
            BasisCurve2d::Parabola(c) => c.first_parameter(),
            BasisCurve2d::Bezier(c) => c.first_parameter(),
            BasisCurve2d::BSpline(c) => c.first_parameter(),
        }
    }

    /// Last parameter of the underlying curve.
    pub fn last_parameter(&self) -> f64 {
        match self {
            BasisCurve2d::Line(c) => c.last_parameter(),
            BasisCurve2d::Circle(c) => c.last_parameter(),
            BasisCurve2d::Ellipse(c) => c.last_parameter(),
            BasisCurve2d::Hyperbola(c) => c.last_parameter(),
            BasisCurve2d::Parabola(c) => c.last_parameter(),
            BasisCurve2d::Bezier(c) => c.last_parameter(),
            BasisCurve2d::BSpline(c) => c.last_parameter(),
        }
    }

    /// Whether the curve is closed.
    pub fn is_closed(&self) -> bool {
        match self {
            BasisCurve2d::Line(c) => c.is_closed(),
            BasisCurve2d::Circle(c) => c.is_closed(),
            BasisCurve2d::Ellipse(c) => c.is_closed(),
            BasisCurve2d::Hyperbola(c) => c.is_closed(),
            BasisCurve2d::Parabola(c) => c.is_closed(),
            BasisCurve2d::Bezier(c) => c.is_closed(),
            BasisCurve2d::BSpline(c) => c.is_closed(),
        }
    }

    /// Whether the curve is periodic.
    pub fn is_periodic(&self) -> bool {
        match self {
            BasisCurve2d::Line(c) => c.is_periodic(),
            BasisCurve2d::Circle(c) => c.is_periodic(),
            BasisCurve2d::Ellipse(c) => c.is_periodic(),
            BasisCurve2d::Hyperbola(c) => c.is_periodic(),
            BasisCurve2d::Parabola(c) => c.is_periodic(),
            BasisCurve2d::Bezier(c) => c.is_periodic(),
            BasisCurve2d::BSpline(c) => c.is_periodic(),
        }
    }

    /// Point at parameter `u` (`D0`).
    pub fn d0(&self, u: f64) -> Pnt2d {
        match self {
            BasisCurve2d::Line(c) => c.eval_d0(u),
            BasisCurve2d::Circle(c) => c.eval_d0(u),
            BasisCurve2d::Ellipse(c) => c.eval_d0(u),
            BasisCurve2d::Hyperbola(c) => c.d0(u),
            BasisCurve2d::Parabola(c) => c.d0(u),
            BasisCurve2d::Bezier(c) => c.eval_d0(u),
            BasisCurve2d::BSpline(c) => c.d0(u),
        }
    }

    /// Point and first derivative at `u` (`D1`).
    pub fn d1(&self, u: f64) -> (Pnt2d, Vec2d) {
        match self {
            BasisCurve2d::Line(c) => {
                let r = c.eval_d1(u);
                (r.p, r.v1)
            }
            BasisCurve2d::Circle(c) => c.eval_d1(u),
            BasisCurve2d::Ellipse(c) => c.eval_d1(u),
            BasisCurve2d::Hyperbola(c) => c.d1(u),
            BasisCurve2d::Parabola(c) => c.d1(u),
            BasisCurve2d::Bezier(c) => c.eval_d1(u),
            BasisCurve2d::BSpline(c) => c.d1(u),
        }
    }

    /// Point, first and second derivative at `u` (`D2`).
    pub fn d2(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d) {
        match self {
            BasisCurve2d::Line(c) => {
                let r = c.eval_d2(u);
                (r.p, r.v1, r.v2)
            }
            BasisCurve2d::Circle(c) => c.eval_d2(u),
            BasisCurve2d::Ellipse(c) => c.eval_d2(u),
            BasisCurve2d::Hyperbola(c) => c.d2(u),
            BasisCurve2d::Parabola(c) => c.d2(u),
            BasisCurve2d::Bezier(c) => c.eval_d2(u),
            BasisCurve2d::BSpline(c) => c.d2(u),
        }
    }

    /// Point, first, second and third derivative at `u` (`D3`).
    pub fn d3(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d, Vec2d) {
        match self {
            BasisCurve2d::Line(c) => {
                let r = c.eval_d3(u);
                (r.p, r.v1, r.v2, r.v3)
            }
            BasisCurve2d::Circle(c) => c.eval_d3(u),
            BasisCurve2d::Ellipse(c) => c.eval_d3(u),
            BasisCurve2d::Hyperbola(c) => c.d3(u),
            BasisCurve2d::Parabola(c) => c.d3(u),
            BasisCurve2d::Bezier(c) => c.eval_d3(u),
            BasisCurve2d::BSpline(c) => c.d3(u),
        }
    }

    /// `N`-th derivative vector at `u` (`N >= 1`).
    pub fn dn(&self, u: f64, n: i32) -> Vec2d {
        match self {
            BasisCurve2d::Line(c) => c.eval_dn(u, n),
            BasisCurve2d::Circle(c) => c.eval_dn(u, n),
            BasisCurve2d::Ellipse(c) => c.eval_dn(u, n),
            BasisCurve2d::Hyperbola(c) => c.dn(u, n),
            BasisCurve2d::Parabola(c) => c.dn(u, n),
            BasisCurve2d::Bezier(c) => c.eval_dn(u, n),
            BasisCurve2d::BSpline(c) => c.dn(u, n as usize),
        }
    }

    /// Apply a transform in place.
    pub fn transform(&mut self, t: &Trsf2d) {
        match self {
            BasisCurve2d::Line(c) => c.transform(t),
            BasisCurve2d::Circle(c) => c.transform(t),
            BasisCurve2d::Ellipse(c) => c.transform(t),
            BasisCurve2d::Hyperbola(c) => c.transform(t),
            BasisCurve2d::Parabola(c) => c.transform(t),
            BasisCurve2d::Bezier(c) => c.transform(t),
            BasisCurve2d::BSpline(c) => c.transform(t),
        }
    }

    /// Reverse the orientation of the curve.
    ///
    /// For the closed conic types (circle, ellipse) that lack a `reverse()`
    /// method, the reversal is handled at the [`Geom2dOffsetCurve`] level by
    /// negating the offset, so this is a no-op for those variants.
    pub fn reverse(&mut self) {
        match self {
            BasisCurve2d::Line(c) => c.reverse(),
            BasisCurve2d::Circle(_) => {
                // No reverse() on Geom2dCircle; the offset-negate at the
                // Geom2dOffsetCurve level handles the traversal reversal.
            }
            BasisCurve2d::Ellipse(_) => {
                // Same as circle.
            }
            BasisCurve2d::Hyperbola(c) => c.reverse(),
            BasisCurve2d::Parabola(c) => c.reverse(),
            BasisCurve2d::Bezier(c) => c.reverse(),
            BasisCurve2d::BSpline(c) => c.reverse(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Geom2d_OffsetCurve
// ─────────────────────────────────────────────────────────────────────────────

/// `Geom2d_OffsetCurve` — an offset of a 2D curve by a signed distance `d`
/// in the direction normal to the curve tangent (tangent rotated `+90°`).
///
/// Given the basis curve `C` and offset `d`, the offset curve is:
///
/// ```text
/// Q(u) = C(u) + d * normalize( perp(C'(u)) )
/// ```
///
/// The parameter range and topology (closed / periodic) are inherited from the
/// basis curve.
// occt: Geom2d_OffsetCurve
#[derive(Clone, Debug)]
pub struct Geom2dOffsetCurve {
    /// The underlying base curve.
    basis: BasisCurve2d,
    /// Signed offset distance.
    offset: f64,
}

impl Geom2dOffsetCurve {
    // ──────────────────────────────────── Constructor ─────────────────────────

    /// `Geom2d_OffsetCurve(C, Offset)` — offset curve built from basis curve
    /// `C` and signed offset distance `Offset`.
    pub fn new(basis: BasisCurve2d, offset: f64) -> Self {
        Self { basis, offset }
    }

    // ──────────────────────────────────── Getters ─────────────────────────────

    /// `BasisCurve()` — the underlying base curve.
    pub fn basis_curve(&self) -> &BasisCurve2d {
        &self.basis
    }

    /// `Offset()` — the signed offset distance.
    pub fn offset(&self) -> f64 {
        self.offset
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
    /// endpoints coincide within [`precision::CONFUSION`].
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
    /// `Q(u) = C(u) + d * normalize( perp(C'(u)) )`
    pub fn value(&self, u: f64) -> Pnt2d {
        let (p, t) = self.basis.d1(u);
        let n = offset_normal_2d(t);
        p + n * self.offset
    }

    /// `D1(u)` — point and first derivative of the offset curve.
    ///
    /// `Q'(u) = C'(u) + d * N'(u)`
    ///
    /// where `N'(u) = d/du normalize( perp(C'(u)) )`.
    pub fn d1(&self, u: f64) -> (Pnt2d, Vec2d) {
        let (cp, t, t1) = self.basis.d2(u);
        // f = perp(T) = (-Ty, Tx)
        let f = perp(t);
        // f' = perp(T') = (-T'y, T'x)
        let f1 = perp(t1);
        let n = unit_vec_2d(f);
        let n1 = unit_vec_deriv_2d(f, f1);
        let q = cp + n * self.offset;
        let dq = t + n1 * self.offset;
        (q, dq)
    }

    /// `D2(u)` — point and first two derivatives of the offset curve.
    ///
    /// `Q''(u) = C''(u) + d * N''(u)`
    pub fn d2(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d) {
        let (cp, t, t1, t2) = self.basis.d3(u);
        let f = perp(t);
        let f1 = perp(t1);
        let f2 = perp(t2);
        let n = unit_vec_2d(f);
        let n1 = unit_vec_deriv_2d(f, f1);
        let n2 = unit_vec_deriv2_2d(f, f1, f2);
        let q = cp + n * self.offset;
        let dq = t + n1 * self.offset;
        let d2q = t1 + n2 * self.offset;
        (q, dq, d2q)
    }

    /// `D3(u)` — point and first three derivatives of the offset curve.
    ///
    /// `Q'''(u) = C'''(u) + d * N'''(u)`
    ///
    /// Requires up to the 4th derivative of the basis curve.
    pub fn d3(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d, Vec2d) {
        let t0 = self.basis.d0(u);
        let t1 = self.basis.dn(u, 1);
        let t2 = self.basis.dn(u, 2);
        let t3 = self.basis.dn(u, 3);
        let t4 = self.basis.dn(u, 4);
        let f = perp(t1);
        let f1 = perp(t2);
        let f2 = perp(t3);
        let f3 = perp(t4);
        let n = unit_vec_2d(f);
        let n1 = unit_vec_deriv_2d(f, f1);
        let n2 = unit_vec_deriv2_2d(f, f1, f2);
        let n3 = unit_vec_deriv3_2d(f, f1, f2, f3);
        let q = t0 + n * self.offset;
        let dq = t1 + n1 * self.offset;
        let d2q = t2 + n2 * self.offset;
        let d3q = t3 + n3 * self.offset;
        (q, dq, d2q, d3q)
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
    pub fn reversed(&self) -> Geom2dOffsetCurve {
        let mut c = self.clone();
        c.reverse();
        c
    }

    // ──────────────────────────────────── Transform ───────────────────────────

    /// `Transform(T)` — apply an affine 2D transform `T` to this curve in
    /// place.
    ///
    /// The basis curve is transformed; the offset is scaled by the transform's
    /// scale factor.
    pub fn transform(&mut self, t: &Trsf2d) {
        self.basis.transform(t);
        let scale = trsf2d_scale_factor(t);
        self.offset *= scale;
    }

    /// Returns a transformed copy of this offset curve.
    pub fn transformed(&self, t: &Trsf2d) -> Geom2dOffsetCurve {
        let mut c = self.clone();
        c.transform(t);
        c
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper: 2D perpendicular (rotate +90°)
// ─────────────────────────────────────────────────────────────────────────────

/// Rotate a 2D vector by `+90°` (CCW): `(x, y) → (-y, x)`.
#[inline]
fn perp(v: Vec2d) -> Vec2d {
    Vec2d::new(-v.y, v.x)
}

/// Compute the offset normal `N = normalize( perp(T) )`.
#[inline]
fn offset_normal_2d(t: Vec2d) -> Vec2d {
    unit_vec_2d(perp(t))
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper: unit-vector arithmetic in 2D
// ─────────────────────────────────────────────────────────────────────────────

/// Small epsilon used to guard against near-zero magnitude vectors.
const EPS: f64 = 1.0e-15;

/// Compute the unit vector of `f`, or the zero vector when `|f|` is
/// negligibly small.
#[inline]
fn unit_vec_2d(f: Vec2d) -> Vec2d {
    let len = f.norm();
    if len < EPS {
        Vec2d::new(0.0, 0.0)
    } else {
        f * (1.0 / len)
    }
}

/// First derivative of `f / |f|` with respect to the curve parameter.
///
/// `(f / |f|)' = (f' - (f·f'/|f|^2) * f) / |f|`
#[inline]
fn unit_vec_deriv_2d(f: Vec2d, f1: Vec2d) -> Vec2d {
    let len2 = f.dot(f);
    if len2 < EPS * EPS {
        return Vec2d::new(0.0, 0.0);
    }
    let len = len2.sqrt();
    let proj = f.dot(f1) / len2;
    let perp_part = f1 - f * proj;
    perp_part * (1.0 / len)
}

/// Second derivative of `f / |f|` with respect to the curve parameter.
///
/// `N'' = (f'' - (f·f'')/|f|^2 * f - 2*(f·f')/|f|^2 * f'
///         + (3*(f·f')^2/|f|^4 - (f'·f')/|f|^2) * f) / |f|`
#[inline]
fn unit_vec_deriv2_2d(f: Vec2d, f1: Vec2d, f2: Vec2d) -> Vec2d {
    let len2 = f.dot(f);
    if len2 < EPS * EPS {
        return Vec2d::new(0.0, 0.0);
    }
    let len = len2.sqrt();
    let a = f.dot(f1) / len2; // (f·f') / |f|^2
    let b = f.dot(f2) / len2; // (f·f'') / |f|^2
    let c = f1.dot(f1) / len2; // |f'|^2 / |f|^2
    let raw = f2 - f * b - f1 * (2.0 * a) + f * (3.0 * a * a - c);
    raw * (1.0 / len)
}

/// Third derivative of `f / |f|` with respect to the curve parameter.
#[inline]
fn unit_vec_deriv3_2d(f: Vec2d, f1: Vec2d, f2: Vec2d, f3: Vec2d) -> Vec2d {
    let len2 = f.dot(f);
    if len2 < EPS * EPS {
        return Vec2d::new(0.0, 0.0);
    }
    let len = len2.sqrt();
    let a = f.dot(f1) / len2;  // (f·f') / |f|^2
    let b = f.dot(f2) / len2;  // (f·f'') / |f|^2
    let c = f1.dot(f1) / len2; // |f'|^2 / |f|^2
    let d = f.dot(f3) / len2;  // (f·f''') / |f|^2
    let e = f1.dot(f2) / len2; // (f'·f'') / |f|^2

    // Derivative of a = (f·f')/|f|^2:
    // a' = b + c - 2*a^2
    let da = b + c - 2.0 * a * a;

    // Derivative of b = (f·f'')/|f|^2:
    // b' = (e + d) - 2*a*b
    let db = e + d - 2.0 * a * b;

    // Derivative of c = (f'·f')/|f|^2:
    // c' = 2*e - 4*a*c  (using chain rule and dot derivative)
    let dc = 2.0 * e - 4.0 * a * c;

    // Numerator of N'' = f2 - b*f - 2*a*f1 + (3*a^2 - c)*f
    let n2_num = f2 - f * b - f1 * (2.0 * a) + f * (3.0 * a * a - c);

    // Derivative of the numerator of N'':
    let num_deriv_inner = f3
        - f * db
        - f1 * b
        - f1 * (2.0 * da)
        - f2 * (2.0 * a)
        + f * (6.0 * a * da - dc)
        + f1 * (3.0 * a * a - c);

    // N''' = (num_deriv_inner - n2_num * a) / |f|
    // (because d/du(1/|f|) = -a/|f|)
    let raw = num_deriv_inner - n2_num * a;
    raw * (1.0 / len)
}

/// The uniform scale factor of a 2D transform (`gp_Trsf2d::ScaleFactor()`):
/// the square root of the absolute value of the 2x2 linear determinant.
#[inline]
fn trsf2d_scale_factor(t: &Trsf2d) -> f64 {
    let det = t.m[0][0] * t.m[1][1] - t.m[0][1] * t.m[1][0];
    det.abs().sqrt()
}
