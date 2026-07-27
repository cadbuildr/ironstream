// FILE: tests/occt_brep_bnd_lib.rs
//! Integration tests for `brep_bnd_lib` — BRepBndLib bounding-box module.
//!
//! All tests exercise the public API only (no access to private fields).
//! Mirrors the spirit of OCCT's `BRepBndLib` usage patterns in real CAD code.

extern crate ironstream;

use ironstream::brep_bnd_lib::{
    make_edge_from_points, make_rect_face, make_vertex, BRepBndLib,
};
use ironstream::bnd_box::BndBox;
use ironstream::gp::Pnt;
use ironstream::top_exp::{
    TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shell, TopoDS_Solid, TopoDS_Shape,
    TopoDS_Vertex, TopoDS_Wire,
};

const TOL: f64 = 1e-10;

fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= TOL,
        "{label}: expected {a} ≈ {b}, diff = {}",
        (a - b).abs()
    );
}

// ─── helper: build a translated unit-box solid ────────────────────────────────

fn translated_box_solid(tx: f64, ty: f64, tz: f64) -> TopoDS_Shape {
    let corners: Vec<(f64, f64, f64)> = vec![
        (tx,       ty,       tz),
        (tx + 1.0, ty,       tz),
        (tx + 1.0, ty + 1.0, tz),
        (tx,       ty + 1.0, tz),
        (tx,       ty,       tz + 1.0),
        (tx + 1.0, ty,       tz + 1.0),
        (tx + 1.0, ty + 1.0, tz + 1.0),
        (tx,       ty + 1.0, tz + 1.0),
    ];
    let bottom_edge = make_edge_from_points(1, &corners[0..4]);
    let top_edge    = make_edge_from_points(2, &corners[4..8]);
    let bottom_wire = TopoDS_Wire { id: 1, edges: vec![bottom_edge] };
    let top_wire    = TopoDS_Wire { id: 2, edges: vec![top_edge] };
    let bottom_face = TopoDS_Face { id: 1, wires: vec![bottom_wire] };
    let top_face    = TopoDS_Face { id: 2, wires: vec![top_wire] };
    let shell       = TopoDS_Shell { id: 1, faces: vec![bottom_face, top_face] };
    let solid       = TopoDS_Solid { id: 1, shells: vec![shell] };
    TopoDS_Shape::Solid(solid)
}

// ─── Test 1: single vertex bounding box ──────────────────────────────────────

#[test]
fn single_vertex_bounding_box() {
    let v = make_vertex(1, 7.0, -3.0, 2.5);
    let mut bbox = BndBox::new();
    BRepBndLib::add(&v, &mut bbox);
    assert!(!bbox.is_void(), "vertex must not give void box");
    let l = bbox.get().unwrap();
    near(l.xmin, 7.0,  "xmin");
    near(l.xmax, 7.0,  "xmax");
    near(l.ymin, -3.0, "ymin");
    near(l.ymax, -3.0, "ymax");
    near(l.zmin, 2.5,  "zmin");
    near(l.zmax, 2.5,  "zmax");
}

// ─── Test 2: empty compound gives void bounding box ──────────────────────────

#[test]
fn empty_compound_gives_void_bbox() {
    let empty = TopoDS_Shape::Compound(TopoDS_Compound { id: 1, shapes: vec![] });
    let bbox = BRepBndLib::bounding_box(&empty);
    assert!(bbox.is_void(), "empty compound must give void box");
}

// ─── Test 3: axis-aligned edge spanning [-2, 2] on X ─────────────────────────

#[test]
fn axis_aligned_edge_bounding_box() {
    let e = TopoDS_Shape::Edge(make_edge_from_points(
        1,
        &[(-2.0, 0.0, 0.0), (2.0, 0.0, 0.0)],
    ));
    let bbox = BRepBndLib::bounding_box(&e);
    let l = bbox.get().expect("edge must not give void box");
    near(l.xmin, -2.0, "xmin");
    near(l.xmax,  2.0, "xmax");
    near(l.ymin,  0.0, "ymin");
    near(l.ymax,  0.0, "ymax");
    near(l.zmin,  0.0, "zmin");
    near(l.zmax,  0.0, "zmax");
}

// ─── Test 4: unit box solid has correct bounding box ─────────────────────────

#[test]
fn unit_box_solid_has_correct_bbox() {
    let s = translated_box_solid(0.0, 0.0, 0.0);
    let bbox = BRepBndLib::bounding_box(&s);
    let l = bbox.get().expect("solid must not give void box");
    near(l.xmin, 0.0, "xmin");
    near(l.xmax, 1.0, "xmax");
    near(l.ymin, 0.0, "ymin");
    near(l.ymax, 1.0, "ymax");
    near(l.zmin, 0.0, "zmin");
    near(l.zmax, 1.0, "zmax");
}

// ─── Test 5: translated solid bounding box ───────────────────────────────────

#[test]
fn translated_solid_bounding_box() {
    let s = translated_box_solid(10.0, -5.0, 3.0);
    let bbox = BRepBndLib::bounding_box(&s);
    let l = bbox.get().expect("translated solid must not give void box");
    near(l.xmin, 10.0, "xmin");
    near(l.xmax, 11.0, "xmax");
    near(l.ymin, -5.0, "ymin");
    near(l.ymax, -4.0, "ymax");
    near(l.zmin,  3.0, "zmin");
    near(l.zmax,  4.0, "zmax");
}

// ─── Test 6: compound of two non-overlapping solids ──────────────────────────

#[test]
fn compound_of_two_solids_gives_union_bbox() {
    let s1 = translated_box_solid(0.0, 0.0, 0.0);
    let s2 = translated_box_solid(5.0, 5.0, 5.0);
    let compound = TopoDS_Shape::Compound(TopoDS_Compound {
        id: 1,
        shapes: vec![s1, s2],
    });
    let bbox = BRepBndLib::bounding_box(&compound);
    let l = bbox.get().expect("compound must not give void box");
    near(l.xmin, 0.0, "xmin");
    near(l.xmax, 6.0, "xmax");
    near(l.ymin, 0.0, "ymin");
    near(l.ymax, 6.0, "ymax");
    near(l.zmin, 0.0, "zmin");
    near(l.zmax, 6.0, "zmax");
}

// ─── Test 7: diagonal of unit cube is sqrt(3) ────────────────────────────────

#[test]
fn diagonal_of_unit_cube() {
    let s = translated_box_solid(0.0, 0.0, 0.0);
    let d = BRepBndLib::diagonal(&s);
    let expected = 3.0_f64.sqrt();
    assert!(
        (d - expected).abs() < 1e-10,
        "diagonal should be sqrt(3) = {expected}, got {d}"
    );
}

// ─── Test 8: volume of bounding box of unit cube is 1 ────────────────────────

#[test]
fn volume_of_unit_cube_bbox_is_one() {
    let s = translated_box_solid(0.0, 0.0, 0.0);
    let vol = BRepBndLib::volume(&s);
    near(vol, 1.0, "volume");
}

// ─── Test 9: center of unit box is (0.5, 0.5, 0.5) ───────────────────────────

#[test]
fn center_of_unit_box() {
    let s = translated_box_solid(0.0, 0.0, 0.0);
    let c = BRepBndLib::center(&s).expect("unit box must have a center");
    near(c.x, 0.5, "cx");
    near(c.y, 0.5, "cy");
    near(c.z, 0.5, "cz");
}

// ─── Test 10: center of empty shape is None ───────────────────────────────────

#[test]
fn center_of_empty_shape_is_none() {
    let empty = TopoDS_Shape::Compound(TopoDS_Compound { id: 1, shapes: vec![] });
    assert!(BRepBndLib::center(&empty).is_none());
}

// ─── Test 11: contains_point detects interior vs exterior ────────────────────

#[test]
fn contains_point_inside_and_outside() {
    let s = translated_box_solid(0.0, 0.0, 0.0);
    // Interior point.
    assert!(
        BRepBndLib::contains_point(&s, Pnt::new(0.5, 0.5, 0.5)),
        "center should be inside bbox"
    );
    // Exterior point.
    assert!(
        !BRepBndLib::contains_point(&s, Pnt::new(10.0, 10.0, 10.0)),
        "far point should be outside bbox"
    );
}

// ─── Test 12: bounding_boxes_overlap ─────────────────────────────────────────

#[test]
fn bounding_boxes_overlap_and_not_overlap() {
    let s1 = translated_box_solid(0.0, 0.0, 0.0);
    let s2 = translated_box_solid(0.5, 0.5, 0.5); // overlaps s1
    let s3 = translated_box_solid(5.0, 5.0, 5.0); // does not overlap s1

    assert!(
        BRepBndLib::bounding_boxes_overlap(&s1, &s2),
        "overlapping solids must report overlap"
    );
    assert!(
        !BRepBndLib::bounding_boxes_overlap(&s1, &s3),
        "separated solids must not report overlap"
    );
}

// ─── Test 13: rectangular face bounding box ───────────────────────────────────

#[test]
fn rectangular_face_bounding_box() {
    let face = make_rect_face(1, -5.0, -3.0, 2.0, 5.0, 3.0);
    let bbox = BRepBndLib::face_bounding_box(&face);
    let l = bbox.get().expect("rect face must not give void box");
    near(l.xmin, -5.0, "xmin");
    near(l.xmax,  5.0, "xmax");
    near(l.ymin, -3.0, "ymin");
    near(l.ymax,  3.0, "ymax");
    near(l.zmin,  2.0, "zmin");
    near(l.zmax,  2.0, "zmax");
}

// ─── Test 14: add_with_gap inflates the bounding box ─────────────────────────

#[test]
fn add_with_gap_inflates_bbox() {
    let v = make_vertex(1, 0.0, 0.0, 0.0);
    let mut bbox = BndBox::new();
    BRepBndLib::add_with_gap(&v, &mut bbox, 1.0);
    let l = bbox.get().expect("gap-inflated box must not be void");
    assert!(l.xmin <= -1.0 + TOL, "xmin must be <= -1 after gap inflation");
    assert!(l.xmax >=  1.0 - TOL, "xmax must be >= +1 after gap inflation");
    assert!(l.ymin <= -1.0 + TOL, "ymin must be <= -1 after gap inflation");
    assert!(l.ymax >=  1.0 - TOL, "ymax must be >= +1 after gap inflation");
    assert!(l.zmin <= -1.0 + TOL, "zmin must be <= -1 after gap inflation");
    assert!(l.zmax >=  1.0 - TOL, "zmax must be >= +1 after gap inflation");
}

// ─── Test 15: add accumulates boxes across multiple calls ─────────────────────

#[test]
fn accumulate_bounding_boxes_with_successive_add_calls() {
    let mut bbox = BndBox::new();
    // Add four vertices at the corners of a 2×2×2 cube centered at origin.
    for &(x, y, z) in &[
        (-1.0, -1.0, -1.0),
        ( 1.0, -1.0, -1.0),
        (-1.0,  1.0,  1.0),
        ( 1.0,  1.0,  1.0),
    ] {
        let v = make_vertex(1, x, y, z);
        BRepBndLib::add(&v, &mut bbox);
    }
    let l = bbox.get().expect("accumulated box must not be void");
    near(l.xmin, -1.0, "xmin");
    near(l.xmax,  1.0, "xmax");
    near(l.ymin, -1.0, "ymin");
    near(l.ymax,  1.0, "ymax");
    near(l.zmin, -1.0, "zmin");
    near(l.zmax,  1.0, "zmax");
}
