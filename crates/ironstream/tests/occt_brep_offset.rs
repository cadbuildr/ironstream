// FILE: rust/ironstream/crates/ironstream/tests/occt_brep_offset.rs
//! Integration tests for `brep_offset` —
//! `BRepOffset_Mode`, `GeomAbs_JoinType`, `BRepOffsetAPI_MakeThickSolid`,
//! and `BRepOffsetAPI_MakeOffset` (2-D wire offset) stubs.
//!
//! All tests exercise the public API only (no access to private fields unless
//! the field is declared `pub`).

extern crate ironstream;

use ironstream::brep_offset::{
    BrepMakeOffset, BrepMakeThickSolid, BrepOffsetJoinType, BrepOffsetMode, BrepOffsetParams,
};

// ─────────────────────────────────────────────────────────────────────────────
// BrepOffsetMode
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn offset_mode_variants_are_distinct() {
    assert_ne!(BrepOffsetMode::Skin, BrepOffsetMode::Pipe);
    assert_ne!(BrepOffsetMode::Pipe, BrepOffsetMode::RectoVerso);
    assert_ne!(BrepOffsetMode::Skin, BrepOffsetMode::RectoVerso);
}

#[test]
fn offset_mode_copy_and_clone() {
    let m = BrepOffsetMode::Pipe;
    let m2 = m; // Copy
    let m3 = m.clone();
    assert_eq!(m, m2);
    assert_eq!(m, m3);
}

#[test]
fn offset_mode_debug_format_is_non_empty() {
    let s = format!("{:?}", BrepOffsetMode::Skin);
    assert!(!s.is_empty());
    let s = format!("{:?}", BrepOffsetMode::RectoVerso);
    assert!(!s.is_empty());
}

// ─────────────────────────────────────────────────────────────────────────────
// BrepOffsetJoinType
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn join_type_variants_are_distinct() {
    assert_ne!(BrepOffsetJoinType::Arc, BrepOffsetJoinType::Tangent);
    assert_ne!(BrepOffsetJoinType::Tangent, BrepOffsetJoinType::Intersection);
    assert_ne!(BrepOffsetJoinType::Arc, BrepOffsetJoinType::Intersection);
}

#[test]
fn join_type_copy_and_clone() {
    let j = BrepOffsetJoinType::Intersection;
    let j2 = j;
    let j3 = j.clone();
    assert_eq!(j, j2);
    assert_eq!(j, j3);
}

#[test]
fn join_type_debug_format_is_non_empty() {
    let s = format!("{:?}", BrepOffsetJoinType::Arc);
    assert!(!s.is_empty());
}

// ─────────────────────────────────────────────────────────────────────────────
// BrepOffsetParams — construction and accessors
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn params_new_stores_offset_and_tolerance() {
    let p = BrepOffsetParams::new(2.5, 0.001);
    assert_eq!(p.offset(), 2.5);
    assert_eq!(p.tolerance(), 0.001);
}

#[test]
fn params_new_default_mode_is_skin() {
    let p = BrepOffsetParams::new(1.0, 0.01);
    assert_eq!(p.mode(), BrepOffsetMode::Skin);
}

#[test]
fn params_new_default_join_type_is_arc() {
    let p = BrepOffsetParams::new(1.0, 0.01);
    assert_eq!(p.join_type(), BrepOffsetJoinType::Arc);
}

#[test]
fn params_new_default_intersection_is_false() {
    let p = BrepOffsetParams::new(1.0, 0.01);
    assert!(!p.intersection());
}

#[test]
fn params_set_mode_skin() {
    let mut p = BrepOffsetParams::new(1.0, 0.01);
    p.set_mode(BrepOffsetMode::Skin);
    assert_eq!(p.mode(), BrepOffsetMode::Skin);
}

#[test]
fn params_set_mode_pipe() {
    let mut p = BrepOffsetParams::new(1.0, 0.01);
    p.set_mode(BrepOffsetMode::Pipe);
    assert_eq!(p.mode(), BrepOffsetMode::Pipe);
}

#[test]
fn params_set_mode_recto_verso() {
    let mut p = BrepOffsetParams::new(1.0, 0.01);
    p.set_mode(BrepOffsetMode::RectoVerso);
    assert_eq!(p.mode(), BrepOffsetMode::RectoVerso);
}

#[test]
fn params_set_join_type_arc() {
    let mut p = BrepOffsetParams::new(1.0, 0.01);
    p.set_join_type(BrepOffsetJoinType::Arc);
    assert_eq!(p.join_type(), BrepOffsetJoinType::Arc);
}

#[test]
fn params_set_join_type_tangent() {
    let mut p = BrepOffsetParams::new(1.0, 0.01);
    p.set_join_type(BrepOffsetJoinType::Tangent);
    assert_eq!(p.join_type(), BrepOffsetJoinType::Tangent);
}

#[test]
fn params_set_join_type_intersection() {
    let mut p = BrepOffsetParams::new(1.0, 0.01);
    p.set_join_type(BrepOffsetJoinType::Intersection);
    assert_eq!(p.join_type(), BrepOffsetJoinType::Intersection);
}

#[test]
fn params_set_intersection_true() {
    let mut p = BrepOffsetParams::new(1.0, 0.01);
    p.set_intersection(true);
    assert!(p.intersection());
}

#[test]
fn params_set_intersection_false() {
    let mut p = BrepOffsetParams::new(1.0, 0.01);
    p.set_intersection(true);
    p.set_intersection(false);
    assert!(!p.intersection());
}

#[test]
fn params_negative_offset_stored_correctly() {
    let p = BrepOffsetParams::new(-3.0, 0.01);
    assert_eq!(p.offset(), -3.0);
}

#[test]
fn params_zero_offset_stored_correctly() {
    let p = BrepOffsetParams::new(0.0, 1e-7);
    assert_eq!(p.offset(), 0.0);
    assert_eq!(p.tolerance(), 1e-7);
}

#[test]
fn params_clone_is_independent() {
    let mut p = BrepOffsetParams::new(1.0, 0.01);
    let mut p2 = p.clone();
    p.set_mode(BrepOffsetMode::Pipe);
    p2.set_mode(BrepOffsetMode::RectoVerso);
    assert_eq!(p.mode(), BrepOffsetMode::Pipe);
    assert_eq!(p2.mode(), BrepOffsetMode::RectoVerso);
}

// ─────────────────────────────────────────────────────────────────────────────
// BrepMakeThickSolid — construction and lifecycle
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn thick_solid_not_done_before_perform() {
    let p = BrepOffsetParams::new(1.0, 0.01);
    let b = BrepMakeThickSolid::new(p);
    assert!(!b.is_done());
}

#[test]
fn thick_solid_nb_faces_removed_zero_before_perform() {
    let p = BrepOffsetParams::new(1.0, 0.01);
    let b = BrepMakeThickSolid::new(p);
    assert_eq!(b.nb_faces_removed(), 0);
}

#[test]
fn thick_solid_is_done_after_perform() {
    let p = BrepOffsetParams::new(1.0, 0.01);
    let mut b = BrepMakeThickSolid::new(p);
    b.perform(1);
    assert!(b.is_done());
}

#[test]
fn thick_solid_nb_faces_removed_after_perform() {
    let p = BrepOffsetParams::new(1.0, 0.01);
    let mut b = BrepMakeThickSolid::new(p);
    b.perform(5);
    assert_eq!(b.nb_faces_removed(), 5);
}

#[test]
fn thick_solid_perform_zero_faces() {
    let p = BrepOffsetParams::new(0.5, 1e-6);
    let mut b = BrepMakeThickSolid::new(p);
    b.perform(0);
    assert!(b.is_done());
    assert_eq!(b.nb_faces_removed(), 0);
}

#[test]
fn thick_solid_perform_updates_nb_faces_on_second_call() {
    let p = BrepOffsetParams::new(1.0, 0.01);
    let mut b = BrepMakeThickSolid::new(p);
    b.perform(2);
    assert_eq!(b.nb_faces_removed(), 2);
    b.perform(7);
    assert_eq!(b.nb_faces_removed(), 7);
}

#[test]
fn thick_solid_params_offset_accessible() {
    let p = BrepOffsetParams::new(4.0, 0.001);
    let b = BrepMakeThickSolid::new(p);
    assert_eq!(b.params.offset(), 4.0);
}

#[test]
fn thick_solid_params_tolerance_accessible() {
    let p = BrepOffsetParams::new(1.0, 1e-4);
    let b = BrepMakeThickSolid::new(p);
    assert_eq!(b.params.tolerance(), 1e-4);
}

#[test]
fn thick_solid_params_mode_accessible() {
    let mut p = BrepOffsetParams::new(1.0, 0.01);
    p.set_mode(BrepOffsetMode::Pipe);
    let b = BrepMakeThickSolid::new(p);
    assert_eq!(b.params.mode(), BrepOffsetMode::Pipe);
}

#[test]
fn thick_solid_params_join_type_accessible() {
    let mut p = BrepOffsetParams::new(1.0, 0.01);
    p.set_join_type(BrepOffsetJoinType::Tangent);
    let b = BrepMakeThickSolid::new(p);
    assert_eq!(b.params.join_type(), BrepOffsetJoinType::Tangent);
}

#[test]
fn thick_solid_clone_is_independent() {
    let p = BrepOffsetParams::new(1.0, 0.01);
    let mut b = BrepMakeThickSolid::new(p);
    b.perform(3);
    let mut b2 = b.clone();
    b2.perform(9);
    assert_eq!(b.nb_faces_removed(), 3);
    assert_eq!(b2.nb_faces_removed(), 9);
}

// ─────────────────────────────────────────────────────────────────────────────
// BrepMakeOffset — construction and lifecycle
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn make_offset_not_done_before_perform() {
    let mo = BrepMakeOffset::new(1.0, BrepOffsetJoinType::Arc);
    assert!(!mo.is_done());
}

#[test]
fn make_offset_is_done_after_perform() {
    let mut mo = BrepMakeOffset::new(1.0, BrepOffsetJoinType::Arc);
    mo.perform();
    assert!(mo.is_done());
}

#[test]
fn make_offset_offset_accessor() {
    let mo = BrepMakeOffset::new(3.14, BrepOffsetJoinType::Arc);
    assert_eq!(mo.offset(), 3.14);
}

#[test]
fn make_offset_negative_distance() {
    let mut mo = BrepMakeOffset::new(-2.0, BrepOffsetJoinType::Intersection);
    mo.perform();
    assert!(mo.is_done());
    assert_eq!(mo.offset(), -2.0);
}

#[test]
fn make_offset_zero_distance() {
    let mut mo = BrepMakeOffset::new(0.0, BrepOffsetJoinType::Tangent);
    mo.perform();
    assert!(mo.is_done());
    assert_eq!(mo.offset(), 0.0);
}

#[test]
fn make_offset_join_type_arc_stored() {
    let mo = BrepMakeOffset::new(1.0, BrepOffsetJoinType::Arc);
    assert_eq!(mo.join_type, BrepOffsetJoinType::Arc);
}

#[test]
fn make_offset_join_type_tangent_stored() {
    let mo = BrepMakeOffset::new(1.0, BrepOffsetJoinType::Tangent);
    assert_eq!(mo.join_type, BrepOffsetJoinType::Tangent);
}

#[test]
fn make_offset_join_type_intersection_stored() {
    let mo = BrepMakeOffset::new(1.0, BrepOffsetJoinType::Intersection);
    assert_eq!(mo.join_type, BrepOffsetJoinType::Intersection);
}

#[test]
fn make_offset_perform_idempotent() {
    let mut mo = BrepMakeOffset::new(0.5, BrepOffsetJoinType::Arc);
    mo.perform();
    mo.perform();
    assert!(mo.is_done());
    assert_eq!(mo.offset(), 0.5);
}

#[test]
fn make_offset_clone_is_independent() {
    let mut mo = BrepMakeOffset::new(1.0, BrepOffsetJoinType::Arc);
    mo.perform();
    let mo2 = mo.clone();
    assert!(mo2.is_done());
    assert_eq!(mo2.offset(), 1.0);
}
