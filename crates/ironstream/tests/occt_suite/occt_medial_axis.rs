// FILE: tests/occt_medial_axis.rs
extern crate ironstream;

use ironstream::medial_axis::{
    MatArcType, MatBasicElt, MatArc, MatBisector, MatGraph,
};

#[test]
fn mat_basic_elt_index_and_geom() {
    let e = MatBasicElt::new(7, 42);
    assert_eq!(e.index(), 7);
    assert_eq!(e.geom_id(), 42);
    assert_eq!(e.start_arc_id, 0);
    assert_eq!(e.end_arc_id, 0);
}

#[test]
fn mat_arc_fields() {
    let a = MatArc::new(0, 10, 20, 5000);
    assert_eq!(a.index(), 0);
    assert_eq!(a.first_elt(), 10);
    assert_eq!(a.second_elt(), 20);
    assert_eq!(a.bisector(), 5000);
    assert!((a.first_param() - 0.0).abs() < 1e-10);
    assert!((a.second_param() - 1.0).abs() < 1e-10);
    assert_eq!(a.arc_type, MatArcType::BisectorCurve);
}

#[test]
fn mat_bisector_distance_interpolation() {
    let b = MatBisector::new(1, 3, 7);
    // distance_at_start = 0.0, distance_at_end = 1.0
    assert!((b.distance_at(0.0) - 0.0).abs() < 1e-10);
    assert!((b.distance_at(1.0) - 1.0).abs() < 1e-10);
    assert!((b.distance_at(0.5) - 0.5).abs() < 1e-10);
    // bisector_curve_id = prim1 + prim2 + 95000
    assert_eq!(b.bisector_curve_id, 3 + 7 + 95000);
}

#[test]
fn mat_bisector_prim_ids() {
    let b = MatBisector::new(5, 100, 200);
    assert_eq!(b.index(), 5);
    assert_eq!(b.first_prim(), 100);
    assert_eq!(b.second_prim(), 200);
}

#[test]
fn mat_graph_add_and_count() {
    let mut g = MatGraph::new();
    g.add_elt(MatBasicElt::new(0, 10));
    g.add_elt(MatBasicElt::new(1, 20));
    g.add_arc(MatArc::new(0, 0, 1, 9000));
    g.add_bisector(MatBisector::new(0, 10, 20));
    assert_eq!(g.nb_elts(), 2);
    assert_eq!(g.nb_arcs(), 1);
    assert_eq!(g.nb_bisectors(), 1);
}

#[test]
fn mat_graph_access_by_index() {
    let mut g = MatGraph::new();
    g.add_elt(MatBasicElt::new(3, 99));
    assert_eq!(g.elt(0).map(|e| e.index()), Some(3));
    assert_eq!(g.elt(0).map(|e| e.geom_id()), Some(99));
    assert!(g.elt(1).is_none());
    assert!(g.arc(0).is_none());
    assert!(g.bisector(0).is_none());
}
