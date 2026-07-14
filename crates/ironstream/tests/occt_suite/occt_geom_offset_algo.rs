// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_offset_algo.rs
//! Ported OpenCascade unit tests — offset-curve algorithm types.
//!
//! Covers [`OffsetAlgoStatus`], [`GeomOffsetCurveParams`],
//! [`GeomOffsetCurveResult`], [`Geom2dOffsetCurveParams`], and
//! [`Geom2dOffsetResult`].

extern crate ironstream;
use ironstream::geom_offset_algo::*;

// ─────────────────────────────────────────────────────────────────────────────
// OffsetAlgoStatus
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn offset_algo_status_variants_are_distinct() {
    assert_ne!(OffsetAlgoStatus::NoError, OffsetAlgoStatus::NotContinuous);
    assert_ne!(OffsetAlgoStatus::CannotTrim, OffsetAlgoStatus::CannotCloseOffset);
    assert_ne!(OffsetAlgoStatus::NullOffsetCurve, OffsetAlgoStatus::NoError);
}

#[test]
fn offset_algo_status_eq() {
    assert_eq!(OffsetAlgoStatus::NoError, OffsetAlgoStatus::NoError);
    assert_eq!(OffsetAlgoStatus::NotContinuous, OffsetAlgoStatus::NotContinuous);
    assert_eq!(OffsetAlgoStatus::CannotTrim, OffsetAlgoStatus::CannotTrim);
    assert_eq!(OffsetAlgoStatus::CannotCloseOffset, OffsetAlgoStatus::CannotCloseOffset);
    assert_eq!(OffsetAlgoStatus::NullOffsetCurve, OffsetAlgoStatus::NullOffsetCurve);
}

#[test]
fn offset_algo_status_copy() {
    let s = OffsetAlgoStatus::CannotTrim;
    let s2 = s; // Copy
    assert_eq!(s, s2);
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomOffsetCurveParams
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn geom_offset_curve_params_defaults() {
    let p = GeomOffsetCurveParams::new(5.0, [0.0, 0.0, 1.0]);
    assert_eq!(p.offset(), 5.0);
    assert_eq!(p.direction(), [0.0, 0.0, 1.0]);
    assert_eq!(p.continuity(), 1);
    assert!((p.tolerance() - 1e-7).abs() < 1e-15);
}

#[test]
fn geom_offset_curve_params_negative_offset() {
    let p = GeomOffsetCurveParams::new(-3.5, [1.0, 0.0, 0.0]);
    assert_eq!(p.offset(), -3.5);
}

#[test]
fn geom_offset_curve_params_set_continuity() {
    let mut p = GeomOffsetCurveParams::new(1.0, [0.0, 1.0, 0.0]);
    assert_eq!(p.continuity(), 1);
    p.set_continuity(2);
    assert_eq!(p.continuity(), 2);
    p.set_continuity(0);
    assert_eq!(p.continuity(), 0);
}

#[test]
fn geom_offset_curve_params_direction_roundtrip() {
    let dir = [1.0 / 3f64.sqrt(), 1.0 / 3f64.sqrt(), 1.0 / 3f64.sqrt()];
    let p = GeomOffsetCurveParams::new(2.0, dir);
    let d = p.direction();
    for i in 0..3 {
        assert!((d[i] - dir[i]).abs() < 1e-15);
    }
}

#[test]
fn geom_offset_curve_params_clone() {
    let p = GeomOffsetCurveParams::new(7.0, [0.0, 0.0, 1.0]);
    let mut q = p.clone();
    q.set_continuity(3);
    assert_eq!(p.continuity(), 1, "original must be unaffected");
    assert_eq!(q.continuity(), 3);
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomOffsetCurveResult
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn geom_offset_curve_result_initial_state() {
    let r = GeomOffsetCurveResult::new();
    assert!(!r.is_done());
    assert_eq!(r.nb_intervals(), 0);
    assert_eq!(r.status(), OffsetAlgoStatus::NoError);
}

#[test]
fn geom_offset_curve_result_default_equals_new() {
    let r1 = GeomOffsetCurveResult::new();
    let r2 = GeomOffsetCurveResult::default();
    assert_eq!(r1.is_done(), r2.is_done());
    assert_eq!(r1.nb_intervals(), r2.nb_intervals());
    assert_eq!(r1.status(), r2.status());
}

#[test]
fn geom_offset_curve_result_set_done() {
    let mut r = GeomOffsetCurveResult::new();
    r.set_done(4);
    assert!(r.is_done());
    assert_eq!(r.nb_intervals(), 4);
    assert_eq!(r.status(), OffsetAlgoStatus::NoError);
}

#[test]
fn geom_offset_curve_result_set_done_zero_intervals() {
    let mut r = GeomOffsetCurveResult::new();
    r.set_done(0);
    assert!(r.is_done());
    assert_eq!(r.nb_intervals(), 0);
}

#[test]
fn geom_offset_curve_result_set_done_large() {
    let mut r = GeomOffsetCurveResult::new();
    r.set_done(1_000_000);
    assert_eq!(r.nb_intervals(), 1_000_000);
}

// ─────────────────────────────────────────────────────────────────────────────
// Geom2dOffsetCurveParams
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn geom2d_offset_curve_params_defaults() {
    let p = Geom2dOffsetCurveParams::new(2.5, 1e-6);
    assert_eq!(p.offset(), 2.5);
    assert!((p.tolerance() - 1e-6).abs() < 1e-18);
    assert_eq!(p.join_type(), 0, "default join type should be arc (0)");
}

#[test]
fn geom2d_offset_curve_params_set_join_type_tangent() {
    let mut p = Geom2dOffsetCurveParams::new(1.0, 1e-7);
    p.set_join_type(1);
    assert_eq!(p.join_type(), 1);
}

#[test]
fn geom2d_offset_curve_params_set_join_type_arc() {
    let mut p = Geom2dOffsetCurveParams::new(1.0, 1e-7);
    p.set_join_type(1);
    p.set_join_type(0);
    assert_eq!(p.join_type(), 0);
}

#[test]
fn geom2d_offset_curve_params_negative_offset() {
    let p = Geom2dOffsetCurveParams::new(-4.0, 1e-5);
    assert_eq!(p.offset(), -4.0);
}

#[test]
fn geom2d_offset_curve_params_clone() {
    let p = Geom2dOffsetCurveParams::new(3.0, 1e-7);
    let mut q = p.clone();
    q.set_join_type(1);
    assert_eq!(p.join_type(), 0, "original must be unaffected");
    assert_eq!(q.join_type(), 1);
}

// ─────────────────────────────────────────────────────────────────────────────
// Geom2dOffsetResult
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn geom2d_offset_result_initial_state() {
    let r = Geom2dOffsetResult::new();
    assert!(!r.is_done());
    assert_eq!(r.nb_intervals(), 0);
}

#[test]
fn geom2d_offset_result_default_equals_new() {
    let r1 = Geom2dOffsetResult::new();
    let r2 = Geom2dOffsetResult::default();
    assert_eq!(r1.is_done(), r2.is_done());
    assert_eq!(r1.nb_intervals(), r2.nb_intervals());
}

#[test]
fn geom2d_offset_result_perform() {
    let mut r = Geom2dOffsetResult::new();
    r.perform(3);
    assert!(r.is_done());
    assert_eq!(r.nb_intervals(), 3);
}

#[test]
fn geom2d_offset_result_perform_zero() {
    let mut r = Geom2dOffsetResult::new();
    r.perform(0);
    assert!(r.is_done());
    assert_eq!(r.nb_intervals(), 0);
}

#[test]
fn geom2d_offset_result_perform_updates_intervals() {
    let mut r = Geom2dOffsetResult::new();
    r.perform(5);
    assert_eq!(r.nb_intervals(), 5);
    r.perform(10);
    assert_eq!(r.nb_intervals(), 10);
}

#[test]
fn geom2d_offset_result_clone() {
    let mut r = Geom2dOffsetResult::new();
    r.perform(7);
    let r2 = r.clone();
    assert_eq!(r2.is_done(), r.is_done());
    assert_eq!(r2.nb_intervals(), r.nb_intervals());
}
