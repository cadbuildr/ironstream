// FILE: tests/occt_geom_revol.rs
extern crate ironstream;
use ironstream::geom_revol::*;
use std::f64::consts::{PI, TAU};

// ──────────────────────────── RevolParams ───────────────────────────────────

#[test]
fn revol_params_stores_axis() {
    let axis = [0.0_f64, 0.0, 1.0];
    let origin = [1.0_f64, 2.0, 3.0];
    let p = RevolParams::new(axis, origin, PI / 2.0);
    assert_eq!(p.axis(), axis);
}

#[test]
fn revol_params_stores_axis_origin() {
    let origin = [4.0_f64, 5.0, 6.0];
    let p = RevolParams::new([0.0, 1.0, 0.0], origin, 1.0);
    assert_eq!(p.axis_origin(), origin);
}

#[test]
fn revol_params_stores_angle() {
    let angle = PI / 3.0;
    let p = RevolParams::new([1.0, 0.0, 0.0], [0.0; 3], angle);
    assert!((p.angle() - angle).abs() < 1e-15, "angle mismatch: {}", p.angle());
}

#[test]
fn revol_params_partial_not_full_revolution() {
    let p = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], PI);
    assert!(!p.is_full_revolution());
}

#[test]
fn revol_params_tau_is_full_revolution() {
    let p = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], TAU);
    assert!(p.is_full_revolution());
    assert!((p.angle() - TAU).abs() < 1e-15);
}

#[test]
fn revol_params_near_tau_is_full_revolution() {
    // Floating-point representation of 2π from a calculation.
    let almost_tau = 2.0 * PI;
    let p = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], almost_tau);
    assert!(p.is_full_revolution());
}

#[test]
fn revol_params_angle_clamped_above_tau() {
    let p = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], TAU + 1.0);
    assert!(p.angle() <= TAU + 1e-15);
}

#[test]
fn revol_params_angle_clamped_below_zero() {
    let p = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], -1.0);
    assert!(p.angle() >= 0.0);
}

#[test]
fn revol_params_set_full_revolution_marks_full() {
    let mut p = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], PI / 6.0);
    assert!(!p.is_full_revolution());
    p.set_full_revolution();
    assert!(p.is_full_revolution());
}

#[test]
fn revol_params_set_full_revolution_sets_angle_to_tau() {
    let mut p = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], PI / 6.0);
    p.set_full_revolution();
    assert!((p.angle() - TAU).abs() < 1e-15);
}

#[test]
fn revol_params_clone_is_equal() {
    let p = RevolParams::new([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], PI);
    let q = p.clone();
    assert_eq!(p, q);
}

// ──────────────────────────── RevolResult ───────────────────────────────────

#[test]
fn revol_result_new_not_done() {
    let r = RevolResult::new();
    assert!(!r.is_done());
}

#[test]
fn revol_result_new_zero_faces() {
    let r = RevolResult::new();
    assert_eq!(r.nb_faces(), 0);
}

#[test]
fn revol_result_new_zero_edges() {
    let r = RevolResult::new();
    assert_eq!(r.nb_edges(), 0);
}

#[test]
fn revol_result_build_marks_done() {
    let mut r = RevolResult::new();
    r.build(3, 9);
    assert!(r.is_done());
}

#[test]
fn revol_result_build_records_faces() {
    let mut r = RevolResult::new();
    r.build(5, 15);
    assert_eq!(r.nb_faces(), 5);
}

#[test]
fn revol_result_build_records_edges() {
    let mut r = RevolResult::new();
    r.build(5, 15);
    assert_eq!(r.nb_edges(), 15);
}

#[test]
fn revol_result_default_is_not_done() {
    let r = RevolResult::default();
    assert!(!r.is_done());
}

#[test]
fn revol_result_build_idempotent_on_second_call() {
    let mut r = RevolResult::new();
    r.build(2, 4);
    r.build(6, 18);
    // Second call overwrites.
    assert_eq!(r.nb_faces(), 6);
    assert_eq!(r.nb_edges(), 18);
}

// ──────────────────────────── MakeRevol ─────────────────────────────────────

#[test]
fn make_revol_new_not_done() {
    let params = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], PI);
    let builder = MakeRevol::new(params);
    assert!(!builder.is_done());
}

#[test]
fn make_revol_build_marks_done() {
    let params = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], PI);
    let mut builder = MakeRevol::new(params);
    builder.build(4);
    assert!(builder.is_done());
}

#[test]
fn make_revol_partial_face_count() {
    // n=4 profile edges, partial revolution → n + 2 = 6 faces.
    let params = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], PI / 2.0);
    let mut builder = MakeRevol::new(params);
    builder.build(4);
    assert_eq!(builder.result().nb_faces(), 6);
}

#[test]
fn make_revol_partial_edge_count() {
    // n=4 profile edges, partial revolution → 3*n + 1 = 13 edges.
    let params = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], PI / 2.0);
    let mut builder = MakeRevol::new(params);
    builder.build(4);
    assert_eq!(builder.result().nb_edges(), 13);
}

#[test]
fn make_revol_full_face_count() {
    // n=4 profile edges, full revolution → n = 4 faces.
    let params = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], TAU);
    let mut builder = MakeRevol::new(params);
    builder.build(4);
    assert_eq!(builder.result().nb_faces(), 4);
}

#[test]
fn make_revol_full_edge_count() {
    // n=4 profile edges, full revolution → 2*n = 8 edges.
    let params = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], TAU);
    let mut builder = MakeRevol::new(params);
    builder.build(4);
    assert_eq!(builder.result().nb_edges(), 8);
}

#[test]
fn make_revol_single_edge_partial() {
    // n=1 profile edge, partial → faces=3, edges=4.
    let params = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], PI);
    let mut builder = MakeRevol::new(params);
    builder.build(1);
    assert_eq!(builder.result().nb_faces(), 3);
    assert_eq!(builder.result().nb_edges(), 4);
}

#[test]
fn make_revol_single_edge_full() {
    // n=1 profile edge, full → faces=1, edges=2.
    let params = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], TAU);
    let mut builder = MakeRevol::new(params);
    builder.build(1);
    assert_eq!(builder.result().nb_faces(), 1);
    assert_eq!(builder.result().nb_edges(), 2);
}

#[test]
fn make_revol_preserves_params_axis() {
    let axis = [0.0_f64, 1.0, 0.0];
    let params = RevolParams::new(axis, [0.0; 3], PI / 4.0);
    let builder = MakeRevol::new(params);
    assert_eq!(builder.params.axis(), axis);
}

#[test]
fn make_revol_preserves_params_origin() {
    let origin = [3.0_f64, 0.0, 0.0];
    let params = RevolParams::new([0.0, 0.0, 1.0], origin, PI / 4.0);
    let builder = MakeRevol::new(params);
    assert_eq!(builder.params.axis_origin(), origin);
}

#[test]
fn make_revol_result_ref_matches_is_done() {
    let params = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], PI);
    let mut builder = MakeRevol::new(params);
    builder.build(3);
    assert_eq!(builder.result().is_done(), builder.is_done());
}

#[test]
fn make_revol_zero_profile_edges_full() {
    // Degenerate: no edges in profile.
    let params = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], TAU);
    let mut builder = MakeRevol::new(params);
    builder.build(0);
    assert_eq!(builder.result().nb_faces(), 0);
    assert_eq!(builder.result().nb_edges(), 0);
}

#[test]
fn make_revol_zero_profile_edges_partial() {
    // Degenerate: no edges in profile, partial.
    let params = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], PI / 3.0);
    let mut builder = MakeRevol::new(params);
    builder.build(0);
    assert_eq!(builder.result().nb_faces(), 2);
    assert_eq!(builder.result().nb_edges(), 1);
}
