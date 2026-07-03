// FILE: tests/occt_brep_class3d.rs
//! Integration tests for `BRepClass3d` — the solid point-in-solid classifier.
//!
//! These tests exercise [`BRepClass3d_SolidClassifier`] and
//! [`BRepClass3d_SolidExplorer`] through the public crate API.

extern crate ironstream;

use ironstream::brep_class3d::{
    classify, BRepClass3d_SolidClassifier, BRepClass3d_SolidExplorer,
};
use ironstream::gp::Pnt;
use ironstream::top_abs::State;
use ironstream::top_exp::{
    TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shape, TopoDS_Shell,
    TopoDS_Solid, TopoDS_Vertex, TopoDS_Wire, TopoDS_CompSolid,
};

// ─────────────────────────── helper builders ─────────────────────────────────

fn vtx(id: u64, x: f64, y: f64, z: f64) -> TopoDS_Vertex {
    TopoDS_Vertex { id, x, y, z }
}

/// Build a single triangular face.
fn tri_face(id: u64, p0: (f64, f64, f64), p1: (f64, f64, f64), p2: (f64, f64, f64)) -> TopoDS_Face {
    let base = id * 100;
    let v0 = vtx(base,     p0.0, p0.1, p0.2);
    let v1 = vtx(base + 1, p1.0, p1.1, p1.2);
    let v2 = vtx(base + 2, p2.0, p2.1, p2.2);
    let e0 = TopoDS_Edge { id: base + 10, vertices: vec![v0.clone(), v1.clone()] };
    let e1 = TopoDS_Edge { id: base + 11, vertices: vec![v1.clone(), v2.clone()] };
    let e2 = TopoDS_Edge { id: base + 12, vertices: vec![v2.clone(), v0.clone()] };
    let wire = TopoDS_Wire { id: base + 20, edges: vec![e0, e1, e2] };
    TopoDS_Face { id, wires: vec![wire] }
}

/// Build a closed unit-cube solid (12 triangular faces).
fn unit_cube() -> TopoDS_Solid {
    let mut faces = Vec::new();
    let mut id = 0u64;

    // bottom z=0
    faces.push(tri_face(id, (0.,0.,0.), (1.,0.,0.), (1.,1.,0.))); id += 1;
    faces.push(tri_face(id, (0.,0.,0.), (1.,1.,0.), (0.,1.,0.))); id += 1;
    // top z=1
    faces.push(tri_face(id, (0.,0.,1.), (1.,1.,1.), (1.,0.,1.))); id += 1;
    faces.push(tri_face(id, (0.,0.,1.), (0.,1.,1.), (1.,1.,1.))); id += 1;
    // front y=0
    faces.push(tri_face(id, (0.,0.,0.), (1.,0.,1.), (1.,0.,0.))); id += 1;
    faces.push(tri_face(id, (0.,0.,0.), (0.,0.,1.), (1.,0.,1.))); id += 1;
    // back y=1
    faces.push(tri_face(id, (0.,1.,0.), (1.,1.,0.), (1.,1.,1.))); id += 1;
    faces.push(tri_face(id, (0.,1.,0.), (1.,1.,1.), (0.,1.,1.))); id += 1;
    // left x=0
    faces.push(tri_face(id, (0.,0.,0.), (0.,1.,0.), (0.,1.,1.))); id += 1;
    faces.push(tri_face(id, (0.,0.,0.), (0.,1.,1.), (0.,0.,1.))); id += 1;
    // right x=1
    faces.push(tri_face(id, (1.,0.,0.), (1.,1.,1.), (1.,1.,0.))); id += 1;
    faces.push(tri_face(id, (1.,0.,0.), (1.,0.,1.), (1.,1.,1.)));

    let shell = TopoDS_Shell { id: 9000, faces };
    TopoDS_Solid { id: 9001, shells: vec![shell] }
}

/// Build a solid that is a scaled cube [0..2]^3.
fn scaled_cube_2() -> TopoDS_Solid {
    let mut faces = Vec::new();
    let mut id = 200u64;
    let s = 2.0_f64;

    faces.push(tri_face(id, (0.,0.,0.), (s,0.,0.), (s,s,0.))); id += 1;
    faces.push(tri_face(id, (0.,0.,0.), (s,s,0.), (0.,s,0.))); id += 1;
    faces.push(tri_face(id, (0.,0.,s), (s,s,s), (s,0.,s))); id += 1;
    faces.push(tri_face(id, (0.,0.,s), (0.,s,s), (s,s,s))); id += 1;
    faces.push(tri_face(id, (0.,0.,0.), (s,0.,s), (s,0.,0.))); id += 1;
    faces.push(tri_face(id, (0.,0.,0.), (0.,0.,s), (s,0.,s))); id += 1;
    faces.push(tri_face(id, (0.,s,0.), (s,s,0.), (s,s,s))); id += 1;
    faces.push(tri_face(id, (0.,s,0.), (s,s,s), (0.,s,s))); id += 1;
    faces.push(tri_face(id, (0.,0.,0.), (0.,s,0.), (0.,s,s))); id += 1;
    faces.push(tri_face(id, (0.,0.,0.), (0.,s,s), (0.,0.,s))); id += 1;
    faces.push(tri_face(id, (s,0.,0.), (s,s,s), (s,s,0.))); id += 1;
    faces.push(tri_face(id, (s,0.,0.), (s,0.,s), (s,s,s)));

    let shell = TopoDS_Shell { id: 8000, faces };
    TopoDS_Solid { id: 8001, shells: vec![shell] }
}

// ─────────────────────────── tests ───────────────────────────────────────────

/// 1. Centre of unit cube is classified IN.
#[test]
fn test_point_inside_unit_cube() {
    let shape = TopoDS_Shape::Solid(unit_cube());
    let mut clf = BRepClass3d_SolidClassifier::new();
    clf.load(&shape);
    clf.perform(Pnt::new(0.5, 0.5, 0.5), 1e-7);
    assert_eq!(clf.state(), State::IN, "centre should be IN");
}

/// 2. A point well outside the unit cube is classified OUT.
#[test]
fn test_point_outside_unit_cube() {
    let shape = TopoDS_Shape::Solid(unit_cube());
    let s = classify(&shape, Pnt::new(10.0, 10.0, 10.0), 1e-7);
    assert_eq!(s, State::OUT, "far point should be OUT");
}

/// 3. Negative coordinate point is outside the unit cube.
#[test]
fn test_point_negative_coords_outside() {
    let shape = TopoDS_Shape::Solid(unit_cube());
    let s = classify(&shape, Pnt::new(-1.0, 0.5, 0.5), 1e-7);
    assert_eq!(s, State::OUT);
}

/// 4. Multiple performs on the same classifier give independent results.
#[test]
fn test_multiple_performs_independent() {
    let shape = TopoDS_Shape::Solid(unit_cube());
    let mut clf = BRepClass3d_SolidClassifier::new();
    clf.load(&shape);

    clf.perform(Pnt::new(0.5, 0.5, 0.5), 1e-7);
    assert_eq!(clf.state(), State::IN);

    clf.perform(Pnt::new(5.0, 5.0, 5.0), 1e-7);
    assert_eq!(clf.state(), State::OUT);

    clf.perform(Pnt::new(0.3, 0.3, 0.3), 1e-7);
    assert_eq!(clf.state(), State::IN);
}

/// 5. An empty solid (no shells) yields OUT for any point.
#[test]
fn test_empty_solid_is_out() {
    let shape = TopoDS_Shape::Solid(TopoDS_Solid { id: 1, shells: vec![] });
    let s = classify(&shape, Pnt::new(0.0, 0.0, 0.0), 1e-7);
    assert_eq!(s, State::OUT);
}

/// 6. Explorer iterates the correct number of faces.
#[test]
fn test_explorer_face_count() {
    let shape = TopoDS_Shape::Solid(unit_cube());
    let exp = BRepClass3d_SolidExplorer::new(&shape);
    assert_eq!(exp.nb_faces(), 12, "unit cube should have 12 triangular faces");
}

/// 7. Explorer iterates all faces exactly once.
#[test]
fn test_explorer_iterates_all_faces() {
    let shape = TopoDS_Shape::Solid(unit_cube());
    let mut exp = BRepClass3d_SolidExplorer::new(&shape);
    let mut count = 0usize;
    while exp.more() {
        let _f = exp.current_face();
        count += 1;
        exp.next();
    }
    assert_eq!(count, 12);
}

/// 8. Explorer init() resets the cursor.
#[test]
fn test_explorer_init_resets() {
    let shape = TopoDS_Shape::Solid(unit_cube());
    let mut exp = BRepClass3d_SolidExplorer::new(&shape);
    // Exhaust the iterator.
    while exp.more() {
        exp.next();
    }
    assert!(!exp.more());
    exp.init();
    assert!(exp.more(), "after init() iterator should restart");
    // Should iterate all faces again.
    let mut count = 0usize;
    while exp.more() {
        count += 1;
        exp.next();
    }
    assert_eq!(count, 12);
}

/// 9. load_and_perform convenience method works correctly.
#[test]
fn test_load_and_perform_convenience() {
    let shape = TopoDS_Shape::Solid(unit_cube());
    let mut clf = BRepClass3d_SolidClassifier::new();
    let s = clf.load_and_perform(&shape, Pnt::new(0.5, 0.5, 0.5), 1e-7);
    assert_eq!(s, State::IN);
    assert!(clf.is_done());
}

/// 10. Points inside a [0..2]^3 cube (but not the unit cube) are classified
///     correctly against each respective solid.
#[test]
fn test_two_different_solids() {
    let small = TopoDS_Shape::Solid(unit_cube());
    let large = TopoDS_Shape::Solid(scaled_cube_2());

    // (1.5, 1.5, 1.5) — outside the unit cube, inside the 2×2×2 cube.
    let p = Pnt::new(1.5, 1.5, 1.5);
    let s_small = classify(&small, p, 1e-7);
    let s_large = classify(&large, p, 1e-7);
    assert_eq!(s_small, State::OUT, "point (1.5,1.5,1.5) should be outside unit cube");
    assert_eq!(s_large, State::IN,  "point (1.5,1.5,1.5) should be inside 2x cube");
}
