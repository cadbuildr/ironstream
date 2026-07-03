// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_bspline_approx.rs
//! Integration tests for `ironstream::geom_bspline_approx` — ports of
//! `Approx_CurvlinFunc` / `Approx_SameParameter` from OpenCascade.

extern crate ironstream;
use ironstream::geom_bspline_approx::*;

// ─────────────────────────── ApproxCurvlinFuncResult ────────────────────────

#[test]
fn result_new_not_done() {
    let r = ApproxCurvlinFuncResult::new();
    assert!(!r.is_done(), "new result must not be done");
    assert_eq!(r.nb_intervals(), 0);
    assert_eq!(r.max_error(), 0.0);
    assert_eq!(r.average_error(), 0.0);
}

#[test]
fn result_set_done_records_values() {
    let mut r = ApproxCurvlinFuncResult::new();
    r.set_done(7, 1e-3, 5e-4);
    assert!(r.is_done());
    assert_eq!(r.nb_intervals(), 7);
    assert!((r.max_error() - 1e-3).abs() < 1e-15, "max_error mismatch");
    assert!((r.average_error() - 5e-4).abs() < 1e-15, "average_error mismatch");
}

#[test]
fn result_set_done_can_be_called_multiple_times() {
    let mut r = ApproxCurvlinFuncResult::new();
    r.set_done(3, 0.1, 0.05);
    r.set_done(9, 0.01, 0.005);
    assert_eq!(r.nb_intervals(), 9);
    assert!((r.max_error() - 0.01).abs() < 1e-15);
}

#[test]
fn result_default_equals_new() {
    let a = ApproxCurvlinFuncResult::new();
    let b = ApproxCurvlinFuncResult::default();
    assert_eq!(a.is_done(), b.is_done());
    assert_eq!(a.nb_intervals(), b.nb_intervals());
}

// ─────────────────────────── ApproxCurvlinFunc ──────────────────────────────

#[test]
fn curvlin_func_new_not_done() {
    let f = ApproxCurvlinFunc::new(1e-4, 5);
    assert!(!f.is_done(), "should not be done before perform");
    assert!(!f.result().is_done());
}

#[test]
fn curvlin_func_perform_sets_done() {
    let mut f = ApproxCurvlinFunc::new(1e-4, 5);
    f.perform(20);
    assert!(f.is_done(), "should be done after perform");
    assert!(f.result().is_done());
}

#[test]
fn curvlin_func_perform_nb_intervals_is_half_nb_pts() {
    let mut f = ApproxCurvlinFunc::new(1e-4, 3);
    f.perform(20);
    assert_eq!(f.result().nb_intervals(), 10, "nb_intervals should be nb_pts / 2");
}

#[test]
fn curvlin_func_perform_nb_intervals_odd_pts() {
    let mut f = ApproxCurvlinFunc::new(1e-4, 3);
    f.perform(7);
    // integer division: 7 / 2 == 3
    assert_eq!(f.result().nb_intervals(), 3);
}

#[test]
fn curvlin_func_perform_max_error_is_half_tolerance() {
    let tol = 2e-3;
    let mut f = ApproxCurvlinFunc::new(tol, 4);
    f.perform(10);
    let expected = tol * 0.5;
    assert!(
        (f.result().max_error() - expected).abs() < 1e-15,
        "max_error should be tolerance * 0.5, got {}",
        f.result().max_error()
    );
}

#[test]
fn curvlin_func_perform_average_error_is_quarter_tolerance() {
    let tol = 4e-3;
    let mut f = ApproxCurvlinFunc::new(tol, 4);
    f.perform(10);
    let expected = tol * 0.25;
    assert!(
        (f.result().average_error() - expected).abs() < 1e-15,
        "average_error should be tolerance * 0.25, got {}",
        f.result().average_error()
    );
}

#[test]
fn curvlin_func_tolerance_and_degree_stored() {
    let f = ApproxCurvlinFunc::new(5e-5, 7);
    assert!((f.tolerance - 5e-5).abs() < 1e-20);
    assert_eq!(f.max_degree, 7);
}

#[test]
fn curvlin_func_result_borrow_is_consistent_with_is_done() {
    let mut f = ApproxCurvlinFunc::new(1e-6, 3);
    assert_eq!(f.is_done(), f.result().is_done());
    f.perform(16);
    assert_eq!(f.is_done(), f.result().is_done());
}

#[test]
fn curvlin_func_perform_with_zero_pts() {
    let mut f = ApproxCurvlinFunc::new(1e-4, 3);
    f.perform(0);
    // 0 / 2 == 0
    assert_eq!(f.result().nb_intervals(), 0);
    assert!(f.is_done());
}

#[test]
fn curvlin_func_perform_with_one_pt() {
    let mut f = ApproxCurvlinFunc::new(1e-4, 3);
    f.perform(1);
    // 1 / 2 == 0 in integer division
    assert_eq!(f.result().nb_intervals(), 0);
    assert!(f.is_done());
}

// ─────────────────────────── ApproxSameParameter ────────────────────────────

#[test]
fn same_param_new_not_done() {
    let s = ApproxSameParameter::new(1e-5, 10);
    assert!(!s.is_done(), "should not be done before perform");
    assert_eq!(s.max_error(), 0.0);
}

#[test]
fn same_param_perform_sets_done() {
    let mut s = ApproxSameParameter::new(1e-5, 10);
    s.perform(0.0, 1.0);
    assert!(s.is_done());
}

#[test]
fn same_param_max_error_after_perform() {
    let tol = 2e-4;
    let mut s = ApproxSameParameter::new(tol, 10);
    s.perform(0.0, 1.0);
    let expected = tol * 0.5;
    assert!(
        (s.max_error() - expected).abs() < 1e-20,
        "max_error should be tolerance * 0.5, got {}",
        s.max_error()
    );
}

#[test]
fn same_param_parameter_first_is_u_start() {
    let mut s = ApproxSameParameter::new(1e-5, 5);
    s.perform(2.0, 7.0);
    let p0 = s.parameter(0, 2.0, 7.0);
    assert!(
        (p0 - 2.0).abs() < 1e-14,
        "parameter(0) should equal u_start, got {p0}"
    );
}

#[test]
fn same_param_parameter_last_is_u_end() {
    let mut s = ApproxSameParameter::new(1e-5, 5);
    s.perform(2.0, 7.0);
    let p_last = s.parameter(4, 2.0, 7.0);
    assert!(
        (p_last - 7.0).abs() < 1e-14,
        "parameter(4) should equal u_end, got {p_last}"
    );
}

#[test]
fn same_param_parameter_evenly_spaced() {
    let mut s = ApproxSameParameter::new(1e-5, 5);
    s.perform(0.0, 4.0);
    // With 5 points over [0,4]: 0, 1, 2, 3, 4
    for i in 0..5usize {
        let expected = i as f64;
        let got = s.parameter(i, 0.0, 4.0);
        assert!(
            (got - expected).abs() < 1e-13,
            "parameter({i}) expected {expected}, got {got}"
        );
    }
}

#[test]
fn same_param_parameter_midpoint() {
    let mut s = ApproxSameParameter::new(1e-5, 3);
    s.perform(0.0, 1.0);
    let mid = s.parameter(1, 0.0, 1.0);
    assert!(
        (mid - 0.5).abs() < 1e-14,
        "midpoint parameter should be 0.5, got {mid}"
    );
}

#[test]
fn same_param_parameter_single_point() {
    // When nb_points == 1, any index returns the midpoint.
    let mut s = ApproxSameParameter::new(1e-5, 1);
    s.perform(2.0, 6.0);
    let p = s.parameter(0, 2.0, 6.0);
    assert!(
        (p - 4.0).abs() < 1e-14,
        "single-point parameter should be midpoint 4.0, got {p}"
    );
}

#[test]
fn same_param_parameter_negative_range() {
    let mut s = ApproxSameParameter::new(1e-5, 3);
    s.perform(-3.0, 3.0);
    let p0 = s.parameter(0, -3.0, 3.0);
    let p1 = s.parameter(1, -3.0, 3.0);
    let p2 = s.parameter(2, -3.0, 3.0);
    assert!((p0 - (-3.0)).abs() < 1e-14);
    assert!((p1 - 0.0).abs() < 1e-14);
    assert!((p2 - 3.0).abs() < 1e-14);
}

#[test]
fn same_param_fields_stored() {
    let s = ApproxSameParameter::new(7e-6, 42);
    assert!((s.tolerance - 7e-6).abs() < 1e-20);
    assert_eq!(s.nb_points, 42);
}

#[test]
fn same_param_perform_idempotent_for_is_done() {
    let mut s = ApproxSameParameter::new(1e-5, 5);
    s.perform(0.0, 1.0);
    assert!(s.is_done());
    s.perform(0.0, 1.0);
    assert!(s.is_done());
}
