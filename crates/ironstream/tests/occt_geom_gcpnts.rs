// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_gcpnts.rs
//! Integration tests for `geom_gcpnts` — GCPnts point distribution utilities.
//!
//! Tests `GcpntsAbscissaPoint`, `GcpntsUniformAbscissa`, and
//! `GcpntsUniformDeflection` through the public API.

extern crate ironstream;
use ironstream::geom_gcpnts::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn near(a: f64, b: f64, tol: f64, msg: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{msg}: expected {a} ≈ {b} (tol {tol})"
    );
}

// ===========================================================================
// GcpntsAbscissaPoint
// ===========================================================================

/// `GCPnts_AbscissaPoint` starts as not-done.
#[test]
fn abscissa_point_new_is_not_done() {
    let ap = GcpntsAbscissaPoint::new();
    assert!(!ap.is_done());
}

/// Default construction mirrors `new()`.
#[test]
fn abscissa_point_default_is_not_done() {
    let ap = GcpntsAbscissaPoint::default();
    assert!(!ap.is_done());
}

/// Arc-length at the midpoint gives the midpoint parameter.
#[test]
fn abscissa_point_midpoint_parameter() {
    let mut ap = GcpntsAbscissaPoint::new();
    ap.perform(5.0, 10.0, 0.0, 1.0, 1e-10);
    assert!(ap.is_done());
    near(ap.parameter(), 0.5, 1e-12, "midpoint");
}

/// Arc-length 0 should give `u_start`.
#[test]
fn abscissa_point_zero_length_gives_u_start() {
    let mut ap = GcpntsAbscissaPoint::new();
    ap.perform(0.0, 10.0, 3.0, 7.0, 1e-10);
    assert!(ap.is_done());
    near(ap.parameter(), 3.0, 1e-12, "u_start");
}

/// Arc-length equal to `curve_length` gives `u_end`.
#[test]
fn abscissa_point_full_length_gives_u_end() {
    let mut ap = GcpntsAbscissaPoint::new();
    ap.perform(10.0, 10.0, 3.0, 7.0, 1e-10);
    assert!(ap.is_done());
    near(ap.parameter(), 7.0, 1e-12, "u_end");
}

/// Quarter arc-length gives quarter parameter.
#[test]
fn abscissa_point_quarter_parameter() {
    let mut ap = GcpntsAbscissaPoint::new();
    ap.perform(2.5, 10.0, 0.0, 1.0, 1e-10);
    assert!(ap.is_done());
    near(ap.parameter(), 0.25, 1e-12, "quarter");
}

/// Three-quarter arc-length gives three-quarter parameter.
#[test]
fn abscissa_point_three_quarter_parameter() {
    let mut ap = GcpntsAbscissaPoint::new();
    ap.perform(7.5, 10.0, 0.0, 1.0, 1e-10);
    assert!(ap.is_done());
    near(ap.parameter(), 0.75, 1e-12, "three-quarter");
}

/// Non-trivial parameter range: `u_start = 2, u_end = 5`.
#[test]
fn abscissa_point_offset_parameter_range() {
    let mut ap = GcpntsAbscissaPoint::new();
    // Total arc length 9, want at 3: t = 3/9 = 1/3, param = 2 + 1/3*3 = 3.
    ap.perform(3.0, 9.0, 2.0, 5.0, 1e-10);
    assert!(ap.is_done());
    near(ap.parameter(), 3.0, 1e-12, "offset range midpoint");
}

/// `IsDone` is false when `curve_length` is zero (within tolerance).
#[test]
fn abscissa_point_zero_curve_length_not_done() {
    let mut ap = GcpntsAbscissaPoint::new();
    ap.perform(1.0, 0.0, 0.0, 1.0, 1e-10);
    assert!(!ap.is_done());
}

/// `IsDone` is false when requested length exceeds `curve_length`.
#[test]
fn abscissa_point_length_exceeds_curve_not_done() {
    let mut ap = GcpntsAbscissaPoint::new();
    ap.perform(11.0, 10.0, 0.0, 1.0, 1e-10);
    assert!(!ap.is_done());
}

/// `IsDone` is false when requested length is negative beyond tolerance.
#[test]
fn abscissa_point_negative_length_not_done() {
    let mut ap = GcpntsAbscissaPoint::new();
    ap.perform(-1.0, 10.0, 0.0, 1.0, 1e-10);
    assert!(!ap.is_done());
}

/// Perform can be called multiple times; state is overwritten each time.
#[test]
fn abscissa_point_repeated_perform() {
    let mut ap = GcpntsAbscissaPoint::new();
    ap.perform(5.0, 10.0, 0.0, 1.0, 1e-10);
    assert!(ap.is_done());
    near(ap.parameter(), 0.5, 1e-12, "first perform");

    // Second perform with invalid input resets is_done.
    ap.perform(20.0, 10.0, 0.0, 1.0, 1e-10);
    assert!(!ap.is_done());
}

// ===========================================================================
// GcpntsUniformAbscissa
// ===========================================================================

/// `GCPnts_UniformAbscissa` starts as not-done.
#[test]
fn uniform_abscissa_new_is_not_done() {
    let ua = GcpntsUniformAbscissa::new();
    assert!(!ua.is_done());
    assert_eq!(ua.nb_points(), 0);
}

/// Two points: endpoints only.
#[test]
fn uniform_abscissa_two_points_are_endpoints() {
    let mut ua = GcpntsUniformAbscissa::new();
    ua.perform(2, 0.0, 1.0);
    assert!(ua.is_done());
    assert_eq!(ua.nb_points(), 2);
    near(ua.parameter(0), 0.0, 1e-12, "first");
    near(ua.parameter(1), 1.0, 1e-12, "last");
}

/// Five points on [0, 4] are equally spaced by 1.
#[test]
fn uniform_abscissa_five_points_unit_spacing() {
    let mut ua = GcpntsUniformAbscissa::new();
    ua.perform(5, 0.0, 4.0);
    assert!(ua.is_done());
    assert_eq!(ua.nb_points(), 5);
    for i in 0u32..5 {
        near(ua.parameter(i as usize), i as f64, 1e-12, &format!("param[{i}]"));
    }
}

/// A single point is placed at `u_start`.
#[test]
fn uniform_abscissa_single_point_at_u_start() {
    let mut ua = GcpntsUniformAbscissa::new();
    ua.perform(1, 3.0, 9.0);
    assert!(ua.is_done());
    assert_eq!(ua.nb_points(), 1);
    near(ua.parameter(0), 3.0, 1e-12, "single point");
}

/// Zero requested points results in not-done.
#[test]
fn uniform_abscissa_zero_points_not_done() {
    let mut ua = GcpntsUniformAbscissa::new();
    ua.perform(0, 0.0, 1.0);
    assert!(!ua.is_done());
    assert_eq!(ua.nb_points(), 0);
}

/// First and last parameters always match `u_start` and `u_end`.
#[test]
fn uniform_abscissa_endpoints_exact() {
    let mut ua = GcpntsUniformAbscissa::new();
    ua.perform(10, 2.5, 7.5);
    assert!(ua.is_done());
    near(ua.parameter(0), 2.5, 1e-12, "u_start");
    near(ua.parameter(9), 7.5, 1e-12, "u_end");
}

/// Step between consecutive parameters is constant.
#[test]
fn uniform_abscissa_spacing_is_constant() {
    let mut ua = GcpntsUniformAbscissa::new();
    ua.perform(6, 0.0, 5.0);
    assert!(ua.is_done());
    let expected_step = 1.0; // 5 / (6-1) = 1
    for i in 0..5 {
        let step = ua.parameter(i + 1) - ua.parameter(i);
        near(step, expected_step, 1e-12, &format!("step {i}"));
    }
}

/// Non-unit range: `[1, 3]` with 3 points should give [1, 2, 3].
#[test]
fn uniform_abscissa_non_unit_range() {
    let mut ua = GcpntsUniformAbscissa::new();
    ua.perform(3, 1.0, 3.0);
    assert!(ua.is_done());
    near(ua.parameter(0), 1.0, 1e-12, "p0");
    near(ua.parameter(1), 2.0, 1e-12, "p1");
    near(ua.parameter(2), 3.0, 1e-12, "p2");
}

/// Parameters are monotonically increasing for `u_start < u_end`.
#[test]
fn uniform_abscissa_parameters_monotone() {
    let mut ua = GcpntsUniformAbscissa::new();
    ua.perform(8, 0.0, 7.0);
    assert!(ua.is_done());
    for i in 0..7 {
        assert!(
            ua.parameter(i + 1) > ua.parameter(i),
            "param[{}] < param[{}]",
            i + 1,
            i
        );
    }
}

/// Repeated `perform` replaces the previous result.
#[test]
fn uniform_abscissa_repeated_perform_resets() {
    let mut ua = GcpntsUniformAbscissa::new();
    ua.perform(3, 0.0, 2.0);
    assert_eq!(ua.nb_points(), 3);
    ua.perform(5, 0.0, 4.0);
    assert_eq!(ua.nb_points(), 5);
    near(ua.parameter(4), 4.0, 1e-12, "last after reset");
}

// ===========================================================================
// GcpntsUniformDeflection
// ===========================================================================

/// `GCPnts_UniformDeflection` starts as not-done.
#[test]
fn uniform_deflection_new_is_not_done() {
    let ud = GcpntsUniformDeflection::new();
    assert!(!ud.is_done());
    assert_eq!(ud.nb_points(), 0);
}

/// Three points on [0, 1]: [0, 0.5, 1].
#[test]
fn uniform_deflection_three_points() {
    let mut ud = GcpntsUniformDeflection::new();
    ud.perform(0.1, 0.0, 1.0, 3);
    assert!(ud.is_done());
    assert_eq!(ud.nb_points(), 3);
    near(ud.parameter(0), 0.0, 1e-12, "p0");
    near(ud.parameter(1), 0.5, 1e-12, "p1");
    near(ud.parameter(2), 1.0, 1e-12, "p2");
}

/// Single point is placed at `u_start`.
#[test]
fn uniform_deflection_single_point_at_u_start() {
    let mut ud = GcpntsUniformDeflection::new();
    ud.perform(0.01, 5.0, 10.0, 1);
    assert!(ud.is_done());
    assert_eq!(ud.nb_points(), 1);
    near(ud.parameter(0), 5.0, 1e-12, "single point");
}

/// Zero approximation points results in not-done.
#[test]
fn uniform_deflection_zero_approx_not_done() {
    let mut ud = GcpntsUniformDeflection::new();
    ud.perform(0.1, 0.0, 1.0, 0);
    assert!(!ud.is_done());
}

/// First and last parameters match `u_start` and `u_end`.
#[test]
fn uniform_deflection_endpoints_exact() {
    let mut ud = GcpntsUniformDeflection::new();
    ud.perform(0.05, 1.0, 5.0, 7);
    assert!(ud.is_done());
    assert_eq!(ud.nb_points(), 7);
    near(ud.parameter(0), 1.0, 1e-12, "u_start");
    near(ud.parameter(6), 5.0, 1e-12, "u_end");
}

/// Spacing between consecutive parameters is constant.
#[test]
fn uniform_deflection_spacing_constant() {
    let mut ud = GcpntsUniformDeflection::new();
    ud.perform(0.2, 0.0, 6.0, 4);
    assert!(ud.is_done());
    let expected_step = 2.0; // 6 / (4-1) = 2
    for i in 0..3 {
        let step = ud.parameter(i + 1) - ud.parameter(i);
        near(step, expected_step, 1e-12, &format!("step {i}"));
    }
}

/// The deflection argument does not affect the result (stub).
#[test]
fn uniform_deflection_result_independent_of_deflection() {
    let mut ud1 = GcpntsUniformDeflection::new();
    ud1.perform(0.001, 0.0, 1.0, 5);
    let mut ud2 = GcpntsUniformDeflection::new();
    ud2.perform(999.0, 0.0, 1.0, 5);
    assert!(ud1.is_done() && ud2.is_done());
    for i in 0..5 {
        near(
            ud1.parameter(i),
            ud2.parameter(i),
            1e-12,
            &format!("param[{i}] deflection-independent"),
        );
    }
}

/// Parameters are monotonically increasing.
#[test]
fn uniform_deflection_parameters_monotone() {
    let mut ud = GcpntsUniformDeflection::new();
    ud.perform(0.1, 0.0, 10.0, 6);
    assert!(ud.is_done());
    for i in 0..5 {
        assert!(
            ud.parameter(i + 1) > ud.parameter(i),
            "param[{}] not > param[{}]",
            i + 1,
            i
        );
    }
}

/// Repeated `perform` replaces the previous result.
#[test]
fn uniform_deflection_repeated_perform_resets() {
    let mut ud = GcpntsUniformDeflection::new();
    ud.perform(0.1, 0.0, 1.0, 3);
    assert_eq!(ud.nb_points(), 3);
    ud.perform(0.1, 0.0, 1.0, 6);
    assert_eq!(ud.nb_points(), 6);
}

/// Two-point case returns `u_start` and `u_end`.
#[test]
fn uniform_deflection_two_points_are_endpoints() {
    let mut ud = GcpntsUniformDeflection::new();
    ud.perform(0.5, 2.0, 8.0, 2);
    assert!(ud.is_done());
    assert_eq!(ud.nb_points(), 2);
    near(ud.parameter(0), 2.0, 1e-12, "first");
    near(ud.parameter(1), 8.0, 1e-12, "last");
}
