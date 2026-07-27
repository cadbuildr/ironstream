// FILE: tests/occt_brep_boolean_ext.rs
extern crate ironstream;
use ironstream::brep_boolean_ext::*;

#[test]
fn section_result_default_not_done() {
    let r = BooleanSectionResult::new();
    assert!(!r.is_done());
    assert_eq!(r.nb_edges(), 0);
    assert_eq!(r.nb_vertices(), 0);
}

#[test]
fn section_result_build_sets_done_and_counts() {
    let mut r = BooleanSectionResult::new();
    r.build(7, 8);
    assert!(r.is_done());
    assert_eq!(r.nb_edges(), 7);
    assert_eq!(r.nb_vertices(), 8);
}

#[test]
fn section_result_build_zero_edges() {
    let mut r = BooleanSectionResult::new();
    r.build(0, 0);
    assert!(r.is_done());
    assert_eq!(r.nb_edges(), 0);
    assert_eq!(r.nb_vertices(), 0);
}

#[test]
fn boolean_section_new_approximation_stored() {
    let s = BooleanSection::new(true);
    assert!(s.approximation);
    assert!(!s.is_done());

    let s2 = BooleanSection::new(false);
    assert!(!s2.approximation);
    assert!(!s2.is_done());
}

#[test]
fn boolean_section_build_sets_done() {
    let mut s = BooleanSection::new(true);
    s.build(5);
    assert!(s.is_done());
}

#[test]
fn boolean_section_build_edges_and_vertices() {
    let mut s = BooleanSection::new(false);
    s.build(4);
    assert_eq!(s.result().nb_edges(), 4);
    assert_eq!(s.result().nb_vertices(), 5);
}

#[test]
fn boolean_section_vertices_always_edges_plus_one() {
    for n in [0usize, 1, 10, 100] {
        let mut s = BooleanSection::new(true);
        s.build(n);
        assert_eq!(s.result().nb_vertices(), n + 1);
    }
}

#[test]
fn boolean_section_result_ref_consistent_with_is_done() {
    let mut s = BooleanSection::new(true);
    assert!(!s.result().is_done());
    s.build(2);
    assert!(s.result().is_done());
    assert!(s.is_done());
}

#[test]
fn split_result_default_not_done() {
    let r = BooleanSplitResult::new();
    assert!(!r.is_done());
    assert_eq!(r.nb_parts(), 0);
}

#[test]
fn split_result_build_sets_done_and_parts() {
    let mut r = BooleanSplitResult::new();
    r.build(6);
    assert!(r.is_done());
    assert_eq!(r.nb_parts(), 6);
}

#[test]
fn split_result_build_zero_parts() {
    let mut r = BooleanSplitResult::new();
    r.build(0);
    assert!(r.is_done());
    assert_eq!(r.nb_parts(), 0);
}

#[test]
fn boolean_splitter_new_fields_stored() {
    let sp = BooleanSplitter::new(3, 2);
    assert_eq!(sp.nb_arguments, 3);
    assert_eq!(sp.nb_tools, 2);
    assert!(!sp.is_done());
}

#[test]
fn boolean_splitter_build_sets_done() {
    let mut sp = BooleanSplitter::new(2, 3);
    sp.build();
    assert!(sp.is_done());
}

#[test]
fn boolean_splitter_build_parts_is_product() {
    let mut sp = BooleanSplitter::new(3, 4);
    sp.build();
    assert_eq!(sp.result().nb_parts(), 12);
}

#[test]
fn boolean_splitter_one_argument_one_tool() {
    let mut sp = BooleanSplitter::new(1, 1);
    sp.build();
    assert_eq!(sp.result().nb_parts(), 1);
}

#[test]
fn boolean_splitter_zero_arguments_zero_parts() {
    let mut sp = BooleanSplitter::new(0, 5);
    sp.build();
    assert!(sp.is_done());
    assert_eq!(sp.result().nb_parts(), 0);
}

#[test]
fn boolean_splitter_zero_tools_zero_parts() {
    let mut sp = BooleanSplitter::new(5, 0);
    sp.build();
    assert!(sp.is_done());
    assert_eq!(sp.result().nb_parts(), 0);
}

#[test]
fn boolean_splitter_result_ref_consistent_with_is_done() {
    let mut sp = BooleanSplitter::new(2, 2);
    assert!(!sp.result().is_done());
    sp.build();
    assert!(sp.result().is_done());
    assert!(sp.is_done());
}
