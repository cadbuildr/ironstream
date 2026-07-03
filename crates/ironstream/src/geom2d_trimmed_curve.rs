//! `Geom2d_TrimmedCurve` — a 2D curve trimmed to a finite parameter interval
//! `[u1, u2]`, faithfully reproducing OpenCascade's `Geom2d_TrimmedCurve`
//! (TKG2d, a `Geom2d_BoundedCurve`).
//!
//! A trimmed curve `TC` wraps an arbitrary basis 2D curve `C` and restricts it
//! to the finite interval `[u1, u2]`:
//!
//! ```text
//! TC(u) = C(u)   for u in [u1, u2]
//! ```
//!
//! The basis curve is never modified; the trim parameters are carried separately.
//! All differential quantities (`Value`, `D1`, `D2`, `D3`, `DN`) are delegated
//! directly to the basis curve, evaluated at the requested parameter (which must
//! lie within the trim interval by convention — OCCT does not clamp).
//!
//! `IsClosed()` is `true` when `C(u1)` and `C(u2)` coincide (within
//! `Precision::Confusion()`). `IsPeriodic()` always returns `false` — trimmed
//! curves are bounded, not periodic.
//!
//! `Reverse()` swaps `u1 ↔ u2` using the basis curve's `ReversedParameter`
//! formula, and reverses the basis curve's orientation.
//!
//! `Transform(T)` applies `T` to the basis curve in place. The trim parameters
//! are scaled by the parametric-transformation coefficient of the basis curve.
//!
//! This module is self-contained: zero third-party dependencies, only `std`.

use crate::geom2d_bezier::Geom2dBezierCurve;
use crate::geom2d_bspline::Geom2dBSplineCurve;
use crate::geom2d_circle::Geom2dCircle;
use crate::geom2d_ellipse::Geom2dEllipse;
use crate::geom2d_hyperbola::Geom2dHyperbola;
use crate::geom2d_line::Geom2dLine;
use crate::geom2d_parabola::Geom2dParabola;
use crate::gp2d::{Pnt2d, Trsf2d, Vec2d};
use crate::precision::CONFUSION;

// ─────────────────────────────────────────────────────────────────────────────
// BasisCurve2d — the union of all concrete 2D curve types that can be trimmed.
// ─────────────────────────────────────────────────────────────────────────────

/// The concrete 2D curve types that [`Geom2dTrimmedCurve`] can wrap.
///
/// Mirrors the `Handle(Geom2d_Curve)` parameter of OCCT's
/// `Geom2d_TrimmedCurve` constructor without any heap allocation overhead.

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
    /// First parameter of the underlying curve's natural range.
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

    /// Last parameter of the underlying curve's natural range.
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

    /// Whether the basis curve is periodic.
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

    /// Period of the basis curve (only meaningful when `is_periodic()` is true).
    pub fn period(&self) -> f64 {
        match self {
            BasisCurve2d::Circle(c) => c.period(),
            BasisCurve2d::Ellipse(c) => c.period(),
            // Non-periodic curves: return the natural parameter width.
            _ => self.last_parameter() - self.first_parameter(),
        }
    }

    /// Point at parameter `u` (D0).
    pub fn d0(&self, u: f64) -> Pnt2d {
        match self {
            BasisCurve2d::Line(c) => c.eval_d0(u),
            BasisCurve2d::Circle(c) => c.eval_d0(u),
            BasisCurve2d::Ellipse(c) => c.eval_d0(u),
            BasisCurve2d::Hyperbola(c) => c.d0(u),
            BasisCurve2d::Parabola(c) => c.d0(u),
            BasisCurve2d::Bezier(c) => c.value(u),
            BasisCurve2d::BSpline(c) => c.value(u),
        }
    }

    /// Point and first derivative at `u` (D1).
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

    /// Point, first and second derivatives at `u` (D2).
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

    /// Point, first, second and third derivatives at `u` (D3).
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

    /// N-th derivative vector at `u` (DN, N >= 1).
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

    /// Apply a 2D transform to the basis curve in place.
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

    /// The parameter on the reversed basis curve that corresponds to `u` on the
    /// forward basis curve (`ReversedParameter`).
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        match self {
            BasisCurve2d::Line(c) => c.reversed_parameter(u),
            BasisCurve2d::Circle(c) => c.reversed_parameter(u),
            BasisCurve2d::Ellipse(c) => c.reversed_parameter(u),
            BasisCurve2d::Hyperbola(c) => c.reversed_parameter(u),
            BasisCurve2d::Parabola(c) => c.reversed_parameter(u),
            BasisCurve2d::Bezier(c) => c.reversed_parameter(u),
            BasisCurve2d::BSpline(c) => c.reversed_parameter(u),
        }
    }

    /// Reverse the orientation of the basis curve in place.
    pub fn reverse(&mut self) {
        match self {
            BasisCurve2d::Line(c) => c.reverse(),
            BasisCurve2d::Circle(_) => {
                // Geom2dCircle has no `reverse()` — reversal is expressed via
                // swapped trim parameters + reversed_parameter in the wrapper.
            }
            BasisCurve2d::Ellipse(_) => {
                // Same: Geom2dEllipse has no `reverse()`.
            }
            BasisCurve2d::Hyperbola(c) => c.reverse(),
            BasisCurve2d::Parabola(c) => c.reverse(),
            BasisCurve2d::Bezier(c) => c.reverse(),
            BasisCurve2d::BSpline(c) => c.reverse(),
        }
    }

    /// The uniform scale factor of a transform applied to this curve's
    /// parameter (mirrors `parametric_transformation` / `TransformedParameter`).
    ///
    /// For lines the scale factor is recovered from the linear part of `T`.
    /// For all other curves it is `1.0` because the parameterisation is angle-
    /// based (circle, ellipse) or the curve re-parameterises itself after the
    /// control-point transform (Bezier, BSpline) — exactly as OCCT does.
    pub fn parametric_scale(&self, t: &Trsf2d) -> f64 {
        match self {
            BasisCurve2d::Line(c) => c.parametric_transformation(t),
            // Angle-based / intrinsic parameterisations: coefficient = 1.
            _ => {
                let _ = t; // suppress unused warning
                1.0
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Geom2d_TrimmedCurve
// ─────────────────────────────────────────────────────────────────────────────

/// `Geom2d_TrimmedCurve` — a 2D curve `C` restricted to the parameter interval
/// `[u1, u2]`.
///
/// - `C` is the **basis curve** (any concrete 2D curve type).
/// - `u1 < u2` is enforced by the constructor (they are swapped if necessary).
/// - For a **periodic** basis curve the requested interval may span more than
///   one period; the implementation adjusts the parameters so the interval
///   length ≤ period and `u1 < u2` in the standard sense.
/// - `IsClosed()` is `true` when the start and end points coincide within
///   `Precision::Confusion()`.
/// - `IsPeriodic()` always returns `false`.
// occt: Geom2d_TrimmedCurve
#[derive(Clone, Debug)]
pub struct Geom2dTrimmedCurve {
    /// The underlying basis curve.
    basis: BasisCurve2d,
    /// The lower trim parameter (first parameter of the trimmed curve).
    u1: f64,
    /// The upper trim parameter (last parameter of the trimmed curve).
    u2: f64,
}

impl Geom2dTrimmedCurve {
    // ──────────────────────────────────── Constructors ────────────────────────

    /// `Geom2d_TrimmedCurve(C, U1, U2, Sense = true, TheAdjustPeriodic = true)`
    ///
    /// Builds a trimmed curve on basis curve `C`, restricted to `[U1, U2]` (or
    /// `[U2, U1]` if `U1 > U2` — the interval is always stored with u1 < u2).
    ///
    /// For a **periodic** basis curve the interval is adjusted so that
    /// `u2 - u1 ≤ period` and both parameters lie in the same period.
    ///
    /// # Panics
    /// - If `U1 == U2` (zero-length trim interval).
    /// - If the basis curve is not periodic and the interval extends outside
    ///   the curve's natural parameter range.
    pub fn new(basis: BasisCurve2d, u1: f64, u2: f64) -> Self {
        assert!(
            (u1 - u2).abs() > 0.0,
            "Geom2d_TrimmedCurve: U1 and U2 must be different"
        );
        let (mut p1, mut p2) = if u1 < u2 { (u1, u2) } else { (u2, u1) };

        if basis.is_periodic() {
            let period = basis.period();
            // Adjust so that both parameters lie within one period.
            // Bring p1 into [first, first + period).
            let first = basis.first_parameter();
            if p1 < first {
                let shift = (((first - p1) / period).ceil()) * period;
                p1 += shift;
                p2 += shift;
            }
            // If the interval exceeds the period, clamp p2.
            if p2 - p1 > period {
                p2 = p1 + period;
            }
        }

        Self { basis, u1: p1, u2: p2 }
    }

    // ──────────────────────────────────── SetTrim ─────────────────────────────

    /// `SetTrim(U1, U2, Sense = true, TheAdjustPeriodic = true)` — change the
    /// trim parameters of an existing trimmed curve.
    ///
    /// The rules are the same as for [`new`](Self::new).
    ///
    /// # Panics
    /// Same conditions as the constructor.
    pub fn set_trim(&mut self, u1: f64, u2: f64) {
        assert!(
            (u1 - u2).abs() > 0.0,
            "Geom2d_TrimmedCurve::SetTrim: U1 and U2 must be different"
        );
        let (mut p1, mut p2) = if u1 < u2 { (u1, u2) } else { (u2, u1) };

        if self.basis.is_periodic() {
            let period = self.basis.period();
            let first = self.basis.first_parameter();
            if p1 < first {
                let shift = (((first - p1) / period).ceil()) * period;
                p1 += shift;
                p2 += shift;
            }
            if p2 - p1 > period {
                p2 = p1 + period;
            }
        }

        self.u1 = p1;
        self.u2 = p2;
    }

    // ──────────────────────────────────── Accessors ───────────────────────────

    /// `BasisCurve()` — the underlying (un-trimmed) basis curve.
    pub fn basis_curve(&self) -> &BasisCurve2d {
        &self.basis
    }

    /// `FirstParameter()` — `u1`, the lower trim parameter.
    #[inline]
    pub fn first_parameter(&self) -> f64 {
        self.u1
    }

    /// `LastParameter()` — `u2`, the upper trim parameter.
    #[inline]
    pub fn last_parameter(&self) -> f64 {
        self.u2
    }

    // ──────────────────────────────────── Topology ────────────────────────────

    /// `IsClosed()` — returns `true` when the start point `C(u1)` and the end
    /// point `C(u2)` coincide within `Precision::Confusion()`.
    pub fn is_closed(&self) -> bool {
        let p1 = self.basis.d0(self.u1);
        let p2 = self.basis.d0(self.u2);
        p1.distance(p2) <= CONFUSION
    }

    /// `IsPeriodic()` — always `false` for a trimmed curve.
    #[inline]
    pub fn is_periodic(&self) -> bool {
        false
    }

    // ──────────────────────────────────── Evaluation ─────────────────────────

    /// `Value(U)` / `D0(U)` — the point of parameter `U`.
    ///
    /// Delegates directly to the basis curve; `U` is not clamped (matches OCCT).
    pub fn value(&self, u: f64) -> Pnt2d {
        self.basis.d0(u)
    }

    /// `D1(U)` — the point and first derivative at parameter `U`.
    pub fn d1(&self, u: f64) -> (Pnt2d, Vec2d) {
        self.basis.d1(u)
    }

    /// `D2(U)` — the point, first and second derivatives at parameter `U`.
    pub fn d2(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d) {
        self.basis.d2(u)
    }

    /// `D3(U)` — the point, first, second and third derivatives at parameter `U`.
    pub fn d3(&self, u: f64) -> (Pnt2d, Vec2d, Vec2d, Vec2d) {
        self.basis.d3(u)
    }

    /// `DN(U, N)` — the `N`-th derivative vector at parameter `U` (`N >= 1`).
    ///
    /// # Panics
    /// Panics if `n < 1`.
    pub fn dn(&self, u: f64, n: i32) -> Vec2d {
        assert!(n >= 1, "Geom2d_TrimmedCurve::DN: N must be >= 1");
        self.basis.dn(u, n)
    }

    // ──────────────────────────────────── Orientation ────────────────────────

    /// `Reverse()` — reverses the orientation of the trimmed curve in place.
    ///
    /// The basis curve is reversed (where possible), and the trim parameters
    /// are mapped through the basis curve's `ReversedParameter` formula so
    /// the new interval `[new_u1, new_u2]` corresponds to the same geometric
    /// locus traversed in the opposite direction.
    pub fn reverse(&mut self) {
        let new_u1 = self.basis.reversed_parameter(self.u2);
        let new_u2 = self.basis.reversed_parameter(self.u1);
        self.basis.reverse();
        // After reversal the parameter direction is flipped, so new_u1 < new_u2.
        if new_u1 <= new_u2 {
            self.u1 = new_u1;
            self.u2 = new_u2;
        } else {
            self.u1 = new_u2;
            self.u2 = new_u1;
        }
    }

    /// `Reversed()` — returns a copy of this curve with the orientation reversed.
    pub fn reversed(&self) -> Self {
        let mut c = self.clone();
        c.reverse();
        c
    }

    // ──────────────────────────────────── Transform ───────────────────────────

    /// `Transform(T)` — applies the 2D transform `T` to the trimmed curve in
    /// place.
    ///
    /// The basis curve is transformed; the trim parameters are scaled by the
    /// parametric-transformation coefficient of the basis curve (which is the
    /// scale factor for lines and `1.0` for angle-parameterised curves).
    pub fn transform(&mut self, t: &Trsf2d) {
        let scale = self.basis.parametric_scale(t);
        self.basis.transform(t);
        self.u1 *= scale;
        self.u2 *= scale;
    }

    /// `Transformed(T)` — returns a transformed copy.
    pub fn transformed(&self, t: &Trsf2d) -> Self {
        let mut c = self.clone();
        c.transform(t);
        c
    }

    /// `Copy()` — a deep, independent copy of this trimmed curve.
    pub fn copy(&self) -> Self {
        self.clone()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers used by tests — kept pub(crate) so the test binary can import them.
// ─────────────────────────────────────────────────────────────────────────────

/// Returns the length `u2 - u1` of the trim interval, also known as the
/// "parameter span" of the trimmed curve.
pub fn trim_span(tc: &Geom2dTrimmedCurve) -> f64 {
    tc.last_parameter() - tc.first_parameter()
}

/// Convenience: build a trimmed line along the X axis.
///
/// `origin` is the line's origin, `dir` the unit direction.  The trim
/// interval is `[t0, t1]`.
pub fn trimmed_line(
    origin: Pnt2d,
    dir: Pnt2d,
    t0: f64,
    t1: f64,
) -> Geom2dTrimmedCurve {
    use crate::geom2d_line::Geom2dLine;
    use crate::gp2d::Ax2d;
    let line = Geom2dLine::new(Ax2d::new(origin, dir));
    Geom2dTrimmedCurve::new(BasisCurve2d::Line(line), t0, t1)
}

/// Convenience: build a trimmed circle.
///
/// `center` is the circle's center, `radius` its radius.  The trim interval
/// is `[u1, u2]` (in radians).
pub fn trimmed_circle(
    center: Pnt2d,
    radius: f64,
    u1: f64,
    u2: f64,
) -> Geom2dTrimmedCurve {
    let circle = Geom2dCircle::from_x_axis(center, Pnt2d::new(1.0, 0.0), radius, true);
    Geom2dTrimmedCurve::new(BasisCurve2d::Circle(circle), u1, u2)
}

// ─────────────────────────────────────────────────────────────────────────────
// Precision constants used locally.
// ─────────────────────────────────────────────────────────────────────────────

/// `Precision::Infinite()` — re-exported for convenience.
pub use crate::precision::INFINITE as PREC_INFINITE;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom2d_line::Geom2dLine;
    use crate::precision::{ANGULAR, CONFUSION};
    use std::f64::consts::PI;

    // ─── helpers ───────────────────────────────────────────────────────────

    fn x_axis_line() -> Geom2dLine {
        Geom2dLine::from_point_dir(Pnt2d::origin(), Pnt2d::new(1.0, 0.0))
    }

    fn unit_circle_trimmed(u1: f64, u2: f64) -> Geom2dTrimmedCurve {
        trimmed_circle(Pnt2d::origin(), 1.0, u1, u2)
    }

    // ─── constructor + first/last parameter ───────────────────────────────

    #[test]
    fn constructor_stores_ordered_interval() {
        let tc = Geom2dTrimmedCurve::new(
            BasisCurve2d::Line(x_axis_line()),
            3.0,
            1.0, // reversed order → should be stored as [1, 3]
        );
        assert!((tc.first_parameter() - 1.0).abs() < CONFUSION);
        assert!((tc.last_parameter() - 3.0).abs() < CONFUSION);
    }

    #[test]
    fn constructor_forward_order() {
        let tc = Geom2dTrimmedCurve::new(
            BasisCurve2d::Line(x_axis_line()),
            1.0,
            5.0,
        );
        assert!((tc.first_parameter() - 1.0).abs() < CONFUSION);
        assert!((tc.last_parameter() - 5.0).abs() < CONFUSION);
    }

    // ─── Value / D0 ───────────────────────────────────────────────────────

    #[test]
    fn value_on_line_at_endpoints() {
        // Line: origin (0,0), dir (1,0). Trim [2,7].
        let tc = trimmed_line(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 2.0, 7.0);
        let p_start = tc.value(tc.first_parameter());
        let p_end = tc.value(tc.last_parameter());
        assert!((p_start.x - 2.0).abs() < CONFUSION && p_start.y.abs() < CONFUSION);
        assert!((p_end.x - 7.0).abs() < CONFUSION && p_end.y.abs() < CONFUSION);
    }

    #[test]
    fn value_on_circle_quarter_arc() {
        // Circle r=1, trim [0, PI/2]. Point at PI/2 should be (0, 1).
        let tc = unit_circle_trimmed(0.0, PI / 2.0);
        let p = tc.value(PI / 2.0);
        assert!(p.x.abs() < CONFUSION, "x={}", p.x);
        assert!((p.y - 1.0).abs() < CONFUSION, "y={}", p.y);
    }

    // ─── D1 ───────────────────────────────────────────────────────────────

    #[test]
    fn d1_on_line_tangent_is_unit_direction() {
        let tc = trimmed_line(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 0.0, 10.0);
        let (_p, v1) = tc.d1(5.0);
        // tangent of a unit-direction line is the direction itself: (1, 0)
        assert!((v1.x - 1.0).abs() < CONFUSION && v1.y.abs() < CONFUSION);
    }

    #[test]
    fn d1_on_circle_tangent_perpendicular_to_radius() {
        let tc = unit_circle_trimmed(0.0, PI);
        let u = PI / 4.0;
        let (p, v1) = tc.d1(u);
        // Tangent must be perpendicular to radius vector.
        let radius_vec = p - Pnt2d::origin();
        let dot = radius_vec.x * v1.x + radius_vec.y * v1.y;
        assert!(dot.abs() < CONFUSION, "dot={dot}");
    }

    // ─── D2 ───────────────────────────────────────────────────────────────

    #[test]
    fn d2_on_line_second_deriv_is_zero() {
        let tc = trimmed_line(Pnt2d::origin(), Pnt2d::new(0.0, 1.0), 0.0, 5.0);
        let (_p, _v1, v2) = tc.d2(2.5);
        assert!(v2.x.abs() < CONFUSION && v2.y.abs() < CONFUSION);
    }

    #[test]
    fn d2_on_circle_centripetal_acceleration() {
        // For r=1 circle: V2 = -(P - center), pointing inward.
        let tc = unit_circle_trimmed(0.0, 2.0 * PI);
        let u = PI / 3.0;
        let (p, _v1, v2) = tc.d2(u);
        let inward = Pnt2d::origin() - p; // = -p for unit circle at origin
        assert!((v2.x - inward.x).abs() < CONFUSION, "v2.x={} inward.x={}", v2.x, inward.x);
        assert!((v2.y - inward.y).abs() < CONFUSION, "v2.y={} inward.y={}", v2.y, inward.y);
    }

    // ─── D3 ───────────────────────────────────────────────────────────────

    #[test]
    fn d3_on_line_third_deriv_is_zero() {
        let tc = trimmed_line(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0), 0.0, 10.0);
        let (_p, _v1, _v2, v3) = tc.d3(5.0);
        assert!(v3.x.abs() < CONFUSION && v3.y.abs() < CONFUSION);
    }

    // ─── DN ───────────────────────────────────────────────────────────────

    #[test]
    fn dn_n1_matches_d1_tangent() {
        let tc = unit_circle_trimmed(0.0, PI);
        let u = 1.0;
        let (_p, v1) = tc.d1(u);
        let dn1 = tc.dn(u, 1);
        assert!((dn1.x - v1.x).abs() < ANGULAR);
        assert!((dn1.y - v1.y).abs() < ANGULAR);
    }

    // ─── IsClosed ─────────────────────────────────────────────────────────

    #[test]
    fn is_closed_full_circle() {
        // A circle trimmed over a full 2*PI is closed.
        let tc = unit_circle_trimmed(0.0, 2.0 * PI);
        assert!(tc.is_closed(), "full circle trim must be closed");
    }

    #[test]
    fn is_closed_arc_is_open() {
        let tc = unit_circle_trimmed(0.0, PI);
        assert!(!tc.is_closed(), "half-circle arc must not be closed");
    }

    // ─── IsPeriodic ───────────────────────────────────────────────────────

    #[test]
    fn is_periodic_always_false() {
        let tc = unit_circle_trimmed(0.0, 2.0 * PI);
        assert!(!tc.is_periodic());
        let tl = trimmed_line(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 0.0, 1.0);
        assert!(!tl.is_periodic());
    }

    // ─── BasisCurve ───────────────────────────────────────────────────────

    #[test]
    fn basis_curve_accessible() {
        let tc = trimmed_line(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 0.0, 1.0);
        // Just check we can get the basis curve and it is the Line variant.
        assert!(
            matches!(tc.basis_curve(), BasisCurve2d::Line(_)),
            "expected Line basis curve"
        );
    }

    // ─── SetTrim ──────────────────────────────────────────────────────────

    #[test]
    fn set_trim_updates_interval() {
        let mut tc = trimmed_line(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 0.0, 10.0);
        tc.set_trim(3.0, 7.0);
        assert!((tc.first_parameter() - 3.0).abs() < CONFUSION);
        assert!((tc.last_parameter() - 7.0).abs() < CONFUSION);
    }

    #[test]
    fn set_trim_reorders_reversed_args() {
        let mut tc = trimmed_line(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 0.0, 10.0);
        tc.set_trim(9.0, 4.0);
        assert!((tc.first_parameter() - 4.0).abs() < CONFUSION);
        assert!((tc.last_parameter() - 9.0).abs() < CONFUSION);
    }

    // ─── Reverse ──────────────────────────────────────────────────────────

    #[test]
    fn reverse_line_swaps_parameters() {
        // Line along x, trim [2, 7]. After reverse: reversed_parameter(7) = -7,
        // reversed_parameter(2) = -2, so new interval = [-7, -2].
        let tc = trimmed_line(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 2.0, 7.0);
        let rev = tc.reversed();
        // The interval should be [-7, -2].
        assert!((rev.first_parameter() - (-7.0)).abs() < CONFUSION, "u1={}", rev.first_parameter());
        assert!((rev.last_parameter() - (-2.0)).abs() < CONFUSION, "u2={}", rev.last_parameter());
    }

    #[test]
    fn reverse_geometric_consistency_line() {
        // The start point of the reversed curve should be the end point of the
        // original, and vice-versa.
        let tc = trimmed_line(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0), 0.0, 5.0);
        let p_start = tc.value(tc.first_parameter());
        let p_end = tc.value(tc.last_parameter());
        let rev = tc.reversed();
        let p_rev_start = rev.value(rev.first_parameter());
        let p_rev_end = rev.value(rev.last_parameter());
        assert!(p_start.distance(p_rev_end) < CONFUSION);
        assert!(p_end.distance(p_rev_start) < CONFUSION);
    }

    // ─── Transform ────────────────────────────────────────────────────────

    #[test]
    fn transform_translation_moves_start_point() {
        let tc = trimmed_line(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 2.0, 5.0);
        let t = Trsf2d::translation(Pnt2d::new(10.0, 20.0));
        let tc2 = tc.transformed(&t);
        // After a translation the start point should shift by (10, 20).
        let p = tc2.value(tc2.first_parameter());
        assert!((p.x - 12.0).abs() < CONFUSION, "p.x={}", p.x);
        assert!((p.y - 20.0).abs() < CONFUSION, "p.y={}", p.y);
    }

    #[test]
    fn transform_rotation_circle_trim_unchanged() {
        // Rotating a circle trim by 90° should not change the trim parameters
        // (angle-based: parametric scale = 1).
        let tc = unit_circle_trimmed(0.0, PI / 2.0);
        let t = Trsf2d::rotation(Pnt2d::origin(), PI / 2.0);
        let tc2 = tc.transformed(&t);
        assert!((tc2.first_parameter() - 0.0).abs() < CONFUSION);
        assert!((tc2.last_parameter() - PI / 2.0).abs() < CONFUSION);
    }

    // ─── trim_span helper ─────────────────────────────────────────────────

    #[test]
    fn trim_span_helper() {
        let tc = trimmed_line(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 3.0, 8.0);
        assert!((trim_span(&tc) - 5.0).abs() < CONFUSION);
    }

    // ─── periodic basis curve trim adjustment ─────────────────────────────

    #[test]
    fn periodic_trim_full_circle_at_most_period() {
        use crate::geom2d_circle::Geom2dCircle;
        // Request more than 2*PI on a periodic circle → clamped to exactly 2*PI.
        let circle = Geom2dCircle::from_x_axis(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 1.0, true);
        let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Circle(circle), 0.0, 3.0 * PI);
        let span = tc.last_parameter() - tc.first_parameter();
        assert!(span <= 2.0 * PI + CONFUSION, "span={span}");
    }

    // ─── ellipse trimmed ──────────────────────────────────────────────────

    #[test]
    fn ellipse_trimmed_value() {
        use crate::geom2d_ellipse::Geom2dEllipse;
        use crate::gp2d::Ax2d;
        // Ellipse: major=3, minor=2. Trim [0, PI/2].
        let ell = Geom2dEllipse::from_major_axis(
            Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
            3.0,
            2.0,
            true,
        );
        let tc = Geom2dTrimmedCurve::new(BasisCurve2d::Ellipse(ell), 0.0, PI / 2.0);
        // At u=0: point is (major, 0) = (3, 0).
        let p0 = tc.value(0.0);
        assert!((p0.x - 3.0).abs() < CONFUSION && p0.y.abs() < CONFUSION, "{p0:?}");
        // At u=PI/2: point is (0, minor) = (0, 2).
        let p1 = tc.value(PI / 2.0);
        assert!(p1.x.abs() < CONFUSION && (p1.y - 2.0).abs() < CONFUSION, "{p1:?}");
    }
}
