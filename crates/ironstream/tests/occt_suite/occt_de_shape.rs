// FILE: tests/occt_de_shape.rs
extern crate ironstream;

use ironstream::de_shape::{
    DeShapeBoundingBox, DeShapeClassifier, DeShapeStats, DeShapeType,
};

#[test]
fn shape_type_rank_vertex_is_zero() {
    assert_eq!(DeShapeType::Vertex.rank(), 0);
}

#[test]
fn shape_type_rank_solid_is_five() {
    assert_eq!(DeShapeType::Solid.rank(), 5);
}

#[test]
fn shape_type_name_roundtrip_via_classify() {
    let types = [
        DeShapeType::Vertex,
        DeShapeType::Edge,
        DeShapeType::Wire,
        DeShapeType::Face,
        DeShapeType::Shell,
        DeShapeType::Solid,
        DeShapeType::CompSolid,
        DeShapeType::Compound,
        DeShapeType::Shape,
    ];
    for st in types {
        assert_eq!(DeShapeClassifier::classify(st.name()), st);
    }
}

#[test]
fn shape_type_ordering_vertex_less_than_solid() {
    assert!(DeShapeType::Vertex < DeShapeType::Solid);
    assert!(DeShapeType::Solid < DeShapeType::Compound);
}

#[test]
fn stats_add_accumulates_all_fields() {
    let mut a = DeShapeStats::new();
    a.nb_vertices = 10;
    a.nb_edges = 20;
    a.nb_wires = 5;
    a.nb_faces = 8;
    a.nb_shells = 3;
    a.nb_solids = 1;

    let mut b = DeShapeStats::new();
    b.nb_vertices = 2;
    b.nb_solids = 4;

    a.add(&b);
    assert_eq!(a.nb_vertices, 12);
    assert_eq!(a.nb_solids, 5);
    assert_eq!(a.total(), 10 + 2 + 20 + 5 + 8 + 3 + 1 + 4);
}

#[test]
fn bbox_from_min_max_is_not_void() {
    let bb = DeShapeBoundingBox::from_min_max(1.0, 4.0, 2.0, 5.0, 3.0, 6.0);
    assert!(!bb.is_void());
    assert_eq!(bb.xmin, 1.0);
    assert_eq!(bb.xmax, 4.0);
}

#[test]
fn bbox_add_point_expands_bounds() {
    let mut bb = DeShapeBoundingBox::new_void();
    bb.add_point(0.0, 0.0, 0.0);
    bb.add_point(3.0, 4.0, 0.0);
    assert_eq!(bb.xmax, 3.0);
    assert_eq!(bb.ymax, 4.0);
    let diag = bb.diagonal();
    assert!((diag - 5.0).abs() < 1e-12);
}

#[test]
fn bbox_diagonal_3_4_0_is_5() {
    let bb = DeShapeBoundingBox::from_min_max(0.0, 3.0, 0.0, 4.0, 0.0, 0.0);
    assert!((bb.diagonal() - 5.0).abs() < 1e-12);
}

#[test]
fn bbox_center_midpoint() {
    let bb = DeShapeBoundingBox::from_min_max(0.0, 4.0, 0.0, 6.0, 0.0, 8.0);
    let c = bb.center();
    assert_eq!(c[0], 2.0);
    assert_eq!(c[1], 3.0);
    assert_eq!(c[2], 4.0);
}

#[test]
fn bbox_volume_unit_cube() {
    let bb = DeShapeBoundingBox::from_min_max(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
    assert!((bb.volume() - 1.0).abs() < 1e-15);
}

#[test]
fn bbox_contains_point_inside_and_outside() {
    let bb = DeShapeBoundingBox::from_min_max(0.0, 10.0, 0.0, 10.0, 0.0, 10.0);
    assert!(bb.contains_point(5.0, 5.0, 5.0));
    assert!(bb.contains_point(0.0, 0.0, 0.0));
    assert!(bb.contains_point(10.0, 10.0, 10.0));
    assert!(!bb.contains_point(11.0, 5.0, 5.0));
    assert!(!bb.contains_point(5.0, -1.0, 5.0));
}

#[test]
fn bbox_merge_void_plus_nonempty_gives_nonempty() {
    let mut a = DeShapeBoundingBox::new_void();
    let b = DeShapeBoundingBox::from_min_max(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    a.merge(&b);
    assert!(!a.is_void());
    assert_eq!(a.xmin, 1.0);
    assert_eq!(a.zmax, 6.0);
}

#[test]
fn classifier_unknown_string_gives_shape() {
    assert_eq!(DeShapeClassifier::classify("BOGUS"), DeShapeType::Shape);
}

#[test]
fn classifier_dimension_and_is_manifold_consistency() {
    for st in [
        DeShapeType::Vertex,
        DeShapeType::Edge,
        DeShapeType::Wire,
        DeShapeType::Face,
        DeShapeType::Shell,
        DeShapeType::Solid,
    ] {
        assert!(DeShapeClassifier::is_manifold(st));
    }
    assert!(!DeShapeClassifier::is_manifold(DeShapeType::Compound));
    assert!(!DeShapeClassifier::is_manifold(DeShapeType::CompSolid));
}
