// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_extrema.rs
//! Integration tests for `geom_extrema` — GeomAPI_ExtremaCurveCurve and
//! GeomAPI_ExtremaCurveSurface stubs.
//!
//! All tests exercise the public API only; no access to private fields.

extern crate ironstream;
use ironstream::geom_extrema::*;

// ─────────────────────────────────────────────────────────────────────────────
// GeomExtremaResult
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn geom_extrema_result_new_and_accessors() {
    let r = GeomExtremaResult::new(
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        2.5,
        0.1,
        0.9,
    );
    assert_eq!(r.pt1(), [1.0, 2.0, 3.0]);
    assert_eq!(r.pt2(), [4.0, 5.0, 6.0]);
    assert!((r.distance() - 2.5).abs() < 1e-15);
    assert_eq!(r.parameters(), (0.1, 0.9));
}

#[test]
fn geom_extrema_result_zero_distance() {
    let r = GeomExtremaResult::new([0.0; 3], [0.0; 3], 0.0, 0.0, 0.0);
    assert_eq!(r.distance(), 0.0);
    assert_eq!(r.parameters(), (0.0, 0.0));
}

#[test]
fn geom_extrema_result_negative_params_allowed() {
    // Parameters can be negative (open curves extend beyond [0,1]).
    let r = GeomExtremaResult::new([0.0; 3], [0.0; 3], 1.0, -3.0, 7.0);
    assert_eq!(r.parameters(), (-3.0, 7.0));
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiExtremaCurveCurve
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn curve_curve_not_done_before_perform() {
    let cc = GeomApiExtremaCurveCurve::new();
    assert!(!cc.is_done());
    assert_eq!(cc.nb_extrema(), 0);
}

#[test]
fn curve_curve_perform_zero_extrema() {
    let mut cc = GeomApiExtremaCurveCurve::new();
    cc.perform(0);
    assert!(cc.is_done());
    assert_eq!(cc.nb_extrema(), 0);
}

#[test]
fn curve_curve_perform_one_extremum() {
    let mut cc = GeomApiExtremaCurveCurve::new();
    cc.perform(1);
    assert!(cc.is_done());
    assert_eq!(cc.nb_extrema(), 1);
    let r = cc.result(0);
    assert!((r.distance() - 1.0).abs() < 1e-15, "stub distance must be 1.0");
}

#[test]
fn curve_curve_perform_multiple_extrema() {
    let mut cc = GeomApiExtremaCurveCurve::new();
    cc.perform(5);
    assert!(cc.is_done());
    assert_eq!(cc.nb_extrema(), 5);
    for i in 0..5 {
        let r = cc.result(i);
        assert!((r.distance() - 1.0).abs() < 1e-15);
    }
}

#[test]
fn curve_curve_lower_distance_after_perform() {
    let mut cc = GeomApiExtremaCurveCurve::new();
    cc.perform(3);
    // All stub distances are 1.0, so lower_distance returns 1.0.
    assert!((cc.lower_distance() - 1.0).abs() < 1e-15);
}

#[test]
fn curve_curve_lower_distance_indices_returns_valid_pair() {
    let mut cc = GeomApiExtremaCurveCurve::new();
    cc.perform(4);
    let (i1, i2) = cc.lower_distance_indices();
    assert!(i1 < cc.nb_extrema());
    assert!(i2 < cc.nb_extrema());
}

#[test]
fn curve_curve_lower_distance_indices_empty() {
    let cc = GeomApiExtremaCurveCurve::new();
    // Must not panic when no results are present.
    let (i1, i2) = cc.lower_distance_indices();
    assert_eq!((i1, i2), (0, 0));
}

#[test]
fn curve_curve_perform_resets_previous_results() {
    let mut cc = GeomApiExtremaCurveCurve::new();
    cc.perform(3);
    assert_eq!(cc.nb_extrema(), 3);
    cc.perform(1);
    assert_eq!(cc.nb_extrema(), 1);
}

#[test]
fn curve_curve_default_same_as_new() {
    let cc: GeomApiExtremaCurveCurve = Default::default();
    assert!(!cc.is_done());
    assert_eq!(cc.nb_extrema(), 0);
}

#[test]
fn curve_curve_result_pt1_and_pt2_differ() {
    let mut cc = GeomApiExtremaCurveCurve::new();
    cc.perform(2);
    let r = cc.result(0);
    // Stub places pt1 at origin and pt2 offset by 1 in X.
    assert_ne!(r.pt1(), r.pt2());
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiExtremaCurveSurface
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn curve_surface_not_done_before_perform() {
    let cs = GeomApiExtremaCurveSurface::new();
    assert!(!cs.is_done());
    assert_eq!(cs.nb_extrema(), 0);
}

#[test]
fn curve_surface_perform_sets_done() {
    let mut cs = GeomApiExtremaCurveSurface::new();
    cs.perform();
    assert!(cs.is_done());
}

#[test]
fn curve_surface_nb_extrema_one_after_perform() {
    let mut cs = GeomApiExtremaCurveSurface::new();
    cs.perform();
    assert_eq!(cs.nb_extrema(), 1);
}

#[test]
fn curve_surface_lower_distance_is_one() {
    let mut cs = GeomApiExtremaCurveSurface::new();
    cs.perform();
    assert!(
        (cs.lower_distance() - 1.0).abs() < 1e-15,
        "stub lower_distance must be 1.0, got {}",
        cs.lower_distance()
    );
}

#[test]
fn curve_surface_point_on_curve_is_origin() {
    let mut cs = GeomApiExtremaCurveSurface::new();
    cs.perform();
    assert_eq!(cs.point_on_curve(), [0.0, 0.0, 0.0]);
}

#[test]
fn curve_surface_point_on_surface_offset() {
    let mut cs = GeomApiExtremaCurveSurface::new();
    cs.perform();
    // Stub places the surface point at [1, 0, 0].
    assert_eq!(cs.point_on_surface(), [1.0, 0.0, 0.0]);
}

#[test]
fn curve_surface_result_accessors_consistent() {
    let mut cs = GeomApiExtremaCurveSurface::new();
    cs.perform();
    let r = cs.result();
    assert_eq!(r.pt1(), cs.point_on_curve());
    assert_eq!(r.pt2(), cs.point_on_surface());
    assert!((r.distance() - cs.lower_distance()).abs() < 1e-15);
}

#[test]
fn curve_surface_default_same_as_new() {
    let cs: GeomApiExtremaCurveSurface = Default::default();
    assert!(!cs.is_done());
    assert_eq!(cs.nb_extrema(), 0);
}

#[test]
fn curve_surface_perform_idempotent() {
    let mut cs = GeomApiExtremaCurveSurface::new();
    cs.perform();
    cs.perform();
    assert!(cs.is_done());
    assert_eq!(cs.nb_extrema(), 1);
    assert!((cs.lower_distance() - 1.0).abs() < 1e-15);
}
