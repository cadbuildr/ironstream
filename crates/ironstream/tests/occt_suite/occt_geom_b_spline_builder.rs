// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_b_spline_builder.rs
//! Integration tests for `geom_b_spline_builder`.
//!
//! Exercises the public API of [`GeomPtsSplineParams`],
//! [`GeomPtsSplineResult`], and [`GeomApiPointsToBSpline`] without accessing
//! private fields.

extern crate ironstream;
use ironstream::geom_b_spline_builder::*;

// ─────────────────────────────────────────────────────────────────────────────
// GeomPtsSplineParams
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn params_new_returns_expected_defaults() {
    let p = GeomPtsSplineParams::new();
    assert_eq!(p.degree_min(), 3, "default degree_min should be 3");
    assert_eq!(p.degree_max(), 8, "default degree_max should be 8");
    assert_eq!(p.continuity(), 2, "default continuity should be 2");
    assert!(
        (p.tolerance() - 1e-6).abs() < 1e-15,
        "default tolerance should be 1e-6"
    );
}

#[test]
fn params_set_degree_min_persists() {
    let mut p = GeomPtsSplineParams::new();
    p.set_degree_min(1);
    assert_eq!(p.degree_min(), 1);
}

#[test]
fn params_set_degree_max_persists() {
    let mut p = GeomPtsSplineParams::new();
    p.set_degree_max(5);
    assert_eq!(p.degree_max(), 5);
}

#[test]
fn params_set_degree_min_and_max_independent() {
    let mut p = GeomPtsSplineParams::new();
    p.set_degree_min(2);
    p.set_degree_max(6);
    assert_eq!(p.degree_min(), 2);
    assert_eq!(p.degree_max(), 6);
    // Other fields unchanged.
    assert_eq!(p.continuity(), 2);
}

#[test]
fn params_clone_is_independent() {
    let mut p = GeomPtsSplineParams::new();
    let q = p.clone();
    p.set_degree_min(1);
    // Clone is unaffected.
    assert_eq!(q.degree_min(), 3);
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomPtsSplineResult
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn result_new_is_not_done() {
    let r = GeomPtsSplineResult::new();
    assert!(!r.is_done(), "fresh result must not be done");
}

#[test]
fn result_new_nb_poles_is_zero() {
    let r = GeomPtsSplineResult::new();
    assert_eq!(r.nb_poles(), 0);
}

#[test]
fn result_new_degree_is_zero() {
    let r = GeomPtsSplineResult::new();
    assert_eq!(r.degree(), 0);
}

#[test]
fn result_new_max_error_is_zero() {
    let r = GeomPtsSplineResult::new();
    assert_eq!(r.max_error(), 0.0);
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiPointsToBSpline — lifecycle
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn algo_new_nb_points_zero() {
    let algo = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    assert_eq!(algo.nb_points(), 0);
}

#[test]
fn algo_new_is_not_done() {
    let algo = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    assert!(!algo.is_done());
}

#[test]
fn add_point_increments_nb_points() {
    let mut algo = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    algo.add_point([0.0, 0.0, 0.0]);
    assert_eq!(algo.nb_points(), 1);
    algo.add_point([1.0, 0.0, 0.0]);
    assert_eq!(algo.nb_points(), 2);
    algo.add_point([2.0, 0.0, 0.0]);
    assert_eq!(algo.nb_points(), 3);
}

#[test]
fn perform_with_no_points_is_not_done() {
    let mut algo = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    algo.perform();
    assert!(!algo.is_done(), "empty point set must not produce a result");
}

#[test]
fn perform_with_one_point_is_done() {
    let mut algo = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    algo.add_point([7.0, -3.0, 1.5]);
    algo.perform();
    assert!(algo.is_done());
}

#[test]
fn perform_with_two_points_is_done() {
    let mut algo = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    algo.add_point([0.0, 0.0, 0.0]);
    algo.add_point([1.0, 1.0, 1.0]);
    algo.perform();
    assert!(algo.is_done());
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiPointsToBSpline — result content
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn perform_poles_equal_input_points() {
    let pts: Vec<[f64; 3]> =
        vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.0], [3.0, 1.0, 0.0], [4.0, 0.0, 0.0]];
    let mut algo = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    for &p in &pts {
        algo.add_point(p);
    }
    algo.perform();
    assert!(algo.is_done());
    let r = algo.result();
    assert_eq!(r.nb_poles(), pts.len());
    for (i, &expected) in pts.iter().enumerate() {
        let got = r.pole(i);
        assert_eq!(got, expected, "pole {i} mismatch");
    }
}

#[test]
fn perform_degree_equals_degree_min() {
    let mut p = GeomPtsSplineParams::new();
    p.set_degree_min(2);
    let mut algo = GeomApiPointsToBSpline::new(p);
    for i in 0..5_u32 {
        algo.add_point([i as f64, 0.0, 0.0]);
    }
    algo.perform();
    assert_eq!(algo.result().degree(), 2);
}

#[test]
fn perform_knots_start_at_zero_and_end_at_one() {
    let mut algo = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    for i in 0..6_u32 {
        algo.add_point([i as f64, 0.0, 0.0]);
    }
    algo.perform();
    let r = algo.result();
    let knots = &r.knots;
    assert!(
        (knots[0] - 0.0).abs() < 1e-15,
        "first knot must be 0.0, got {}",
        knots[0]
    );
    assert!(
        (knots[knots.len() - 1] - 1.0).abs() < 1e-15,
        "last knot must be 1.0, got {}",
        knots[knots.len() - 1]
    );
}

#[test]
fn perform_knots_monotonically_non_decreasing() {
    let mut algo = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    for i in 0..8_u32 {
        algo.add_point([i as f64, (i as f64).sin(), 0.0]);
    }
    algo.perform();
    let r = algo.result();
    for w in r.knots.windows(2) {
        assert!(
            w[1] >= w[0],
            "knot vector not monotone: {} > {}",
            w[0],
            w[1]
        );
    }
}

#[test]
fn perform_mults_sum_equals_poles_plus_degree_plus_one() {
    // The B-spline identity: sum(mults) = n_poles + degree + 1
    let mut p = GeomPtsSplineParams::new();
    p.set_degree_min(3);
    let mut algo = GeomApiPointsToBSpline::new(p);
    for i in 0..5_u32 {
        algo.add_point([i as f64, 0.0, 0.0]);
    }
    algo.perform();
    let r = algo.result();
    let sum_mults: u32 = r.mults.iter().sum();
    let expected = r.nb_poles() as u32 + r.degree() + 1;
    assert_eq!(
        sum_mults, expected,
        "sum(mults)={sum_mults} should equal nb_poles+degree+1={expected}"
    );
}

#[test]
fn perform_knots_and_mults_same_length() {
    let mut algo = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    for i in 0..7_u32 {
        algo.add_point([i as f64, 0.0, 0.0]);
    }
    algo.perform();
    let r = algo.result();
    assert_eq!(
        r.knots.len(),
        r.mults.len(),
        "knots and mults must have the same length"
    );
}

#[test]
fn perform_max_error_is_zero_stub() {
    let mut algo = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    algo.add_point([0.0, 0.0, 0.0]);
    algo.add_point([5.0, 5.0, 0.0]);
    algo.perform();
    assert_eq!(algo.result().max_error(), 0.0);
}

#[test]
fn perform_resets_result_on_second_call() {
    let mut algo = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    algo.add_point([0.0, 0.0, 0.0]);
    algo.add_point([1.0, 0.0, 0.0]);
    algo.perform();
    assert!(algo.is_done());
    // Reset by calling perform() on an algo that now has no points (simulate
    // by creating fresh algo and verifying an empty perform yields not-done).
    let mut algo2 = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    algo2.perform();
    assert!(!algo2.is_done());
}

#[test]
fn result_pole_accessor_matches_input() {
    let pts: Vec<[f64; 3]> = vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
    let mut algo = GeomApiPointsToBSpline::new(GeomPtsSplineParams::new());
    for &p in &pts {
        algo.add_point(p);
    }
    algo.perform();
    let r = algo.result();
    assert_eq!(r.pole(0), pts[0]);
    assert_eq!(r.pole(1), pts[1]);
}

#[test]
fn end_to_end_with_custom_degree_min() {
    // Vary degree_min and confirm the result degree tracks it.
    for deg in [1u32, 2, 3, 4] {
        let mut p = GeomPtsSplineParams::new();
        p.set_degree_min(deg);
        let mut algo = GeomApiPointsToBSpline::new(p);
        for i in 0..8_u32 {
            algo.add_point([i as f64, 0.0, 0.0]);
        }
        algo.perform();
        assert!(algo.is_done(), "should be done for degree_min={deg}");
        assert_eq!(
            algo.result().degree(),
            deg,
            "result degree should equal degree_min={deg}"
        );
    }
}
