// FILE: tests/occt_brep_adaptor.rs
//! Integration tests for the `brep_adaptor` module.
//!
//! These tests exercise the public API of `BRepAdaptorCurve`,
//! `BRepAdaptorSurface`, and `BRepAdaptorCompCurve` through the crate's public
//! interface.  All assertions use only public types and methods.

extern crate ironstream;

use ironstream::brep_adaptor::{
    BRepAdaptorCurve, BRepAdaptorSurface, BRepAdaptorCompCurve,
};
use ironstream::gp::{Ax1, Ax3, Pnt};
use ironstream::geom_abs::{CurveType, SurfaceType};

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}

fn pnt_approx(a: Pnt, b: Pnt, tol: f64) -> bool {
    a.distance(b) < tol
}

fn make_xy_circle(radius: f64) -> BRepAdaptorCurve {
    BRepAdaptorCurve::from_circle(Ax3::identity(), radius, 0.0, 2.0 * PI)
}

fn make_x_line(len: f64) -> BRepAdaptorCurve {
    let ax = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    BRepAdaptorCurve::from_line(ax, 0.0, len)
}

fn make_xy_plane() -> BRepAdaptorSurface {
    BRepAdaptorSurface::from_plane(Ax3::identity(), -10.0, 10.0, -10.0, 10.0)
}

// ---------------------------------------------------------------------------
// BRepAdaptorCurve — line
// ---------------------------------------------------------------------------

#[test]
fn line_get_type_is_line() {
    let c = make_x_line(10.0);
    assert_eq!(c.get_type(), CurveType::Line);
}

#[test]
fn line_parameter_range() {
    let c = make_x_line(7.0);
    assert!(approx(c.first_parameter(), 0.0, 1e-15));
    assert!(approx(c.last_parameter(), 7.0, 1e-15));
}

#[test]
fn line_value_at_various_params() {
    let c = make_x_line(10.0);
    for i in 0..=10 {
        let t = i as f64;
        let p = c.value(t);
        assert!(approx(p.x, t, 1e-12), "x={} expected={}", p.x, t);
        assert!(approx(p.y, 0.0, 1e-12));
        assert!(approx(p.z, 0.0, 1e-12));
    }
}

#[test]
fn line_d1_tangent_constant() {
    let c = make_x_line(10.0);
    for t in [0.0, 3.0, 7.5, 10.0] {
        let (_, d1) = c.d1(t);
        assert!(approx(d1.x, 1.0, 1e-12));
        assert!(approx(d1.y, 0.0, 1e-12));
        assert!(approx(d1.z, 0.0, 1e-12));
    }
}

#[test]
fn line_d2_is_zero() {
    let c = make_x_line(10.0);
    let (_, _, d2) = c.d2(5.0);
    assert!(d2.norm() < 1e-15);
}

#[test]
fn line_not_closed_not_periodic() {
    let c = make_x_line(5.0);
    assert!(!c.is_closed());
    assert!(!c.is_periodic());
}

#[test]
fn line_length_matches_parameter_span() {
    let len = 8.0;
    let c = make_x_line(len);
    let l = c.length();
    assert!(approx(l, len, 1e-5), "length={} expected={}", l, len);
}

#[test]
fn line_accessor() {
    let c = make_x_line(5.0);
    let ax = c.line().expect("should be a line");
    assert!(approx(ax.direction.x, 1.0, 1e-12));
}

// ---------------------------------------------------------------------------
// BRepAdaptorCurve — circle
// ---------------------------------------------------------------------------

#[test]
fn circle_get_type_is_circle() {
    let c = make_xy_circle(5.0);
    assert_eq!(c.get_type(), CurveType::Circle);
}

#[test]
fn circle_is_periodic_and_closed() {
    let c = make_xy_circle(1.0);
    assert!(c.is_periodic());
    assert!(c.is_closed());
    assert!(approx(c.period(), 2.0 * PI, 1e-12));
}

#[test]
fn circle_arc_not_closed() {
    let c = BRepAdaptorCurve::from_circle(Ax3::identity(), 2.0, 0.0, PI);
    assert!(!c.is_closed());
    assert!(c.is_periodic());
}

#[test]
fn circle_value_at_angle_zero_is_on_x_axis() {
    let r = 5.0;
    let c = make_xy_circle(r);
    let p = c.value(0.0);
    assert!(pnt_approx(p, Pnt::new(r, 0.0, 0.0), 1e-12));
}

#[test]
fn circle_value_at_pi_over_2_is_on_y_axis() {
    let r = 3.0;
    let c = make_xy_circle(r);
    let p = c.value(PI / 2.0);
    assert!(pnt_approx(p, Pnt::new(0.0, r, 0.0), 1e-12));
}

#[test]
fn circle_points_lie_on_radius() {
    let r = 4.0;
    let c = make_xy_circle(r);
    for i in 0..16 {
        let u = i as f64 * PI / 8.0;
        let p = c.value(u);
        // All points in XY plane at distance r from origin
        let dist = (p.x * p.x + p.y * p.y + p.z * p.z).sqrt();
        assert!(approx(dist, r, 1e-10), "dist={} u={}", dist, u);
    }
}

#[test]
fn circle_circumference_integration() {
    let r = 3.0;
    let c = make_xy_circle(r);
    let l = c.length();
    let expected = 2.0 * PI * r;
    assert!(approx(l, expected, 1e-3), "length={} expected={}", l, expected);
}

#[test]
fn circle_d1_fd_consistency() {
    let r = 3.0;
    let c = make_xy_circle(r);
    let u = 1.23;
    let h = 1e-6;
    let (_, d1) = c.d1(u);
    let p_fwd = c.value(u + h);
    let p_bwd = c.value(u - h);
    let fd = (p_fwd - p_bwd) * (1.0 / (2.0 * h));
    assert!(approx(d1.x, fd.x, 1e-7));
    assert!(approx(d1.y, fd.y, 1e-7));
}

#[test]
fn circle_d2_fd_consistency() {
    let r = 3.0;
    let c = make_xy_circle(r);
    let u = 0.7;
    let h = 1e-5;
    let (_, _, d2) = c.d2(u);
    let (_, d1_fwd) = c.d1(u + h);
    let (_, d1_bwd) = c.d1(u - h);
    let fd2 = (d1_fwd - d1_bwd) * (1.0 / (2.0 * h));
    assert!(approx(d2.x, fd2.x, 1e-6));
    assert!(approx(d2.y, fd2.y, 1e-6));
}

#[test]
fn circle_accessor_returns_correct_radius() {
    let r = 7.5;
    let c = make_xy_circle(r);
    let circ = c.circle().expect("should have circle");
    assert!(approx(circ.radius(), r, 1e-12));
}

#[test]
fn circle_wrong_accessor_returns_none() {
    let c = make_xy_circle(1.0);
    assert!(c.line().is_none());
    assert!(c.bspline().is_none());
}

// ---------------------------------------------------------------------------
// BRepAdaptorCurve — ellipse
// ---------------------------------------------------------------------------

#[test]
fn ellipse_get_type() {
    let c = BRepAdaptorCurve::from_ellipse(Ax3::identity(), 5.0, 3.0, 0.0, 2.0 * PI);
    assert_eq!(c.get_type(), CurveType::Ellipse);
}

#[test]
fn ellipse_value_at_extremes() {
    let a = 5.0;
    let b = 3.0;
    let c = BRepAdaptorCurve::from_ellipse(Ax3::identity(), a, b, 0.0, 2.0 * PI);
    let p0 = c.value(0.0);
    let p90 = c.value(PI / 2.0);
    assert!(pnt_approx(p0, Pnt::new(a, 0.0, 0.0), 1e-12));
    assert!(pnt_approx(p90, Pnt::new(0.0, b, 0.0), 1e-12));
}

#[test]
fn ellipse_is_periodic() {
    let c = BRepAdaptorCurve::from_ellipse(Ax3::identity(), 5.0, 3.0, 0.0, 2.0 * PI);
    assert!(c.is_periodic());
    assert!(approx(c.period(), 2.0 * PI, 1e-12));
}

// ---------------------------------------------------------------------------
// BRepAdaptorCurve — polyline
// ---------------------------------------------------------------------------

#[test]
fn polyline_get_type_is_other() {
    let pts = vec![Pnt::origin(), Pnt::new(1.0, 0.0, 0.0)];
    let c = BRepAdaptorCurve::from_polyline(pts);
    assert_eq!(c.get_type(), CurveType::OtherCurve);
}

#[test]
fn polyline_value_at_endpoints() {
    let pts = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(2.0, 3.0, 0.0),
    ];
    let c = BRepAdaptorCurve::from_polyline(pts.clone());
    let p0 = c.value(0.0);
    let p_end = c.value(c.last_parameter());
    assert!(pnt_approx(p0, pts[0], 1e-10));
    assert!(pnt_approx(p_end, pts[2], 1e-10));
}

#[test]
fn polyline_total_length() {
    let pts = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
        Pnt::new(3.0, 4.0, 0.0),
    ];
    let c = BRepAdaptorCurve::from_polyline(pts);
    // length = 3 + 4 = 7
    assert!(approx(c.last_parameter(), 7.0, 1e-12));
}

// ---------------------------------------------------------------------------
// BRepAdaptorSurface — plane
// ---------------------------------------------------------------------------

#[test]
fn plane_get_type() {
    let s = make_xy_plane();
    assert_eq!(s.get_type(), SurfaceType::Plane);
}

#[test]
fn plane_value_is_correct() {
    let s = make_xy_plane();
    let p = s.value(3.0, -2.0);
    assert!(pnt_approx(p, Pnt::new(3.0, -2.0, 0.0), 1e-12));
}

#[test]
fn plane_d1_partials() {
    let s = make_xy_plane();
    let (p, du, dv) = s.d1(1.0, 2.0);
    assert!(pnt_approx(p, Pnt::new(1.0, 2.0, 0.0), 1e-12));
    assert!(pnt_approx(du, Pnt::new(1.0, 0.0, 0.0), 1e-12));
    assert!(pnt_approx(dv, Pnt::new(0.0, 1.0, 0.0), 1e-12));
}

#[test]
fn plane_not_periodic() {
    let s = make_xy_plane();
    assert!(!s.is_u_periodic());
    assert!(!s.is_v_periodic());
    assert!(!s.is_u_closed());
    assert!(!s.is_v_closed());
}

#[test]
fn plane_normal_is_z() {
    let s = make_xy_plane();
    let n = s.normal(0.0, 0.0);
    assert!(approx(n.z.abs(), 1.0, 1e-12));
    assert!(n.x.abs() < 1e-12);
    assert!(n.y.abs() < 1e-12);
}

#[test]
fn plane_accessor() {
    let s = make_xy_plane();
    let ax3 = s.plane().expect("should be a plane");
    // z_dir should point along Z
    assert!(approx(ax3.z_dir.z, 1.0, 1e-12));
    assert!(s.cylinder().is_none());
    assert!(s.sphere().is_none());
}

// ---------------------------------------------------------------------------
// BRepAdaptorSurface — cylinder
// ---------------------------------------------------------------------------

#[test]
fn cylinder_get_type() {
    let s = BRepAdaptorSurface::from_cylinder(Ax3::identity(), 2.0, 0.0, 2.0*PI, -5.0, 5.0);
    assert_eq!(s.get_type(), SurfaceType::Cylinder);
}

#[test]
fn cylinder_value_on_circle() {
    let r = 2.0;
    let s = BRepAdaptorSurface::from_cylinder(Ax3::identity(), r, 0.0, 2.0*PI, -5.0, 5.0);
    for i in 0..8 {
        let u = i as f64 * PI / 4.0;
        let p = s.value(u, 0.0);
        let dist_xy = (p.x * p.x + p.y * p.y).sqrt();
        assert!(approx(dist_xy, r, 1e-10), "dist_xy={} u={}", dist_xy, u);
    }
}

#[test]
fn cylinder_u_periodic_not_v() {
    let s = BRepAdaptorSurface::from_cylinder(Ax3::identity(), 2.0, 0.0, 2.0*PI, -5.0, 5.0);
    assert!(s.is_u_periodic());
    assert!(!s.is_v_periodic());
}

// ---------------------------------------------------------------------------
// BRepAdaptorSurface — sphere
// ---------------------------------------------------------------------------

#[test]
fn sphere_get_type() {
    let s = BRepAdaptorSurface::from_sphere(Ax3::identity(), 1.0, 0.0, 2.0*PI, -PI/2.0, PI/2.0);
    assert_eq!(s.get_type(), SurfaceType::Sphere);
}

#[test]
fn sphere_all_points_on_surface() {
    let r = 3.0;
    let s = BRepAdaptorSurface::from_sphere(Ax3::identity(), r, 0.0, 2.0*PI, -PI/2.0, PI/2.0);
    for i in 0..8 {
        for j in 0..5 {
            let u = i as f64 * PI / 4.0;
            let v = -PI/2.0 + j as f64 * PI / 4.0;
            let p = s.value(u, v);
            let dist = p.norm();
            assert!(approx(dist, r, 1e-9), "dist={} u={} v={}", dist, u, v);
        }
    }
}

#[test]
fn sphere_north_pole() {
    let r = 5.0;
    let s = BRepAdaptorSurface::from_sphere(Ax3::identity(), r, 0.0, 2.0*PI, -PI/2.0, PI/2.0);
    let p = s.value(0.0, PI / 2.0);
    assert!(pnt_approx(p, Pnt::new(0.0, 0.0, r), 1e-10));
}

// ---------------------------------------------------------------------------
// BRepAdaptorSurface — torus
// ---------------------------------------------------------------------------

#[test]
fn torus_get_type() {
    let s = BRepAdaptorSurface::from_torus(Ax3::identity(), 3.0, 1.0, 0.0, 2.0*PI, 0.0, 2.0*PI);
    assert_eq!(s.get_type(), SurfaceType::Torus);
}

#[test]
fn torus_outer_point() {
    // At (u=0, v=0): point is at (major+minor, 0, 0)
    let major = 3.0;
    let minor = 1.0;
    let s = BRepAdaptorSurface::from_torus(Ax3::identity(), major, minor, 0.0, 2.0*PI, 0.0, 2.0*PI);
    let p = s.value(0.0, 0.0);
    assert!(pnt_approx(p, Pnt::new(major + minor, 0.0, 0.0), 1e-10));
}

#[test]
fn torus_inner_point() {
    // At (u=0, v=π): point is at (major-minor, 0, 0)
    let major = 3.0;
    let minor = 1.0;
    let s = BRepAdaptorSurface::from_torus(Ax3::identity(), major, minor, 0.0, 2.0*PI, 0.0, 2.0*PI);
    let p = s.value(0.0, PI);
    assert!(pnt_approx(p, Pnt::new(major - minor, 0.0, 0.0), 1e-10));
}

#[test]
fn torus_uv_periodic() {
    let s = BRepAdaptorSurface::from_torus(Ax3::identity(), 3.0, 1.0, 0.0, 2.0*PI, 0.0, 2.0*PI);
    assert!(s.is_u_periodic());
    assert!(s.is_v_periodic());
}

// ---------------------------------------------------------------------------
// BRepAdaptorCompCurve
// ---------------------------------------------------------------------------

#[test]
fn comp_curve_parameter_range_from_single_circle() {
    let c = make_xy_circle(1.0);
    let expected_len = c.length();
    let comp = BRepAdaptorCompCurve::new(vec![c], true);
    assert!(approx(comp.first_parameter(), 0.0, 1e-12));
    assert!(approx(comp.last_parameter(), expected_len, 1e-3));
    assert!(comp.is_closed());
}

#[test]
fn comp_curve_two_lines_total_length() {
    let ax = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let l1 = BRepAdaptorCurve::from_line(ax, 0.0, 3.0);
    let l2 = BRepAdaptorCurve::from_line(ax, 0.0, 4.0);
    let comp = BRepAdaptorCompCurve::new(vec![l1, l2], false);
    // Total = 3 + 4 = 7
    assert!(approx(comp.last_parameter(), 7.0, 1e-5));
}

#[test]
fn comp_curve_value_in_first_segment() {
    let ax = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let seg1 = BRepAdaptorCurve::from_line(ax, 0.0, 2.0);
    let ax2 = Ax1::new(Pnt::new(2.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
    let seg2 = BRepAdaptorCurve::from_line(ax2, 0.0, 2.0);
    let comp = BRepAdaptorCompCurve::new(vec![seg1, seg2], false);
    // At u=1.0 (halfway through segment 1) we should be at x≈1.0
    let p = comp.value(1.0);
    assert!(approx(p.x, 1.0, 1e-6), "x={}", p.x);
    assert!(approx(p.y, 0.0, 1e-6), "y={}", p.y);
}

#[test]
fn comp_curve_nb_intervals() {
    let ax = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let segments: Vec<_> = (0..5).map(|_| BRepAdaptorCurve::from_line(ax, 0.0, 1.0)).collect();
    let comp = BRepAdaptorCompCurve::new(segments, false);
    assert_eq!(comp.nb_intervals(), 5);
}

#[test]
fn comp_curve_d1_in_segment() {
    let ax = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let seg1 = BRepAdaptorCurve::from_line(ax, 0.0, 3.0);
    let comp = BRepAdaptorCompCurve::new(vec![seg1], false);
    let (_, d1) = comp.d1(1.5);
    // Tangent of a line along X is (1, 0, 0)
    assert!(approx(d1.x, 1.0, 1e-10));
}

#[test]
fn comp_curve_bounding_box_non_void() {
    let c = make_xy_circle(2.0);
    let comp = BRepAdaptorCompCurve::new(vec![c], true);
    let bb = comp.bounding_box();
    assert!(!bb.is_void());
}
