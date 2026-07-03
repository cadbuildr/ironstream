// FILE: tests/occt_geom_loft.rs
//! Integration tests for `ironstream::geom_loft` — the BRepOffsetAPI_ThruSections
//! (loft) port.
//!
//! Tests exercise the public API only, covering:
//! - [`LoftSection`]   — construction, accessors, `set_wire`
//! - [`LoftParams`]    — defaults and setters
//! - [`LoftResult`]    — pre/post build state
//! - [`ThruSections`]  — section management, build formula, invalidation

extern crate ironstream;
use ironstream::geom_loft::*;

// ---------------------------------------------------------------------------
// LoftSection
// ---------------------------------------------------------------------------

#[test]
fn section_new_label_stored() {
    let s = LoftSection::new("profile_A", 4, 0.0);
    assert_eq!(s.label(), "profile_A");
}

#[test]
fn section_new_nb_poles_stored() {
    let s = LoftSection::new("s", 7, 0.0);
    assert_eq!(s.nb_poles(), 7);
}

#[test]
fn section_new_parameter_stored() {
    let s = LoftSection::new("s", 4, 0.333);
    assert!((s.parameter() - 0.333).abs() < 1.0e-15);
}

#[test]
fn section_new_is_wire_false_by_default() {
    let s = LoftSection::new("s", 4, 0.0);
    assert!(!s.is_wire());
}

#[test]
fn section_set_wire_true() {
    let mut s = LoftSection::new("s", 4, 0.0);
    s.set_wire(true);
    assert!(s.is_wire());
}

#[test]
fn section_set_wire_false_after_true() {
    let mut s = LoftSection::new("s", 4, 0.0);
    s.set_wire(true);
    s.set_wire(false);
    assert!(!s.is_wire());
}

#[test]
fn section_clone_independent() {
    let original = LoftSection::new("orig", 5, 0.1);
    let mut cloned = original.clone();
    cloned.set_wire(true);
    assert!(!original.is_wire());
    assert_eq!(original.label(), "orig");
}

#[test]
fn section_label_with_empty_string() {
    let s = LoftSection::new("", 2, 0.0);
    assert_eq!(s.label(), "");
}

#[test]
fn section_parameter_negative() {
    let s = LoftSection::new("s", 4, -1.0);
    assert!((s.parameter() - (-1.0)).abs() < f64::EPSILON);
}

// ---------------------------------------------------------------------------
// LoftParams
// ---------------------------------------------------------------------------

#[test]
fn params_new_solid_false() {
    let p = LoftParams::new();
    assert!(!p.is_solid());
}

#[test]
fn params_new_ruled_false() {
    let p = LoftParams::new();
    assert!(!p.is_ruled());
}

#[test]
fn params_new_pres3d_1e6() {
    let p = LoftParams::new();
    assert!((p.pres3d() - 1.0e-6).abs() < 1.0e-20);
}

#[test]
fn params_set_solid_true() {
    let mut p = LoftParams::new();
    p.set_solid(true);
    assert!(p.is_solid());
}

#[test]
fn params_set_solid_toggle() {
    let mut p = LoftParams::new();
    p.set_solid(true);
    p.set_solid(false);
    assert!(!p.is_solid());
}

#[test]
fn params_set_ruled_true() {
    let mut p = LoftParams::new();
    p.set_ruled(true);
    assert!(p.is_ruled());
}

#[test]
fn params_default_trait_matches_new() {
    let p1 = LoftParams::new();
    let p2: LoftParams = Default::default();
    assert_eq!(p1.is_solid(), p2.is_solid());
    assert_eq!(p1.is_ruled(), p2.is_ruled());
    assert!((p1.pres3d() - p2.pres3d()).abs() < f64::EPSILON);
}

// ---------------------------------------------------------------------------
// LoftResult
// ---------------------------------------------------------------------------

#[test]
fn result_new_not_done() {
    let r = LoftResult::new();
    assert!(!r.is_done());
}

#[test]
fn result_new_zero_faces() {
    let r = LoftResult::new();
    assert_eq!(r.nb_faces(), 0);
}

#[test]
fn result_new_zero_edges() {
    let r = LoftResult::new();
    assert_eq!(r.nb_edges(), 0);
}

#[test]
fn result_build_sets_done() {
    let mut r = LoftResult::new();
    r.build(3, 9);
    assert!(r.is_done());
}

#[test]
fn result_build_stores_faces() {
    let mut r = LoftResult::new();
    r.build(10, 0);
    assert_eq!(r.nb_faces(), 10);
}

#[test]
fn result_build_stores_edges() {
    let mut r = LoftResult::new();
    r.build(0, 24);
    assert_eq!(r.nb_edges(), 24);
}

#[test]
fn result_build_zero_counts_still_done() {
    let mut r = LoftResult::new();
    r.build(0, 0);
    assert!(r.is_done());
    assert_eq!(r.nb_faces(), 0);
    assert_eq!(r.nb_edges(), 0);
}

#[test]
fn result_default_trait_not_done() {
    let r: LoftResult = Default::default();
    assert!(!r.is_done());
}

#[test]
fn result_build_overwrites_previous() {
    let mut r = LoftResult::new();
    r.build(5, 15);
    r.build(8, 24);
    assert_eq!(r.nb_faces(), 8);
    assert_eq!(r.nb_edges(), 24);
}

// ---------------------------------------------------------------------------
// ThruSections — construction
// ---------------------------------------------------------------------------

#[test]
fn thru_sections_new_empty() {
    let ts = ThruSections::new(LoftParams::new());
    assert_eq!(ts.nb_sections(), 0);
}

#[test]
fn thru_sections_new_not_done() {
    let ts = ThruSections::new(LoftParams::new());
    assert!(!ts.is_done());
}

// ---------------------------------------------------------------------------
// ThruSections — add_section
// ---------------------------------------------------------------------------

#[test]
fn thru_sections_add_one_section() {
    let mut ts = ThruSections::new(LoftParams::new());
    ts.add_section(LoftSection::new("s0", 4, 0.0));
    assert_eq!(ts.nb_sections(), 1);
}

#[test]
fn thru_sections_add_three_sections() {
    let mut ts = ThruSections::new(LoftParams::new());
    for i in 0..3 {
        ts.add_section(LoftSection::new("s", 4, i as f64));
    }
    assert_eq!(ts.nb_sections(), 3);
}

// ---------------------------------------------------------------------------
// ThruSections — build
// ---------------------------------------------------------------------------

#[test]
fn thru_sections_build_zero_sections_not_done() {
    let mut ts = ThruSections::new(LoftParams::new());
    ts.build();
    assert!(!ts.is_done());
}

#[test]
fn thru_sections_build_one_section_not_done() {
    let mut ts = ThruSections::new(LoftParams::new());
    ts.add_section(LoftSection::new("s0", 4, 0.0));
    ts.build();
    assert!(!ts.is_done());
}

#[test]
fn thru_sections_build_two_sections_done() {
    let mut ts = ThruSections::new(LoftParams::new());
    ts.add_section(LoftSection::new("s0", 4, 0.0));
    ts.add_section(LoftSection::new("s1", 4, 1.0));
    ts.build();
    assert!(ts.is_done());
}

#[test]
fn thru_sections_build_two_sections_nb_faces() {
    let mut ts = ThruSections::new(LoftParams::new());
    ts.add_section(LoftSection::new("s0", 4, 0.0));
    ts.add_section(LoftSection::new("s1", 4, 1.0));
    ts.build();
    // (2-1)*4 = 4
    assert_eq!(ts.result().nb_faces(), 4);
}

#[test]
fn thru_sections_build_two_sections_nb_edges() {
    let mut ts = ThruSections::new(LoftParams::new());
    ts.add_section(LoftSection::new("s0", 4, 0.0));
    ts.add_section(LoftSection::new("s1", 4, 1.0));
    ts.build();
    // (2-1)*4 + 2*4 = 12
    assert_eq!(ts.result().nb_edges(), 12);
}

#[test]
fn thru_sections_build_three_sections_nb_faces() {
    let mut ts = ThruSections::new(LoftParams::new());
    ts.add_section(LoftSection::new("s0", 6, 0.0));
    ts.add_section(LoftSection::new("s1", 6, 0.5));
    ts.add_section(LoftSection::new("s2", 6, 1.0));
    ts.build();
    // (3-1)*6 = 12
    assert_eq!(ts.result().nb_faces(), 12);
}

#[test]
fn thru_sections_build_three_sections_nb_edges() {
    let mut ts = ThruSections::new(LoftParams::new());
    ts.add_section(LoftSection::new("s0", 6, 0.0));
    ts.add_section(LoftSection::new("s1", 6, 0.5));
    ts.add_section(LoftSection::new("s2", 6, 1.0));
    ts.build();
    // 12 + 3*6 = 30
    assert_eq!(ts.result().nb_edges(), 30);
}

#[test]
fn thru_sections_build_four_sections_nb_faces() {
    let mut ts = ThruSections::new(LoftParams::new());
    for i in 0..4 {
        ts.add_section(LoftSection::new("s", 3, i as f64 / 3.0));
    }
    ts.build();
    // (4-1)*3 = 9
    assert_eq!(ts.result().nb_faces(), 9);
}

#[test]
fn thru_sections_build_four_sections_nb_edges() {
    let mut ts = ThruSections::new(LoftParams::new());
    for i in 0..4 {
        ts.add_section(LoftSection::new("s", 3, i as f64 / 3.0));
    }
    ts.build();
    // 9 + 4*3 = 21
    assert_eq!(ts.result().nb_edges(), 21);
}

// ---------------------------------------------------------------------------
// ThruSections — invalidation
// ---------------------------------------------------------------------------

#[test]
fn thru_sections_add_section_after_build_invalidates() {
    let mut ts = ThruSections::new(LoftParams::new());
    ts.add_section(LoftSection::new("s0", 4, 0.0));
    ts.add_section(LoftSection::new("s1", 4, 1.0));
    ts.build();
    assert!(ts.is_done());
    ts.add_section(LoftSection::new("s2", 4, 2.0));
    assert!(!ts.is_done());
}

#[test]
fn thru_sections_rebuild_after_add_section() {
    let mut ts = ThruSections::new(LoftParams::new());
    ts.add_section(LoftSection::new("s0", 4, 0.0));
    ts.add_section(LoftSection::new("s1", 4, 1.0));
    ts.build();
    ts.add_section(LoftSection::new("s2", 4, 2.0));
    ts.build();
    assert!(ts.is_done());
    // (3-1)*4 = 8
    assert_eq!(ts.result().nb_faces(), 8);
}

// ---------------------------------------------------------------------------
// ThruSections — result accessor consistency
// ---------------------------------------------------------------------------

#[test]
fn thru_sections_is_done_matches_result_is_done() {
    let mut ts = ThruSections::new(LoftParams::new());
    ts.add_section(LoftSection::new("s0", 5, 0.0));
    ts.add_section(LoftSection::new("s1", 5, 1.0));
    ts.build();
    assert_eq!(ts.is_done(), ts.result().is_done());
}

#[test]
fn thru_sections_result_reference_stable_across_calls() {
    let mut ts = ThruSections::new(LoftParams::new());
    ts.add_section(LoftSection::new("s0", 4, 0.0));
    ts.add_section(LoftSection::new("s1", 4, 1.0));
    ts.build();
    let faces1 = ts.result().nb_faces();
    let faces2 = ts.result().nb_faces();
    assert_eq!(faces1, faces2);
}

// ---------------------------------------------------------------------------
// ThruSections — wire sections
// ---------------------------------------------------------------------------

#[test]
fn thru_sections_wire_sections_build() {
    let mut ts = ThruSections::new(LoftParams::new());
    let mut s0 = LoftSection::new("w0", 8, 0.0);
    s0.set_wire(true);
    let mut s1 = LoftSection::new("w1", 8, 1.0);
    s1.set_wire(true);
    ts.add_section(s0);
    ts.add_section(s1);
    ts.build();
    assert!(ts.is_done());
    assert_eq!(ts.result().nb_faces(), 8); // (2-1)*8
    assert_eq!(ts.result().nb_edges(), 24); // 8 + 2*8
}

// ---------------------------------------------------------------------------
// ThruSections — solid / ruled params do not change formula
// ---------------------------------------------------------------------------

#[test]
fn thru_sections_solid_params_same_formula() {
    let mut p = LoftParams::new();
    p.set_solid(true);
    p.set_ruled(true);
    let mut ts = ThruSections::new(p);
    ts.add_section(LoftSection::new("s0", 3, 0.0));
    ts.add_section(LoftSection::new("s1", 3, 1.0));
    ts.build();
    assert!(ts.is_done());
    assert_eq!(ts.result().nb_faces(), 3); // (2-1)*3
    assert_eq!(ts.result().nb_edges(), 9); // 3 + 2*3
}
