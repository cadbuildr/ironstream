// FILE: rust/ironstream/crates/ironstream/tests/occt_law_interpolate.rs
//! Integration tests for `LawInterpolate` and `LawS`.

extern crate ironstream;
use ironstream::law_interpolate::*;

// ── helpers ──────────────────────────────────────────────────────────────────

fn near(a: f64, b: f64, tol: f64, label: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{label}: |{a} - {b}| = {} > tol {tol}",
        (a - b).abs()
    );
}

// ── LawInterpolate ────────────────────────────────────────────────────────────

#[test]
fn test_interpolate_new_sorts_points() {
    // Points supplied in reverse order must be sorted on construction.
    let law = LawInterpolate::new(vec![(3.0, 9.0), (1.0, 1.0), (2.0, 4.0)], false);
    assert_eq!(law.nb_points(), 3);
    assert_eq!(law.point(0), (1.0, 1.0));
    assert_eq!(law.point(1), (2.0, 4.0));
    assert_eq!(law.point(2), (3.0, 9.0));
}

#[test]
fn test_interpolate_is_done_before_perform() {
    let law = LawInterpolate::new(vec![(0.0, 0.0), (1.0, 1.0)], false);
    assert!(!law.is_done());
}

#[test]
fn test_interpolate_is_done_after_perform() {
    let mut law = LawInterpolate::new(vec![(0.0, 0.0), (1.0, 1.0)], false);
    law.perform();
    assert!(law.is_done());
}

#[test]
fn test_interpolate_exact_knot_values() {
    let pts = vec![(0.0, 5.0), (1.0, 10.0), (2.0, 3.0)];
    let mut law = LawInterpolate::new(pts, false);
    law.perform();

    near(law.evaluate(0.0), 5.0, 1e-12, "at first knot");
    near(law.evaluate(1.0), 10.0, 1e-12, "at middle knot");
    near(law.evaluate(2.0), 3.0, 1e-12, "at last knot");
}

#[test]
fn test_interpolate_midpoint_linear() {
    // Between (0,0) and (1,1) the midpoint should be 0.5.
    let mut law = LawInterpolate::new(vec![(0.0, 0.0), (1.0, 1.0)], false);
    law.perform();
    near(law.evaluate(0.5), 0.5, 1e-12, "midpoint");
}

#[test]
fn test_interpolate_midpoint_general_segment() {
    // Between (2,4) and (4,8) at t=3 the value should be 6.
    let mut law =
        LawInterpolate::new(vec![(2.0, 4.0), (4.0, 8.0)], false);
    law.perform();
    near(law.evaluate(3.0), 6.0, 1e-12, "general midpoint");
}

#[test]
fn test_interpolate_extrapolate_below() {
    // Extend segment (1,2)-(3,6) below t=1.
    let mut law = LawInterpolate::new(vec![(1.0, 2.0), (3.0, 6.0)], false);
    law.perform();
    // slope = (6-2)/(3-1) = 2; value at t=0 should be 2 - 2*1 = 0
    near(law.evaluate(0.0), 0.0, 1e-12, "extrapolate below");
}

#[test]
fn test_interpolate_extrapolate_above() {
    // Extend segment (0,0)-(1,1) above t=1.
    let mut law = LawInterpolate::new(vec![(0.0, 0.0), (1.0, 1.0)], false);
    law.perform();
    near(law.evaluate(2.0), 2.0, 1e-12, "extrapolate above");
}

#[test]
fn test_interpolate_multiple_segments_correct_segment_chosen() {
    // Three segments; evaluation should pick the right one.
    let mut law = LawInterpolate::new(
        vec![(0.0, 0.0), (1.0, 10.0), (2.0, 0.0), (3.0, 10.0)],
        false,
    );
    law.perform();
    // In segment [1,2]: at t=1.5 value = 10 + (1.5-1)/(2-1) * (0-10) = 5
    near(law.evaluate(1.5), 5.0, 1e-12, "middle segment midpoint");
    // In segment [2,3]: at t=2.5 value = 0 + 0.5 * 10 = 5
    near(law.evaluate(2.5), 5.0, 1e-12, "last segment midpoint");
}

#[test]
fn test_interpolate_nb_points() {
    let law = LawInterpolate::new(vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)], false);
    assert_eq!(law.nb_points(), 3);
}

#[test]
fn test_interpolate_point_accessor() {
    let law = LawInterpolate::new(vec![(7.0, 3.0), (1.0, 0.5)], false);
    assert_eq!(law.point(0), (1.0, 0.5));
    assert_eq!(law.point(1), (7.0, 3.0));
}

#[test]
fn test_interpolate_load_tangents_does_not_change_evaluation() {
    // load() stores tangents but must not affect linear evaluation.
    let mut law = LawInterpolate::new(vec![(0.0, 0.0), (1.0, 1.0)], false);
    law.load(vec![999.0, -999.0]);
    law.perform();
    near(law.evaluate(0.5), 0.5, 1e-12, "with tangents loaded");
}

#[test]
fn test_interpolate_periodic_flag_stored() {
    // The periodic flag is accepted without panicking.
    let mut law = LawInterpolate::new(vec![(0.0, 0.0), (1.0, 1.0)], true);
    law.perform();
    assert!(law.is_done());
}

// ── LawS ─────────────────────────────────────────────────────────────────────

#[test]
fn test_laws_first_last_accessors() {
    let s = LawS::new(2.0, 5.0);
    assert_eq!(s.first(), 2.0);
    assert_eq!(s.last(), 5.0);
}

#[test]
fn test_laws_evaluate_at_first_is_zero() {
    let s = LawS::new(1.0, 4.0);
    near(s.evaluate(1.0), 0.0, 1e-12, "at first");
}

#[test]
fn test_laws_evaluate_at_last_is_one() {
    let s = LawS::new(1.0, 4.0);
    near(s.evaluate(4.0), 1.0, 1e-12, "at last");
}

#[test]
fn test_laws_evaluate_at_midpoint() {
    // Smooth-step at u=0.5: 3*(0.25) - 2*(0.125) = 0.75 - 0.25 = 0.5
    let s = LawS::new(0.0, 1.0);
    near(s.evaluate(0.5), 0.5, 1e-12, "midpoint symmetric");
}

#[test]
fn test_laws_evaluate_clamp_below() {
    let s = LawS::new(2.0, 4.0);
    near(s.evaluate(0.0), 0.0, 1e-12, "clamped below");
    near(s.evaluate(1.9), 0.0, 1e-12, "just below first");
}

#[test]
fn test_laws_evaluate_clamp_above() {
    let s = LawS::new(2.0, 4.0);
    near(s.evaluate(6.0), 1.0, 1e-12, "clamped above");
    near(s.evaluate(4.001), 1.0, 1e-12, "just above last");
}

#[test]
fn test_laws_evaluate_monotone() {
    // Values must be strictly increasing over [first, last].
    let s = LawS::new(0.0, 10.0);
    let steps = 100;
    let mut prev = -1.0f64;
    for i in 0..=steps {
        let t = i as f64 * 10.0 / steps as f64;
        let v = s.evaluate(t);
        assert!(v >= prev, "not monotone at t={t}: {v} < {prev}");
        prev = v;
    }
}

#[test]
fn test_laws_evaluate_quarter_point() {
    // u = 0.25: 3*(0.0625) - 2*(0.015625) = 0.1875 - 0.03125 = 0.15625
    let s = LawS::new(0.0, 4.0);
    near(s.evaluate(1.0), 0.15625, 1e-12, "quarter point");
}

#[test]
fn test_laws_evaluate_three_quarter_point() {
    // u = 0.75: 3*(0.5625) - 2*(0.421875) = 1.6875 - 0.84375 = 0.84375
    let s = LawS::new(0.0, 4.0);
    near(s.evaluate(3.0), 0.84375, 1e-12, "three-quarter point");
}

#[test]
fn test_laws_symmetry() {
    // f(first + d) + f(last - d) == 1 for any d.
    let s = LawS::new(0.0, 1.0);
    for i in 1..10 {
        let d = i as f64 * 0.1;
        let lo = s.evaluate(d);
        let hi = s.evaluate(1.0 - d);
        near(lo + hi, 1.0, 1e-12, &format!("symmetry at d={d}"));
    }
}
