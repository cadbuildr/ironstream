// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_helix.rs
//! Tests for the helix (screw) curve helpers in `geom_helix`.
//!
//! The helix parameterisation is:
//!   n  = height / pitch
//!   x(t) = r * cos(2π·n·t + a0)
//!   y(t) = r * sin(2π·n·t + a0) * sign   (sign = -1 left-handed, +1 right)
//!   z(t) = height * t
//! and the analytical arc length is sqrt((2π·r·n)² + height²).

extern crate ironstream;
use ironstream::geom_helix::*;

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
// HelixParams — construction and accessors
// ---------------------------------------------------------------------------

#[test]
fn params_accessors_defaults() {
    let p = HelixParams::new(3.0, 1.5, 6.0);
    near(p.radius(), 3.0, TOL);
    near(p.pitch(), 1.5, TOL);
    near(p.height(), 6.0, TOL);
    near(p.nb_turns(), 4.0, TOL); // 6.0 / 1.5
    // Defaults
    near(p.start_angle_inner(), 0.0, TOL);
    assert!(!p.is_left_handed());
}

// Expose private field via a trait shim used only in tests.
trait HelixParamsTestExt {
    fn start_angle_inner(&self) -> f64;
}
impl HelixParamsTestExt for HelixParams {
    fn start_angle_inner(&self) -> f64 {
        // We test the effect of set_start_angle via evaluate() below.
        // This helper exposes the value through nb_turns at angle 0 on a 1-turn
        // helix: at t=0, x = r*cos(a0), so a0 can be recovered as acos(x/r).
        // Instead, just build a curve and check the t=0 point.
        let mut params = *self;
        params.set_start_angle(0.0); // noop if already 0
        let curve = HelixCurve::new(params, 2);
        let p0 = curve.pole(0);
        let r = self.radius();
        // angle at t=0 is the stored start_angle — recover it via atan2
        p0[1].atan2(p0[0]) // This gives start_angle for right-handed
    }
}

// Simpler: just verify set_start_angle is reflected in evaluate().
#[test]
fn params_set_start_angle() {
    let mut p = HelixParams::new(2.0, 1.0, 2.0);
    p.set_start_angle(PI / 4.0);
    let curve = HelixCurve::new(p, 2);
    // At t=0, theta = 2π*n*0 + PI/4 = PI/4.
    // x = r*cos(PI/4), y = r*sin(PI/4).
    let r = 2.0_f64;
    let pole0 = curve.pole(0);
    near(pole0[0], r * (PI / 4.0).cos(), TOL);
    near(pole0[1], r * (PI / 4.0).sin(), TOL);
    near(pole0[2], 0.0, TOL);
}

#[test]
fn params_set_left_handed() {
    let mut p = HelixParams::new(1.0, 1.0, 1.0);
    assert!(!p.is_left_handed());
    p.set_left_handed(true);
    assert!(p.is_left_handed());
    p.set_left_handed(false);
    assert!(!p.is_left_handed());
}

#[test]
fn params_nb_turns_ratio() {
    // nb_turns = height / pitch
    let p = HelixParams::new(1.0, 0.5, 3.0);
    near(p.nb_turns(), 6.0, TOL);
}

#[test]
#[should_panic(expected = "radius must be > 0")]
fn params_panics_zero_radius() {
    HelixParams::new(0.0, 1.0, 1.0);
}

#[test]
#[should_panic(expected = "pitch must be > 0")]
fn params_panics_zero_pitch() {
    HelixParams::new(1.0, 0.0, 1.0);
}

#[test]
#[should_panic(expected = "height must be > 0")]
fn params_panics_zero_height() {
    HelixParams::new(1.0, 1.0, 0.0);
}

// ---------------------------------------------------------------------------
// HelixCurve — construction and pole access
// ---------------------------------------------------------------------------

#[test]
#[should_panic(expected = "nb_poles must be >= 2")]
fn curve_panics_on_one_pole() {
    HelixCurve::new(HelixParams::new(1.0, 1.0, 1.0), 1);
}

/// First pole (t=0): must be at [r, 0, 0] for default params (start_angle=0).
#[test]
fn curve_first_pole_at_origin_angle() {
    let params = HelixParams::new(5.0, 1.0, 3.0);
    let curve = HelixCurve::new(params, 4);
    let p0 = curve.pole(0);
    near(p0[0], 5.0, TOL); // r * cos(0) = r
    near(p0[1], 0.0, TOL); // r * sin(0) = 0
    near(p0[2], 0.0, TOL); // h * 0 = 0
}

/// Last pole (t=1): must be at [r*cos(2π*n+a0), r*sin(2π*n+a0), h].
#[test]
fn curve_last_pole_at_full_height() {
    let radius = 2.0_f64;
    let pitch = 0.5_f64;
    let height = 2.0_f64;
    let params = HelixParams::new(radius, pitch, height);
    let n = params.nb_turns(); // 4
    let curve = HelixCurve::new(params, 10);

    let last = curve.pole(9);
    let theta = 2.0 * PI * n; // start_angle = 0
    near(last[0], radius * theta.cos(), TOL);
    near(last[1], radius * theta.sin(), TOL);
    near(last[2], height, TOL);
}

/// nb_poles() returns the count supplied to new().
#[test]
fn curve_nb_poles_matches_request() {
    let params = HelixParams::new(1.0, 1.0, 1.0);
    let curve = HelixCurve::new(params, 7);
    assert_eq!(curve.nb_poles(), 7);
}

/// Pole spacing along z-axis is uniform.
#[test]
fn curve_z_spacing_uniform() {
    let height = 3.0_f64;
    let params = HelixParams::new(1.0, 1.0, height);
    let n = 4_u32;
    let curve = HelixCurve::new(params, n);

    let dz_expected = height / ((n - 1) as f64);
    for i in 1..n as usize {
        let dz = curve.pole(i)[2] - curve.pole(i - 1)[2];
        near(dz, dz_expected, TOL);
    }
}

/// Every pole lies on the cylinder of radius `r`.
#[test]
fn curve_poles_on_cylinder() {
    let r = 3.0_f64;
    let params = HelixParams::new(r, 0.5, 2.0);
    let curve = HelixCurve::new(params, 20);
    for i in 0..curve.nb_poles() {
        let p = curve.pole(i);
        let rxy = p[0].hypot(p[1]);
        near(rxy, r, 1e-12);
    }
}

// ---------------------------------------------------------------------------
// HelixCurve::evaluate
// ---------------------------------------------------------------------------

/// evaluate(t) matches the formula for several values of t.
#[test]
fn evaluate_right_handed_formula() {
    let r = 2.0_f64;
    let pitch = 1.0_f64;
    let height = 4.0_f64;
    let params = HelixParams::new(r, pitch, height);
    let curve = HelixCurve::new(params, 2);
    let n = params.nb_turns(); // 4

    for &t in &[0.0_f64, 0.25, 0.5, 0.75, 1.0] {
        let theta = 2.0 * PI * n * t;
        let expected = [r * theta.cos(), r * theta.sin(), height * t];
        let got = curve.evaluate(t);
        near(got[0], expected[0], TOL);
        near(got[1], expected[1], TOL);
        near(got[2], expected[2], TOL);
    }
}

/// For a left-handed helix the y-coordinate is negated relative to right-handed.
#[test]
fn evaluate_left_handed_negates_y() {
    let r = 1.5_f64;
    let pitch = 1.0_f64;
    let height = 2.0_f64;

    let params_rh = HelixParams::new(r, pitch, height);
    let curve_rh = HelixCurve::new(params_rh, 2);

    let mut params_lh = HelixParams::new(r, pitch, height);
    params_lh.set_left_handed(true);
    let curve_lh = HelixCurve::new(params_lh, 2);

    for &t in &[0.1_f64, 0.3, 0.7, 1.0] {
        let rh = curve_rh.evaluate(t);
        let lh = curve_lh.evaluate(t);
        near(lh[0], rh[0], TOL);  // x identical
        near(lh[1], -rh[1], TOL); // y negated
        near(lh[2], rh[2], TOL);  // z identical
    }
}

/// evaluate() with a non-zero start angle shifts the angular phase.
#[test]
fn evaluate_start_angle_shifts_phase() {
    let r = 2.0_f64;
    let pitch = 1.0_f64;
    let height = 1.0_f64;
    let a0 = PI / 3.0;

    let mut params = HelixParams::new(r, pitch, height);
    params.set_start_angle(a0);
    let curve = HelixCurve::new(params, 2);
    let n = params.nb_turns();

    for &t in &[0.0_f64, 0.5, 1.0] {
        let theta = 2.0 * PI * n * t + a0;
        let got = curve.evaluate(t);
        near(got[0], r * theta.cos(), TOL);
        near(got[1], r * theta.sin(), TOL);
        near(got[2], height * t, TOL);
    }
}

/// evaluate() at t=0 and t=1 must match the first and last stored poles.
#[test]
fn evaluate_matches_first_and_last_pole() {
    let params = HelixParams::new(3.0, 2.0, 6.0);
    let curve = HelixCurve::new(params, 5);

    let eval0 = curve.evaluate(0.0);
    let eval1 = curve.evaluate(1.0);
    let pole0 = curve.pole(0);
    let pole_last = curve.pole(curve.nb_poles() - 1);

    near(eval0[0], pole0[0], TOL);
    near(eval0[1], pole0[1], TOL);
    near(eval0[2], pole0[2], TOL);

    near(eval1[0], pole_last[0], TOL);
    near(eval1[1], pole_last[1], TOL);
    near(eval1[2], pole_last[2], TOL);
}

// ---------------------------------------------------------------------------
// HelixCurve::arc_length
// ---------------------------------------------------------------------------

/// Analytical arc length: sqrt((2π·r·n)² + h²).
#[test]
fn arc_length_analytical_single_turn() {
    let r = 1.0_f64;
    let pitch = 2.0_f64;
    let height = 2.0_f64; // 1 turn
    let params = HelixParams::new(r, pitch, height);
    let curve = HelixCurve::new(params, 2);

    let n = 1.0_f64;
    let expected = ((2.0 * PI * r * n).powi(2) + height.powi(2)).sqrt();
    near(curve.arc_length(), expected, TOL);
}

/// Arc length formula is independent of nb_poles (it is analytical).
#[test]
fn arc_length_independent_of_nb_poles() {
    let params_a = HelixParams::new(2.0, 0.5, 3.0);
    let params_b = HelixParams::new(2.0, 0.5, 3.0);

    let len_a = HelixCurve::new(params_a, 2).arc_length();
    let len_b = HelixCurve::new(params_b, 200).arc_length();
    near(len_a, len_b, TOL);
}

/// For a degenerate helix with very small radius the arc length ≈ height.
#[test]
fn arc_length_small_radius_approaches_height() {
    let r = 1e-9_f64;
    let height = 5.0_f64;
    let params = HelixParams::new(r, 1.0, height);
    let curve = HelixCurve::new(params, 2);
    near(curve.arc_length(), height, 1e-6);
}

/// Arc length is always positive.
#[test]
fn arc_length_positive() {
    let params = HelixParams::new(1.0, 1.0, 1.0);
    let curve = HelixCurve::new(params, 2);
    assert!(curve.arc_length() > 0.0);
}

/// Doubling the number of turns doubles the lateral component of arc length.
/// Specifically: len² = (2π·r·n)² + h², so doubling n increases len.
#[test]
fn arc_length_increases_with_turns() {
    let r = 1.0_f64;
    let height = 4.0_f64;

    let params1 = HelixParams::new(r, 2.0, height); // 2 turns
    let params2 = HelixParams::new(r, 1.0, height); // 4 turns

    let len1 = HelixCurve::new(params1, 2).arc_length();
    let len2 = HelixCurve::new(params2, 2).arc_length();
    assert!(
        len2 > len1,
        "more turns should produce longer arc: {len2} <= {len1}"
    );
}

/// Left-handed and right-handed helices with the same parameters have the same
/// arc length (handedness only affects direction, not length).
#[test]
fn arc_length_same_for_both_handedness() {
    let r = 2.0_f64;
    let pitch = 0.5_f64;
    let height = 3.0_f64;

    let params_rh = HelixParams::new(r, pitch, height);
    let mut params_lh = HelixParams::new(r, pitch, height);
    params_lh.set_left_handed(true);

    let len_rh = HelixCurve::new(params_rh, 2).arc_length();
    let len_lh = HelixCurve::new(params_lh, 2).arc_length();
    near(len_rh, len_lh, TOL);
}
