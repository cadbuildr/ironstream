// FILE: tests/occt_extrema.rs
//! Integration tests for `extrema` — Extrema distance algorithms.
//!
//! Exercises the public API only: `ExtAlgo`, `ExtCC`, `ExtCS`, `ExtSS`,
//! `POnCurv`, `POnSurf`.

extern crate ironstream;

use ironstream::extrema::{ExtAlgo, ExtCC, ExtCS, ExtSS, POnCurv, POnSurf};
use ironstream::geom_adaptor_curve::GeomAdaptorCurve;
use ironstream::geom_adaptor_surface::GeomAdaptorSurface;
use ironstream::geom_circle::GeomCircle;
use ironstream::geom_line::GeomLine;
use ironstream::gp::{Ax3, Pnt};
use std::f64::consts::TAU;

// ────────────────────────────────────────────────────────────────────────────
// Helpers
// ────────────────────────────────────────────────────────────────────────────

fn near(a: f64, b: f64, tol: f64, label: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{label}: expected {a} ≈ {b} (tol={tol}), diff={}",
        (a - b).abs()
    );
}

fn make_line_adaptor(origin: Pnt, dir: Pnt, t0: f64, t1: f64) -> GeomAdaptorCurve {
    let line = GeomLine::from_point_dir(origin, dir);
    GeomAdaptorCurve::from_line(line, t0, t1)
}

fn make_circle_adaptor(placement: Ax3, radius: f64) -> GeomAdaptorCurve {
    let circ = GeomCircle::from_ax3(placement, radius);
    GeomAdaptorCurve::from_circle(circ, 0.0, TAU)
}

fn xy_plane() -> GeomAdaptorSurface {
    GeomAdaptorSurface::from_plane(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    )
}

// ────────────────────────────────────────────────────────────────────────────
// POnCurv / POnSurf API
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn pon_curv_stores_parameter_and_point() {
    let p = POnCurv::new(2.5, Pnt::new(1.0, 0.0, 0.0));
    assert_eq!(p.parameter(), 2.5);
    assert_eq!(p.point().x, 1.0);
    assert_eq!(p.point().y, 0.0);
    assert_eq!(p.point().z, 0.0);
}

#[test]
fn pon_surf_stores_uv_and_point() {
    let p = POnSurf::new(0.25, 0.75, Pnt::new(3.0, 4.0, 5.0));
    let (u, v) = p.parameters();
    near(u, 0.25, 1e-15, "u");
    near(v, 0.75, 1e-15, "v");
    near(p.point().z, 5.0, 1e-15, "z");
}

// ────────────────────────────────────────────────────────────────────────────
// ExtAlgo
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn ext_algo_variants_distinguishable() {
    assert_ne!(ExtAlgo::Grad, ExtAlgo::Tree);
    assert_eq!(ExtAlgo::default(), ExtAlgo::Grad);
}

// ────────────────────────────────────────────────────────────────────────────
// ExtCC — Curve–Curve
// ────────────────────────────────────────────────────────────────────────────

/// Two coaxial parallel lines 5 units apart → min distance = 5.
#[test]
fn extcc_parallel_lines_distance() {
    let c1 = make_line_adaptor(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0), -5.0, 5.0);
    let c2 = make_line_adaptor(Pnt::new(0.0, 5.0, 0.0), Pnt::new(1.0, 0.0, 0.0), -5.0, 5.0);
    let ec = ExtCC::new(&c1, &c2);
    assert!(ec.is_done(), "computation should succeed");
    near(ec.min_distance().unwrap(), 5.0, 1e-5, "parallel line distance");
}

/// Two intersecting lines → min distance = 0.
#[test]
fn extcc_intersecting_lines_zero_distance() {
    let c1 = make_line_adaptor(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0), -5.0, 5.0);
    let c2 = make_line_adaptor(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0), -5.0, 5.0);
    let ec = ExtCC::new(&c1, &c2);
    assert!(ec.is_done());
    near(ec.min_distance().unwrap(), 0.0, 1e-6, "intersecting lines min dist");
}

/// Unit circle vs. offset circle (same Z, centered 4 apart) — min dist = 2 (4 - 1 - 1).
#[test]
fn extcc_two_circles_min_distance() {
    let ax1 = Ax3::identity();
    // Circle 2 centered at (4,0,0).
    let ax2 = Ax3::from_origin_normal(
        Pnt::new(4.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let c1 = make_circle_adaptor(ax1, 1.0);
    let c2 = make_circle_adaptor(ax2, 1.0);
    let ec = ExtCC::new(&c1, &c2);
    assert!(ec.is_done());
    near(ec.min_distance().unwrap(), 2.0, 0.05, "circle-circle min distance");
}

/// `algo()` accessor returns the value passed to `with_algo`.
#[test]
fn extcc_algo_accessor() {
    let c1 = make_line_adaptor(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0), 0.0, 1.0);
    let c2 = make_line_adaptor(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 0.0, 0.0), 0.0, 1.0);
    let ec = ExtCC::with_algo(&c1, &c2, ExtAlgo::Tree);
    assert_eq!(ec.algo(), ExtAlgo::Tree);
}

/// `points()` returns consistent data: the distance between the two reported
/// points should equal the square-root of `square_distance`.
#[test]
fn extcc_points_consistent_with_distance() {
    let c1 = make_line_adaptor(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0), -5.0, 5.0);
    let c2 = make_line_adaptor(Pnt::new(0.0, 3.0, 0.0), Pnt::new(1.0, 0.0, 0.0), -5.0, 5.0);
    let ec = ExtCC::new(&c1, &c2);
    assert!(ec.nb_ext() >= 1);
    let (p1, p2) = ec.points(1);
    let d_actual = p1.point().distance(p2.point());
    let d_stored = ec.square_distance(1).sqrt();
    near(d_actual, d_stored, 1e-9, "points() vs square_distance() consistency");
}

// ────────────────────────────────────────────────────────────────────────────
// ExtCS — Curve–Surface
// ────────────────────────────────────────────────────────────────────────────

/// Line at z=7 above the XY plane → min distance = 7.
#[test]
fn extcs_line_above_plane_distance() {
    let c = make_line_adaptor(Pnt::new(0.0, 0.0, 7.0), Pnt::new(1.0, 0.0, 0.0), -5.0, 5.0);
    let s = xy_plane();
    let ec = ExtCS::new(&c, &s);
    assert!(ec.is_done());
    near(ec.min_distance().unwrap(), 7.0, 1e-5, "line-plane distance");
}

/// Line lying in the XY plane → min distance = 0.
#[test]
fn extcs_line_in_plane_zero_distance() {
    let c = make_line_adaptor(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0), -5.0, 5.0);
    let s = xy_plane();
    let ec = ExtCS::new(&c, &s);
    assert!(ec.is_done());
    near(ec.min_distance().unwrap(), 0.0, 1e-6, "line-in-plane min dist");
}

/// `algo()` accessor.
#[test]
fn extcs_algo_accessor() {
    let c = make_line_adaptor(Pnt::new(0.0, 0.0, 2.0), Pnt::new(1.0, 0.0, 0.0), 0.0, 1.0);
    let s = xy_plane();
    let ec = ExtCS::with_algo(&c, &s, ExtAlgo::Tree);
    assert_eq!(ec.algo(), ExtAlgo::Tree);
}

/// `points()` consistency check: distance between reported points == stored dist.
#[test]
fn extcs_points_consistent() {
    let c = make_line_adaptor(Pnt::new(0.0, 0.0, 4.0), Pnt::new(1.0, 0.0, 0.0), -5.0, 5.0);
    let s = xy_plane();
    let ec = ExtCS::new(&c, &s);
    assert!(ec.nb_ext() >= 1);
    let (pc, ps) = ec.points(1);
    let d_actual = pc.point().distance(ps.point());
    let d_stored = ec.square_distance(1).sqrt();
    near(d_actual, d_stored, 1e-9, "points() vs square_distance() consistency");
}

// ────────────────────────────────────────────────────────────────────────────
// ExtSS — Surface–Surface
// ────────────────────────────────────────────────────────────────────────────

/// Two parallel planes 6 units apart → min distance = 6.
#[test]
fn extss_parallel_planes_distance() {
    let s1 = GeomAdaptorSurface::from_plane(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    let s2 = GeomAdaptorSurface::from_plane(
        [0.0, 0.0, 6.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    let es = ExtSS::new(&s1, &s2);
    assert!(es.is_done());
    near(es.min_distance().unwrap(), 6.0, 1e-5, "parallel planes distance");
}

/// Same plane twice → min distance = 0.
#[test]
fn extss_same_plane_zero_distance() {
    let s1 = GeomAdaptorSurface::from_plane(
        [1.0, 2.0, 3.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    let s2 = GeomAdaptorSurface::from_plane(
        [1.0, 2.0, 3.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    let es = ExtSS::new(&s1, &s2);
    assert!(es.is_done());
    near(es.min_distance().unwrap(), 0.0, 1e-6, "coincident planes min dist");
}

/// `algo()` accessor.
#[test]
fn extss_algo_accessor() {
    let s1 = xy_plane();
    let s2 = GeomAdaptorSurface::from_plane(
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    let es = ExtSS::with_algo(&s1, &s2, ExtAlgo::Tree);
    assert_eq!(es.algo(), ExtAlgo::Tree);
}

/// `points()` consistency check.
#[test]
fn extss_points_consistent() {
    let s1 = GeomAdaptorSurface::from_plane(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    let s2 = GeomAdaptorSurface::from_plane(
        [0.0, 0.0, 3.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    let es = ExtSS::new(&s1, &s2);
    assert!(es.nb_ext() >= 1);
    let (p1, p2) = es.points(1);
    let d_actual = p1.point().distance(p2.point());
    let d_stored = es.square_distance(1).sqrt();
    near(d_actual, d_stored, 1e-9, "points() vs square_distance() consistency");
}

/// Cylinder surface vs. parallel plane — min distance equals gap.
#[test]
fn extss_cylinder_vs_plane_distance() {
    // Cylinder: radius 1, axis along Z, centred at origin.
    // Plane at y = 4 → min gap = 4 - 1 = 3.
    let s_cyl = GeomAdaptorSurface::from_cylinder(
        [0.0, 0.0, 0.0],     // origin
        [0.0, 0.0, 1.0],     // axis (Z)
        [1.0, 0.0, 0.0],     // x_axis
        1.0,                  // radius
    );
    let s_pln = GeomAdaptorSurface::from_plane(
        [0.0, 4.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0],
    );
    let es = ExtSS::new(&s_cyl, &s_pln);
    assert!(es.is_done());
    near(es.min_distance().unwrap(), 3.0, 0.05, "cylinder-plane distance");
}
