// FILE: tests/occt_gce2d_make.rs
//! Integration tests for the `GCE2d` 2D construction algorithm module, mirroring
//! OpenCascade's `GCE2d` package.
//!
//! All imports are from `ironstream::` only. Tolerances follow OCCT:
//!   - linear: `precision::CONFUSION = 1e-7`

extern crate ironstream;

use ironstream::gce2d_make::{
    Gce2dError, Gce2dMakeArcOfCircle, Gce2dMakeCircle, Gce2dMakeEllipse, Gce2dMakeLine,
    Gce2dMakeSegment,
};
use ironstream::geom2d_circle::{Ax22d2, Circ2d};
use ironstream::geom2d_trimmed_curve::BasisCurve2d;
use ironstream::geom2d_line::Lin2d;
use ironstream::gp2d::{Ax2d, Pnt2d};
use ironstream::precision::CONFUSION;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

// ─────────────────────────────────────────────────────────────────────────────
// GCE2d_MakeLine
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn make_line_from_two_points_evaluates_correctly() {
    let p1 = Pnt2d::new(2.0, 3.0);
    let p2 = Pnt2d::new(6.0, 3.0); // horizontal line
    let maker = Gce2dMakeLine::from_two_points(p1, p2);
    assert!(maker.is_done(), "line from two points should succeed");
    let line = maker.value();
    // At u=0 the origin of the line should equal p1.
    let at0 = line.eval_d0(0.0);
    assert!(
        at0.distance(p1) < CONFUSION,
        "origin mismatch: {at0:?} vs {p1:?}"
    );
    // The direction should be (1, 0).
    let dir = line.direction();
    assert!((dir.x - 1.0).abs() < CONFUSION && dir.y.abs() < CONFUSION);
}

#[test]
fn make_line_coincident_points_returns_error() {
    let p = Pnt2d::new(5.0, 7.0);
    let result = Gce2dMakeLine::from_two_points(p, p).result();
    assert!(matches!(result, Err(Gce2dError::CoincidentPoints)));
}

#[test]
fn make_line_from_ax2d_preserves_origin_and_direction() {
    let ax = Ax2d::new(Pnt2d::new(0.0, 5.0), Pnt2d::new(0.0, 1.0)); // vertical line at x=0
    let maker = Gce2dMakeLine::from_ax2d(ax);
    assert!(maker.is_done());
    let line = maker.value();
    assert!(line.location().distance(Pnt2d::new(0.0, 5.0)) < CONFUSION);
    // Direction should be (0, 1).
    assert!(line.direction().x.abs() < CONFUSION);
    assert!((line.direction().y - 1.0).abs() < CONFUSION);
}

#[test]
fn make_line_parallel_offset_creates_parallel_line() {
    // X-axis line offset upward by 4 units.
    let lin = Lin2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let maker = Gce2dMakeLine::from_parallel_offset(lin, 4.0);
    assert!(maker.is_done());
    let line = maker.value();
    // The offset line should have y = 4 at any x.
    assert!((line.location().y - 4.0).abs() < CONFUSION);
    // Direction still horizontal.
    assert!((line.direction().x - 1.0).abs() < CONFUSION);
    assert!(line.direction().y.abs() < CONFUSION);
}

#[test]
fn make_line_null_direction_returns_error() {
    let result = Gce2dMakeLine::from_point_dir(Pnt2d::origin(), Pnt2d::new(0.0, 0.0)).result();
    assert!(matches!(result, Err(Gce2dError::NullVector)));
}

// ─────────────────────────────────────────────────────────────────────────────
// GCE2d_MakeSegment
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn make_segment_from_two_points_has_correct_length() {
    let p1 = Pnt2d::new(0.0, 0.0);
    let p2 = Pnt2d::new(3.0, 4.0); // 3-4-5 triangle
    let maker = Gce2dMakeSegment::from_two_points(p1, p2);
    assert!(maker.is_done());
    let seg = maker.value();
    let len = seg.last_parameter() - seg.first_parameter();
    assert!((len - 5.0).abs() < CONFUSION, "expected length 5, got {len}");
}

#[test]
fn make_segment_endpoints_match_input_points() {
    let p1 = Pnt2d::new(1.0, 2.0);
    let p2 = Pnt2d::new(4.0, 6.0);
    let maker = Gce2dMakeSegment::from_two_points(p1, p2);
    assert!(maker.is_done());
    let seg = maker.value();
    let start = seg.value(seg.first_parameter());
    let end = seg.value(seg.last_parameter());
    assert!(start.distance(p1) < 1e-7, "start mismatch: {start:?} vs {p1:?}");
    assert!(end.distance(p2) < 1e-7, "end mismatch: {end:?} vs {p2:?}");
}

#[test]
fn make_segment_coincident_fails() {
    let p = Pnt2d::new(2.0, 2.0);
    assert!(matches!(
        Gce2dMakeSegment::from_two_points(p, p).result(),
        Err(Gce2dError::CoincidentPoints)
    ));
}

#[test]
fn make_segment_from_lin_params_trims_correctly() {
    let lin = Lin2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0));
    let maker = Gce2dMakeSegment::from_lin_params(lin, 3.0, 9.0);
    assert!(maker.is_done());
    let seg = maker.value();
    assert!((seg.first_parameter() - 3.0).abs() < CONFUSION);
    assert!((seg.last_parameter() - 9.0).abs() < CONFUSION);
    // Length of segment = 6 units.
    assert!((seg.last_parameter() - seg.first_parameter() - 6.0).abs() < CONFUSION);
}

#[test]
fn make_segment_from_lin_param_point() {
    // Y-axis line, from param=0 to the point (0, 5).
    let lin = Lin2d::new(Pnt2d::origin(), Pnt2d::new(0.0, 1.0));
    let end_pt = Pnt2d::new(0.0, 5.0);
    let maker = Gce2dMakeSegment::from_lin_param_point(lin, 0.0, end_pt);
    assert!(maker.is_done());
    let seg = maker.value();
    assert!((seg.first_parameter() - 0.0).abs() < CONFUSION);
    assert!((seg.last_parameter() - 5.0).abs() < CONFUSION);
}

#[test]
fn make_segment_basis_curve_is_line_variant() {
    let p1 = Pnt2d::new(0.0, 0.0);
    let p2 = Pnt2d::new(1.0, 1.0);
    let maker = Gce2dMakeSegment::from_two_points(p1, p2);
    assert!(maker.is_done());
    let seg = maker.value();
    assert!(
        matches!(seg.basis_curve(), BasisCurve2d::Line(_)),
        "expected Line basis curve"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// GCE2d_MakeCircle
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn make_circle_from_three_unit_circle_points() {
    let p1 = Pnt2d::new(1.0, 0.0);
    let p2 = Pnt2d::new(0.0, 1.0);
    let p3 = Pnt2d::new(-1.0, 0.0);
    let maker = Gce2dMakeCircle::from_three_points(p1, p2, p3);
    assert!(maker.is_done());
    let circ = maker.value();
    assert!((circ.radius() - 1.0).abs() < 1e-7, "radius: {}", circ.radius());
    assert!(
        circ.location().distance(Pnt2d::origin()) < 1e-7,
        "center: {:?}",
        circ.location()
    );
}

#[test]
fn make_circle_from_center_and_point() {
    let center = Pnt2d::new(2.0, 3.0);
    let pt = Pnt2d::new(2.0, 7.0); // radius = 4
    let maker = Gce2dMakeCircle::from_center_and_point(center, pt);
    assert!(maker.is_done());
    let circ = maker.value();
    assert!((circ.radius() - 4.0).abs() < CONFUSION);
    assert!(circ.location().distance(center) < CONFUSION);
}

#[test]
fn make_circle_negative_radius_fails() {
    let ax = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let result = Gce2dMakeCircle::from_ax2d_radius(ax, -3.0, true).result();
    assert!(matches!(result, Err(Gce2dError::NegativeRadius)));
}

#[test]
fn make_circle_parametrisation_correct() {
    // Circle of radius 2 at origin; at u=PI the point should be (-2, 0).
    let ax = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let maker = Gce2dMakeCircle::from_ax2d_radius(ax, 2.0, true);
    assert!(maker.is_done());
    let circ = maker.value();
    let p = circ.eval_d0(PI);
    assert!((p.x + 2.0).abs() < 1e-7 && p.y.abs() < 1e-7, "{p:?}");
}

// ─────────────────────────────────────────────────────────────────────────────
// GCE2d_MakeArcOfCircle
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn make_arc_from_three_points_covers_upper_semicircle() {
    // Arc: (1,0) → (0,1) → (-1,0) (upper semicircle)
    let p1 = Pnt2d::new(1.0, 0.0);
    let p2 = Pnt2d::new(0.0, 1.0);
    let p3 = Pnt2d::new(-1.0, 0.0);
    let maker = Gce2dMakeArcOfCircle::from_three_points(p1, p2, p3);
    assert!(maker.is_done());
    let arc = maker.value();
    let start = arc.value(arc.first_parameter());
    let end = arc.value(arc.last_parameter());
    let ok1 = start.distance(p1) < 1e-6 && end.distance(p3) < 1e-6;
    let ok2 = start.distance(p3) < 1e-6 && end.distance(p1) < 1e-6;
    assert!(ok1 || ok2, "arc endpoints: start={start:?}, end={end:?}");
}

#[test]
fn make_arc_from_circ_angles_quarter_arc() {
    let circ = Circ2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 2.0, true);
    let maker = Gce2dMakeArcOfCircle::from_circ_angles(circ, 0.0, FRAC_PI_2, true);
    assert!(maker.is_done());
    let arc = maker.value();
    let start = arc.value(arc.first_parameter());
    let end = arc.value(arc.last_parameter());
    // At angle 0 on radius-2 circle: (2, 0)
    assert!((start.x - 2.0).abs() < 1e-7 && start.y.abs() < 1e-7, "{start:?}");
    // At angle PI/2: (0, 2)
    assert!(end.x.abs() < 1e-7 && (end.y - 2.0).abs() < 1e-7, "{end:?}");
}

#[test]
fn make_arc_clockwise_sense() {
    // CW arc from 0 to -PI/2 (i.e. angle goes from 0 toward 3PI/2 = 270°
    // when sense=false, but result stored with u1 < u2).
    let circ = Circ2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 1.0, true);
    let maker = Gce2dMakeArcOfCircle::from_circ_angles(circ, 0.0, -FRAC_PI_2, false);
    assert!(maker.is_done());
    let arc = maker.value();
    // Span should be PI/2
    let span = arc.last_parameter() - arc.first_parameter();
    assert!((span - FRAC_PI_2).abs() < 1e-7, "span={span}");
}

#[test]
fn make_arc_same_angle_fails() {
    let circ = Circ2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 1.0, true);
    let result = Gce2dMakeArcOfCircle::from_circ_angles(circ, 1.0, 1.0, true).result();
    assert!(matches!(result, Err(Gce2dError::BadAngles)));
}

#[test]
fn make_arc_from_circ_two_points() {
    let circ = Circ2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0), 1.0, true);
    let p1 = Pnt2d::new(1.0, 0.0);  // angle 0
    let p2 = Pnt2d::new(0.0, 1.0);  // angle PI/2
    let maker = Gce2dMakeArcOfCircle::from_circ_two_points(circ, p1, p2, true);
    assert!(maker.is_done());
    let arc = maker.value();
    let span = arc.last_parameter() - arc.first_parameter();
    assert!((span - FRAC_PI_2).abs() < 1e-7, "span={span}");
}

// ─────────────────────────────────────────────────────────────────────────────
// GCE2d_MakeEllipse
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn make_ellipse_from_major_axis_radii() {
    let ax = Ax2d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0));
    let maker = Gce2dMakeEllipse::from_major_axis(ax, 6.0, 4.0, true);
    assert!(maker.is_done());
    let ell = maker.value();
    assert!((ell.major_radius() - 6.0).abs() < CONFUSION);
    assert!((ell.minor_radius() - 4.0).abs() < CONFUSION);
    assert!(ell.location().distance(Pnt2d::new(1.0, 2.0)) < CONFUSION);
}

#[test]
fn make_ellipse_parametrisation_vertices() {
    // Major=5, minor=3, centred at origin, X axis = (1, 0).
    let ax = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let maker = Gce2dMakeEllipse::from_major_axis(ax, 5.0, 3.0, true);
    assert!(maker.is_done());
    let ell = maker.value();
    // Major vertex at u=0: (5, 0)
    let p0 = ell.eval_d0(0.0);
    assert!((p0.x - 5.0).abs() < 1e-7 && p0.y.abs() < 1e-7, "{p0:?}");
    // Minor vertex at u=PI/2: (0, 3)
    let p1 = ell.eval_d0(FRAC_PI_2);
    assert!(p1.x.abs() < 1e-7 && (p1.y - 3.0).abs() < 1e-7, "{p1:?}");
    // Back vertex at u=PI: (-5, 0)
    let p2 = ell.eval_d0(PI);
    assert!((p2.x + 5.0).abs() < 1e-7 && p2.y.abs() < 1e-7, "{p2:?}");
}

#[test]
fn make_ellipse_inverted_radii_fails() {
    let ax = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let result = Gce2dMakeEllipse::from_major_axis(ax, 3.0, 7.0, true).result();
    assert!(matches!(result, Err(Gce2dError::InvertedRadii)));
}

#[test]
fn make_ellipse_negative_minor_radius_fails() {
    let ax = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let result = Gce2dMakeEllipse::from_major_axis(ax, 5.0, -2.0, true).result();
    assert!(matches!(result, Err(Gce2dError::NegativeRadius)));
}

#[test]
fn make_ellipse_from_foci_and_point_computes_correct_radii() {
    // Classical 3-4-5 ellipse: foci at (±3, 0), point at (0, 4).
    // major=5, minor=4.
    let f1 = Pnt2d::new(-3.0, 0.0);
    let f2 = Pnt2d::new(3.0, 0.0);
    let p  = Pnt2d::new(0.0, 4.0);
    let maker = Gce2dMakeEllipse::from_foci_and_point(f1, f2, p);
    assert!(maker.is_done());
    let ell = maker.value();
    assert!((ell.major_radius() - 5.0).abs() < 1e-7, "major={}", ell.major_radius());
    assert!((ell.minor_radius() - 4.0).abs() < 1e-7, "minor={}", ell.minor_radius());
    // Center at origin.
    assert!(ell.location().distance(Pnt2d::origin()) < 1e-7);
}

#[test]
fn make_ellipse_from_ax22d() {
    let pos = Ax22d2::from_x_axis(Pnt2d::new(3.0, 3.0), Pnt2d::new(1.0, 0.0), true);
    let maker = Gce2dMakeEllipse::from_ax22d(pos, 7.0, 2.0);
    assert!(maker.is_done());
    let ell = maker.value();
    assert!((ell.major_radius() - 7.0).abs() < CONFUSION);
    assert!((ell.minor_radius() - 2.0).abs() < CONFUSION);
    assert!(ell.location().distance(Pnt2d::new(3.0, 3.0)) < CONFUSION);
}

#[test]
fn make_ellipse_circle_degenerate_case() {
    // When major == minor the result is a circle (eccentricity == 0).
    let ax = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let maker = Gce2dMakeEllipse::from_major_axis(ax, 4.0, 4.0, true);
    assert!(maker.is_done());
    let ell = maker.value();
    assert!((ell.eccentricity() - 0.0).abs() < 1e-7);
    // All points on the ellipse are equidistant from the center.
    for k in 0..8 {
        let u = k as f64 * FRAC_PI_4;
        let pt = ell.eval_d0(u);
        assert!((pt.distance(Pnt2d::origin()) - 4.0).abs() < 1e-7);
    }
}
