extern crate ironstream;

use ironstream::shape_build::{
    ShapeBuildEdge, ShapeBuildEdgeError, ShapeBuildReShape, ShapeBuildVertex, ShapeBuildWire,
};

// ---------------------------------------------------------------------------
// ShapeBuildEdge
// ---------------------------------------------------------------------------

#[test]
fn edge_new_defaults() {
    let e = ShapeBuildEdge::new();
    assert_eq!(e.nb_pcurves(), 0);
    assert!(!e.has_3d_curve());
}

#[test]
fn edge_validate_fails_no_curve() {
    let e = ShapeBuildEdge::new();
    assert_eq!(e.validate(), Err(ShapeBuildEdgeError::NoCurve3d));
}

#[test]
fn edge_validate_ok_with_3d_curve() {
    let mut e = ShapeBuildEdge::new();
    e.set_has_3d_curve(true);
    assert!(e.validate().is_ok());
}

#[test]
fn edge_validate_ok_with_pcurve() {
    let mut e = ShapeBuildEdge::new();
    e.add_pcurve();
    assert!(e.validate().is_ok());
}

#[test]
fn edge_add_pcurve_increments() {
    let mut e = ShapeBuildEdge::new();
    e.add_pcurve();
    e.add_pcurve();
    e.add_pcurve();
    assert_eq!(e.nb_pcurves(), 3);
}

#[test]
fn edge_set_tolerance_does_not_panic() {
    let mut e = ShapeBuildEdge::new();
    e.set_tolerance(0.001);
    e.add_pcurve();
    assert!(e.validate().is_ok());
}

#[test]
fn edge_set_same_parameter_false() {
    let mut e = ShapeBuildEdge::new();
    e.set_same_parameter(false);
    e.set_has_3d_curve(true);
    assert!(e.validate().is_ok());
}

#[test]
fn edge_default_matches_new() {
    let a = ShapeBuildEdge::new();
    let b = ShapeBuildEdge::default();
    assert_eq!(a.nb_pcurves(), b.nb_pcurves());
    assert_eq!(a.has_3d_curve(), b.has_3d_curve());
}

// ---------------------------------------------------------------------------
// ShapeBuildVertex
// ---------------------------------------------------------------------------

#[test]
fn vertex_distance_zero() {
    let a = ShapeBuildVertex::new(3.0, 4.0, 5.0, 1e-7);
    let b = ShapeBuildVertex::new(3.0, 4.0, 5.0, 1e-7);
    assert!((a.distance(&b)).abs() < 1e-14);
}

#[test]
fn vertex_distance_unit_x() {
    let a = ShapeBuildVertex::new(0.0, 0.0, 0.0, 1e-7);
    let b = ShapeBuildVertex::new(1.0, 0.0, 0.0, 1e-7);
    assert!((a.distance(&b) - 1.0).abs() < 1e-14);
}

#[test]
fn vertex_distance_3_4_0_is_5() {
    let a = ShapeBuildVertex::new(0.0, 0.0, 0.0, 1e-7);
    let b = ShapeBuildVertex::new(3.0, 4.0, 0.0, 1e-7);
    assert!((a.distance(&b) - 5.0).abs() < 1e-12);
}

#[test]
fn vertex_set_point_then_distance() {
    let mut a = ShapeBuildVertex::new(0.0, 0.0, 0.0, 1e-7);
    a.set_point(0.0, 0.0, 7.0);
    let b = ShapeBuildVertex::new(0.0, 0.0, 0.0, 1e-7);
    assert!((a.distance(&b) - 7.0).abs() < 1e-12);
}

#[test]
fn vertex_set_tolerance() {
    let mut v = ShapeBuildVertex::new(0.0, 0.0, 0.0, 1e-7);
    v.set_tolerance(0.005);
    assert!((v.tolerance - 0.005).abs() < 1e-14);
}

// ---------------------------------------------------------------------------
// ShapeBuildWire
// ---------------------------------------------------------------------------

#[test]
fn wire_empty_on_new() {
    let w = ShapeBuildWire::new(1e-6);
    assert_eq!(w.nb_edges(), 0);
    assert!(!w.is_closed());
    assert_eq!(w.edge(0), None);
}

#[test]
fn wire_add_edges_and_access() {
    let mut w = ShapeBuildWire::new(1e-6);
    w.add_edge(10);
    w.add_edge(20);
    w.add_edge(30);
    assert_eq!(w.nb_edges(), 3);
    assert_eq!(w.edge(0), Some(10));
    assert_eq!(w.edge(1), Some(20));
    assert_eq!(w.edge(2), Some(30));
    assert_eq!(w.edge(3), None);
}

#[test]
fn wire_close() {
    let mut w = ShapeBuildWire::new(1e-6);
    assert!(!w.is_closed());
    w.add_edge(1);
    w.add_edge(2);
    w.close();
    assert!(w.is_closed());
    assert_eq!(w.nb_edges(), 2);
}

// ---------------------------------------------------------------------------
// ShapeBuildReShape
// ---------------------------------------------------------------------------

#[test]
fn reshape_unmapped_returns_self() {
    let r = ShapeBuildReShape::new();
    assert_eq!(r.apply(42), Some(42));
    assert_eq!(r.nb_replacements(), 0);
}

#[test]
fn reshape_replace_and_apply() {
    let mut r = ShapeBuildReShape::new();
    r.replace(1, 99);
    assert_eq!(r.apply(1), Some(99));
    assert_eq!(r.nb_replacements(), 1);
}

#[test]
fn reshape_remove_gives_none() {
    let mut r = ShapeBuildReShape::new();
    r.remove(7);
    assert_eq!(r.apply(7), None);
    assert!(r.is_removed(7));
}

#[test]
fn reshape_is_removed_false_for_replacement() {
    let mut r = ShapeBuildReShape::new();
    r.replace(3, 4);
    assert!(!r.is_removed(3));
}

#[test]
fn reshape_nb_replacements_counts_removes() {
    let mut r = ShapeBuildReShape::new();
    r.replace(1, 2);
    r.remove(3);
    assert_eq!(r.nb_replacements(), 2);
}

#[test]
fn reshape_replace_overwrites() {
    let mut r = ShapeBuildReShape::new();
    r.replace(5, 10);
    r.replace(5, 20);
    assert_eq!(r.apply(5), Some(20));
    assert_eq!(r.nb_replacements(), 1);
}

#[test]
fn reshape_default_matches_new() {
    let r = ShapeBuildReShape::default();
    assert_eq!(r.nb_replacements(), 0);
    assert_eq!(r.apply(0), Some(0));
}
