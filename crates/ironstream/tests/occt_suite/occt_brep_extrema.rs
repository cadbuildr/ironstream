// FILE: tests/occt_brep_extrema.rs
//! Integration tests for `brep_extrema` — BRepExtrema_DistShapeShape,
//! BRepExtrema_SolutionElem, BRepExtrema_SupportType.
//!
//! All tests exercise the public API only.  Numeric tolerances follow
//! OCCT conventions; tessellation-derived answers are tested with relaxed
//! tolerances where appropriate.

extern crate ironstream;

use ironstream::brep_extrema::{
    make_edge, make_rect_face, make_vertex, BRepExtrema_DistShapeShape,
    BRepExtrema_SolutionElem, BRepExtrema_SupportType,
};
use ironstream::gp::Pnt;
use ironstream::top_exp::{
    TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shape, TopoDS_Shell,
    TopoDS_Solid, TopoDS_Vertex, TopoDS_Wire,
};

const TOL: f64 = 1e-9;

fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= TOL,
        "{label}: expected {a} ≈ {b}, diff = {}",
        (a - b).abs()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Build a unit box solid with all 8 corner vertices accessible via edges.
fn unit_box_solid() -> TopoDS_Shape {
    let corners: &[(f64, f64, f64)] = &[
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
        (0.0, 1.0, 0.0),
        (0.0, 0.0, 1.0),
        (1.0, 0.0, 1.0),
        (1.0, 1.0, 1.0),
        (0.0, 1.0, 1.0),
    ];
    let verts: Vec<TopoDS_Vertex> = corners
        .iter()
        .enumerate()
        .map(|(i, &(x, y, z))| TopoDS_Vertex {
            id: i as u64 + 1,
            x,
            y,
            z,
        })
        .collect();
    let edge = TopoDS_Edge {
        id: 1,
        vertices: verts,
    };
    let wire = TopoDS_Wire {
        id: 1,
        edges: vec![edge],
    };
    let face = TopoDS_Face {
        id: 1,
        wires: vec![wire],
    };
    let shell = TopoDS_Shell {
        id: 1,
        faces: vec![face],
    };
    let solid = TopoDS_Solid {
        id: 1,
        shells: vec![shell],
    };
    TopoDS_Shape::Solid(solid)
}

/// Build a translated unit box solid with corners from (tx,ty,tz) to
/// (tx+1, ty+1, tz+1).
fn translated_box(tx: f64, ty: f64, tz: f64) -> TopoDS_Shape {
    let corners: Vec<(f64, f64, f64)> = vec![
        (tx, ty, tz),
        (tx + 1.0, ty, tz),
        (tx + 1.0, ty + 1.0, tz),
        (tx, ty + 1.0, tz),
        (tx, ty, tz + 1.0),
        (tx + 1.0, ty, tz + 1.0),
        (tx + 1.0, ty + 1.0, tz + 1.0),
        (tx, ty + 1.0, tz + 1.0),
    ];
    let verts: Vec<TopoDS_Vertex> = corners
        .iter()
        .enumerate()
        .map(|(i, &(x, y, z))| TopoDS_Vertex {
            id: i as u64 + 1,
            x,
            y,
            z,
        })
        .collect();
    let edge = TopoDS_Edge {
        id: 1,
        vertices: verts,
    };
    let wire = TopoDS_Wire {
        id: 1,
        edges: vec![edge],
    };
    let face = TopoDS_Face {
        id: 1,
        wires: vec![wire],
    };
    let shell = TopoDS_Shell {
        id: 1,
        faces: vec![face],
    };
    let solid = TopoDS_Solid {
        id: 1,
        shells: vec![shell],
    };
    TopoDS_Shape::Solid(solid)
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1: distance between two separate vertices
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn two_vertex_distance_3d() {
    // Points at origin and (1, 2, 2): distance = sqrt(1+4+4) = 3.
    let v1 = make_vertex(1, 0.0, 0.0, 0.0);
    let v2 = make_vertex(2, 1.0, 2.0, 2.0);
    let solver = BRepExtrema_DistShapeShape::new(&v1, &v2);
    assert!(solver.is_done(), "solver should be done");
    near(solver.value(), 3.0, "3D vertex-vertex distance");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2: coincident vertices → distance 0, inner_solution true
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn coincident_vertices_inner_solution() {
    let v1 = make_vertex(1, 5.0, -3.0, 2.0);
    let v2 = make_vertex(2, 5.0, -3.0, 2.0);
    let solver = BRepExtrema_DistShapeShape::new(&v1, &v2);
    assert!(solver.is_done());
    near(solver.value(), 0.0, "coincident distance");
    assert!(solver.inner_solution(), "should report inner solution");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3: two adjacent unit boxes — closest faces share an edge, distance = 0
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn adjacent_boxes_touching_distance_zero() {
    let box1 = unit_box_solid();
    // Box 2 is shifted +1 on X so it shares the face x=1.
    let box2 = translated_box(1.0, 0.0, 0.0);
    let solver = BRepExtrema_DistShapeShape::new(&box1, &box2);
    assert!(solver.is_done());
    near(solver.value(), 0.0, "touching boxes distance");
    assert!(solver.inner_solution());
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4: two separated unit boxes on X — gap of 3 units
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn separated_boxes_gap_three() {
    let box1 = unit_box_solid(); // 0..1 on X
    let box2 = translated_box(4.0, 0.0, 0.0); // 4..5 on X
    let solver = BRepExtrema_DistShapeShape::new(&box1, &box2);
    assert!(solver.is_done());
    near(solver.value(), 3.0, "boxes separated by 3 on X");
    assert!(!solver.inner_solution());
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5: point vs edge — closest point in the interior of the edge
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn vertex_to_edge_interior_closest() {
    // Edge from (0,0,0) to (4,0,0); vertex at (2, 3, 0).
    // Closest point on edge is (2,0,0), distance = 3.
    let edge = make_edge(1, &[(0.0, 0.0, 0.0), (4.0, 0.0, 0.0)]);
    let v = make_vertex(2, 2.0, 3.0, 0.0);
    let solver = BRepExtrema_DistShapeShape::new(&edge, &v);
    assert!(solver.is_done());
    near(solver.value(), 3.0, "vertex-to-edge interior distance");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6: nb_solution ≥ 1 for non-trivial shapes
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn nb_solution_non_zero() {
    let box1 = unit_box_solid();
    let v = make_vertex(1, 5.0, 0.5, 0.5);
    let solver = BRepExtrema_DistShapeShape::new(&box1, &v);
    assert!(solver.is_done());
    assert!(
        solver.nb_solution() >= 1,
        "should have at least 1 solution, got {}",
        solver.nb_solution()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7: solution points lie on the correct shapes
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn solution_points_on_correct_shapes() {
    let v1 = make_vertex(1, 0.0, 0.0, 0.0);
    let v2 = make_vertex(2, 0.0, 0.0, 6.0);
    let solver = BRepExtrema_DistShapeShape::new(&v1, &v2);
    assert!(solver.is_done());

    let p1 = solver.point_on_shape1(0).expect("solution on shape 1");
    let p2 = solver.point_on_shape2(0).expect("solution on shape 2");

    near(p1.x, 0.0, "p1.x");
    near(p1.y, 0.0, "p1.y");
    near(p1.z, 0.0, "p1.z");

    near(p2.x, 0.0, "p2.x");
    near(p2.y, 0.0, "p2.y");
    near(p2.z, 6.0, "p2.z");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 8: SupportType IsVertex for pure vertex shapes
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn support_type_vertex_for_vertex_shapes() {
    let v1 = make_vertex(1, 1.0, 0.0, 0.0);
    let v2 = make_vertex(2, 4.0, 0.0, 0.0);
    let solver = BRepExtrema_DistShapeShape::new(&v1, &v2);
    assert!(solver.is_done());
    assert_eq!(
        solver.support_type_shape1(0),
        Some(BRepExtrema_SupportType::IsVertex),
        "shape 1 support should be IsVertex"
    );
    assert_eq!(
        solver.support_type_shape2(0),
        Some(BRepExtrema_SupportType::IsVertex),
        "shape 2 support should be IsVertex"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9: is_done() returns false for empty shapes
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn empty_shape_is_not_done() {
    let empty = TopoDS_Shape::Compound(TopoDS_Compound {
        id: 1,
        shapes: vec![],
    });
    let v = make_vertex(1, 0.0, 0.0, 0.0);
    let solver = BRepExtrema_DistShapeShape::new(&empty, &v);
    assert!(!solver.is_done(), "empty shape should give is_done=false");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10: SolutionElem constructors carry correct fields
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn solution_elem_constructors() {
    let pt = Pnt::new(1.0, 2.0, 3.0);

    let se_v = BRepExtrema_SolutionElem::vertex(pt, 42);
    assert_eq!(se_v.support_type, BRepExtrema_SupportType::IsVertex);
    near(se_v.point.x, 1.0, "vertex.x");
    near(se_v.point.y, 2.0, "vertex.y");
    near(se_v.point.z, 3.0, "vertex.z");
    assert_eq!(se_v.element_id, 42);
    near(se_v.param_u, 0.0, "vertex param_u");

    let se_e = BRepExtrema_SolutionElem::on_edge(pt, 7, 0.3);
    assert_eq!(se_e.support_type, BRepExtrema_SupportType::IsOnEdge);
    near(se_e.param_u, 0.3, "edge param_u");
    assert_eq!(se_e.element_id, 7);

    let se_f = BRepExtrema_SolutionElem::in_face(pt, 99, 0.25, 0.75);
    assert_eq!(se_f.support_type, BRepExtrema_SupportType::IsInFace);
    near(se_f.param_u, 0.25, "face param_u");
    near(se_f.param_v, 0.75, "face param_v");
    assert_eq!(se_f.element_id, 99);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 11: distance from compound to vertex picks the closest member
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn compound_vs_vertex_picks_closest() {
    // Compound with vertices at (1,0,0) and (10,0,0); reference at (0,0,0).
    // Closest should be (1,0,0) → distance 1.
    let compound = TopoDS_Shape::Compound(TopoDS_Compound {
        id: 10,
        shapes: vec![
            TopoDS_Shape::Vertex(TopoDS_Vertex { id: 1, x: 1.0, y: 0.0, z: 0.0 }),
            TopoDS_Shape::Vertex(TopoDS_Vertex { id: 2, x: 10.0, y: 0.0, z: 0.0 }),
        ],
    });
    let ref_v = make_vertex(3, 0.0, 0.0, 0.0);
    let solver = BRepExtrema_DistShapeShape::new(&compound, &ref_v);
    assert!(solver.is_done());
    near(solver.value(), 1.0, "closest member of compound");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 12: edge-to-edge closest points for skew (non-intersecting) segments
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn edge_to_edge_skew_segments() {
    // Segment 1: along X from (0,0,0) to (2,0,0).
    // Segment 2: along Y from (1,1,1) to (1,3,1).
    // Closest pair: (1,0,0) on seg1, (1,1,1) on seg2, distance = sqrt(0+1+1) = sqrt(2).
    let e1 = make_edge(1, &[(0.0, 0.0, 0.0), (2.0, 0.0, 0.0)]);
    let e2 = make_edge(2, &[(1.0, 1.0, 1.0), (1.0, 3.0, 1.0)]);
    let solver = BRepExtrema_DistShapeShape::new(&e1, &e2);
    assert!(solver.is_done());
    let expected = (2.0_f64).sqrt();
    assert!(
        (solver.value() - expected).abs() < 1e-9,
        "skew edge-edge distance: expected {expected}, got {}",
        solver.value()
    );
}
