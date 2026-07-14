// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_api_int.rs
//! Integration tests for `geom_api_int` — GeomAPI intersection stubs.
//!
//! Each test exercises only the public API.  Because these are stubs the tests
//! validate interface correctness rather than numerical accuracy.

extern crate ironstream;
use ironstream::geom_api_int::*;

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiIntCSPoint
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn cs_point_stores_point() {
    let p = GeomApiIntCSPoint::new([1.0, 2.0, 3.0], 0.1, 0.2, 0.5);
    assert_eq!(p.point(), [1.0, 2.0, 3.0]);
}

#[test]
fn cs_point_stores_u_v_w() {
    let p = GeomApiIntCSPoint::new([0.0; 3], 0.25, 0.75, 1.5);
    let (u, v, w) = p.parameters();
    assert!((u - 0.25).abs() < 1e-15);
    assert!((v - 0.75).abs() < 1e-15);
    assert!((w - 1.5).abs() < 1e-15);
}

#[test]
fn cs_point_zero_parameters() {
    let p = GeomApiIntCSPoint::new([0.0; 3], 0.0, 0.0, 0.0);
    assert_eq!(p.parameters(), (0.0, 0.0, 0.0));
}

#[test]
fn cs_point_negative_parameters() {
    let p = GeomApiIntCSPoint::new([-1.0, -2.0, -3.0], -0.5, -0.5, -1.0);
    let (u, v, w) = p.parameters();
    assert!((u - (-0.5)).abs() < 1e-15);
    assert!((v - (-0.5)).abs() < 1e-15);
    assert!((w - (-1.0)).abs() < 1e-15);
    assert_eq!(p.point(), [-1.0, -2.0, -3.0]);
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiIntCS
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn cs_not_done_initially() {
    let cs = GeomApiIntCS::new();
    assert!(!cs.is_done());
}

#[test]
fn cs_zero_points_initially() {
    let cs = GeomApiIntCS::new();
    assert_eq!(cs.nb_points(), 0);
}

#[test]
fn cs_done_after_perform() {
    let mut cs = GeomApiIntCS::new();
    cs.perform(1);
    assert!(cs.is_done());
}

#[test]
fn cs_perform_zero_results() {
    let mut cs = GeomApiIntCS::new();
    cs.perform(0);
    assert!(cs.is_done());
    assert_eq!(cs.nb_points(), 0);
}

#[test]
fn cs_perform_one_result() {
    let mut cs = GeomApiIntCS::new();
    cs.perform(1);
    assert_eq!(cs.nb_points(), 1);
}

#[test]
fn cs_perform_five_results() {
    let mut cs = GeomApiIntCS::new();
    cs.perform(5);
    assert_eq!(cs.nb_points(), 5);
}

#[test]
fn cs_point_accessor_returns_stub_values() {
    let mut cs = GeomApiIntCS::new();
    cs.perform(3);
    for i in 0..3 {
        let pt = cs.point(i);
        assert_eq!(pt.point(), [0.0, 0.0, 0.0]);
        assert_eq!(pt.parameters(), (0.0, 0.0, 0.0));
    }
}

#[test]
fn cs_perform_clears_previous_results() {
    let mut cs = GeomApiIntCS::new();
    cs.perform(5);
    assert_eq!(cs.nb_points(), 5);
    cs.perform(2);
    assert_eq!(cs.nb_points(), 2);
}

#[test]
fn cs_default_not_done() {
    let cs = GeomApiIntCS::default();
    assert!(!cs.is_done());
    assert_eq!(cs.nb_points(), 0);
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiIntSSPoint
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn ss_point_is_line_true() {
    let p = GeomApiIntSSPoint::new(true, 10);
    assert!(p.is_line());
}

#[test]
fn ss_point_is_line_false() {
    let p = GeomApiIntSSPoint::new(false, 3);
    assert!(!p.is_line());
}

#[test]
fn ss_point_nb_points() {
    let p = GeomApiIntSSPoint::new(true, 7);
    assert_eq!(p.nb_points(), 7);
}

#[test]
fn ss_point_zero_segment_points() {
    let p = GeomApiIntSSPoint::new(false, 0);
    assert_eq!(p.nb_points(), 0);
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomApiIntSS
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn ss_not_done_initially() {
    let ss = GeomApiIntSS::new();
    assert!(!ss.is_done());
}

#[test]
fn ss_zero_lines_initially() {
    let ss = GeomApiIntSS::new();
    assert_eq!(ss.nb_lines(), 0);
}

#[test]
fn ss_done_after_perform() {
    let mut ss = GeomApiIntSS::new();
    ss.perform(1);
    assert!(ss.is_done());
}

#[test]
fn ss_perform_zero_results() {
    let mut ss = GeomApiIntSS::new();
    ss.perform(0);
    assert!(ss.is_done());
    assert_eq!(ss.nb_lines(), 0);
}

#[test]
fn ss_perform_one_result() {
    let mut ss = GeomApiIntSS::new();
    ss.perform(1);
    assert_eq!(ss.nb_lines(), 1);
}

#[test]
fn ss_perform_four_results() {
    let mut ss = GeomApiIntSS::new();
    ss.perform(4);
    assert_eq!(ss.nb_lines(), 4);
}

#[test]
fn ss_line_accessor_returns_stub_values() {
    let mut ss = GeomApiIntSS::new();
    ss.perform(2);
    for i in 0..2 {
        let line = ss.line(i);
        assert!(!line.is_line());
        assert_eq!(line.nb_points(), 0);
    }
}

#[test]
fn ss_perform_clears_previous_results() {
    let mut ss = GeomApiIntSS::new();
    ss.perform(6);
    assert_eq!(ss.nb_lines(), 6);
    ss.perform(1);
    assert_eq!(ss.nb_lines(), 1);
}

#[test]
fn ss_default_not_done() {
    let ss = GeomApiIntSS::default();
    assert!(!ss.is_done());
    assert_eq!(ss.nb_lines(), 0);
}
