//! Ported OCCT tests for `Convert_HyperbolicArcToBSplineCurve`
//! (package `TKMath`).
//!
//! Each `#[test]` mirrors a case from OCCT's test suite or verifies a
//! mathematical invariant of the rational quadratic B-spline conic construction
//! for hyperbolas.

use ironstream::convert_hypr_to_bspline::HyprToBSpline;
use ironstream::gp::{Ax3, Pnt};
use ironstream::gp_hypr::Hypr;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, PI};

// ─────────────────────────────────── Helpers ──────────────────────────────────

fn hypr_3_4() -> Hypr {
    Hypr::new(Ax3::identity(), 3.0, 4.0)
}

fn hypr_on_curve(h: &Hypr, t: f64) -> Pnt {
    let a = h.major_radius();
    let b = h.minor_radius();
    let pos = h.position();
    pos.location + pos.x_dir * (a * t.cosh()) + pos.y_dir * (b * t.sinh())
}

fn check_on_hypr(p: Pnt, a: f64, b: f64) -> f64 {
    (p.x / a) * (p.x / a) - (p.y / b) * (p.y / b) - 1.0
}

// ─────────────────────────────────── Tests ────────────────────────────────────

/// A short arc (< π/2) → 1 segment → 3 poles, 2 knots.
#[test]
fn short_arc_one_segment() {
    let conv = HyprToBSpline::new(&hypr_3_4(), -0.5, 0.5);
    assert_eq!(conv.nb_poles(), 3, "short arc must have 3 poles");
    assert_eq!(conv.nb_knots(), 2, "short arc must have 2 knots");
}

/// An arc just over π/2 → 2 segments → 5 poles, 3 knots.
#[test]
fn medium_arc_two_segments() {
    let conv = HyprToBSpline::new(&hypr_3_4(), 0.0, FRAC_PI_2 + 0.01);
    assert_eq!(conv.nb_poles(), 5, "medium arc must have 5 poles");
    assert_eq!(conv.nb_knots(), 3, "medium arc must have 3 knots");
}

/// An arc of π → 2 segments → 5 poles, 3 knots.
#[test]
fn pi_arc_two_segments() {
    let conv = HyprToBSpline::new(&hypr_3_4(), 0.0, PI);
    assert_eq!(conv.nb_poles(), 5);
    assert_eq!(conv.nb_knots(), 3);
}

/// An arc of 2π → 4 segments → 9 poles, 5 knots.
#[test]
fn two_pi_arc_four_segments() {
    let conv = HyprToBSpline::new(&hypr_3_4(), 0.0, 2.0 * PI);
    assert_eq!(conv.nb_poles(), 9);
    assert_eq!(conv.nb_knots(), 5);
}

/// Knots are strictly increasing.
#[test]
fn knots_strictly_increasing() {
    let conv = HyprToBSpline::new(&hypr_3_4(), -1.5, 1.5);
    for i in 1..conv.nb_knots() {
        assert!(
            conv.knot(i + 1) > conv.knot(i),
            "knots not strictly increasing at i={}",
            i
        );
    }
}

/// End-point multiplicities are 3; interior multiplicities are 2.
#[test]
fn mults_clamped_structure() {
    let conv = HyprToBSpline::new(&hypr_3_4(), 0.0, 2.0 * PI);
    let n = conv.nb_knots();
    assert_eq!(conv.multiplicity(1), 3, "first knot mult must be 3");
    assert_eq!(conv.multiplicity(n), 3, "last knot mult must be 3");
    for i in 2..n {
        assert_eq!(
            conv.multiplicity(i),
            2,
            "interior knot {} mult must be 2",
            i
        );
    }
}

/// Endpoint poles must lie exactly on the hyperbola.
#[test]
fn endpoint_poles_on_hyperbola() {
    let h = hypr_3_4();
    let t0 = -1.0_f64;
    let t1 = 1.0_f64;
    let conv = HyprToBSpline::new(&h, t0, t1);

    let expected_start = hypr_on_curve(&h, t0);
    let expected_end = hypr_on_curve(&h, t1);

    let p_start = conv.pole(1);
    let p_end = conv.pole(conv.nb_poles());

    assert!(
        p_start.is_equal(expected_start, CONFUSION * 10.0),
        "start pole: got {p_start:?}, expected {expected_start:?}"
    );
    assert!(
        p_end.is_equal(expected_end, CONFUSION * 10.0),
        "end pole: got {p_end:?}, expected {expected_end:?}"
    );
}

/// End-point weights are 1.0; inner weight is > 1.0 (hyperbola case).
#[test]
fn weights_structure() {
    let h = hypr_3_4();
    let conv = HyprToBSpline::new(&h, -0.5, 0.5);

    let tol = ANGULAR * 10.0;
    assert!((conv.weight(1) - 1.0).abs() < tol, "w1={}", conv.weight(1));
    assert!((conv.weight(3) - 1.0).abs() < tol, "w3={}", conv.weight(3));
    // Inner weight for the hyperbola is cosh(half-span) > 1.
    assert!(
        conv.weight(2) > 1.0,
        "inner weight {} should be > 1.0 for hyperbola",
        conv.weight(2)
    );
}

/// Inner weight for a single segment matches cosh(half-span).
#[test]
fn inner_weight_equals_cosh_half_span() {
    let h = hypr_3_4();
    let t0 = -0.6_f64;
    let t1 = 0.6_f64;
    let conv = HyprToBSpline::new(&h, t0, t1);

    let dt = 0.5 * (t1 - t0); // half-span
    // For the hyperbola NURBS, the inner weight is cosh(half-span).
    let expected_w = dt.cosh();
    let tol = ANGULAR * 10.0;
    assert!(
        (conv.weight(2) - expected_w).abs() < tol,
        "w2={}, expected cosh({})={}",
        conv.weight(2),
        dt,
        expected_w
    );
}

/// Evaluating the NURBS at alpha1 and alpha2 gives the correct hyperbola endpoints.
#[test]
fn value_at_endpoints_matches_hyperbola() {
    let h = hypr_3_4();
    let t0 = -0.8_f64;
    let t1 = 0.8_f64;
    let conv = HyprToBSpline::new(&h, t0, t1);

    let p_start = conv.value(t0);
    let p_end = conv.value(t1);

    let expected_start = hypr_on_curve(&h, t0);
    let expected_end = hypr_on_curve(&h, t1);

    assert!(
        p_start.is_equal(expected_start, CONFUSION * 100.0),
        "value at t0: got {p_start:?}, expected {expected_start:?}"
    );
    assert!(
        p_end.is_equal(expected_end, CONFUSION * 100.0),
        "value at t1: got {p_end:?}, expected {expected_end:?}"
    );
}

/// All sampled NURBS points must lie on the hyperbola (x/a)² - (y/b)² = 1.
#[test]
fn sampled_points_lie_on_hyperbola() {
    let a = 3.0_f64;
    let b = 4.0_f64;
    let h = Hypr::new(Ax3::identity(), a, b);
    let t0 = -1.0_f64;
    let t1 = 1.0_f64;
    let conv = HyprToBSpline::new(&h, t0, t1);

    let n = 50;
    for i in 0..=n {
        let u = t0 + (t1 - t0) * i as f64 / n as f64;
        let p = conv.value(u);
        let err = check_on_hypr(p, a, b);
        assert!(
            err.abs() < CONFUSION * 1000.0,
            "u={u:.4}: err={err:e}, p={p:?}"
        );
    }
}

/// Multi-segment arc: all sampled points lie on the hyperbola.
#[test]
fn multi_segment_sampled_points_on_hyperbola() {
    let a = 5.0_f64;
    let b = 2.0_f64;
    let h = Hypr::new(Ax3::identity(), a, b);
    let t0 = -2.0_f64;
    let t1 = 2.0_f64;
    let conv = HyprToBSpline::new(&h, t0, t1);

    let n = 60;
    for i in 0..=n {
        let u = t0 + (t1 - t0) * i as f64 / n as f64;
        let p = conv.value(u);
        let err = check_on_hypr(p, a, b);
        assert!(
            err.abs() < CONFUSION * 5000.0,
            "u={u:.4}: err={err:e}, p={p:?}"
        );
    }
}

/// Pole(0) must panic (1-based index).
#[test]
fn pole_index_zero_panics() {
    let conv = HyprToBSpline::new(&hypr_3_4(), -0.5, 0.5);
    let r = std::panic::catch_unwind(move || conv.pole(0));
    assert!(r.is_err(), "pole(0) must panic");
}

/// Weight(NbPoles + 1) must panic.
#[test]
fn weight_out_of_range_panics() {
    let conv = HyprToBSpline::new(&hypr_3_4(), -0.5, 0.5);
    let n = conv.nb_poles();
    let r = std::panic::catch_unwind(move || {
        let h2 = Hypr::new(Ax3::identity(), 3.0, 4.0);
        HyprToBSpline::new(&h2, -0.5, 0.5).weight(n + 1)
    });
    assert!(r.is_err(), "weight(n+1) must panic");
}

/// Knot(0) must panic (1-based index).
#[test]
fn knot_index_zero_panics() {
    let conv = HyprToBSpline::new(&hypr_3_4(), -0.5, 0.5);
    let r = std::panic::catch_unwind(move || conv.knot(0));
    assert!(r.is_err(), "knot(0) must panic");
}

/// alpha2 <= alpha1 must panic.
#[test]
fn reversed_interval_panics() {
    let h = hypr_3_4();
    let r = std::panic::catch_unwind(|| HyprToBSpline::new(&h, 1.0, 0.5));
    assert!(r.is_err(), "reversed interval must panic");
}

/// The NURBS correctly represents the vertex H(0) = (a, 0, 0) for a symmetric arc.
#[test]
fn symmetric_arc_vertex_at_zero() {
    let a = 3.0_f64;
    let b = 4.0_f64;
    let h = Hypr::new(Ax3::identity(), a, b);
    let conv = HyprToBSpline::new(&h, -0.3, 0.3);

    let p = conv.value(0.0);
    let expected = Pnt::new(a, 0.0, 0.0);
    assert!(
        p.is_equal(expected, CONFUSION * 100.0),
        "vertex at t=0: got {p:?}, expected {expected:?}"
    );
}
