// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_approx_curve.rs
//! Integration tests for `ironstream::geom_approx_curve` —
//! `ApproxCurve3d`, `ApproxCurve3dResult`, and `ApproxCurve2dResult`,
//! mirroring the OpenCascade `Approx_Curve3d` /
//! `Approx_Curve2dOn3dSurface` API surface.

extern crate ironstream;
use ironstream::geom_approx_curve::*;

// ─── ApproxCurve3dResult ──────────────────────────────────────────────────────

// Test 1: new() yields a not-done result with zero poles.
#[test]
fn result3d_new_not_done() {
    let r = ApproxCurve3dResult::new();
    assert!(!r.is_done(), "fresh result must not be done");
    assert_eq!(r.nb_poles(), 0, "fresh result has no poles");
}

// Test 2: default() is equivalent to new().
#[test]
fn result3d_default_equivalent_to_new() {
    let r: ApproxCurve3dResult = Default::default();
    assert!(!r.is_done());
    assert_eq!(r.nb_poles(), 0);
    assert_eq!(r.degree(), 0);
}

// Test 3: degree() accessor round-trips the stored degree.
#[test]
fn result3d_degree_accessor() {
    let mut r = ApproxCurve3dResult::new();
    r.degree = 5;
    r.is_done = true;
    assert_eq!(r.degree(), 5);
}

// Test 4: max_error() and average_error() accessors.
#[test]
fn result3d_error_accessors() {
    let mut r = ApproxCurve3dResult::new();
    r.max_error = 1e-3;
    r.average_error = 5e-4;
    assert!((r.max_error() - 1e-3).abs() < 1e-15);
    assert!((r.average_error() - 5e-4).abs() < 1e-15);
}

// Test 5: pole(i) returns the stored pole at that index.
#[test]
fn result3d_pole_accessor() {
    let mut r = ApproxCurve3dResult::new();
    r.poles = vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
    r.is_done = true;
    assert_eq!(r.pole(0), [1.0, 2.0, 3.0]);
    assert_eq!(r.pole(1), [4.0, 5.0, 6.0]);
    assert_eq!(r.pole(2), [7.0, 8.0, 9.0]);
    assert_eq!(r.nb_poles(), 3);
}

// Test 6: nb_poles() matches vec length after mutation.
#[test]
fn result3d_nb_poles_matches_vec() {
    let mut r = ApproxCurve3dResult::new();
    for i in 0..7 {
        r.poles.push([i as f64, 0.0, 0.0]);
    }
    assert_eq!(r.nb_poles(), 7);
}

// ─── ApproxCurve3d ────────────────────────────────────────────────────────────

// Test 7: new() stores the constructor arguments.
#[test]
fn approx3d_new_stores_params() {
    let a = ApproxCurve3d::new(1e-4, 2, 7);
    assert!((a.tolerance - 1e-4).abs() < 1e-15, "tolerance stored");
    assert_eq!(a.degree_min, 2);
    assert_eq!(a.degree_max, 7);
    assert_eq!(a.continuity, 2, "continuity defaults to 2");
}

// Test 8: is_done() is false before perform().
#[test]
fn approx3d_not_done_before_perform() {
    let a = ApproxCurve3d::new(1e-3, 3, 8);
    assert!(!a.is_done(), "must not be done before perform");
}

// Test 9: perform() sets is_done to true.
#[test]
fn approx3d_is_done_after_perform() {
    let mut a = ApproxCurve3d::new(1e-4, 3, 8);
    a.perform(10);
    assert!(a.is_done(), "must be done after perform");
}

// Test 10: perform() produces exactly nb_pts poles.
#[test]
fn approx3d_perform_nb_poles_equals_nb_pts() {
    for nb in [2, 5, 10, 20] {
        let mut a = ApproxCurve3d::new(1e-4, 3, 8);
        a.perform(nb);
        assert_eq!(
            a.result().nb_poles(),
            nb,
            "nb_poles should equal nb_pts={nb}"
        );
    }
}

// Test 11: result degree equals degree_min.
#[test]
fn approx3d_perform_degree_is_degree_min() {
    let mut a = ApproxCurve3d::new(1e-3, 2, 9);
    a.perform(8);
    assert_eq!(a.result().degree(), 2, "degree must equal degree_min");
}

// Test 12: perform() reports max_error = tolerance * 0.01.
#[test]
fn approx3d_perform_max_error_is_fraction_of_tolerance() {
    let tol = 5e-3;
    let mut a = ApproxCurve3d::new(tol, 3, 8);
    a.perform(6);
    let expected = tol * 0.01;
    assert!(
        (a.result().max_error() - expected).abs() < 1e-15,
        "max_error={} expected={expected}",
        a.result().max_error()
    );
}

// Test 13: poles lie on the X axis (Y=0, Z=0).
#[test]
fn approx3d_poles_on_x_axis() {
    let mut a = ApproxCurve3d::new(1e-4, 3, 8);
    a.perform(8);
    for (i, p) in a.result().poles.iter().enumerate() {
        assert_eq!(p[1], 0.0, "pole[{i}].y must be 0");
        assert_eq!(p[2], 0.0, "pole[{i}].z must be 0");
    }
}

// Test 14: first pole has t=0, last pole has t=1.
#[test]
fn approx3d_first_and_last_pole_t_values() {
    let nb = 11;
    let mut a = ApproxCurve3d::new(1e-4, 3, 8);
    a.perform(nb);
    let poles = &a.result().poles;
    assert!(
        (poles[0][0] - 0.0).abs() < 1e-15,
        "first pole x must be 0"
    );
    assert!(
        (poles[nb - 1][0] - 1.0).abs() < 1e-15,
        "last pole x must be 1"
    );
}

// Test 15: knots vector is non-empty after perform().
#[test]
fn approx3d_knots_non_empty_after_perform() {
    let mut a = ApproxCurve3d::new(1e-4, 3, 8);
    a.perform(5);
    assert!(!a.result().knots.is_empty(), "knots must be non-empty");
}

// Test 16: mults vector length matches knots vector length.
#[test]
fn approx3d_mults_length_matches_knots() {
    let mut a = ApproxCurve3d::new(1e-4, 3, 8);
    a.perform(7);
    assert_eq!(
        a.result().mults.len(),
        a.result().knots.len(),
        "mults and knots must have the same length"
    );
}

// Test 17: knots start at 0.0 and end at 1.0.
#[test]
fn approx3d_knots_range() {
    let mut a = ApproxCurve3d::new(1e-4, 3, 8);
    a.perform(6);
    let k = &a.result().knots;
    assert!((k[0] - 0.0).abs() < 1e-15, "first knot must be 0");
    assert!((k[k.len() - 1] - 1.0).abs() < 1e-15, "last knot must be 1");
}

// Test 18: perform() with nb_pts=2 (minimum) still succeeds.
#[test]
fn approx3d_perform_minimum_nb_pts() {
    let mut a = ApproxCurve3d::new(1e-4, 1, 3);
    a.perform(2);
    assert!(a.is_done());
    assert_eq!(a.result().nb_poles(), 2);
}

// Test 19: result() reference is consistent with is_done().
#[test]
fn approx3d_result_consistent_with_is_done() {
    let mut a = ApproxCurve3d::new(1e-4, 3, 8);
    a.perform(5);
    assert_eq!(a.is_done(), a.result().is_done());
}

// ─── ApproxCurve2dResult ─────────────────────────────────────────────────────

// Test 20: new() yields a not-done result with zero poles.
#[test]
fn result2d_new_not_done() {
    let r = ApproxCurve2dResult::new();
    assert!(!r.is_done(), "fresh 2-D result must not be done");
    assert_eq!(r.nb_poles(), 0);
}

// Test 21: default() is equivalent to new().
#[test]
fn result2d_default_equivalent_to_new() {
    let r: ApproxCurve2dResult = Default::default();
    assert!(!r.is_done());
    assert_eq!(r.nb_poles(), 0);
    assert_eq!(r.degree(), 0);
}

// Test 22: degree() accessor round-trips.
#[test]
fn result2d_degree_accessor() {
    let mut r = ApproxCurve2dResult::new();
    r.degree = 3;
    r.is_done = true;
    assert_eq!(r.degree(), 3);
}

// Test 23: max_error() accessor.
#[test]
fn result2d_max_error_accessor() {
    let mut r = ApproxCurve2dResult::new();
    r.max_error = 7.5e-5;
    assert!((r.max_error() - 7.5e-5).abs() < 1e-20);
}

// Test 24: pole(i) returns the 2-D pole at index i.
#[test]
fn result2d_pole_accessor() {
    let mut r = ApproxCurve2dResult::new();
    r.poles = vec![[0.0, 1.0], [0.5, 0.5], [1.0, 0.0]];
    r.degree = 2;
    r.is_done = true;
    assert_eq!(r.pole(0), [0.0, 1.0]);
    assert_eq!(r.pole(1), [0.5, 0.5]);
    assert_eq!(r.pole(2), [1.0, 0.0]);
}

// Test 25: nb_poles() matches the underlying vec length.
#[test]
fn result2d_nb_poles_matches_vec() {
    let mut r = ApproxCurve2dResult::new();
    r.poles = vec![[0.0, 0.0]; 9];
    assert_eq!(r.nb_poles(), 9);
}

// Test 26: is_done() can be set to true by mutating the field.
#[test]
fn result2d_is_done_set_to_true() {
    let mut r = ApproxCurve2dResult::new();
    r.is_done = true;
    assert!(r.is_done());
}

// Test 27: knots field stores arbitrary values.
#[test]
fn result2d_knots_field_accessible() {
    let mut r = ApproxCurve2dResult::new();
    r.knots = vec![0.0, 0.25, 0.75, 1.0];
    assert_eq!(r.knots.len(), 4);
    assert!((r.knots[0] - 0.0).abs() < 1e-15);
    assert!((r.knots[3] - 1.0).abs() < 1e-15);
}

// Test 28: poles are mutable through the public field.
#[test]
fn result2d_poles_are_mutable() {
    let mut r = ApproxCurve2dResult::new();
    r.poles.push([3.0, 4.0]);
    assert_eq!(r.nb_poles(), 1);
    assert_eq!(r.pole(0), [3.0, 4.0]);
}
