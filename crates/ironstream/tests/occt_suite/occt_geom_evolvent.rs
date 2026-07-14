//! Tests for the involute (evolvent) curve helpers in `geom_evolvent`.
//!
//! Numerical expectations follow the OCCT involute definition:
//!   P(t) = [r*(cos(t)+t*sin(t)), r*(sin(t)-t*cos(t))]
//! and its derivative:
//!   P'(t) = [r*t*cos(t), r*t*sin(t)]

extern crate ironstream;
use ironstream::geom_evolvent::*;

use std::f64::consts::PI;

/// Absolute-error helper.
fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol}), diff = {}",
        (a - b).abs()
    );
}

const TOL: f64 = 1e-12;

// ---------------------------------------------------------------------------
// involute_point
// ---------------------------------------------------------------------------

/// At t = 0 the involute starts at (r, 0) regardless of base radius.
#[test]
fn involute_point_at_zero() {
    let r = 3.0_f64;
    let p = involute_point(r, 0.0);
    near(p[0], r, TOL);   // r*(cos(0)+0*sin(0)) = r
    near(p[1], 0.0, TOL); // r*(sin(0)-0*cos(0)) = 0
}

/// At t = PI/2:
///   x = r*(cos(PI/2) + (PI/2)*sin(PI/2)) = r*(0 + PI/2) = r*PI/2
///   y = r*(sin(PI/2) - (PI/2)*cos(PI/2)) = r*(1 - 0)    = r
#[test]
fn involute_point_at_half_pi() {
    let r = 1.0_f64;
    let t = PI / 2.0;
    let p = involute_point(r, t);
    near(p[0], r * t, TOL); // r * PI/2
    near(p[1], r * 1.0, TOL);
}

/// At t = PI:
///   x = r*(cos(PI) + PI*sin(PI)) = r*(-1 + 0)  = -r
///   y = r*(sin(PI) - PI*cos(PI)) = r*(0 - (-PI)) = r*PI
#[test]
fn involute_point_at_pi() {
    let r = 2.0_f64;
    let t = PI;
    let p = involute_point(r, t);
    near(p[0], -r, TOL);
    near(p[1], r * PI, TOL);
}

/// Scaling the radius scales both coordinates proportionally.
#[test]
fn involute_point_scales_with_radius() {
    let t = 1.23_f64;
    let p1 = involute_point(1.0, t);
    let p2 = involute_point(5.0, t);
    near(p2[0], 5.0 * p1[0], TOL);
    near(p2[1], 5.0 * p1[1], TOL);
}

// ---------------------------------------------------------------------------
// involute_tangent
// ---------------------------------------------------------------------------

/// At t = 0 the tangent is the zero vector (the involute is at rest there).
#[test]
fn involute_tangent_at_zero_is_zero() {
    let tan = involute_tangent(5.0, 0.0);
    near(tan[0], 0.0, TOL);
    near(tan[1], 0.0, TOL);
}

/// At t = PI/2:
///   dx = r*(PI/2)*cos(PI/2) = 0
///   dy = r*(PI/2)*sin(PI/2) = r*PI/2
#[test]
fn involute_tangent_at_half_pi() {
    let r = 1.0_f64;
    let t = PI / 2.0;
    let tan = involute_tangent(r, t);
    near(tan[0], 0.0, TOL);
    near(tan[1], r * t, TOL);
}

/// At t = PI:
///   dx = r*PI*cos(PI) = -r*PI
///   dy = r*PI*sin(PI) = 0
#[test]
fn involute_tangent_at_pi() {
    let r = 3.0_f64;
    let t = PI;
    let tan = involute_tangent(r, t);
    near(tan[0], -r * PI, TOL);
    near(tan[1], 0.0, TOL);
}

/// Tangent direction is perpendicular to the radius at the contact point on the
/// base circle. At parameter t, the contact point on the base circle is
/// (r*cos(t), r*sin(t)), and the tangent [r*t*cos(t), r*t*sin(t)] is parallel
/// to (cos(t), sin(t)), which is indeed perpendicular to (-sin(t), cos(t))
/// (the radius direction). Here we just verify the tangent has the expected
/// magnitude: |tangent| = r*t.
#[test]
fn involute_tangent_magnitude() {
    let r = 2.0_f64;
    let t = 1.5_f64;
    let tan = involute_tangent(r, t);
    let mag = tan[0].hypot(tan[1]);
    near(mag, r * t, TOL);
}

// ---------------------------------------------------------------------------
// InvoluteParams
// ---------------------------------------------------------------------------

#[test]
fn params_accessors() {
    let p = InvoluteParams::new(5.0, 0.1, 2.0, 10);
    near(p.base_radius(), 5.0, TOL);
    near(p.start_angle(), 0.1, TOL);
    near(p.end_angle(), 2.0, TOL);
    assert_eq!(p.nb_poles(), 10);
}

#[test]
#[should_panic(expected = "base_radius must be > 0")]
fn params_panics_on_zero_radius() {
    InvoluteParams::new(0.0, 0.0, 1.0, 5);
}

#[test]
#[should_panic(expected = "nb_poles must be >= 2")]
fn params_panics_on_single_pole() {
    InvoluteParams::new(1.0, 0.0, 1.0, 1);
}

#[test]
#[should_panic(expected = "start_angle and end_angle must differ")]
fn params_panics_on_equal_angles() {
    InvoluteParams::new(1.0, 1.0, 1.0, 4);
}

// ---------------------------------------------------------------------------
// InvoluteCurve — construction & pole access
// ---------------------------------------------------------------------------

/// With 2 poles the first pole must equal involute_point(r, start) and the
/// second must equal involute_point(r, end).
#[test]
fn curve_two_poles_endpoints() {
    let r = 4.0_f64;
    let t0 = 0.0_f64;
    let t1 = PI;
    let params = InvoluteParams::new(r, t0, t1, 2);
    let curve = InvoluteCurve::new(params);

    assert_eq!(curve.nb_poles(), 2);

    let p0_expected = involute_point(r, t0);
    let p1_expected = involute_point(r, t1);

    near(curve.pole(0)[0], p0_expected[0], TOL);
    near(curve.pole(0)[1], p0_expected[1], TOL);
    near(curve.pole(1)[0], p1_expected[0], TOL);
    near(curve.pole(1)[1], p1_expected[1], TOL);
}

/// With n poles, intermediate poles are uniformly spaced in parameter.
#[test]
fn curve_intermediate_poles_uniform() {
    let r = 1.0_f64;
    let t0 = 0.0_f64;
    let t1 = 2.0_f64;
    let n = 5_u32;
    let params = InvoluteParams::new(r, t0, t1, n);
    let curve = InvoluteCurve::new(params);

    assert_eq!(curve.nb_poles(), n as usize);

    for i in 0..n as usize {
        let t = t0 + (t1 - t0) * (i as f64) / ((n - 1) as f64);
        let expected = involute_point(r, t);
        near(curve.pole(i)[0], expected[0], TOL);
        near(curve.pole(i)[1], expected[1], TOL);
    }
}

// ---------------------------------------------------------------------------
// InvoluteCurve::evaluate
// ---------------------------------------------------------------------------

/// evaluate() calls involute_point directly — it is not limited to the
/// sampled range.
#[test]
fn curve_evaluate_matches_involute_point() {
    let r = 3.5_f64;
    let params = InvoluteParams::new(r, 0.0, 1.0, 4);
    let curve = InvoluteCurve::new(params);

    for &t in &[0.0_f64, 0.5, 1.0, 1.7, PI] {
        let expected = involute_point(r, t);
        let got = curve.evaluate(t);
        near(got[0], expected[0], TOL);
        near(got[1], expected[1], TOL);
    }
}

// ---------------------------------------------------------------------------
// InvoluteCurve::arc_length
// ---------------------------------------------------------------------------

/// A curve with 2 poles spanning [0, t1] must have arc_length equal to the
/// chord length between the two endpoints.
#[test]
fn arc_length_two_poles_is_chord() {
    let r = 2.0_f64;
    let t0 = 0.5_f64;
    let t1 = 1.5_f64;
    let params = InvoluteParams::new(r, t0, t1, 2);
    let curve = InvoluteCurve::new(params);

    let p0 = involute_point(r, t0);
    let p1 = involute_point(r, t1);
    let chord = (p1[0] - p0[0]).hypot(p1[1] - p0[1]);

    near(curve.arc_length(), chord, TOL);
}

/// arc_length is monotonically non-decreasing as more poles are used (the
/// chord approximation converges from below to the true arc length).
#[test]
fn arc_length_increases_with_more_poles() {
    let r = 1.0_f64;
    let t0 = 0.0_f64;
    let t1 = PI;

    let len_coarse = InvoluteCurve::new(InvoluteParams::new(r, t0, t1, 4)).arc_length();
    let len_fine   = InvoluteCurve::new(InvoluteParams::new(r, t0, t1, 64)).arc_length();
    let len_finer  = InvoluteCurve::new(InvoluteParams::new(r, t0, t1, 256)).arc_length();

    assert!(
        len_coarse <= len_fine,
        "coarser approximation should not exceed finer: {len_coarse} > {len_fine}"
    );
    assert!(
        len_fine <= len_finer,
        "coarser approximation should not exceed finer: {len_fine} > {len_finer}"
    );
}

/// For a small arc (t in [0, eps]) the chord ≈ r*t^2/2 which is close to zero;
/// arc_length must be non-negative.
#[test]
fn arc_length_non_negative() {
    let params = InvoluteParams::new(1.0, 0.0, 0.01, 2);
    let curve = InvoluteCurve::new(params);
    assert!(curve.arc_length() >= 0.0);
}

/// arc_length with a reversed range (start > end) is the same as the forward
/// range (the curve is the same, just traversed in reverse).
#[test]
fn arc_length_symmetric_forward_reverse() {
    let r = 2.0_f64;
    let t0 = 0.3_f64;
    let t1 = 1.8_f64;
    let n = 32;

    let forward = InvoluteCurve::new(InvoluteParams::new(r, t0, t1, n)).arc_length();
    let reverse = InvoluteCurve::new(InvoluteParams::new(r, t1, t0, n)).arc_length();

    near(forward, reverse, 1e-10);
}
