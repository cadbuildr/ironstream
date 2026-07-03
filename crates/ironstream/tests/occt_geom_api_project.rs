// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_api_project.rs
//! Integration tests for `geom_api_project` — GeomAPI projection stubs.
//!
//! Each test exercises only the public API.  Tolerances are loose because the
//! implementations are stubs — they validate interface correctness, not
//! numerical accuracy.

extern crate ironstream;
use ironstream::geom_api_project::*;

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiProjectResult
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn result_new_stores_point() {
    let r = GeomApiProjectResult::new([1.0, 2.0, 3.0], 0.1, 0.9, 4.5);
    assert_eq!(r.point(), [1.0, 2.0, 3.0]);
}

#[test]
fn result_parameters_roundtrip() {
    let r = GeomApiProjectResult::new([0.0; 3], 0.3, 0.7, 0.0);
    let (u, v) = r.parameters();
    assert!((u - 0.3).abs() < 1e-15);
    assert!((v - 0.7).abs() < 1e-15);
}

#[test]
fn result_distance_roundtrip() {
    let r = GeomApiProjectResult::new([0.0; 3], 0.0, 0.0, 42.0);
    assert!((r.distance() - 42.0).abs() < 1e-15);
}

#[test]
fn result_zero_distance() {
    let r = GeomApiProjectResult::new([5.0, 5.0, 5.0], 0.5, 0.5, 0.0);
    assert_eq!(r.distance(), 0.0);
}

#[test]
fn result_negative_coordinates_allowed() {
    let r = GeomApiProjectResult::new([-1.0, -2.0, -3.0], -0.5, -0.5, 10.0);
    assert_eq!(r.point(), [-1.0, -2.0, -3.0]);
    let (u, v) = r.parameters();
    assert!((u - (-0.5)).abs() < 1e-15);
    assert!((v - (-0.5)).abs() < 1e-15);
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiProjectPointOnSurf
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn surf_new_is_not_done() {
    let proj = GeomApiProjectPointOnSurf::new([0.0, 0.0, 0.0]);
    assert!(!proj.is_done(), "must not be done before perform()");
}

#[test]
fn surf_new_has_zero_points() {
    let proj = GeomApiProjectPointOnSurf::new([0.0, 0.0, 0.0]);
    assert_eq!(proj.nb_points(), 0);
}

#[test]
fn surf_perform_sets_done() {
    let mut proj = GeomApiProjectPointOnSurf::new([1.0, 2.0, 3.0]);
    proj.perform();
    assert!(proj.is_done());
}

#[test]
fn surf_perform_adds_one_result() {
    let mut proj = GeomApiProjectPointOnSurf::new([1.0, 2.0, 3.0]);
    proj.perform();
    assert_eq!(proj.nb_points(), 1);
}

#[test]
fn surf_perform_stub_distance_is_1() {
    let mut proj = GeomApiProjectPointOnSurf::new([0.0, 0.0, 0.0]);
    proj.perform();
    assert!((proj.lower_distance() - 1.0).abs() < 1e-15);
}

#[test]
fn surf_perform_stub_parameters_are_zero() {
    let mut proj = GeomApiProjectPointOnSurf::new([0.0, 0.0, 0.0]);
    proj.perform();
    assert_eq!(proj.parameter(0), (0.0, 0.0));
}

#[test]
fn surf_nearest_point_after_perform() {
    let pt = [3.0, 4.0, 5.0];
    let mut proj = GeomApiProjectPointOnSurf::new(pt);
    proj.perform();
    // The stub sets the foot point equal to the query point.
    assert_eq!(proj.nearest_point(), pt);
}

#[test]
fn surf_nearest_point_before_perform_returns_query() {
    let pt = [7.0, 8.0, 9.0];
    let proj = GeomApiProjectPointOnSurf::new(pt);
    assert_eq!(proj.nearest_point(), pt);
}

#[test]
fn surf_lower_distance_before_perform_is_max() {
    let proj = GeomApiProjectPointOnSurf::new([0.0, 0.0, 0.0]);
    assert_eq!(proj.lower_distance(), f64::MAX);
}

#[test]
fn surf_parameter_out_of_range_returns_zero() {
    let mut proj = GeomApiProjectPointOnSurf::new([0.0, 0.0, 0.0]);
    proj.perform();
    assert_eq!(proj.parameter(1), (0.0, 0.0));
    assert_eq!(proj.parameter(100), (0.0, 0.0));
}

#[test]
fn surf_perform_idempotent_increments_results() {
    // Calling perform twice accumulates results (stub appends each time).
    let mut proj = GeomApiProjectPointOnSurf::new([0.0, 0.0, 0.0]);
    proj.perform();
    proj.perform();
    assert!(proj.nb_points() >= 1);
    assert!(proj.is_done());
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiProjectPointOnCurve
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn curve_new_is_not_done() {
    let proj = GeomApiProjectPointOnCurve::new([0.0, 0.0, 0.0]);
    assert!(!proj.is_done(), "must not be done before perform()");
}

#[test]
fn curve_new_has_zero_points() {
    let proj = GeomApiProjectPointOnCurve::new([0.0, 0.0, 0.0]);
    assert_eq!(proj.nb_points(), 0);
}

#[test]
fn curve_perform_sets_done() {
    let mut proj = GeomApiProjectPointOnCurve::new([1.0, 0.0, 0.0]);
    proj.perform();
    assert!(proj.is_done());
}

#[test]
fn curve_perform_nb_points_is_one() {
    let mut proj = GeomApiProjectPointOnCurve::new([1.0, 0.0, 0.0]);
    proj.perform();
    assert_eq!(proj.nb_points(), 1);
}

#[test]
fn curve_perform_stub_parameter_is_half() {
    let mut proj = GeomApiProjectPointOnCurve::new([0.0, 0.0, 0.0]);
    proj.perform();
    assert!((proj.parameter() - 0.5).abs() < 1e-15);
}

#[test]
fn curve_perform_stub_distance_is_1() {
    let mut proj = GeomApiProjectPointOnCurve::new([0.0, 0.0, 0.0]);
    proj.perform();
    assert!((proj.lower_distance() - 1.0).abs() < 1e-15);
}

#[test]
fn curve_perform_does_not_change_query_point() {
    // Ensure new() accepts any point without panic.
    let pt = [-5.0, 10.0, 0.5];
    let mut proj = GeomApiProjectPointOnCurve::new(pt);
    proj.perform();
    assert!(proj.is_done());
}

#[test]
fn curve_lower_distance_before_perform_is_zero() {
    // Before perform, distance is the default (0.0).
    let proj = GeomApiProjectPointOnCurve::new([0.0, 0.0, 0.0]);
    assert_eq!(proj.lower_distance(), 0.0);
}

#[test]
fn curve_parameter_before_perform_is_zero() {
    let proj = GeomApiProjectPointOnCurve::new([1.0, 1.0, 1.0]);
    assert_eq!(proj.parameter(), 0.0);
}

#[test]
fn curve_perform_multiple_times_stays_done() {
    let mut proj = GeomApiProjectPointOnCurve::new([0.0, 0.0, 0.0]);
    proj.perform();
    proj.perform();
    assert!(proj.is_done());
    assert!((proj.parameter() - 0.5).abs() < 1e-15);
    assert!((proj.lower_distance() - 1.0).abs() < 1e-15);
}
