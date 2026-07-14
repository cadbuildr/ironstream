// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_offset_surface_builder.rs
//! Integration tests for `geom_offset_surface_builder`.
//!
//! Exercises the public API of [`OffsetSurfaceParams`],
//! [`OffsetSurfaceResult`], and [`GeomOffsetSurfaceBuilder`] without
//! accessing private fields.

extern crate ironstream;
use ironstream::geom_offset_surface_builder::*;

// ─────────────────────────────────────────────────────────────────────────────
// OffsetSurfaceParams
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn params_new_stores_offset() {
    let p = OffsetSurfaceParams::new(3.5);
    assert!((p.offset() - 3.5).abs() < 1e-15, "offset should be 3.5");
}

#[test]
fn params_new_is_not_implicit() {
    let p = OffsetSurfaceParams::new(1.0);
    assert!(!p.is_implicit(), "default is_implicit should be false");
}

#[test]
fn params_new_u_range_is_zero_to_one() {
    let p = OffsetSurfaceParams::new(0.0);
    assert_eq!(p.u_range(), (0.0, 1.0), "default U range should be [0, 1]");
}

#[test]
fn params_new_v_range_is_zero_to_one() {
    let p = OffsetSurfaceParams::new(0.0);
    assert_eq!(p.v_range(), (0.0, 1.0), "default V range should be [0, 1]");
}

#[test]
fn params_set_offset_persists() {
    let mut p = OffsetSurfaceParams::new(1.0);
    p.set_offset(7.0);
    assert!((p.offset() - 7.0).abs() < 1e-15);
}

#[test]
fn params_set_offset_negative() {
    let mut p = OffsetSurfaceParams::new(1.0);
    p.set_offset(-2.5);
    assert!((p.offset() - (-2.5)).abs() < 1e-15);
}

#[test]
fn params_set_u_range_persists() {
    let mut p = OffsetSurfaceParams::new(0.0);
    p.set_u_range(-1.0, 5.0);
    assert_eq!(p.u_range(), (-1.0, 5.0));
}

#[test]
fn params_set_v_range_persists() {
    let mut p = OffsetSurfaceParams::new(0.0);
    p.set_v_range(0.5, 3.14);
    let (vf, vl) = p.v_range();
    assert!((vf - 0.5).abs() < 1e-15);
    assert!((vl - 3.14).abs() < 1e-15);
}

#[test]
fn params_u_range_and_v_range_independent() {
    let mut p = OffsetSurfaceParams::new(0.0);
    p.set_u_range(2.0, 4.0);
    p.set_v_range(0.0, 1.0);
    assert_eq!(p.u_range(), (2.0, 4.0));
    assert_eq!(p.v_range(), (0.0, 1.0));
}

#[test]
fn params_clone_is_independent() {
    let mut p = OffsetSurfaceParams::new(1.0);
    let q = p.clone();
    p.set_offset(99.0);
    assert!((q.offset() - 1.0).abs() < 1e-15, "clone should not be affected by mutation");
}

#[test]
fn params_default_offset_is_zero() {
    let p = OffsetSurfaceParams::default();
    assert_eq!(p.offset(), 0.0);
}

// ─────────────────────────────────────────────────────────────────────────────
// OffsetSurfaceResult
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn result_new_is_not_done() {
    let r = OffsetSurfaceResult::new();
    assert!(!r.is_done(), "fresh result must not be done");
}

#[test]
fn result_new_nb_patches_is_zero() {
    let r = OffsetSurfaceResult::new();
    assert_eq!(r.nb_patches(), 0);
}

#[test]
fn result_new_max_offset_error_is_zero() {
    let r = OffsetSurfaceResult::new();
    assert_eq!(r.max_offset_error(), 0.0);
}

#[test]
fn result_build_sets_done() {
    let mut r = OffsetSurfaceResult::new();
    r.build(4, 1.5);
    assert!(r.is_done());
}

#[test]
fn result_build_sets_nb_patches() {
    let mut r = OffsetSurfaceResult::new();
    r.build(7, 0.5);
    assert_eq!(r.nb_patches(), 7);
}

#[test]
fn result_build_max_error_is_abs_offset() {
    let mut r = OffsetSurfaceResult::new();
    r.build(2, -3.0);
    assert!((r.max_offset_error() - 3.0).abs() < 1e-15);
}

#[test]
fn result_build_zero_offset_gives_zero_error() {
    let mut r = OffsetSurfaceResult::new();
    r.build(1, 0.0);
    assert_eq!(r.max_offset_error(), 0.0);
}

#[test]
fn result_build_second_call_overwrites() {
    let mut r = OffsetSurfaceResult::new();
    r.build(3, 1.0);
    r.build(9, 2.5);
    assert_eq!(r.nb_patches(), 9);
    assert!((r.max_offset_error() - 2.5).abs() < 1e-15);
}

#[test]
fn result_default_is_not_done() {
    let r = OffsetSurfaceResult::default();
    assert!(!r.is_done());
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomOffsetSurfaceBuilder — lifecycle
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn builder_new_is_not_done() {
    let b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(1.0));
    assert!(!b.is_done(), "builder must not be done before build()");
}

#[test]
fn builder_new_result_nb_patches_is_zero() {
    let b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(1.0));
    assert_eq!(b.result().nb_patches(), 0);
}

#[test]
fn builder_build_sets_done() {
    let mut b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(2.0));
    b.build(4);
    assert!(b.is_done());
}

#[test]
fn builder_build_nb_patches_matches_argument() {
    let mut b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(1.0));
    b.build(6);
    assert_eq!(b.result().nb_patches(), 6);
}

#[test]
fn builder_build_max_error_is_abs_offset() {
    let mut b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(-4.0));
    b.build(2);
    assert!((b.result().max_offset_error() - 4.0).abs() < 1e-15);
}

#[test]
fn builder_build_zero_offset_gives_zero_error() {
    let mut b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(0.0));
    b.build(1);
    assert_eq!(b.result().max_offset_error(), 0.0);
}

#[test]
fn builder_build_second_call_overwrites() {
    let mut b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(1.0));
    b.build(3);
    b.build(10);
    assert_eq!(b.result().nb_patches(), 10);
    assert!(b.is_done());
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomOffsetSurfaceBuilder — evaluate stub
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn evaluate_returns_u_v_offset() {
    let b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(5.0));
    let pt = b.evaluate(0.3, 0.7);
    assert!((pt[0] - 0.3).abs() < 1e-15, "x component should be u");
    assert!((pt[1] - 0.7).abs() < 1e-15, "y component should be v");
    assert!((pt[2] - 5.0).abs() < 1e-15, "z component should be offset");
}

#[test]
fn evaluate_at_origin_with_zero_offset() {
    let b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(0.0));
    let pt = b.evaluate(0.0, 0.0);
    assert_eq!(pt, [0.0, 0.0, 0.0]);
}

#[test]
fn evaluate_with_negative_offset() {
    let b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(-2.0));
    let pt = b.evaluate(1.0, 1.0);
    assert!((pt[2] - (-2.0)).abs() < 1e-15);
}

#[test]
fn evaluate_consistent_across_uv() {
    let b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(3.0));
    let pt1 = b.evaluate(0.25, 0.75);
    let pt2 = b.evaluate(0.5, 0.5);
    // z is always the offset regardless of u, v
    assert!((pt1[2] - pt2[2]).abs() < 1e-15);
}

#[test]
fn evaluate_before_build_still_works() {
    // evaluate does not require build() to have been called
    let b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(1.5));
    let pt = b.evaluate(0.1, 0.9);
    assert!((pt[2] - 1.5).abs() < 1e-15);
}

// ─────────────────────────────────────────────────────────────────────────────
// Clone
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn builder_clone_is_independent() {
    let mut b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(2.0));
    b.build(3);
    let c = b.clone();
    assert!(c.is_done());
    assert_eq!(c.result().nb_patches(), 3);
}
