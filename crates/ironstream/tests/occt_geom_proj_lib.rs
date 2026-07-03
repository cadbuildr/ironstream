// FILE: tests/occt_geom_proj_lib.rs
//! Integration tests for `geom_proj_lib` — `GeomProjLib` projection utilities.
//!
//! Tests exercise the public API only (no access to private fields).
//! Tolerances mirror OCCT test-suite conventions.

extern crate ironstream;

use ironstream::geom_bspline_curve::GeomBSplineCurve;
use ironstream::geom_bspline_surface::GeomBSplineSurface;
use ironstream::geom_plane::GeomPlane;
use ironstream::geom_proj_lib::{GeomProjLib, ProjectOnPlane, ProjectOnSurface, Pnt2d};
use ironstream::gp::{Ax3, Pnt};

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

const TOL: f64 = 1e-4;

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} within {tol}, diff={}",
        (a - b).abs()
    );
}

fn pnt_near(a: Pnt, b: Pnt, tol: f64) {
    assert!(
        a.distance(b) <= tol,
        "expected {:?} ≈ {:?} within {tol}, dist={}",
        a,
        b,
        a.distance(b)
    );
}

/// Degree-1 segment along the x-axis at `y=0, z=0`.
fn segment_x(x0: f64, x1: f64) -> GeomBSplineCurve {
    let poles = [Pnt::new(x0, 0.0, 0.0), Pnt::new(x1, 0.0, 0.0)];
    GeomBSplineCurve::new(&poles, &[0.0, 1.0], &[2, 2], 1, false)
}

/// Degree-1 segment lifted to `z = h`.
fn segment_x_lifted(x0: f64, x1: f64, z: f64) -> GeomBSplineCurve {
    let poles = [Pnt::new(x0, 0.0, z), Pnt::new(x1, 0.0, z)];
    GeomBSplineCurve::new(&poles, &[0.0, 1.0], &[2, 2], 1, false)
}

/// Degree-2 arc roughly tracing a quarter-circle in the XY plane lifted to z=1.
fn arc_lifted() -> GeomBSplineCurve {
    let poles = [
        Pnt::new(1.0, 0.0, 2.0),
        Pnt::new(1.0, 1.0, 2.0),
        Pnt::new(0.0, 1.0, 2.0),
    ];
    GeomBSplineCurve::new(&poles, &[0.0, 1.0], &[3, 3], 2, false)
}

/// Flat bilinear B-spline patch over [0,1]×[0,1] at z=0.
fn flat_patch() -> GeomBSplineSurface {
    let poles = vec![
        vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0)],
        vec![Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)],
    ];
    GeomBSplineSurface::new(
        poles,
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![2, 2],
        vec![2, 2],
        1,
        1,
        false,
        false,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomProjLib — static project_point_on_plane
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn project_point_ortho_xy_plane_lands_at_z0() {
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let p = Pnt::new(3.0, 4.0, 7.0);
    let foot = GeomProjLib::project_point_on_plane(p, &plane);
    pnt_near(foot, Pnt::new(3.0, 4.0, 0.0), TOL);
}

#[test]
fn project_point_ortho_tilted_plane() {
    // Plane with normal (1,0,0) through origin — the YZ plane.
    let origin = Pnt::origin();
    let normal = Pnt::new(1.0, 0.0, 0.0);
    let plane = GeomPlane::from_point_normal(origin, normal);
    let p = Pnt::new(5.0, 2.0, 3.0);
    let foot = GeomProjLib::project_point_on_plane(p, &plane);
    // Foot should be at x=0, same y and z.
    near(foot.x, 0.0, TOL);
    near(foot.y, 2.0, TOL);
    near(foot.z, 3.0, TOL);
}

#[test]
fn project_point_directed_along_z() {
    // Directed projection: point at (2,3,5), direction -Z, onto z=0.
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let p = Pnt::new(2.0, 3.0, 5.0);
    let dir = Pnt::new(0.0, 0.0, -1.0);
    let foot = GeomProjLib::project_point_directed(p, &plane, dir).unwrap();
    near(foot.x, 2.0, TOL);
    near(foot.y, 3.0, TOL);
    near(foot.z, 0.0, TOL);
}

#[test]
fn project_point_parallel_direction_returns_none() {
    // Direction parallel to the plane → no projection.
    let plane = GeomPlane::from_ax3(Ax3::identity()); // z=0 plane
    let p = Pnt::new(1.0, 1.0, 5.0);
    let dir = Pnt::new(1.0, 0.0, 0.0); // lies in the plane
    let result = GeomProjLib::project_point_directed(p, &plane, dir);
    assert!(result.is_none(), "parallel direction must yield None");
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomProjLib::curve_2d — 2D parametric image on plane
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn curve_2d_length_matches_n_samples() {
    let curve = segment_x(0.0, 5.0);
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let uv = GeomProjLib::curve_2d(&curve, &plane, 10);
    assert_eq!(uv.len(), 10, "curve_2d must return exactly n_samples points");
}

#[test]
fn curve_2d_segment_on_xy_plane_v_is_zero() {
    // Segment on y=0,z=0 projected onto z=0 plane: V coords must be ≈ 0.
    let curve = segment_x(0.0, 4.0);
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let uv = GeomProjLib::curve_2d(&curve, &plane, 8);
    for p in &uv {
        near(p.v, 0.0, TOL);
    }
}

#[test]
fn curve_2d_u_range_matches_x_range_of_segment() {
    // Segment from x=0 to x=3 on z=0: U coords should span [0, 3].
    let curve = segment_x(0.0, 3.0);
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let uv = GeomProjLib::curve_2d(&curve, &plane, 6);
    // First point ≈ (0, 0), last point ≈ (3, 0).
    near(uv.first().unwrap().u, 0.0, 1e-3);
    near(uv.last().unwrap().u, 3.0, 1e-3);
}

#[test]
fn curve_2d_lifted_curve_z_stripped() {
    // Curve at z=7 above the XY plane: orthogonal projection strips z.
    let curve = segment_x_lifted(1.0, 5.0, 7.0);
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let uv = GeomProjLib::curve_2d(&curve, &plane, 5);
    // All V=0, U runs from 1 to 5.
    for p in &uv {
        near(p.v, 0.0, TOL);
    }
    near(uv[0].u, 1.0, 1e-3);
    near(uv[4].u, 5.0, 1e-3);
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomProjLib::curve_2d_bspline
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn curve_2d_bspline_endpoints_match_original() {
    // Project a segment from (0,0,3) to (6,0,3) onto z=0 plane.
    // The 2D result's X endpoints should be ≈ 0 and 6.
    let curve = segment_x_lifted(0.0, 6.0, 3.0);
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let c2d = GeomProjLib::curve_2d_bspline(&curve, &plane, 10, 2)
        .expect("curve_2d_bspline must succeed");
    let s = c2d.start_point();
    let e = c2d.end_point();
    near(s.x, 0.0, 5e-3);
    near(e.x, 6.0, 5e-3);
    // Y (≡ V) should be ≈ 0.
    near(s.y, 0.0, 5e-3);
    near(e.y, 0.0, 5e-3);
}

#[test]
fn curve_2d_bspline_degree_clamped() {
    // With only 3 samples and requested degree 5, result must still be valid.
    let curve = segment_x(0.0, 2.0);
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let c2d = GeomProjLib::curve_2d_bspline(&curve, &plane, 3, 5);
    assert!(c2d.is_some(), "must succeed even with clamped degree");
    assert!(c2d.unwrap().degree() >= 1);
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomProjLib::project_curve_on_plane (orthogonal)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn project_curve_on_plane_lifts_to_z0() {
    // Segment at z=5 projected orthogonally onto z=0: result z ≈ 0.
    let curve = segment_x_lifted(0.0, 4.0, 5.0);
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let proj = GeomProjLib::project_curve_on_plane(&curve, &plane, 12)
        .expect("orthogonal projection must succeed");
    // Evaluate mid-curve — z should be near zero.
    let mid_t =
        proj.first_parameter() + (proj.last_parameter() - proj.first_parameter()) * 0.5;
    let mid = proj.value(mid_t);
    near(mid.z, 0.0, 5e-3);
}

#[test]
fn project_curve_on_plane_preserves_x_extent() {
    // Segment from x=0 to x=8 above the plane: projected x range ≈ [0, 8].
    let curve = segment_x_lifted(0.0, 8.0, 3.0);
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let proj = GeomProjLib::project_curve_on_plane(&curve, &plane, 16)
        .expect("projection must succeed");
    let s = proj.start_point();
    let e = proj.end_point();
    near(s.x, 0.0, 5e-2);
    near(e.x, 8.0, 5e-2);
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomProjLib::project_curve_directed
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn directed_projection_arc_on_xy_plane() {
    // Quarter-circle arc at z=2, directed projection along -Z onto z=0.
    // The foot curve should lie in the z=0 plane.
    let curve = arc_lifted();
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let dir = Pnt::new(0.0, 0.0, -1.0);
    let proj = GeomProjLib::project_curve_directed(&curve, &plane, dir, 12)
        .expect("directed projection must succeed");
    // Every evaluated point on the result must have z ≈ 0.
    let t0 = proj.first_parameter();
    let t1 = proj.last_parameter();
    for i in 0..5 {
        let t = t0 + (t1 - t0) * i as f64 / 4.0;
        let p = proj.value(t);
        near(p.z, 0.0, 5e-3);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ProjectOnPlane — builder API
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn project_on_plane_builder_not_done_before_perform() {
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let proj = ProjectOnPlane::orthogonal(plane, 8);
    assert!(!proj.is_done(), "must not be done before perform");
    assert!(proj.projected_curve().is_none());
}

#[test]
fn project_on_plane_builder_done_after_perform() {
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let curve = segment_x_lifted(0.0, 3.0, 1.0);
    let mut proj = ProjectOnPlane::orthogonal(plane, 8);
    proj.perform(&curve);
    assert!(proj.is_done(), "must be done after perform on valid input");
    assert!(proj.projected_curve().is_some());
}

#[test]
fn project_on_plane_into_curve_consumes() {
    let plane = GeomPlane::from_ax3(Ax3::identity());
    let curve = segment_x_lifted(0.0, 2.0, 1.5);
    let mut proj = ProjectOnPlane::orthogonal(plane, 6);
    proj.perform(&curve);
    let c = proj.into_curve().expect("into_curve must return Some after perform");
    assert!(c.degree() >= 1);
}

// ─────────────────────────────────────────────────────────────────────────────
// ProjectOnSurface
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn project_on_flat_surface_foot_z_near_zero() {
    // Segment above a flat z=0 patch: projected foot should be at z≈0.
    let surface = flat_patch();
    let poles = [Pnt::new(0.1, 0.1, 3.0), Pnt::new(0.9, 0.9, 3.0)];
    let curve = GeomBSplineCurve::new(&poles, &[0.0, 1.0], &[2, 2], 1, false);
    let proj = GeomProjLib::project_curve_on_surface(&curve, &surface, 8)
        .expect("projection on flat surface must succeed");
    // Mid-curve z should be near 0.
    let mid_t =
        proj.first_parameter() + (proj.last_parameter() - proj.first_parameter()) * 0.5;
    let mid = proj.value(mid_t);
    near(mid.z, 0.0, 1e-3);
}

#[test]
fn project_on_surface_builder_api() {
    let surface = flat_patch();
    let curve = segment_x(0.1, 0.9);
    let mut proj = ProjectOnSurface::new(surface, 8, 2);
    assert!(!proj.is_done());
    proj.perform(&curve);
    assert!(proj.is_done(), "must be done after perform");
    assert!(proj.projected_curve().is_some());
}

// ─────────────────────────────────────────────────────────────────────────────
// Pnt2d
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn pnt2d_distance_correct() {
    let a = Pnt2d::new(0.0, 0.0);
    let b = Pnt2d::new(3.0, 4.0);
    near(a.distance(b), 5.0, 1e-12);
}

#[test]
fn pnt2d_fields_accessible() {
    let p = Pnt2d::new(7.5, -3.2);
    near(p.u, 7.5, 1e-12);
    near(p.v, -3.2, 1e-12);
}
