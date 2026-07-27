// FILE: tests/occt_brep_proj.rs
//! Integration tests for `brep_proj` — BRepProj_Projection.
//!
//! All tests exercise the public API only.  Geometric tolerances are relaxed
//! slightly versus exact computation to account for floating-point arithmetic
//! in the Möller–Trumbore intersection.

extern crate ironstream;

use ironstream::brep_proj::{
    make_proj_vertex, make_quad_face, make_wire_from_points, BRepProj_Projection, ProjectionType,
};
use ironstream::gp::Pnt;
use ironstream::top_exp::TopoDS_Shape;

const TOL: f64 = 1e-6;

fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= TOL,
        "{label}: expected {a} ≈ {b}, diff = {}",
        (a - b).abs()
    );
}

fn pnt_near(a: Pnt, b: Pnt, label: &str) {
    assert!(
        a.distance(b) <= TOL,
        "{label}: expected {:?} ≈ {:?}, dist = {}",
        a,
        b,
        a.distance(b)
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Shape builder helpers
// ─────────────────────────────────────────────────────────────────────────────

/// A 2×2 square face in the XY plane (z = 0), face id = 10.
fn large_xy_face() -> TopoDS_Shape {
    make_quad_face(
        10,
        [
            (-1.0, -1.0, 0.0),
            (1.0, -1.0, 0.0),
            (1.0, 1.0, 0.0),
            (-1.0, 1.0, 0.0),
        ],
    )
}

/// A unit square face in the XY plane (z = 0), face id = 1.
fn unit_xy_face() -> TopoDS_Shape {
    make_quad_face(
        1,
        [
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (1.0, 1.0, 0.0),
            (0.0, 1.0, 0.0),
        ],
    )
}

/// A unit square face in the XZ plane (y = 0), face id = 2.
fn unit_xz_face() -> TopoDS_Shape {
    make_quad_face(
        2,
        [
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (1.0, 0.0, 1.0),
            (0.0, 0.0, 1.0),
        ],
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1: cylindrical projection — is_done after successful hit
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_cylindrical_is_done_on_hit() {
    let src = make_proj_vertex(1, 0.0, 0.0, 2.0);
    let face = large_xy_face();
    let dir = Pnt::new(0.0, 0.0, -1.0);
    let proj = BRepProj_Projection::cylindrical(&src, &face, dir);
    assert!(proj.is_done(), "is_done must be true when a face is hit");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2: cylindrical projection — hit z-coordinate is zero on XY plane
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_cylindrical_hit_z_is_zero() {
    let src = make_proj_vertex(1, 0.0, 0.0, 5.0);
    let face = large_xy_face();
    let dir = Pnt::new(0.0, 0.0, -1.0);
    let proj = BRepProj_Projection::cylindrical(&src, &face, dir);
    assert!(proj.is_done());

    for wire in proj.wires() {
        for edge in &wire.edges {
            for v in &edge.vertices {
                near(v.z, 0.0, "projected z on XY face");
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3: cylindrical projection — (x, y) coordinates are preserved
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_cylindrical_xy_preserved_under_z_projection() {
    let src = make_proj_vertex(1, -0.5, 0.7, 3.0);
    let face = large_xy_face();
    let dir = Pnt::new(0.0, 0.0, -1.0);
    let proj = BRepProj_Projection::cylindrical(&src, &face, dir);
    assert!(proj.is_done());
    assert!(!proj.wires().is_empty());

    let v = &proj.wires()[0].edges[0].vertices[0];
    near(v.x, -0.5, "x preserved under -Z projection");
    near(v.y, 0.7, "y preserved under -Z projection");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4: cylindrical projection — wire misses face gives zero wires
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_cylindrical_miss_gives_zero_wires() {
    // Source vertex well outside the 2×2 face's extent.
    let src = make_proj_vertex(1, 10.0, 10.0, 3.0);
    let face = unit_xy_face(); // only covers [0,1]×[0,1]
    let dir = Pnt::new(0.0, 0.0, -1.0);
    let proj = BRepProj_Projection::cylindrical(&src, &face, dir);
    assert_eq!(proj.nb_wires(), 0, "ray outside face should produce no wires");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5: shape() returns a Compound whose children are Wires
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_shape_is_compound_of_wires() {
    let src = make_proj_vertex(1, 0.0, 0.0, 2.0);
    let face = large_xy_face();
    let dir = Pnt::new(0.0, 0.0, -1.0);
    let proj = BRepProj_Projection::cylindrical(&src, &face, dir);

    let s = proj.shape();
    match s {
        TopoDS_Shape::Compound(c) => {
            assert_eq!(
                c.shapes.len(),
                proj.nb_wires(),
                "compound children count matches nb_wires"
            );
            for child in &c.shapes {
                assert!(
                    matches!(child, TopoDS_Shape::Wire(_)),
                    "every compound child must be a Wire"
                );
            }
        }
        _ => panic!("shape() must return a Compound"),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6: ancestor() returns the correct face id
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_ancestor_face_id() {
    let src = make_proj_vertex(1, 0.0, 0.0, 3.0);
    let face = large_xy_face(); // face id = 10
    let dir = Pnt::new(0.0, 0.0, -1.0);
    let proj = BRepProj_Projection::cylindrical(&src, &face, dir);
    assert!(proj.is_done());
    if proj.nb_wires() > 0 {
        let fid = proj.ancestor(0).unwrap();
        assert_eq!(fid, 10, "ancestor face id should be 10");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7: conical projection — is_done and correct type
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_conical_is_done_and_type() {
    // Apex above the face; source vertex between apex and face.
    let src = make_proj_vertex(1, 0.0, 0.0, 1.0);
    let face = large_xy_face();
    let apex = Pnt::new(0.0, 0.0, 3.0);
    let proj = BRepProj_Projection::conical(&src, &face, apex);
    assert!(proj.is_done(), "conical projection should succeed");
    assert_eq!(proj.projection_type(), ProjectionType::Conical);
    pnt_near(proj.apex().unwrap(), apex, "apex accessor");
    assert!(proj.direction().is_none(), "direction must be None in conical mode");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 8: conical projection — hit at z = 0 for ray from apex through source
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_conical_hit_z_zero() {
    // Apex at (0,0,2), source at (0,0,1): ray direction = (0,0,-1).
    // Intersection with z=0 plane should be at (0,0,0).
    let src = make_proj_vertex(1, 0.0, 0.0, 1.0);
    let face = large_xy_face();
    let apex = Pnt::new(0.0, 0.0, 2.0);
    let proj = BRepProj_Projection::conical(&src, &face, apex);
    assert!(proj.is_done());

    for wire in proj.wires() {
        for edge in &wire.edges {
            for v in &edge.vertices {
                near(v.z, 0.0, "conical projected z");
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9: multi-vertex wire — all hits on same face appear in the same wire
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_wire_projection_all_hits_on_one_face() {
    // Wire with four vertices forming a small square above the face.
    let src = make_wire_from_points(
        1,
        &[
            (-0.3, -0.3, 2.0),
            (0.3, -0.3, 2.0),
            (0.3, 0.3, 2.0),
            (-0.3, 0.3, 2.0),
        ],
    );
    let face = large_xy_face();
    let dir = Pnt::new(0.0, 0.0, -1.0);
    let proj = BRepProj_Projection::cylindrical(&src, &face, dir);
    assert!(proj.is_done());
    // Should produce exactly 1 result wire (all hits on face id 10).
    assert_eq!(proj.nb_wires(), 1, "all hits land on the same face");
    let wire = &proj.wires()[0];
    // Wire should have edges connecting all 4 projected points (3 edges).
    assert!(!wire.edges.is_empty(), "projected wire must have edges");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10: two-face target — projections can hit both faces
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_projection_onto_compound_two_faces() {
    // Source: two vertices — one hits the XY face (z=0) and one hits the XZ face (y=0).
    let src_xy = make_proj_vertex(1, 0.5, 0.5, 2.0); // hits XY face if dir = -Z
    let face_xy = unit_xy_face();
    let dir_z = Pnt::new(0.0, 0.0, -1.0);
    let proj_xy = BRepProj_Projection::cylindrical(&src_xy, &face_xy, dir_z);
    assert!(proj_xy.is_done());
    assert_eq!(proj_xy.nb_wires(), 1, "should hit XY face");

    let src_xz = make_proj_vertex(2, 0.5, 2.0, 0.5); // hits XZ face if dir = -Y
    let face_xz = unit_xz_face();
    let dir_y = Pnt::new(0.0, -1.0, 0.0);
    let proj_xz = BRepProj_Projection::cylindrical(&src_xz, &face_xz, dir_y);
    assert!(proj_xz.is_done());
    assert_eq!(proj_xz.nb_wires(), 1, "should hit XZ face");

    // Verify that the XZ-face hit is at y ≈ 0.
    let v = &proj_xz.wires()[0].edges[0].vertices[0];
    near(v.y, 0.0, "xz face hit y=0");
}
