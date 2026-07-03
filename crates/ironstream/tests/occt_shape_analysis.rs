// FILE: tests/occt_shape_analysis.rs
extern crate ironstream;

use ironstream::gp::{Ax3, Pnt};
use ironstream::shape_analysis::{
    EdgeGeometry, ShapeAnalysis_Curve, ShapeAnalysis_Edge, ShapeAnalysis_Surface,
    ShapeAnalysis_Wire, WireGeometry,
};
use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a unit-square closed wire in the XY plane.
fn unit_square_wire() -> WireGeometry {
    let mut wire = WireGeometry::new();
    wire.add_edge(EdgeGeometry::straight(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
    ));
    wire.add_edge(EdgeGeometry::straight(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
    ));
    wire.add_edge(EdgeGeometry::straight(
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ));
    wire.add_edge(EdgeGeometry::straight(
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 0.0),
    ));
    wire
}

/// Build a triangle wire in the XY plane.
fn triangle_wire() -> WireGeometry {
    let mut wire = WireGeometry::new();
    wire.add_edge(EdgeGeometry::straight(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ));
    wire.add_edge(EdgeGeometry::straight(
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(1.0, 2.0, 0.0),
    ));
    wire.add_edge(EdgeGeometry::straight(
        Pnt::new(1.0, 2.0, 0.0),
        Pnt::new(0.0, 0.0, 0.0),
    ));
    wire
}

// ---------------------------------------------------------------------------
// ShapeAnalysis_Surface integration tests
// ---------------------------------------------------------------------------

/// Evaluating the XY-plane at (u,v) = (1, 2) should give (1, 2, 0).
#[test]
fn test_surface_plane_value() {
    let surf = ShapeAnalysis_Surface::from_plane(Ax3::identity());
    let p = surf.value(1.0, 2.0);
    assert!((p.x - 1.0).abs() < 1e-12, "expected x=1 got {}", p.x);
    assert!((p.y - 2.0).abs() < 1e-12, "expected y=2 got {}", p.y);
    assert!((p.z).abs() < 1e-12, "expected z=0 got {}", p.z);
}

/// Projecting a point above the plane should return its XY coordinates.
#[test]
fn test_surface_plane_project() {
    let surf = ShapeAnalysis_Surface::from_plane(Ax3::identity());
    let p = Pnt::new(5.0, 7.0, 3.0);
    let (u, v, foot) = surf.project(p);
    assert!((u - 5.0).abs() < 1e-10, "u mismatch: {}", u);
    assert!((v - 7.0).abs() < 1e-10, "v mismatch: {}", v);
    assert!((foot.z).abs() < 1e-12, "foot z should be 0, got {}", foot.z);
}

/// The plane normal should point in +Z when using Ax3::identity().
#[test]
fn test_surface_plane_normal() {
    let surf = ShapeAnalysis_Surface::from_plane(Ax3::identity());
    let n = surf.normal_at(0.0, 0.0);
    assert!((n.z - 1.0).abs() < 1e-12);
    assert!((n.x).abs() < 1e-12);
    assert!((n.y).abs() < 1e-12);
}

/// A unit sphere's north pole should be at (0, 0, 1).
#[test]
fn test_surface_sphere_north_pole() {
    let surf = ShapeAnalysis_Surface::from_sphere(Pnt::new(0.0, 0.0, 0.0), 1.0);
    let p = surf.value(0.0, PI / 2.0);
    assert!((p.z - 1.0).abs() < 1e-10, "north pole z should be 1, got {}", p.z);
}

/// Projecting a point directly above the sphere center should return v ≈ π/2.
#[test]
fn test_surface_sphere_project_top() {
    let surf = ShapeAnalysis_Surface::from_sphere(Pnt::new(0.0, 0.0, 0.0), 2.0);
    let p = Pnt::new(0.0, 0.0, 10.0);
    let (_, v, foot) = surf.project(p);
    assert!((v - PI / 2.0).abs() < 1e-10, "v should be pi/2, got {}", v);
    assert!((foot.z - 2.0).abs() < 1e-10, "foot should be at z=2, got {}", foot.z);
}

/// Cylinder evaluation at angle 0 should land on the +X side.
#[test]
fn test_surface_cylinder_value_at_angle_0() {
    let surf = ShapeAnalysis_Surface::from_cylinder(Ax3::identity(), 3.0, 10.0);
    let p = surf.value(0.0, 0.0);
    assert!((p.x - 3.0).abs() < 1e-12, "expected x=3, got {}", p.x);
    assert!((p.y).abs() < 1e-12);
}

/// A bilinear patch at center (0.5, 0.5) should return the centroid of the
/// four corners.
#[test]
fn test_surface_bilinear_center() {
    let p00 = Pnt::new(0.0, 0.0, 0.0);
    let p10 = Pnt::new(2.0, 0.0, 0.0);
    let p01 = Pnt::new(0.0, 2.0, 0.0);
    let p11 = Pnt::new(2.0, 2.0, 0.0);
    let surf = ShapeAnalysis_Surface::from_bilinear(p00, p10, p01, p11);
    let c = surf.value(0.5, 0.5);
    assert!((c.x - 1.0).abs() < 1e-12);
    assert!((c.y - 1.0).abs() < 1e-12);
}

// ---------------------------------------------------------------------------
// ShapeAnalysis_Curve integration tests
// ---------------------------------------------------------------------------

/// A unit line at t=5 should give x=5, y=0.
#[test]
fn test_curve_line_value() {
    let curve = ShapeAnalysis_Curve::from_line_trimmed(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        0.0,
        20.0,
    );
    let p = curve.value(5.0);
    assert!((p.x - 5.0).abs() < 1e-12);
    assert!((p.y).abs() < 1e-12);
}

/// Projecting a point beside a line should give the correct parameter.
#[test]
fn test_curve_line_project_beside() {
    let curve = ShapeAnalysis_Curve::from_line_trimmed(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        0.0,
        10.0,
    );
    let (t, foot) = curve.project(Pnt::new(3.0, 99.0, 0.0));
    assert!((t - 3.0).abs() < 1e-10, "t should be 3, got {}", t);
    assert!((foot.y).abs() < 1e-12);
}

/// Full circumference of a radius-5 circle.
#[test]
fn test_curve_circle_circumference() {
    let curve = ShapeAnalysis_Curve::from_circle(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
        5.0,
    );
    let expected = 2.0 * PI * 5.0;
    assert!(
        (curve.length() - expected).abs() < 1e-10,
        "expected {:.6} got {:.6}",
        expected,
        curve.length()
    );
}

/// Polyline [0,0,0]→[1,0,0]→[1,1,0]→[0,1,0] should have length 3.
#[test]
fn test_curve_polyline_length() {
    let pts = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ];
    let curve = ShapeAnalysis_Curve::from_polyline(pts);
    assert!((curve.length() - 3.0).abs() < 1e-12);
}

/// Closest point on a polyline to a point off the second segment.
#[test]
fn test_curve_polyline_project() {
    let pts = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(2.0, 2.0, 0.0),
    ];
    let curve = ShapeAnalysis_Curve::from_polyline(pts);
    // Point (2, 1, 0) lies exactly on segment 2.
    let (t, foot) = curve.project(Pnt::new(2.0, 1.0, 0.0));
    assert!((foot.x - 2.0).abs() < 1e-10);
    assert!((foot.y - 1.0).abs() < 1e-10);
    let _ = t; // parameter location on segment 2
}

// ---------------------------------------------------------------------------
// ShapeAnalysis_Edge integration tests
// ---------------------------------------------------------------------------

/// A straight edge of length 5 (3-4-5 right triangle).
#[test]
fn test_edge_straight_length() {
    let e = ShapeAnalysis_Edge::new(EdgeGeometry::straight(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(3.0, 4.0, 0.0),
    ));
    assert!((e.chord_length() - 5.0).abs() < 1e-12);
    assert!((e.arc_length() - 5.0).abs() < 1e-12);
}

/// A degenerate edge (same start and end) within tolerance.
#[test]
fn test_edge_degenerate_same_point() {
    let p = Pnt::new(1.0, 2.0, 3.0);
    let e = ShapeAnalysis_Edge::new(EdgeGeometry::straight(p, p));
    assert!(e.is_degenerate(1e-6));
    assert!(!e.is_valid() || e.chord_length() < 1e-12);
}

/// Start tangent of a horizontal edge should point in +X.
#[test]
fn test_edge_start_tangent_x_axis() {
    let e = ShapeAnalysis_Edge::new(EdgeGeometry::straight(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(5.0, 0.0, 0.0),
    ));
    let t = e.start_tangent();
    assert!((t.x - 1.0).abs() < 1e-12);
    assert!((t.y).abs() < 1e-12);
    assert!((t.z).abs() < 1e-12);
}

/// Midpoint of a segment from (0,0,0) to (4,0,0) should be (2,0,0).
#[test]
fn test_edge_midpoint() {
    let e = ShapeAnalysis_Edge::new(EdgeGeometry::straight(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(4.0, 0.0, 0.0),
    ));
    let mid = e.midpoint();
    assert!((mid.x - 2.0).abs() < 1e-12);
    assert!((mid.y).abs() < 1e-12);
}

// ---------------------------------------------------------------------------
// ShapeAnalysis_Wire integration tests
// ---------------------------------------------------------------------------

/// Unit square wire perimeter should be 4.
#[test]
fn test_wire_square_perimeter() {
    let w = ShapeAnalysis_Wire::new(unit_square_wire());
    assert!((w.perimeter() - 4.0).abs() < 1e-12, "perimeter: {}", w.perimeter());
}

/// Unit square wire should be closed.
#[test]
fn test_wire_square_is_closed() {
    let w = ShapeAnalysis_Wire::new(unit_square_wire());
    assert!(w.is_closed(1e-10));
}

/// Unit square wire should be connected.
#[test]
fn test_wire_square_is_connected() {
    let w = ShapeAnalysis_Wire::new(unit_square_wire());
    assert!(w.is_connected(1e-10));
}

/// A wire with a gap between edges should report gaps_exceeding.
#[test]
fn test_wire_gap_detected() {
    let mut wire = WireGeometry::new();
    wire.add_edge(EdgeGeometry::straight(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
    ));
    // deliberate gap: next edge starts at (2,0,0) instead of (1,0,0)
    wire.add_edge(EdgeGeometry::straight(
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 0.0),
    ));
    let w = ShapeAnalysis_Wire::new(wire);
    let gaps = w.gaps_exceeding(0.5);
    assert!(!gaps.is_empty(), "should have detected a gap > 0.5");
    assert!((gaps[0].1 - 1.0).abs() < 1e-10, "gap should be 1.0, got {}", gaps[0].1);
}

/// Triangle wire should have 3 edges and be closed.
#[test]
fn test_wire_triangle_nb_edges_and_closed() {
    let w = ShapeAnalysis_Wire::new(triangle_wire());
    assert_eq!(w.nb_edges(), 3);
    assert!(w.is_closed(1e-10));
}

/// Max gap should be zero for a connected closed wire.
#[test]
fn test_wire_max_gap_zero_for_square() {
    let w = ShapeAnalysis_Wire::new(unit_square_wire());
    assert!(w.max_gap() < 1e-12, "max_gap: {}", w.max_gap());
}
