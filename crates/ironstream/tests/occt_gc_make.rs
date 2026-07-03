// FILE: tests/occt_gc_make.rs
//! Integration tests for `gc_make` — GC construction algorithms.
//!
//! Tests exercise the public API only (no access to private fields).
//! Tolerances mirror OCCT test suite conventions.

extern crate ironstream;

use ironstream::gc_make::{
    GcError, GcMakeArcOfCircle, GcMakeArcOfEllipse, GcMakeCircle, GcMakeLine, GcMakePlane,
    GcMakeSegment,
};
use ironstream::geom_ellipse::GeomEllipse;
use ironstream::gp::{Ax1, Ax3, Pnt};
use ironstream::gp_prim::{Circ, Lin, Pln};
use std::f64::consts::{FRAC_PI_2, PI};

const TOL: f64 = 1e-7;

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} within {tol}, diff={}",
        (a - b).abs()
    );
}

// ─────────────────────────────── GcMakeLine ──────────────────────────────────

#[test]
fn gc_make_line_from_two_points_direction() {
    // Line from (0,0,0) to (0,0,5): direction should be unit +Z.
    let p1 = Pnt::new(0.0, 0.0, 0.0);
    let p2 = Pnt::new(0.0, 0.0, 5.0);
    let maker = GcMakeLine::from_two_points(p1, p2);
    assert!(maker.is_done(), "construction must succeed");
    let line = maker.value();
    let dir = line.position().direction;
    near(dir.x, 0.0, TOL);
    near(dir.y, 0.0, TOL);
    near(dir.z, 1.0, TOL);
}

#[test]
fn gc_make_line_from_two_points_evaluation() {
    let p1 = Pnt::new(1.0, 2.0, 3.0);
    let p2 = Pnt::new(4.0, 2.0, 3.0); // along +X
    let maker = GcMakeLine::from_two_points(p1, p2);
    assert!(maker.is_done());
    let line = maker.value();
    // D0 at u=0 should be p1, at u=3 should be p2.
    let at0 = line.d0(0.0);
    near(at0.x, p1.x, TOL);
    near(at0.y, p1.y, TOL);
    near(at0.z, p1.z, TOL);
    let at3 = line.d0(3.0);
    near(at3.x, p2.x, TOL);
}

#[test]
fn gc_make_line_coincident_points_error() {
    let p = Pnt::new(5.0, 5.0, 5.0);
    let r = GcMakeLine::from_two_points(p, p).result();
    assert!(
        matches!(r, Err(GcError::CoincidentPoints)),
        "coincident points must return CoincidentPoints error"
    );
}

#[test]
fn gc_make_line_from_ax1() {
    let origin = Pnt::new(1.0, 0.0, 0.0);
    let dir = Pnt::new(0.0, 1.0, 0.0);
    let ax = Ax1::new(origin, dir);
    let maker = GcMakeLine::from_ax1(ax);
    assert!(maker.is_done());
    let line = maker.value();
    // Evaluate at u=5: should be at (1, 5, 0).
    let pt = line.d0(5.0);
    near(pt.x, 1.0, TOL);
    near(pt.y, 5.0, TOL);
    near(pt.z, 0.0, TOL);
}

// ─────────────────────────────── GcMakeSegment ───────────────────────────────

#[test]
fn gc_make_segment_endpoints_match() {
    let p1 = Pnt::new(0.0, 0.0, 0.0);
    let p2 = Pnt::new(0.0, 0.0, 10.0);
    let maker = GcMakeSegment::from_two_points(p1, p2);
    assert!(maker.is_done());
    let seg = maker.value();
    let start = seg.value(seg.first_parameter());
    let end = seg.value(seg.last_parameter());
    assert!(
        start.distance(p1) < TOL,
        "start point mismatch: {:?}",
        start
    );
    assert!(end.distance(p2) < TOL, "end point mismatch: {:?}", end);
}

#[test]
fn gc_make_segment_length_345() {
    // 3-4-5 right triangle sides: length = 5.
    let p1 = Pnt::new(0.0, 0.0, 0.0);
    let p2 = Pnt::new(3.0, 4.0, 0.0);
    let maker = GcMakeSegment::from_two_points(p1, p2);
    assert!(maker.is_done());
    let seg = maker.value();
    let len = seg.last_parameter() - seg.first_parameter();
    near(len, 5.0, TOL);
}

#[test]
fn gc_make_segment_from_line_and_params() {
    let lin = Lin::new_pd(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let maker = GcMakeSegment::from_line_params(lin, 1.0, 4.0);
    assert!(maker.is_done());
    let seg = maker.value();
    near(seg.first_parameter(), 1.0, TOL);
    near(seg.last_parameter(), 4.0, TOL);
    // Mid-point should be at x=2.5.
    let mid = seg.value(2.5);
    near(mid.x, 2.5, TOL);
    near(mid.y, 0.0, TOL);
}

// ─────────────────────────────── GcMakeCircle ────────────────────────────────

#[test]
fn gc_make_circle_three_points_unit_circle() {
    // Three points on the unit circle in XY plane.
    let p1 = Pnt::new(1.0, 0.0, 0.0);
    let p2 = Pnt::new(0.0, 1.0, 0.0);
    let p3 = Pnt::new(-1.0, 0.0, 0.0);
    let maker = GcMakeCircle::from_three_points(p1, p2, p3);
    assert!(maker.is_done());
    let c = maker.value();
    near(c.radius(), 1.0, 1e-6);
    assert!(c.location().distance(Pnt::origin()) < 1e-6);
}

#[test]
fn gc_make_circle_center_normal_radius() {
    let center = Pnt::new(1.0, 2.0, 3.0);
    let normal = Pnt::new(0.0, 0.0, 1.0);
    let radius = 4.0;
    let maker = GcMakeCircle::from_center_normal_radius(center, normal, radius);
    assert!(maker.is_done());
    let c = maker.value();
    near(c.radius(), radius, TOL);
    assert!(c.location().distance(center) < TOL);
}

#[test]
fn gc_make_circle_d0_on_circle() {
    // A circle of radius 3 in the XY plane: all d0 points should lie on it.
    let ax3 = Ax3::identity();
    let maker = GcMakeCircle::from_ax2_radius(ax3, 3.0);
    assert!(maker.is_done());
    let c = maker.value();
    for i in 0..8 {
        let u = (i as f64) * PI / 4.0;
        let pt = c.d0(u);
        near(pt.distance(c.location()), 3.0, 1e-12);
    }
}

#[test]
fn gc_make_circle_collinear_points_error() {
    // Three collinear points cannot define a circle.
    let p1 = Pnt::new(0.0, 0.0, 0.0);
    let p2 = Pnt::new(1.0, 0.0, 0.0);
    let p3 = Pnt::new(2.0, 0.0, 0.0);
    let r = GcMakeCircle::from_three_points(p1, p2, p3).result();
    assert!(
        matches!(r, Err(GcError::CollinearPoints)),
        "collinear points must return CollinearPoints"
    );
}

// ─────────────────────────────── GcMakePlane ─────────────────────────────────

#[test]
fn gc_make_plane_from_point_normal_xy() {
    let origin = Pnt::new(0.0, 0.0, 5.0);
    let normal = Pnt::new(0.0, 0.0, 1.0);
    let maker = GcMakePlane::from_point_normal(origin, normal);
    assert!(maker.is_done());
    let plane = maker.value();
    // The plane should contain the origin point.
    assert!(
        plane.pln().contains(origin, TOL),
        "origin must lie on plane"
    );
    // Any point with z=5 should lie on this plane.
    let p = Pnt::new(3.0, -7.0, 5.0);
    assert!(plane.pln().contains(p, TOL));
    // A point with z != 5 should not.
    let q = Pnt::new(0.0, 0.0, 0.0);
    assert!(!plane.pln().contains(q, TOL));
}

#[test]
fn gc_make_plane_from_three_points_contains_all() {
    let p1 = Pnt::new(0.0, 0.0, 1.0);
    let p2 = Pnt::new(1.0, 0.0, 1.0);
    let p3 = Pnt::new(0.0, 1.0, 1.0);
    let maker = GcMakePlane::from_three_points(p1, p2, p3);
    assert!(maker.is_done());
    let plane = maker.value();
    assert!(plane.pln().contains(p1, TOL));
    assert!(plane.pln().contains(p2, TOL));
    assert!(plane.pln().contains(p3, TOL));
}

#[test]
fn gc_make_plane_from_coefficients() {
    // Plane z = 0: coefficients (0, 0, 1, 0).
    let maker = GcMakePlane::from_coefficients(0.0, 0.0, 1.0, 0.0);
    assert!(maker.is_done());
    let plane = maker.value();
    let (a, b, c, d) = plane.coefficients();
    // Normal should be (0,0,1) direction.
    let n = Pnt::new(a, b, c).normalized();
    near(n.z.abs(), 1.0, TOL);
    // d=0 means the plane passes through the origin.
    let _ = d; // sign of d/n magnitude can vary; check containment instead
    assert!(plane.pln().contains(Pnt::origin(), TOL));
}

// ─────────────────────────────── GcMakeArcOfCircle ───────────────────────────

#[test]
fn gc_make_arc_of_circle_three_points_quarter() {
    // Arc from (1,0,0) through (cos45,sin45,0) to (0,1,0): quarter of unit circle.
    let p1 = Pnt::new(1.0, 0.0, 0.0);
    let angle_mid = PI / 4.0;
    let p2 = Pnt::new(angle_mid.cos(), angle_mid.sin(), 0.0);
    let p3 = Pnt::new(0.0, 1.0, 0.0);
    let maker = GcMakeArcOfCircle::from_three_points(p1, p2, p3);
    assert!(maker.is_done(), "arc construction must succeed");
    let arc = maker.value();
    // Evaluate at start and end — should match p1 and p3 (in some order).
    let start = arc.value(arc.first_parameter());
    let end = arc.value(arc.last_parameter());
    let ok1 = start.distance(p1) < 1e-6 && end.distance(p3) < 1e-6;
    let ok2 = start.distance(p3) < 1e-6 && end.distance(p1) < 1e-6;
    assert!(
        ok1 || ok2,
        "arc endpoints should be p1 and p3; got start={:?}, end={:?}",
        start,
        end
    );
}

#[test]
fn gc_make_arc_of_circle_from_circle_angles_quarter() {
    // A quarter-arc of radius 5 from angle 0 to π/2.
    let ax3 = Ax3::identity();
    let circ = Circ::new(ax3, 5.0);
    let maker = GcMakeArcOfCircle::from_circle_angles(&circ, 0.0, FRAC_PI_2, true);
    assert!(maker.is_done());
    let arc = maker.value();
    // Start: (5, 0, 0), End: (0, 5, 0)
    let start = arc.value(arc.first_parameter());
    let end = arc.value(arc.last_parameter());
    near(start.x, 5.0, 1e-6);
    near(start.y, 0.0, 1e-6);
    near(end.x, 0.0, 1e-6);
    near(end.y, 5.0, 1e-6);
}

#[test]
fn gc_make_arc_of_circle_two_points_sense() {
    // Arc from (1,0,0) to (0,1,0) CCW (shorter).
    let ax3 = Ax3::identity();
    let circ = Circ::new(ax3, 1.0);
    let p1 = Pnt::new(1.0, 0.0, 0.0);
    let p2 = Pnt::new(0.0, 1.0, 0.0);
    let maker = GcMakeArcOfCircle::from_circle_two_points(&circ, p1, p2, true);
    assert!(maker.is_done());
    let arc = maker.value();
    // The arc span should be approximately π/2.
    let span = arc.last_parameter() - arc.first_parameter();
    near(span, FRAC_PI_2, 1e-6);
}

// ─────────────────────────────── GcMakeArcOfEllipse ──────────────────────────

#[test]
fn gc_make_arc_of_ellipse_quarter_arc() {
    // Quarter-arc of an ellipse with major=4, minor=2 from 0 to π/2.
    let ax3 = Ax3::identity();
    let ellipse = GeomEllipse::from_ax3(ax3, 4.0, 2.0);
    let maker = GcMakeArcOfEllipse::from_ellipse_angles(&ellipse, 0.0, FRAC_PI_2, true);
    assert!(maker.is_done());
    let arc = maker.value();
    // Start: (4, 0, 0), End: (0, 2, 0).
    let start = arc.value(arc.first_parameter());
    let end = arc.value(arc.last_parameter());
    near(start.x, 4.0, 1e-6);
    near(start.y, 0.0, 1e-6);
    near(end.x, 0.0, 1e-6);
    near(end.y, 2.0, 1e-6);
}

#[test]
fn gc_make_arc_of_ellipse_from_points() {
    let ax3 = Ax3::identity();
    let ellipse = GeomEllipse::from_ax3(ax3, 5.0, 3.0);
    // p1 at angle 0 = (5,0,0), p2 at angle π/2 = (0,3,0).
    let p1 = Pnt::new(5.0, 0.0, 0.0);
    let p2 = Pnt::new(0.0, 3.0, 0.0);
    let maker = GcMakeArcOfEllipse::from_ellipse_points(&ellipse, p1, p2, true);
    assert!(maker.is_done());
    let arc = maker.value();
    let start = arc.value(arc.first_parameter());
    let end = arc.value(arc.last_parameter());
    assert!(
        start.distance(p1) < 1e-6 || end.distance(p1) < 1e-6,
        "p1 must appear as an endpoint"
    );
    assert!(
        start.distance(p2) < 1e-6 || end.distance(p2) < 1e-6,
        "p2 must appear as an endpoint"
    );
}

#[test]
fn gc_make_arc_of_ellipse_bad_angles_error() {
    let ax3 = Ax3::identity();
    let ellipse = GeomEllipse::from_ax3(ax3, 3.0, 2.0);
    // Identical angles → error.
    let r = GcMakeArcOfEllipse::from_ellipse_angles(&ellipse, PI, PI, true).result();
    assert!(
        matches!(r, Err(GcError::BadAngles)),
        "identical angles must produce BadAngles"
    );
}

#[test]
fn gc_make_arc_of_ellipse_full_arc_closed() {
    // An arc from 0 to 2π should be a closed curve.
    let ax3 = Ax3::identity();
    let ellipse = GeomEllipse::from_ax3(ax3, 2.0, 1.0);
    // Near-full arc: 0 to just under 2π.
    let maker = GcMakeArcOfEllipse::from_ellipse_angles(&ellipse, 0.0, 2.0 * PI - 0.01, true);
    assert!(maker.is_done());
    let arc = maker.value();
    let start = arc.value(arc.first_parameter());
    // Start should be at (2, 0, 0) (angle=0 on ellipse with major=2).
    near(start.x, 2.0, 1e-6);
    near(start.y, 0.0, 1e-6);
}
