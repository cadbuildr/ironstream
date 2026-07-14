// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_chamfer.rs
extern crate ironstream;
use ironstream::geom_chamfer::*;

// --- ChamferMode ---

#[test]
fn chamfer_mode_variants_are_distinct() {
    assert_ne!(ChamferMode::Equal, ChamferMode::Unequal);
}

#[test]
fn chamfer_mode_is_copy() {
    let m = ChamferMode::Equal;
    let m2 = m;
    assert_eq!(m, m2);
}

// --- ChamferEdge::new (Equal mode) ---

#[test]
fn chamfer_edge_new_stores_label() {
    let e = ChamferEdge::new("edge_1", 2.0);
    assert_eq!(e.edge_label(), "edge_1");
}

#[test]
fn chamfer_edge_new_sets_equal_mode() {
    let e = ChamferEdge::new("e", 1.5);
    assert_eq!(e.mode(), ChamferMode::Equal);
}

#[test]
fn chamfer_edge_new_dist1_equals_dist() {
    let e = ChamferEdge::new("e", 3.0);
    assert!((e.dist1() - 3.0).abs() < 1e-15);
}

#[test]
fn chamfer_edge_new_dist2_equals_dist() {
    let e = ChamferEdge::new("e", 3.0);
    assert!((e.dist2() - 3.0).abs() < 1e-15);
}

#[test]
fn chamfer_edge_new_dist1_equals_dist2() {
    let e = ChamferEdge::new("e", 4.5);
    assert!((e.dist1() - e.dist2()).abs() < 1e-15);
}

#[test]
fn chamfer_edge_new_zero_dist_allowed() {
    let e = ChamferEdge::new("zero", 0.0);
    assert!(e.dist1().abs() < 1e-15);
    assert!(e.dist2().abs() < 1e-15);
}

// --- ChamferEdge::new_unequal (Unequal mode) ---

#[test]
fn chamfer_edge_new_unequal_stores_label() {
    let e = ChamferEdge::new_unequal("edge_2", 1.0, 2.0);
    assert_eq!(e.edge_label(), "edge_2");
}

#[test]
fn chamfer_edge_new_unequal_sets_unequal_mode() {
    let e = ChamferEdge::new_unequal("e", 1.0, 2.0);
    assert_eq!(e.mode(), ChamferMode::Unequal);
}

#[test]
fn chamfer_edge_new_unequal_stores_dist1() {
    let e = ChamferEdge::new_unequal("e", 1.5, 3.0);
    assert!((e.dist1() - 1.5).abs() < 1e-15);
}

#[test]
fn chamfer_edge_new_unequal_stores_dist2() {
    let e = ChamferEdge::new_unequal("e", 1.5, 3.0);
    assert!((e.dist2() - 3.0).abs() < 1e-15);
}

#[test]
fn chamfer_edge_new_unequal_dist1_and_dist2_independent() {
    let e = ChamferEdge::new_unequal("e", 0.5, 10.0);
    assert!((e.dist1() - 0.5).abs() < 1e-15);
    assert!((e.dist2() - 10.0).abs() < 1e-15);
}

#[test]
fn chamfer_edge_clone_is_independent() {
    let e = ChamferEdge::new("orig", 5.0);
    let e2 = e.clone();
    assert_eq!(e.edge_label(), e2.edge_label());
    assert!((e.dist1() - e2.dist1()).abs() < 1e-15);
}

// --- ChamferResult ---

#[test]
fn chamfer_result_new_is_not_done() {
    let r = ChamferResult::new();
    assert!(!r.is_done());
}

#[test]
fn chamfer_result_new_has_zero_contours() {
    let r = ChamferResult::new();
    assert_eq!(r.nb_contours(), 0);
}

#[test]
fn chamfer_result_new_has_zero_surfaces() {
    let r = ChamferResult::new();
    assert_eq!(r.nb_surfaces(), 0);
}

#[test]
fn chamfer_result_build_marks_done() {
    let mut r = ChamferResult::new();
    r.build(2, 2);
    assert!(r.is_done());
}

#[test]
fn chamfer_result_build_stores_nb_contours() {
    let mut r = ChamferResult::new();
    r.build(4, 7);
    assert_eq!(r.nb_contours(), 4);
}

#[test]
fn chamfer_result_build_stores_nb_surfaces() {
    let mut r = ChamferResult::new();
    r.build(4, 7);
    assert_eq!(r.nb_surfaces(), 7);
}

#[test]
fn chamfer_result_build_zero_counts_allowed() {
    let mut r = ChamferResult::new();
    r.build(0, 0);
    assert!(r.is_done());
    assert_eq!(r.nb_contours(), 0);
    assert_eq!(r.nb_surfaces(), 0);
}

#[test]
fn chamfer_result_default_equals_new() {
    let r: ChamferResult = Default::default();
    assert!(!r.is_done());
    assert_eq!(r.nb_contours(), 0);
    assert_eq!(r.nb_surfaces(), 0);
}

// --- MakeChamfer ---

#[test]
fn make_chamfer_new_has_no_edges() {
    let c = MakeChamfer::new();
    assert_eq!(c.nb_edges(), 0);
}

#[test]
fn make_chamfer_new_is_not_done() {
    let c = MakeChamfer::new();
    assert!(!c.is_done());
}

#[test]
fn make_chamfer_add_edge_increments_count() {
    let mut c = MakeChamfer::new();
    c.add_edge(ChamferEdge::new("e1", 1.0));
    assert_eq!(c.nb_edges(), 1);
    c.add_edge(ChamferEdge::new("e2", 2.0));
    assert_eq!(c.nb_edges(), 2);
}

#[test]
fn make_chamfer_build_marks_done() {
    let mut c = MakeChamfer::new();
    c.add_edge(ChamferEdge::new("e", 1.0));
    c.build();
    assert!(c.is_done());
}

#[test]
fn make_chamfer_build_sets_nb_contours_to_edge_count() {
    let mut c = MakeChamfer::new();
    c.add_edge(ChamferEdge::new("a", 0.5));
    c.add_edge(ChamferEdge::new("b", 0.5));
    c.add_edge(ChamferEdge::new("c", 0.5));
    c.build();
    assert_eq!(c.result().nb_contours(), 3);
}

#[test]
fn make_chamfer_build_sets_nb_surfaces_to_edge_count() {
    let mut c = MakeChamfer::new();
    c.add_edge(ChamferEdge::new("x", 1.0));
    c.add_edge(ChamferEdge::new("y", 2.0));
    c.build();
    assert_eq!(c.result().nb_surfaces(), 2);
}

#[test]
fn make_chamfer_build_with_no_edges_succeeds() {
    let mut c = MakeChamfer::new();
    c.build();
    assert!(c.is_done());
    assert_eq!(c.result().nb_contours(), 0);
    assert_eq!(c.result().nb_surfaces(), 0);
}

#[test]
fn make_chamfer_result_is_done_matches_is_done() {
    let mut c = MakeChamfer::new();
    c.build();
    assert_eq!(c.result().is_done(), c.is_done());
}

#[test]
fn make_chamfer_with_unequal_edges_build_succeeds() {
    let mut c = MakeChamfer::new();
    c.add_edge(ChamferEdge::new_unequal("e1", 1.0, 2.0));
    c.add_edge(ChamferEdge::new_unequal("e2", 0.5, 1.5));
    c.build();
    assert!(c.is_done());
    assert_eq!(c.result().nb_contours(), 2);
    assert_eq!(c.result().nb_surfaces(), 2);
}

#[test]
fn make_chamfer_nb_edges_matches_edges_vec_len() {
    let mut c = MakeChamfer::new();
    c.add_edge(ChamferEdge::new("a", 1.0));
    c.add_edge(ChamferEdge::new("b", 1.0));
    c.add_edge(ChamferEdge::new("c", 1.0));
    assert_eq!(c.nb_edges(), c.edges.len());
}

#[test]
fn make_chamfer_default_equals_new() {
    let c: MakeChamfer = Default::default();
    assert_eq!(c.nb_edges(), 0);
    assert!(!c.is_done());
}
