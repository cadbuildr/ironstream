// FILE: rust/ironstream/crates/ironstream/tests/occt_law_bspline.rs
//! Integration tests for `LawBSpline` — the scalar B-spline law.
//!
//! Each test is independent and verifies a specific aspect of the
//! Cox-de Boor evaluation or the accessor API.

extern crate ironstream;
use ironstream::law_bspline::*;

// ── helpers ──────────────────────────────────────────────────────────────────

fn near(a: f64, b: f64, tol: f64, label: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{label}: |{a} - {b}| = {} > tol {tol}",
        (a - b).abs()
    );
}

// ── degree-1 (linear) ────────────────────────────────────────────────────────

/// A degree-1 B-spline with poles [0.0, 1.0] and knots [0.0, 1.0] with
/// multiplicities [2, 2] (flat vector [0,0,1,1]) must behave as a linear
/// interpolant.  At t = 0.5 the value must be 0.5.
#[test]
fn degree1_linear_midpoint_is_half() {
    let spline = LawBSpline::new(
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![2, 2],
        1,
        false,
    );
    near(spline.value(0.5), 0.5, 1e-12, "value at t=0.5");
}

/// End-point interpolation: a clamped degree-1 spline must pass exactly
/// through its poles at the parametric endpoints.
#[test]
fn degree1_endpoint_interpolation() {
    let spline = LawBSpline::new(
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![2, 2],
        1,
        false,
    );
    near(spline.value(0.0), 0.0, 1e-12, "value at t=0");
    near(spline.value(1.0), 1.0, 1e-12, "value at t=1");
}

/// A degree-1 spline between arbitrary poles must be exactly linear.
#[test]
fn degree1_arbitrary_poles_linear() {
    let spline = LawBSpline::new(
        vec![3.0, 7.0],
        vec![0.0, 1.0],
        vec![2, 2],
        1,
        false,
    );
    near(spline.value(0.25), 4.0, 1e-12, "value at t=0.25");
    near(spline.value(0.75), 6.0, 1e-12, "value at t=0.75");
}

// ── degree-2 (quadratic) ─────────────────────────────────────────────────────

/// A clamped quadratic B-spline with symmetric poles [0, 1, 0] must peak
/// at 0.5 in the middle and return to 0 at both ends.
#[test]
fn degree2_symmetric_peak() {
    let spline = LawBSpline::new(
        vec![0.0, 1.0, 0.0],
        vec![0.0, 1.0],
        vec![3, 3],
        2,
        false,
    );
    near(spline.value(0.0), 0.0, 1e-12, "start");
    near(spline.value(1.0), 0.0, 1e-12, "end");
    // Bernstein B_{1,2}(0.5) = 2 * 0.5 * 0.5 = 0.5
    near(spline.value(0.5), 0.5, 1e-12, "midpoint peak");
}

/// A constant quadratic spline (all poles equal) must return that constant.
#[test]
fn degree2_constant_poles() {
    let spline = LawBSpline::new(
        vec![4.0, 4.0, 4.0],
        vec![0.0, 1.0],
        vec![3, 3],
        2,
        false,
    );
    near(spline.value(0.0), 4.0, 1e-12, "t=0");
    near(spline.value(0.5), 4.0, 1e-12, "t=0.5");
    near(spline.value(1.0), 4.0, 1e-12, "t=1");
}

// ── degree-3 (cubic) ─────────────────────────────────────────────────────────

/// A clamped cubic B-spline with constant poles must return that constant.
#[test]
fn degree3_constant_poles() {
    let spline = LawBSpline::new(
        vec![2.0, 2.0, 2.0, 2.0],
        vec![0.0, 1.0],
        vec![4, 4],
        3,
        false,
    );
    near(spline.value(0.3), 2.0, 1e-12, "t=0.3");
    near(spline.value(0.7), 2.0, 1e-12, "t=0.7");
}

/// End-point interpolation for a clamped cubic spline: value must match first
/// and last pole.
#[test]
fn degree3_endpoint_interpolation() {
    let spline = LawBSpline::new(
        vec![1.0, 3.0, 5.0, 2.0],
        vec![0.0, 1.0],
        vec![4, 4],
        3,
        false,
    );
    near(spline.value(0.0), 1.0, 1e-12, "start pole");
    near(spline.value(1.0), 2.0, 1e-12, "end pole");
}

// ── multi-segment (inner knots) ───────────────────────────────────────────────

/// A degree-1 spline with an inner knot creates two linear segments.
/// Poles [0, 1, 0] with knots [0, 0.5, 1] and mults [2, 1, 2] gives a tent
/// function: rises 0→1 on [0,0.5] and falls 1→0 on [0.5,1].
#[test]
fn degree1_tent_function() {
    let spline = LawBSpline::new(
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.5, 1.0],
        vec![2, 1, 2],
        1,
        false,
    );
    near(spline.value(0.0), 0.0, 1e-12, "start");
    near(spline.value(0.25), 0.5, 1e-12, "rising midpoint");
    near(spline.value(0.5), 1.0, 1e-12, "peak");
    near(spline.value(0.75), 0.5, 1e-12, "falling midpoint");
    near(spline.value(1.0), 0.0, 1e-12, "end");
}

// ── accessor API ─────────────────────────────────────────────────────────────

/// nb_poles, nb_knots, degree, is_periodic must all match construction args.
#[test]
fn accessors_metadata() {
    let spline = LawBSpline::new(
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![2, 2],
        1,
        false,
    );
    assert_eq!(spline.nb_poles(), 2);
    assert_eq!(spline.nb_knots(), 2);
    assert_eq!(spline.degree(), 1);
    assert!(!spline.is_periodic());
}

/// pole() and knot() and multiplicity() must return the values passed at
/// construction (0-based indexing).
#[test]
fn accessors_pole_knot_mult() {
    let spline = LawBSpline::new(
        vec![3.0, 7.0],
        vec![0.0, 2.0],
        vec![2, 2],
        1,
        false,
    );
    assert_eq!(spline.pole(0), 3.0);
    assert_eq!(spline.pole(1), 7.0);
    assert_eq!(spline.knot(0), 0.0);
    assert_eq!(spline.knot(1), 2.0);
    assert_eq!(spline.multiplicity(0), 2);
    assert_eq!(spline.multiplicity(1), 2);
}

/// set_pole must update the pole and affect subsequent evaluations.
#[test]
fn set_pole_mutates_value() {
    let mut spline = LawBSpline::new(
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![2, 2],
        1,
        false,
    );
    near(spline.value(1.0), 1.0, 1e-12, "before mutation");
    spline.set_pole(1, 2.0);
    near(spline.value(1.0), 2.0, 1e-12, "after mutation");
}

/// set_knot must update the knot and keep the spline consistent.
#[test]
fn set_knot_mutates_domain() {
    let mut spline = LawBSpline::new(
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![2, 2],
        1,
        false,
    );
    spline.set_knot(1, 2.0);
    assert_eq!(spline.knot(1), 2.0);
    // After the domain change the midpoint is now at t=1.0.
    near(spline.value(1.0), 0.5, 1e-12, "midpoint of new domain");
}

/// is_periodic flag is stored and returned unchanged.
#[test]
fn is_periodic_flag_stored() {
    let spline = LawBSpline::new(
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![2, 2],
        1,
        true,
    );
    assert!(spline.is_periodic());
}

// ── clamping behaviour ────────────────────────────────────────────────────────

/// Parameters outside the valid domain are clamped to the nearest boundary.
#[test]
fn out_of_range_parameter_clamps() {
    let spline = LawBSpline::new(
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![2, 2],
        1,
        false,
    );
    near(spline.value(-1.0), 0.0, 1e-12, "below domain");
    near(spline.value(2.0), 1.0, 1e-12, "above domain");
}
