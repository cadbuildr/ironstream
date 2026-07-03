// FILE: tests/occt_brep_lib.rs
//! Integration tests for `brep_lib` — BRepLib utility module.
//!
//! All tests exercise the public API only (no access to private fields).
//! Tolerances mirror OCCT test suite conventions.

extern crate ironstream;

use ironstream::brep_lib::{
    BRepLib, BRepLibError, BRepLibMakeEdge, BRepLibMakeFace, BRepLibMakeWire, MakeEdge,
    MakeFace, MakeWire,
};
use ironstream::gp::{Ax1, Ax3, Pnt};
use ironstream::gp_prim::{Circ, Lin};
use ironstream::topods::Wire;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

const TOL: f64 = 1e-9;

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} within {tol}, diff={}",
        (a - b).abs()
    );
}

// ─────────────────────────────── BRepLib utilities ───────────────────────────

#[test]
fn brep_lib_tolerance_constants_reasonable() {
    // The linear tolerance should be much smaller than typical geometry scales
    // and larger than machine epsilon.
    assert!(
        BRepLib::TOLERANCE > 0.0,
        "tolerance must be positive"
    );
    assert!(
        BRepLib::TOLERANCE < 1.0,
        "tolerance must be less than 1"
    );
    assert!(
        BRepLib::ANGULAR_TOLERANCE > 0.0,
        "angular tolerance must be positive"
    );
    assert!(
        BRepLib::ANGULAR_TOLERANCE < BRepLib::TOLERANCE,
        "angular tolerance should be tighter than linear"
    );
}

#[test]
fn brep_lib_wire_is_planar_triangle_in_xy() {
    let w = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
        Pnt::new(0.0, 4.0, 0.0),
    ]);
    let n = BRepLib::wire_is_planar(&w, 1e-7);
    assert!(n.is_some(), "triangle in XY must be planar");
    let n = n.unwrap();
    // Normal should be +Z or -Z.
    assert!(
        n.z.abs() > 0.9,
        "normal of XY triangle must be aligned with Z, got {:?}",
        n
    );
}

#[test]
fn brep_lib_wire_is_planar_tilted_square() {
    // A square in the XZ plane (y=0).
    let w = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 1.0),
        Pnt::new(0.0, 0.0, 1.0),
    ]);
    let n = BRepLib::wire_is_planar(&w, 1e-7);
    assert!(n.is_some(), "XZ square must be planar");
    // Normal should be along Y.
    let n = n.unwrap();
    assert!(
        n.y.abs() > 0.9,
        "normal of XZ square must be along Y, got {:?}",
        n
    );
}

#[test]
fn brep_lib_wire_area_3_4_rectangle() {
    // A 3×4 rectangle in the XY plane should have area 12.
    let w = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
        Pnt::new(3.0, 4.0, 0.0),
        Pnt::new(0.0, 4.0, 0.0),
    ]);
    let area = BRepLib::wire_area(&w).abs();
    near(area, 12.0, 1e-9);
}

#[test]
fn brep_lib_orient_wire_ccw_respects_normal() {
    // This wire winds clockwise around +Z.
    let cw = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
    ]);
    let desired_normal = Pnt::new(0.0, 0.0, 1.0);
    let oriented = BRepLib::orient_wire_ccw(&cw, desired_normal);
    let area = BRepLib::wire_area(&oriented);
    assert!(
        area > 0.0,
        "oriented wire must have positive (CCW) area, got {area}"
    );
}

// ─────────────────────────────── BRepLib_MakeEdge ────────────────────────────

#[test]
fn make_edge_two_points_endpoints_correct() {
    let p1 = Pnt::new(1.0, 2.0, 3.0);
    let p2 = Pnt::new(4.0, 6.0, 3.0);
    let maker = BRepLibMakeEdge::from_two_points(p1, p2);
    assert!(maker.is_done(), "edge from two distinct points must succeed");
    let w = maker.wire();
    assert_eq!(w.pts.len(), 2, "line segment should have exactly 2 sample pts");
    near(w.pts[0].distance(p1), 0.0, TOL);
    near(w.pts[1].distance(p2), 0.0, TOL);
}

#[test]
fn make_edge_coincident_points_returns_error() {
    let p = Pnt::new(5.0, 5.0, 5.0);
    let maker = BRepLibMakeEdge::from_two_points(p, p);
    assert!(
        !maker.is_done(),
        "coincident points must cause failure"
    );
    assert_eq!(
        maker.result().unwrap_err(),
        BRepLibError::CoincidentPoints,
        "error must be CoincidentPoints"
    );
}

#[test]
fn make_edge_from_lin_params_correct_endpoints() {
    // Line along +X, trimmed to [2, 7].
    let lin = Lin::new_pd(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let maker = BRepLibMakeEdge::from_lin_params(lin, 2.0, 7.0);
    assert!(maker.is_done());
    let w = maker.wire();
    near(w.pts[0].x, 2.0, TOL);
    near(w.pts.last().unwrap().x, 7.0, TOL);
    // All points must lie on the X axis.
    for p in &w.pts {
        near(p.y, 0.0, TOL);
        near(p.z, 0.0, TOL);
    }
}

#[test]
fn make_edge_full_circle_all_points_on_circle() {
    // Full circle of radius 3 in the XY plane.
    let ax3 = Ax3::identity();
    let circ = Circ::new(ax3, 3.0);
    let maker = BRepLibMakeEdge::from_circ(circ);
    assert!(maker.is_done(), "full circle edge must succeed");
    for p in &maker.wire().pts {
        near(
            p.distance(Pnt::origin()),
            3.0,
            1e-10,
        );
        near(p.z, 0.0, 1e-12);
    }
}

#[test]
fn make_edge_quarter_circle_arc_endpoints() {
    // Quarter arc of radius 1 from 0 to π/2.
    let ax3 = Ax3::identity();
    let circ = Circ::new(ax3, 1.0);
    let maker = BRepLibMakeEdge::from_circ_params(circ, 0.0, FRAC_PI_2);
    assert!(maker.is_done());
    let w = maker.wire();
    let start = w.pts[0];
    let end = *w.pts.last().unwrap();
    near(start.distance(Pnt::new(1.0, 0.0, 0.0)), 0.0, 1e-10);
    near(end.distance(Pnt::new(0.0, 1.0, 0.0)), 0.0, 1e-10);
}

#[test]
fn make_edge_vertices_returns_start_end() {
    let p1 = Pnt::new(0.0, 0.0, 0.0);
    let p2 = Pnt::new(5.0, 0.0, 0.0);
    let maker = BRepLibMakeEdge::from_two_points(p1, p2);
    let (v_start, v_end) = maker.vertices().expect("must have vertices");
    near(v_start.0.distance(p1), 0.0, TOL);
    near(v_end.0.distance(p2), 0.0, TOL);
}

// ─────────────────────────────── BRepLib_MakeWire ────────────────────────────

#[test]
fn make_wire_square_from_four_edges() {
    // Build a unit square wire from four edges.
    let corners = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ];
    let edges: Vec<Wire> = (0..4)
        .map(|i| {
            BRepLibMakeEdge::from_two_points(corners[i], corners[(i + 1) % 4])
                .into_wire()
                .unwrap()
        })
        .collect();

    let mut builder = BRepLibMakeWire::new();
    builder.add_edges(&edges);
    let wire = builder.wire().expect("square wire must build");
    assert!(wire.is_valid(), "square wire must be valid (>=3 pts)");
    // The wire should be planar in Z=0.
    let n = BRepLib::wire_is_planar(&wire, 1e-7);
    assert!(n.is_some(), "square wire must be planar");
}

#[test]
fn make_wire_chain_edges_head_to_tail() {
    // Three-edge chain with explicit head-to-tail connectivity.
    let e1 = BRepLibMakeEdge::from_two_points(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    )
    .into_wire()
    .unwrap();
    let e2 = BRepLibMakeEdge::from_two_points(
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(2.0, 3.0, 0.0),
    )
    .into_wire()
    .unwrap();
    let e3 = BRepLibMakeEdge::from_two_points(
        Pnt::new(2.0, 3.0, 0.0),
        Pnt::new(0.0, 0.0, 0.0),
    )
    .into_wire()
    .unwrap();

    let mut builder = BRepLibMakeWire::new();
    builder.add_edges(&[e1, e2, e3]);
    assert!(builder.is_done(), "3-edge triangle wire must be done");
    let wire = builder.wire().unwrap();
    assert!(wire.is_valid());
}

#[test]
fn make_wire_disconnected_edges_produce_error() {
    let e1 = BRepLibMakeEdge::from_two_points(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
    )
    .into_wire()
    .unwrap();
    // e2 starts very far from where e1 ends.
    let e2 = BRepLibMakeEdge::from_two_points(
        Pnt::new(1000.0, 0.0, 0.0),
        Pnt::new(1001.0, 0.0, 0.0),
    )
    .into_wire()
    .unwrap();

    let mut builder = BRepLibMakeWire::new();
    builder.add_edge(&e1);
    builder.add_edge(&e2);
    assert!(
        !builder.is_done(),
        "disconnected edges must prevent is_done"
    );
    assert!(
        builder.wire().is_err(),
        "wire() must return Err for disconnected edges"
    );
}

// ─────────────────────────────── BRepLib_MakeFace ────────────────────────────

#[test]
fn make_face_from_triangle_wire_is_valid() {
    let wire = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
        Pnt::new(1.5, 2.0, 0.0),
    ]);
    let mf = BRepLibMakeFace::from_wire(wire);
    assert!(mf.is_done(), "triangle face must be done");
    assert!(
        BRepLib::face_is_valid(mf.face(), 1e-7),
        "triangle face must be valid"
    );
}

#[test]
fn make_face_plane_bounds_correct_corners() {
    // A 10×20 rectangle in the XY plane at Z=5.
    let ax3 = Ax3::from_origin_normal(
        Pnt::new(0.0, 0.0, 5.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let mf = BRepLibMakeFace::from_plane_bounds(ax3, 0.0, 10.0, 0.0, 20.0);
    assert!(mf.is_done(), "plane-bounds face must be done");
    let face = mf.face();
    // All corner points must have z=5.
    for p in &face.outer.pts {
        near(p.z, 5.0, TOL);
    }
    // x range [0,10], y range [0,20].
    let xs: Vec<f64> = face.outer.pts.iter().map(|p| p.x).collect();
    let ys: Vec<f64> = face.outer.pts.iter().map(|p| p.y).collect();
    assert!(
        xs.iter().all(|&x| x >= -1e-9 && x <= 10.0 + 1e-9),
        "x out of [0,10]: {:?}",
        xs
    );
    assert!(
        ys.iter().all(|&y| y >= -1e-9 && y <= 20.0 + 1e-9),
        "y out of [0,20]: {:?}",
        ys
    );
}

#[test]
fn make_face_from_circle_boundary_at_radius() {
    let ax3 = Ax3::identity();
    let circ = Circ::new(ax3, 7.0);
    let mf = BRepLibMakeFace::from_circle(circ);
    assert!(mf.is_done(), "circle face must succeed");
    for p in &mf.face().outer.pts {
        near(p.distance(Pnt::origin()), 7.0, 1e-10);
    }
}

#[test]
fn make_face_non_planar_wire_returns_error() {
    // A wire that is not planar (one point is lifted off the XY plane).
    let wire = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.5), // not in plane
        Pnt::new(0.0, 1.0, 0.0),
    ]);
    let mf = BRepLibMakeFace::from_wire_tol(wire, 1e-7);
    assert!(!mf.is_done(), "non-planar wire must fail");
    assert_eq!(
        mf.into_face().unwrap_err(),
        BRepLibError::NonPlanarWire,
        "error must be NonPlanarWire"
    );
}

#[test]
fn make_face_with_hole_produces_one_hole() {
    let outer = Wire::new(vec![
        Pnt::new(-5.0, -5.0, 0.0),
        Pnt::new(5.0, -5.0, 0.0),
        Pnt::new(5.0, 5.0, 0.0),
        Pnt::new(-5.0, 5.0, 0.0),
    ]);
    let hole = Wire::new(vec![
        Pnt::new(-1.0, -1.0, 0.0),
        Pnt::new(1.0, -1.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(-1.0, 1.0, 0.0),
    ]);
    let mf = BRepLibMakeFace::from_wire_with_holes(outer, vec![hole]);
    assert!(mf.is_done(), "face-with-hole must succeed");
    assert_eq!(
        mf.face().holes.len(),
        1,
        "face must have exactly one hole"
    );
}

#[test]
fn make_face_add_hole_builder_style() {
    let outer = Wire::new(vec![
        Pnt::new(-3.0, -3.0, 2.0),
        Pnt::new(3.0, -3.0, 2.0),
        Pnt::new(3.0, 3.0, 2.0),
        Pnt::new(-3.0, 3.0, 2.0),
    ]);
    let hole = Wire::new(vec![
        Pnt::new(-0.5, -0.5, 2.0),
        Pnt::new(0.5, -0.5, 2.0),
        Pnt::new(0.5, 0.5, 2.0),
        Pnt::new(-0.5, 0.5, 2.0),
    ]);
    let mf = BRepLibMakeFace::from_wire(outer).add_hole(hole);
    assert!(mf.is_done());
    assert_eq!(mf.face().holes.len(), 1);
}

#[test]
fn make_face_normal_matches_expected() {
    // A CCW square in the XY plane should have normal pointing toward +Z.
    let wire = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ]);
    let mf = BRepLibMakeFace::from_wire(wire);
    assert!(mf.is_done());
    let n = mf.normal().unwrap();
    // Normal should be near ±Z.
    assert!(
        n.z.abs() > 0.9,
        "XY face normal must be along Z, got {:?}",
        n
    );
}

#[test]
fn make_face_triangulate_covers_area() {
    // A 2×3 rectangle should triangulate into a non-empty mesh with area ≈ 6.
    let wire = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(2.0, 3.0, 0.0),
        Pnt::new(0.0, 3.0, 0.0),
    ]);
    let mf = BRepLibMakeFace::from_wire(wire);
    assert!(mf.is_done());
    let tris = mf.triangulate().unwrap();
    assert!(!tris.is_empty(), "triangulated face must have triangles");
    // Compute total triangle area.
    let total: f64 = tris
        .iter()
        .map(|t| {
            let ab = t[1] - t[0];
            let ac = t[2] - t[0];
            ab.cross(ac).norm() * 0.5
        })
        .sum();
    assert!(
        (total - 6.0).abs() < 0.5,
        "2×3 face triangulation area should be ~6, got {total}"
    );
}
