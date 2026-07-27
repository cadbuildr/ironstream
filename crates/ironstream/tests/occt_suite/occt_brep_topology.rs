// FILE: tests/occt_brep_topology.rs
extern crate ironstream;

use ironstream::brep_topology::{
    BRepBuilder, BRepCurveKind, BRepEdgeData, BRepFaceData, BRepOrientation,
    BRepPointRepresentation,
};

// ---------------------------------------------------------------------------
// BRepEdgeData — construction and defaults
// ---------------------------------------------------------------------------

#[test]
fn edge_data_default_tolerance() {
    let e = BRepEdgeData::new();
    assert!((e.tolerance - 1e-7).abs() < 1e-20);
}

#[test]
fn edge_data_default_flags() {
    let e = BRepEdgeData::new();
    assert!(e.same_parameter);
    assert!(e.same_range);
    assert!(!e.degenerated);
}

#[test]
fn edge_data_default_curve_kind_is_undefined() {
    let e = BRepEdgeData::new();
    assert_eq!(e.curve_kind, BRepCurveKind::Undefined);
}

// ---------------------------------------------------------------------------
// BRepFaceData — construction and defaults
// ---------------------------------------------------------------------------

#[test]
fn face_data_default_tolerance() {
    let f = BRepFaceData::new();
    assert!((f.tolerance - 1e-7).abs() < 1e-20);
}

#[test]
fn face_data_default_location_index_is_identity() {
    let f = BRepFaceData::new();
    assert_eq!(f.location_index, 0);
}

#[test]
fn face_data_natural_restriction_off_by_default() {
    let f = BRepFaceData::new();
    assert!(!f.natural_restriction);
}

// ---------------------------------------------------------------------------
// BRepPointRepresentation
// ---------------------------------------------------------------------------

#[test]
fn point_repr_stores_xyz_and_tolerance() {
    let p = BRepPointRepresentation::new(3.0, -1.5, 7.0, 2e-6);
    assert_eq!(p.point[0], 3.0);
    assert_eq!(p.point[1], -1.5);
    assert_eq!(p.point[2], 7.0);
    assert!((p.tolerance - 2e-6).abs() < 1e-20);
}

// ---------------------------------------------------------------------------
// BRepBuilder — tolerance update (never-shrink semantics)
// ---------------------------------------------------------------------------

#[test]
fn builder_update_edge_tolerance_grows() {
    let b = BRepBuilder::new();
    let mut e = BRepEdgeData::new();
    b.update_tolerance(&mut e, 1e-3);
    assert!((e.tolerance - 1e-3).abs() < 1e-20);
}

#[test]
fn builder_update_edge_tolerance_does_not_shrink() {
    let b = BRepBuilder::new();
    let mut e = BRepEdgeData::new();
    e.tolerance = 5e-4;
    b.update_tolerance(&mut e, 1e-7);
    assert!((e.tolerance - 5e-4).abs() < 1e-20);
}

#[test]
fn builder_update_face_tolerance_grows() {
    let b = BRepBuilder::new();
    let mut f = BRepFaceData::new();
    b.update_face_tolerance(&mut f, 1e-5);
    assert!((f.tolerance - 1e-5).abs() < 1e-20);
}

#[test]
fn builder_update_face_tolerance_does_not_shrink() {
    let b = BRepBuilder::new();
    let mut f = BRepFaceData::new();
    f.tolerance = 1e-3;
    b.update_face_tolerance(&mut f, 1e-9);
    assert!((f.tolerance - 1e-3).abs() < 1e-20);
}

#[test]
fn builder_update_vertex_tolerance_grows() {
    let b = BRepBuilder::new();
    let mut p = BRepPointRepresentation::new(0.0, 0.0, 0.0, 1e-7);
    b.update_vertex_tolerance(&mut p, 1e-4);
    assert!((p.tolerance - 1e-4).abs() < 1e-20);
}

// ---------------------------------------------------------------------------
// BRepBuilder — flag setters
// ---------------------------------------------------------------------------

#[test]
fn builder_set_same_parameter_false() {
    let b = BRepBuilder::new();
    let mut e = BRepEdgeData::new();
    b.set_same_parameter(&mut e, false);
    assert!(!e.same_parameter);
}

#[test]
fn builder_set_same_range_false() {
    let b = BRepBuilder::new();
    let mut e = BRepEdgeData::new();
    b.set_same_range(&mut e, false);
    assert!(!e.same_range);
}

#[test]
fn builder_set_degenerated_true() {
    let b = BRepBuilder::new();
    let mut e = BRepEdgeData::new();
    b.set_degenerated(&mut e, true);
    assert!(e.degenerated);
}

// ---------------------------------------------------------------------------
// BRepOrientation — enum identity
// ---------------------------------------------------------------------------

#[test]
fn brep_orientation_copy_and_eq() {
    let o = BRepOrientation::Forward;
    let o2 = o;
    assert_eq!(o, o2);
    assert_ne!(o, BRepOrientation::Reversed);
    assert_ne!(BRepOrientation::Internal, BRepOrientation::External);
}
