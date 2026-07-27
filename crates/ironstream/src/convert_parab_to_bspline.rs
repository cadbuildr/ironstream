//! `Convert_ParabolicArcToBSplineCurve` — converts a parabolic arc to a
//! non-rational quadratic B-spline (polynomial Bezier), mirroring OpenCascade's
//! `Convert_ParabolicArcToBSplineCurve` in package `TKMath`.
//!
//! # Algorithm
//!
//! A parabola is a degree-2 polynomial curve, so any arc can be represented
//! **exactly** as a single quadratic Bézier segment (3 poles, all weights 1).
//!
//! In the local frame of the parabola (vertex at origin, axis of symmetry
//! along `XDir`, tangent at vertex along `YDir`):
//!
//! ```text
//! P(t) = Vertex + (t² / (4·focal))·XDir + t·YDir
//! ```
//!
//! For the arc from `t0 = alpha1` to `t1 = alpha2`, the three Bézier poles are:
//!
//! ```text
//! B0 = P(t0)
//! B1 = Vertex + (t0·t1 / (4·focal))·XDir + ((t0 + t1) / 2)·YDir
//! B2 = P(t1)
//! ```
//!
//! The result is a clamped degree-2 B-spline with:
//! - `NbPoles = 3`
//! - `NbKnots = 2`  (knots `alpha1` and `alpha2`, each with multiplicity 3)
//!
//! # Reference
//!
//! This derivation follows the standard quadratic-Bézier-from-polynomial
//! construction (de Casteljau, "A Practical Guide to Splines", Boor; also
//! Piegl & Tiller, "The NURBS Book", §9.1 on conic sections).

// occt-ref: Convert_ConicToBSplineCurve

use crate::gp::Pnt;
use crate::gp_parab::Parab;

/// `Convert_ParabolicArcToBSplineCurve` — converts a parabolic arc to a
/// non-rational quadratic B-spline curve.
///
/// A parabola is a polynomial curve of degree 2, so any arc is represented
/// exactly by a single quadratic Bézier segment.  The result is a clamped
/// degree-2 B-spline with 3 poles, all weights equal to 1, and 2 distinct
/// knots each of multiplicity 3.
///
/// # Parameterisation
///
/// The parabola is parameterised by the displacement along `YDir` from the
/// vertex:
///
/// ```text
/// P(t) = Vertex + (t² / (4·focal))·XDir + t·YDir
/// ```
///
/// `alpha1` and `alpha2` are the parameter values at the arc endpoints,
/// `alpha2 > alpha1`.
// occt-ref: Convert_ConicToBSplineCurve
#[derive(Clone, Debug)]
pub struct ParabToBSpline {
    /// Control points (poles), length 3.
    poles: [Pnt; 3],
    /// Distinct knot values: `[alpha1, alpha2]`.
    knots: [f64; 2],
}

impl ParabToBSpline {
    /// `Convert_ParabolicArcToBSplineCurve(Prb, Alpha1, Alpha2)`
    ///
    /// Constructs the quadratic B-spline that exactly represents the parabolic
    /// arc of `prb` between parameters `alpha1` and `alpha2`.
    ///
    /// # Panics
    ///
    /// Panics if `alpha2 <= alpha1`.
    pub fn new(prb: &Parab, alpha1: f64, alpha2: f64) -> Self {
        assert!(
            alpha2 > alpha1,
            "Convert_ParabolicArcToBSplineCurve: alpha2 must be > alpha1"
        );

        let pos = prb.position();
        let vertex = pos.location;
        let x_dir = pos.x_dir;
        let y_dir = pos.y_dir;
        let focal = prb.focal();

        // Four-focal denominator (parabola parameter p = 2*focal, so
        // P(t) = vertex + t²/(2*p)*X + t*Y = vertex + t²/(4*focal)*X + t*Y).
        // When focal == 0 the parabola degenerates to a line along YDir; the
        // X term vanishes, and the formula still holds.
        let inv_4f = if focal.abs() < 1.0e-15 {
            0.0
        } else {
            1.0 / (4.0 * focal)
        };

        let t0 = alpha1;
        let t1 = alpha2;

        // B0 = P(t0)
        let b0 = vertex + x_dir * (t0 * t0 * inv_4f) + y_dir * t0;
        // B1 (middle control point) = Vertex + (t0*t1/(4f))*X + ((t0+t1)/2)*Y
        // This is the intersection of the tangents at P(t0) and P(t1), which
        // is also the midpoint-construction formula for a polynomial degree-2
        // Bézier: B1 = 2*P(mid) - (B0 + B2)/2.
        let b1 = vertex + x_dir * (t0 * t1 * inv_4f) + y_dir * ((t0 + t1) * 0.5);
        // B2 = P(t1)
        let b2 = vertex + x_dir * (t1 * t1 * inv_4f) + y_dir * t1;

        Self {
            poles: [b0, b1, b2],
            knots: [alpha1, alpha2],
        }
    }

    // ─────────────────────────────────── Accessors ────────────────────────────

    /// `NbPoles()` — number of control points (always 3).
    #[inline]
    pub fn nb_poles(&self) -> i32 {
        3
    }

    /// `NbKnots()` — number of distinct knots (always 2).
    #[inline]
    pub fn nb_knots(&self) -> i32 {
        2
    }

    /// `Degree()` — B-spline degree (always 2).
    #[inline]
    pub fn degree(&self) -> i32 {
        2
    }

    /// `IsRational()` — always `false`; the parabola is a polynomial curve.
    #[inline]
    pub fn is_rational(&self) -> bool {
        false
    }

    /// `Pole(Index)` — 1-based control point.
    ///
    /// # Panics
    /// Panics if `index` is not in `[1, 3]`.
    pub fn pole(&self, index: i32) -> Pnt {
        assert!(
            (1..=3).contains(&index),
            "Convert_ParabolicArcToBSplineCurve::Pole: index {index} out of range [1, 3]"
        );
        self.poles[(index - 1) as usize]
    }

    /// `Knot(Index)` — 1-based distinct knot value.
    ///
    /// # Panics
    /// Panics if `index` is not in `[1, 2]`.
    pub fn knot(&self, index: i32) -> f64 {
        assert!(
            (1..=2).contains(&index),
            "Convert_ParabolicArcToBSplineCurve::Knot: index {index} out of range [1, 2]"
        );
        self.knots[(index - 1) as usize]
    }

    /// `Multiplicity(Index)` — 1-based knot multiplicity.
    ///
    /// Both knots have multiplicity 3 (degree + 1), which is correct for a
    /// clamped Bézier B-spline of degree 2.
    ///
    /// # Panics
    /// Panics if `index` is not in `[1, 2]`.
    pub fn multiplicity(&self, index: i32) -> i32 {
        assert!(
            (1..=2).contains(&index),
            "Convert_ParabolicArcToBSplineCurve::Multiplicity: index {index} out of range [1, 2]"
        );
        3
    }

    // ─────────────────────────────────── Evaluation ───────────────────────────

    /// Evaluate the B-spline curve at parameter `u`.
    ///
    /// Since the curve is a single quadratic Bézier arc, de Casteljau's
    /// algorithm is used directly.  `u` is clamped to `[knot(1), knot(2)]`.
    pub fn value(&self, u: f64) -> Pnt {
        let t0 = self.knots[0];
        let t1 = self.knots[1];
        let u = u.clamp(t0, t1);

        // Normalise u → s ∈ [0, 1].
        let s = if (t1 - t0).abs() < 1.0e-15 {
            0.0
        } else {
            (u - t0) / (t1 - t0)
        };

        // De Casteljau, degree 2.
        let b0 = self.poles[0];
        let b1 = self.poles[1];
        let b2 = self.poles[2];
        let q0 = b0 * (1.0 - s) + b1 * s;
        let q1 = b1 * (1.0 - s) + b2 * s;
        q0 * (1.0 - s) + q1 * s
    }

    /// All poles as a slice (0-based).
    #[inline]
    pub fn poles(&self) -> &[Pnt] {
        &self.poles
    }

    /// Both distinct knot values as a slice (0-based).
    #[inline]
    pub fn knots_slice(&self) -> &[f64] {
        &self.knots
    }
}

// ─────────────────────────────────── Unit tests ───────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::{Ax3, Pnt};
    use crate::precision::CONFUSION;

    fn standard_parab() -> Parab {
        Parab::new(Ax3::identity(), 1.0)
    }

    /// Helper: evaluate P(t) directly from the parabola formula.
    fn parab_point(prb: &Parab, t: f64) -> Pnt {
        let pos = prb.position();
        let focal = prb.focal();
        let inv_4f = 1.0 / (4.0 * focal);
        pos.location + pos.x_dir * (t * t * inv_4f) + pos.y_dir * t
    }

    #[test]
    fn nb_poles_is_always_3() {
        let p = standard_parab();
        let conv = ParabToBSpline::new(&p, -1.0, 1.0);
        assert_eq!(conv.nb_poles(), 3);
    }

    #[test]
    fn nb_knots_is_always_2() {
        let p = standard_parab();
        let conv = ParabToBSpline::new(&p, 0.0, 2.0);
        assert_eq!(conv.nb_knots(), 2);
    }

    #[test]
    fn knots_match_alpha1_alpha2() {
        let p = standard_parab();
        let conv = ParabToBSpline::new(&p, -3.0, 5.0);
        assert!((conv.knot(1) - (-3.0)).abs() < CONFUSION);
        assert!((conv.knot(2) - 5.0).abs() < CONFUSION);
    }

    #[test]
    fn pole1_equals_parab_at_alpha1() {
        let p = standard_parab();
        let a1 = -2.0;
        let a2 = 2.0;
        let conv = ParabToBSpline::new(&p, a1, a2);
        let expected = parab_point(&p, a1);
        assert!(
            conv.pole(1).is_equal(expected, CONFUSION),
            "pole(1)={:?} expected={:?}",
            conv.pole(1),
            expected
        );
    }

    #[test]
    fn pole3_equals_parab_at_alpha2() {
        let p = standard_parab();
        let a1 = -2.0;
        let a2 = 3.0;
        let conv = ParabToBSpline::new(&p, a1, a2);
        let expected = parab_point(&p, a2);
        assert!(
            conv.pole(3).is_equal(expected, CONFUSION),
            "pole(3)={:?} expected={:?}",
            conv.pole(3),
            expected
        );
    }

    #[test]
    fn value_at_alpha1_equals_parab_at_alpha1() {
        let p = standard_parab();
        let a1 = -1.0;
        let a2 = 2.0;
        let conv = ParabToBSpline::new(&p, a1, a2);
        let v = conv.value(a1);
        let expected = parab_point(&p, a1);
        assert!(
            v.is_equal(expected, CONFUSION),
            "value(alpha1)={v:?} expected={expected:?}"
        );
    }

    #[test]
    fn value_at_alpha2_equals_parab_at_alpha2() {
        let p = standard_parab();
        let a1 = -1.0;
        let a2 = 2.0;
        let conv = ParabToBSpline::new(&p, a1, a2);
        let v = conv.value(a2);
        let expected = parab_point(&p, a2);
        assert!(
            v.is_equal(expected, CONFUSION),
            "value(alpha2)={v:?} expected={expected:?}"
        );
    }

    #[test]
    fn sampled_points_lie_on_parabola() {
        // Sample the BSpline at many parameter values and verify each point
        // matches the direct parabola evaluation.
        let p = standard_parab();
        let a1 = -3.0_f64;
        let a2 = 3.0_f64;
        let conv = ParabToBSpline::new(&p, a1, a2);
        let n = 20;
        for i in 0..=n {
            let t = a1 + (a2 - a1) * i as f64 / n as f64;
            let bspline_pt = conv.value(t);
            let direct_pt = parab_point(&p, t);
            assert!(
                bspline_pt.is_equal(direct_pt, CONFUSION),
                "i={i}: bspline={bspline_pt:?} direct={direct_pt:?}"
            );
        }
    }

    #[test]
    fn is_rational_is_false() {
        let p = standard_parab();
        let conv = ParabToBSpline::new(&p, 0.0, 1.0);
        assert!(!conv.is_rational());
    }

    #[test]
    fn degree_is_2() {
        let p = standard_parab();
        let conv = ParabToBSpline::new(&p, 0.0, 1.0);
        assert_eq!(conv.degree(), 2);
    }

    #[test]
    fn multiplicity_is_3() {
        let p = standard_parab();
        let conv = ParabToBSpline::new(&p, 0.0, 1.0);
        assert_eq!(conv.multiplicity(1), 3);
        assert_eq!(conv.multiplicity(2), 3);
    }

    #[test]
    fn pole_out_of_range_panics() {
        let p = standard_parab();
        let conv = ParabToBSpline::new(&p, 0.0, 1.0);
        let r = std::panic::catch_unwind(|| conv.pole(0));
        assert!(r.is_err(), "pole(0) should panic");
        let r = std::panic::catch_unwind(|| conv.pole(4));
        assert!(r.is_err(), "pole(4) should panic");
    }

    #[test]
    fn knot_out_of_range_panics() {
        let p = standard_parab();
        let conv = ParabToBSpline::new(&p, 0.0, 1.0);
        let r = std::panic::catch_unwind(|| conv.knot(3));
        assert!(r.is_err(), "knot(3) should panic");
    }

    #[test]
    fn middle_pole_is_tangent_intersection() {
        // Verify that B1 is the intersection of tangents at B0 and B2.
        // For a parabola P(t) = V + t*Y + t²/(4f)*X:
        //   Tangent direction at t: Y + t/(2f)*X
        // The tangent line at B0 (t=t0):
        //   L(s) = B0 + s*(Y + t0/(2f)*X)
        // Setting Y component: t0 + s == (t0+t1)/2  =>  s = (t1-t0)/2
        // Setting X component: t0²/(4f) + s*t0/(2f)
        //   = t0²/(4f) + (t1-t0)/2 * t0/(2f)
        //   = t0²/(4f) + t0*(t1-t0)/(4f)
        //   = t0*t1/(4f)  ✓  (matches B1's X component)
        let parab = standard_parab();  // focal = 1
        let a1 = 1.0_f64;
        let a2 = 3.0_f64;
        let conv = ParabToBSpline::new(&parab, a1, a2);
        let b1 = conv.pole(2);
        // Expected: B1 = V + (t0*t1/(4f))*X + ((t0+t1)/2)*Y
        //             = origin + (1*3/4) * (1,0,0) + (2) * (0,1,0)
        //             = (0.75, 2.0, 0)
        let expected = Pnt::new(0.75, 2.0, 0.0);
        assert!(
            b1.is_equal(expected, CONFUSION),
            "B1={b1:?} expected={expected:?}"
        );
    }

    #[test]
    fn translated_parabola_poles_shift() {
        use crate::gp::Pnt;
        let ax = crate::gp::Ax3::from_origin_normal(
            Pnt::new(10.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let parab = Parab::new(ax, 1.0);
        let a1 = -1.0;
        let a2 = 1.0;
        let conv = ParabToBSpline::new(&parab, a1, a2);
        // Endpoints lie on the parabola at t=±1: X = vertex.x + t²/(4f) = 10.25, Y = ±1
        assert!((conv.pole(1).x - 10.25).abs() < CONFUSION, "pole(1).x={}", conv.pole(1).x);
        assert!((conv.pole(3).x - 10.25).abs() < CONFUSION, "pole(3).x={}", conv.pole(3).x);
        // The middle control point (tangent intersection) lies INSIDE the parabola
        // at X = vertex.x - focal/4 = 10.0 - 0.25 = 9.75
        assert!((conv.pole(2).x - 9.75).abs() < CONFUSION, "pole(2).x={}", conv.pole(2).x);
    }

    #[test]
    fn symmetric_arc_has_symmetric_poles() {
        // For a symmetric arc [-a, a] around t=0, B0 and B2 should be
        // symmetric about the X-axis; B1 should lie on the X-axis (y=0).
        let p = standard_parab();
        let a = 2.0_f64;
        let conv = ParabToBSpline::new(&p, -a, a);
        let b1 = conv.pole(2);
        // B1 = V + (t0*t1/(4f))*X + ((t0+t1)/2)*Y
        //    = origin + ((-a*a)/4)*X + 0*Y
        // Y-component of B1 should be 0.
        assert!(
            b1.y.abs() < CONFUSION,
            "B1.y={} for symmetric arc, expected 0",
            b1.y
        );
        let b0 = conv.pole(1);
        let b2 = conv.pole(3);
        // B0 and B2 should have the same X coordinate.
        assert!(
            (b0.x - b2.x).abs() < CONFUSION,
            "B0.x={} B2.x={} should match for symmetric arc",
            b0.x,
            b2.x
        );
        // B0 and B2 should have opposite Y coordinates.
        assert!(
            (b0.y + b2.y).abs() < CONFUSION,
            "B0.y={} B2.y={} should be negatives for symmetric arc",
            b0.y,
            b2.y
        );
    }
}
