// FILE: tests/occt_int_curves_face.rs
//! Integration tests for `int_curves_face` —
//! IntCurvesFace_ShapeIntersector and IntCurvesFace_Intersector.
//!
//! All tests exercise the public API only.  Shapes are built using the
//! module-level helper constructors; tolerances follow OCCT conventions.

extern crate ironstream;

use ironstream::gp::Pnt;
use ironstream::int_curves_face::{
    make_circle_face, make_plane_face, IntCurvesFace_Intersector,
    IntCurvesFace_ShapeIntersector, IntersectionPoint,
};
use ironstream::top_exp::{
    TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shape, TopoDS_Shell, TopoDS_Solid,
    TopoDS_Vertex, TopoDS_Wire,
};

const TOL: f64 = 1e-9;
const RTOL: f64 = 1e-6;

fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= TOL,
        "{label}: expected {a} ≈ {b}, diff = {}",
        (a - b).abs()
    );
}

fn near_r(a: f64, b: f64, eps: f64, label: &str) {
    assert!(
        (a - b).abs() <= eps,
        "{label}: expected {a} ≈ {b}, diff = {}",
        (a - b).abs()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1: Vertical ray through the centre of a horizontal plane face.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn vertical_ray_hits_centre_of_plane_face() {
    // Plane face: x ∈ [−3, 3], y ∈ [−3, 3], z = 5.
    let mut inter = IntCurvesFace_ShapeIntersector::new();
    let face = make_plane_face(1, -3.0, 3.0, -3.0, 3.0, 5.0);
    inter.load(&TopoDS_Shape::Face(face), 1e-9);
    inter.perform_line(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        f64::NEG_INFINITY,
        f64::INFINITY,
    );

    assert!(inter.is_done(), "computation must succeed");
    assert_eq!(inter.nb_pnt(), 1, "one intersection expected");
    let pt = inter.point(0).expect("point 0 must exist");
    near(pt.w(), 5.0, "curve parameter t");
    near(pt.pnt().z, 5.0, "hit z");
    near(pt.pnt().x, 0.0, "hit x");
    near(pt.pnt().y, 0.0, "hit y");
    assert!(pt.is_entry(), "ray travelling +Z enters from below");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2: Diagonal ray hits an inclined (non-Z) face.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn diagonal_ray_hits_plane_face() {
    // Face in the x=2 plane (i.e. a face whose vertices all have x=2, spanning
    // y ∈ [−2, 2], z ∈ [−2, 2]).  Build manually.
    let verts = vec![
        TopoDS_Vertex { id: 11, x: 2.0, y: -2.0, z: -2.0 },
        TopoDS_Vertex { id: 12, x: 2.0, y:  2.0, z: -2.0 },
        TopoDS_Vertex { id: 13, x: 2.0, y:  2.0, z:  2.0 },
        TopoDS_Vertex { id: 14, x: 2.0, y: -2.0, z:  2.0 },
    ];
    let face = TopoDS_Face {
        id: 2,
        wires: vec![TopoDS_Wire {
            id: 2,
            edges: vec![TopoDS_Edge { id: 2, vertices: verts }],
        }],
    };

    let mut inter = IntCurvesFace_ShapeIntersector::new();
    inter.load(&TopoDS_Shape::Face(face), 1e-9);
    // Ray from (0,0,0) along (+1,0,0): hits x=2 at t=2.
    inter.perform_line(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        f64::NEG_INFINITY,
        f64::INFINITY,
    );

    assert!(inter.is_done());
    assert_eq!(inter.nb_pnt(), 1, "ray should hit x=2 face once");
    near(inter.w_parameter(0).unwrap(), 2.0, "t at x=2");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3: Ray misses — passes outside face boundary.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn ray_misses_face_outside_boundary() {
    // Face x ∈ [0, 1], y ∈ [0, 1], z = 0.
    let face = make_plane_face(3, 0.0, 1.0, 0.0, 1.0, 0.0);
    let mut inter = IntCurvesFace_ShapeIntersector::new();
    inter.load(&TopoDS_Shape::Face(face), 1e-9);
    // Ray through (5, 5) in XY — outside [0,1]×[0,1].
    inter.perform_line(
        Pnt::new(5.0, 5.0, 1.0),
        Pnt::new(0.0, 0.0, -1.0),
        f64::NEG_INFINITY,
        f64::INFINITY,
    );
    assert!(inter.is_done());
    assert_eq!(inter.nb_pnt(), 0, "ray outside boundary must miss");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4: Ray from outside face t-range misses (t_max clamp).
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn ray_clipped_by_t_range() {
    // Face at z = 100.
    let face = make_plane_face(4, -10.0, 10.0, -10.0, 10.0, 100.0);
    let mut inter = IntCurvesFace_ShapeIntersector::new();
    inter.load(&TopoDS_Shape::Face(face), 1e-9);
    // Limit t to [0, 50]: the face is at t=100, so it should be excluded.
    inter.perform_line(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        0.0,
        50.0,
    );
    assert!(inter.is_done());
    assert_eq!(inter.nb_pnt(), 0, "face at t=100 outside [0,50]");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5: Solid with top and bottom faces — two intersections, sorted by t.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn solid_top_and_bottom_faces_two_hits() {
    let bottom = make_plane_face(1, -4.0, 4.0, -4.0, 4.0, 0.0);
    let top    = make_plane_face(2, -4.0, 4.0, -4.0, 4.0, 6.0);
    let shape  = TopoDS_Shape::Solid(TopoDS_Solid {
        id: 1,
        shells: vec![TopoDS_Shell {
            id: 1,
            faces: vec![bottom, top],
        }],
    });

    let mut inter = IntCurvesFace_ShapeIntersector::new();
    inter.load(&shape, 1e-9);
    inter.perform_line(
        Pnt::new(0.0, 0.0, -2.0),
        Pnt::new(0.0, 0.0,  1.0),
        f64::NEG_INFINITY,
        f64::INFINITY,
    );

    assert!(inter.is_done());
    assert_eq!(inter.nb_pnt(), 2, "should hit bottom then top face");
    // Bottom face at z=0: t = 0 − (−2) = 2.
    near(inter.w_parameter(0).unwrap(), 2.0, "bottom face t=2");
    // Top face at z=6: t = 6 − (−2) = 8.
    near(inter.w_parameter(1).unwrap(), 8.0, "top face t=8");
    // Both faces have a +Z normal; a +Z ray satisfies denom > 0 for both,
    // so both register as entry transitions in our convention.
    assert!(inter.is_entry(0), "bottom = entry");
    assert!(inter.is_entry(1), "top = entry (same normal orientation)");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6: Compound of two separate faces — ray hits both.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn compound_two_faces_both_hit() {
    let f1 = make_plane_face(1, -1.0, 1.0, -1.0, 1.0, 1.0);
    let f2 = make_plane_face(2, -1.0, 1.0, -1.0, 1.0, 3.0);
    let shape = TopoDS_Shape::Compound(TopoDS_Compound {
        id: 10,
        shapes: vec![TopoDS_Shape::Face(f1), TopoDS_Shape::Face(f2)],
    });

    let mut inter = IntCurvesFace_ShapeIntersector::new();
    inter.load(&shape, 1e-9);
    inter.perform_line(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        f64::NEG_INFINITY,
        f64::INFINITY,
    );

    assert!(inter.is_done());
    assert_eq!(inter.nb_pnt(), 2, "compound: ray hits both faces");
    near(inter.w_parameter(0).unwrap(), 1.0, "first face t=1");
    near(inter.w_parameter(1).unwrap(), 3.0, "second face t=3");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7: has_shape() and nb_faces() accessors.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn shape_intersector_metadata_accessors() {
    let mut inter = IntCurvesFace_ShapeIntersector::new();
    assert!(!inter.has_shape(), "no shape yet");
    assert_eq!(inter.nb_faces(), 0);

    let face = make_plane_face(7, -1.0, 1.0, -1.0, 1.0, 0.0);
    inter.load(&TopoDS_Shape::Face(face), 1e-9);
    assert!(inter.has_shape(), "shape loaded");
    assert_eq!(inter.nb_faces(), 1, "one face loaded");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 8: IntCurvesFace_Intersector tolerance accessor.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn intersector_tolerance_accessor() {
    let face = make_plane_face(8, -1.0, 1.0, -1.0, 1.0, 0.0);
    let tol = 1.23e-5;
    let inter = IntCurvesFace_Intersector::new(&face, tol);
    near_r(inter.tolerance(), tol, 1e-15, "tolerance roundtrip");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9: perform_line on a face with no wires (degenerate) — no crash,
//         is_done() true, zero intersection points.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn degenerate_face_no_crash() {
    let face = TopoDS_Face { id: 9, wires: vec![] };
    let mut inter = IntCurvesFace_ShapeIntersector::new();
    inter.load(&TopoDS_Shape::Face(face), 1e-9);
    inter.perform_line(
        Pnt::new(0.0, 0.0, -1.0),
        Pnt::new(0.0, 0.0,  1.0),
        f64::NEG_INFINITY,
        f64::INFINITY,
    );
    assert!(inter.is_done(), "degenerate face: should still be done");
    // A face with < 3 vertices is treated as a trivial plane at origin;
    // whether or not it intersects depends on polygon test — just assert no
    // panic and that the count is ≥ 0 (trivially true for usize).
    let _ = inter.nb_pnt();
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10: IntersectionPoint fields accessible via named accessors.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn intersection_point_accessors() {
    let pt = Pnt::new(1.0, 2.0, 3.0);
    let ip = IntersectionPoint::new(pt, 7.5, 0.25, -0.5, true);

    near(ip.pnt().x, 1.0, "pnt.x");
    near(ip.pnt().y, 2.0, "pnt.y");
    near(ip.pnt().z, 3.0, "pnt.z");
    near(ip.w(),     7.5, "w");
    near(ip.u(),    0.25, "u");
    near(ip.v(),   -0.5,  "v");
    assert!(ip.is_entry(), "is_entry flag");
}
