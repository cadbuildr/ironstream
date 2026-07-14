// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_fillet.rs
extern crate ironstream;
use ironstream::geom_fillet::*;

// --- FilletType ---

#[test]
fn fillet_type_variants_are_distinct() {
    assert_ne!(FilletType::Rational, FilletType::QuasiAngular);
    assert_ne!(FilletType::QuasiAngular, FilletType::Polynomial);
    assert_ne!(FilletType::Rational, FilletType::Polynomial);
}

#[test]
fn fillet_type_is_copy() {
    let t = FilletType::Rational;
    let t2 = t;
    assert_eq!(t, t2);
}

// --- FilletEdge ---

#[test]
fn fillet_edge_new_stores_label_and_radius() {
    let e = FilletEdge::new("edge_1", 2.5);
    assert_eq!(e.label(), "edge_1");
    assert!((e.radius() - 2.5).abs() < 1e-15);
}

#[test]
fn fillet_edge_set_radius_updates_value() {
    let mut e = FilletEdge::new("e", 1.0);
    e.set_radius(3.7);
    assert!((e.radius() - 3.7).abs() < 1e-15);
}

#[test]
fn fillet_edge_label_unchanged_after_radius_update() {
    let mut e = FilletEdge::new("my_edge", 0.5);
    e.set_radius(10.0);
    assert_eq!(e.label(), "my_edge");
}

#[test]
fn fillet_edge_zero_radius_allowed() {
    let e = FilletEdge::new("z", 0.0);
    assert!((e.radius()).abs() < 1e-15);
}

#[test]
fn fillet_edge_clone_is_independent() {
    let e = FilletEdge::new("orig", 5.0);
    let mut e2 = e.clone();
    e2.set_radius(99.0);
    assert!((e.radius() - 5.0).abs() < 1e-15);
    assert!((e2.radius() - 99.0).abs() < 1e-15);
}

// --- FilletResult ---

#[test]
fn fillet_result_new_is_not_done() {
    let r = FilletResult::new();
    assert!(!r.is_done());
}

#[test]
fn fillet_result_new_has_zero_contours_and_surfaces() {
    let r = FilletResult::new();
    assert_eq!(r.nb_contours(), 0);
    assert_eq!(r.nb_surfaces(), 0);
}

#[test]
fn fillet_result_new_has_empty_error_label() {
    let r = FilletResult::new();
    assert_eq!(r.error_label(), "");
}

#[test]
fn fillet_result_set_done_marks_done() {
    let mut r = FilletResult::new();
    r.set_done(3, 3);
    assert!(r.is_done());
}

#[test]
fn fillet_result_set_done_stores_counts() {
    let mut r = FilletResult::new();
    r.set_done(4, 7);
    assert_eq!(r.nb_contours(), 4);
    assert_eq!(r.nb_surfaces(), 7);
}

#[test]
fn fillet_result_set_done_clears_error_label() {
    let mut r = FilletResult::new();
    r.set_error("something went wrong");
    r.set_done(1, 1);
    assert_eq!(r.error_label(), "");
}

#[test]
fn fillet_result_set_error_stores_message() {
    let mut r = FilletResult::new();
    r.set_error("bad topology");
    assert_eq!(r.error_label(), "bad topology");
}

#[test]
fn fillet_result_set_error_leaves_not_done() {
    let mut r = FilletResult::new();
    r.set_error("oops");
    assert!(!r.is_done());
}

#[test]
fn fillet_result_default_equals_new() {
    let r: FilletResult = Default::default();
    assert!(!r.is_done());
    assert_eq!(r.nb_contours(), 0);
    assert_eq!(r.nb_surfaces(), 0);
    assert_eq!(r.error_label(), "");
}

// --- FilletBuilder ---

#[test]
fn fillet_builder_new_has_no_edges() {
    let b = FilletBuilder::new(FilletType::Rational, 1e-6);
    assert_eq!(b.nb_edges(), 0);
}

#[test]
fn fillet_builder_new_is_not_done() {
    let b = FilletBuilder::new(FilletType::Polynomial, 1e-7);
    assert!(!b.is_done());
}

#[test]
fn fillet_builder_stores_fillet_type() {
    let b = FilletBuilder::new(FilletType::QuasiAngular, 1e-6);
    assert_eq!(b.fillet_type, FilletType::QuasiAngular);
}

#[test]
fn fillet_builder_stores_tolerance() {
    let b = FilletBuilder::new(FilletType::Rational, 0.001);
    assert!((b.tolerance - 0.001).abs() < 1e-15);
}

#[test]
fn fillet_builder_add_edge_increments_count() {
    let mut b = FilletBuilder::new(FilletType::Rational, 1e-6);
    b.add_edge(FilletEdge::new("e1", 1.0));
    assert_eq!(b.nb_edges(), 1);
    b.add_edge(FilletEdge::new("e2", 2.0));
    assert_eq!(b.nb_edges(), 2);
}

#[test]
fn fillet_builder_build_marks_done() {
    let mut b = FilletBuilder::new(FilletType::Rational, 1e-6);
    b.add_edge(FilletEdge::new("e1", 1.0));
    b.build();
    assert!(b.is_done());
}

#[test]
fn fillet_builder_build_sets_nb_contours_to_edge_count() {
    let mut b = FilletBuilder::new(FilletType::Polynomial, 1e-6);
    b.add_edge(FilletEdge::new("a", 0.5));
    b.add_edge(FilletEdge::new("b", 0.5));
    b.add_edge(FilletEdge::new("c", 0.5));
    b.build();
    assert_eq!(b.result().nb_contours(), 3);
}

#[test]
fn fillet_builder_build_sets_nb_surfaces_to_edge_count() {
    let mut b = FilletBuilder::new(FilletType::QuasiAngular, 1e-6);
    b.add_edge(FilletEdge::new("x", 1.0));
    b.add_edge(FilletEdge::new("y", 2.0));
    b.build();
    assert_eq!(b.result().nb_surfaces(), 2);
}

#[test]
fn fillet_builder_build_with_no_edges_succeeds() {
    let mut b = FilletBuilder::new(FilletType::Rational, 1e-6);
    b.build();
    assert!(b.is_done());
    assert_eq!(b.result().nb_contours(), 0);
    assert_eq!(b.result().nb_surfaces(), 0);
}

#[test]
fn fillet_builder_result_error_label_empty_after_build() {
    let mut b = FilletBuilder::new(FilletType::Rational, 1e-6);
    b.add_edge(FilletEdge::new("e", 1.0));
    b.build();
    assert_eq!(b.result().error_label(), "");
}

#[test]
fn fillet_builder_result_ref_matches_is_done() {
    let mut b = FilletBuilder::new(FilletType::Polynomial, 1e-6);
    b.build();
    assert_eq!(b.result().is_done(), b.is_done());
}
