// FILE: tests/occt_geom_api.rs
//! Integration tests for `geom_api` — GeomAPI algorithms.
//!
//! Tests exercise the public API only (no access to private fields).
//! Tolerances mirror OCCT test-suite conventions.

extern crate ironstream;

use ironstream::geom_api::{
    GeomApiInterpolate, GeomApiPointsToBSpline, GeomApiProjectPointOnCurve,
    GeomApiProjectPointOnSurf, TangentConstraint,
};
use ironstream::geom_bspline_curve::GeomBSplineCurve;
use ironstream::geom_bspline_surface::GeomBSplineSurface;
use ironstream::gp::Pnt;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

const TOL: f64 = 1e-6;

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

/// Chord-length parameters for a list of points (mirrors the internal helper).
fn chord_params(pts: &[Pnt]) -> Vec<f64> {
    let n = pts.len();
    let mut raw = vec![0.0f64; n];
    for i in 1..n {
        raw[i] = raw[i - 1] + pts[i].distance(pts[i - 1]);
    }
    let total = raw[n - 1];
    if total < 1e-15 {
        return (0..n).map(|i| i as f64 / (n - 1).max(1) as f64).collect();
    }
    raw.iter().map(|&v| v / total).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomAPI_Interpolate — integration tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn interpolate_two_points_creates_linear_curve() {
    let p0 = Pnt::new(0.0, 0.0, 0.0);
    let p1 = Pnt::new(5.0, 0.0, 0.0);
    let mut interp = GeomApiInterpolate::new(&[p0, p1], false);
    interp.perform();
    assert!(interp.is_done(), "two-point interpolation must succeed");
    let c = interp.curve().unwrap();
    // A two-point interpolation yields a degree-1 curve.
    assert_eq!(c.degree(), 1, "two-point curve must be degree 1");
    pnt_near(c.start_point(), p0, TOL);
    pnt_near(c.end_point(), p1, TOL);
}

#[test]
fn interpolate_four_points_passes_through_all() {
    let pts = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 2.0, 0.0),
        Pnt::new(2.5, 1.0, 0.0),
        Pnt::new(4.0, 0.0, 0.0),
    ];
    let mut interp = GeomApiInterpolate::new(&pts, false);
    interp.perform();
    assert!(interp.is_done(), "four-point interpolation must succeed");
    let c = interp.curve().unwrap();

    // Curve must pass through all four data points at the chord-length params.
    let t = chord_params(&pts);
    for (i, &expected) in pts.iter().enumerate() {
        let got = c.value(t[i]);
        pnt_near(got, expected, 5e-6);
    }
}

#[test]
fn interpolate_degree_bounded_by_point_count() {
    // Three points → degree should be at most 3 but never exceed n-1=2.
    let pts = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let mut interp = GeomApiInterpolate::new(&pts, false);
    interp.perform();
    assert!(interp.is_done());
    let d = interp.curve().unwrap().degree();
    assert!(d >= 1 && d <= 3, "degree {d} out of expected range");
}

#[test]
fn interpolate_not_done_on_empty_input() {
    let mut interp = GeomApiInterpolate::new(&[], false);
    interp.perform();
    assert!(!interp.is_done(), "empty input must not succeed");
    assert!(interp.curve().is_none());
}

#[test]
fn interpolate_closed_curve_endpoint_coincidence() {
    // A closed (periodic) interpolation should have start == end.
    let pts = vec![
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(-1.0, 0.0, 0.0),
        Pnt::new(0.0, -1.0, 0.0),
    ];
    let mut interp = GeomApiInterpolate::new(&pts, true);
    interp.perform();
    assert!(interp.is_done(), "closed interpolation must succeed");
    let c = interp.curve().unwrap();
    pnt_near(c.start_point(), c.end_point(), TOL);
}

#[test]
fn interpolate_tangent_load_does_not_break_done_state() {
    let pts = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
    ];
    let mut interp = GeomApiInterpolate::new(&pts, false);
    // Load tangents before performing.
    interp.load_tangents(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    interp.perform();
    // Must still succeed (with or without perfect tangent satisfaction).
    assert!(interp.is_done(), "tangent-constrained interpolation must be done");
    let c = interp.curve().unwrap();
    assert!(c.degree() >= 1 && c.degree() <= 3);
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomAPI_PointsToBSpline — integration tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn points_to_bspline_approximates_collinear_points() {
    // Points on the x-axis — the approximation should land near the line y=0.
    let pts: Vec<Pnt> =
        (0..6).map(|i| Pnt::new(i as f64, 0.0, 0.0)).collect();
    let fitter = GeomApiPointsToBSpline::new(&pts, 1, 5, 3, 1e-3);
    assert!(fitter.is_done(), "approximation of collinear points must succeed");
    let c = fitter.curve().unwrap();
    // Mid-curve should be near the line.
    let mid = c.value(0.5);
    near(mid.y, 0.0, 1e-4);
    near(mid.z, 0.0, 1e-4);
}

#[test]
fn points_to_bspline_from_degree_interpolates_endpoints() {
    let pts = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 2.0, 0.0),
        Pnt::new(3.0, 1.0, 0.0),
        Pnt::new(4.0, 0.0, 0.0),
    ];
    let fitter = GeomApiPointsToBSpline::from_degree(&pts, 3);
    assert!(fitter.is_done(), "degree-3 approximation must succeed");
    let c = fitter.curve().unwrap();
    // A clamped B-spline's start/end poles coincide with first/last data point.
    pnt_near(c.start_point(), pts[0], TOL);
    pnt_near(c.end_point(), *pts.last().unwrap(), TOL);
}

#[test]
fn points_to_bspline_too_few_points_returns_not_done() {
    let pts = vec![Pnt::new(0.0, 0.0, 0.0)];
    let fitter = GeomApiPointsToBSpline::new(&pts, 1, 5, 3, 1e-3);
    assert!(!fitter.is_done(), "single-point fitting must not succeed");
}

#[test]
fn points_to_bspline_degree_clamped_to_point_count() {
    let pts = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    // Request degree 10, should be silently clamped.
    let fitter = GeomApiPointsToBSpline::from_degree(&pts, 10);
    assert!(fitter.is_done(), "degree-clamped fitting must succeed");
    let c = fitter.curve().unwrap();
    assert!(c.degree() <= 2, "degree must be clamped to n-1");
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomAPI_ProjectPointOnCurve — integration tests
// ─────────────────────────────────────────────────────────────────────────────

/// Helper: build a simple degree-1 segment from (x0,0,0) to (x1,0,0).
fn segment(x0: f64, x1: f64) -> GeomBSplineCurve {
    let poles = [Pnt::new(x0, 0.0, 0.0), Pnt::new(x1, 0.0, 0.0)];
    GeomBSplineCurve::new(&poles, &[0.0, 1.0], &[2, 2], 1, false)
}

#[test]
fn project_midpoint_above_segment() {
    // Segment from 0 to 10 along x-axis; query point directly above mid.
    let curve = segment(0.0, 10.0);
    let q = Pnt::new(5.0, 3.0, 0.0);
    let proj = GeomApiProjectPointOnCurve::new(q, &curve);
    assert!(proj.nb_points() >= 1, "must find at least one projection");
    let foot = proj.nearest_point().unwrap();
    near(foot.x, 5.0, 1e-4);
    near(foot.y, 0.0, 1e-4);
    near(proj.lower_distance().unwrap(), 3.0, 1e-4);
}

#[test]
fn project_endpoint_of_segment() {
    // Query at the end of the segment — projection should be the endpoint.
    let curve = segment(0.0, 4.0);
    let q = Pnt::new(4.0, 0.0, 0.0);
    let proj = GeomApiProjectPointOnCurve::new(q, &curve);
    assert!(proj.nb_points() >= 1);
    near(proj.lower_distance().unwrap(), 0.0, 1e-5);
}

#[test]
fn project_curve_parameter_in_domain() {
    let poles = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 2.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let curve = GeomBSplineCurve::new(&poles, &[0.0, 1.0], &[3, 3], 2, false);
    let q = Pnt::new(1.0, 3.0, 0.0);
    let proj = GeomApiProjectPointOnCurve::new(q, &curve);
    assert!(proj.nb_points() >= 1);
    let u = proj.lower_distance_parameter().unwrap();
    assert!(
        u >= curve.first_parameter() - 1e-9 && u <= curve.last_parameter() + 1e-9,
        "parameter {u} outside curve domain"
    );
}

#[test]
fn project_1based_accessors_consistent() {
    let curve = segment(0.0, 6.0);
    let q = Pnt::new(3.0, 2.0, 0.0);
    let proj = GeomApiProjectPointOnCurve::new(q, &curve);
    assert!(proj.nb_points() >= 1);
    let foot = proj.point(1).expect("index 1 must be valid");
    let param = proj.parameter(1).expect("parameter 1 must be valid");
    let dist = proj.distance(1).expect("distance 1 must be valid");
    // Consistency checks.
    pnt_near(foot, curve.value(param), 1e-5);
    near(dist, q.distance(foot), 1e-9);
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomAPI_ProjectPointOnSurf — integration tests
// ─────────────────────────────────────────────────────────────────────────────

/// Helper: build a flat bilinear patch over [0,1] x [0,1] in the XY plane.
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

#[test]
fn project_point_above_flat_patch_finds_nearest() {
    let surf = flat_patch();
    // Point directly above the center of the patch.
    let q = Pnt::new(0.5, 0.5, 3.0);
    let proj = GeomApiProjectPointOnSurf::new(q, &surf);
    assert!(proj.nb_points() >= 1, "must find at least one projection");
    let foot = proj.nearest_point().unwrap();
    // The foot must lie on the XY plane (z ≈ 0) at (0.5, 0.5).
    near(foot.z, 0.0, 1e-4);
    near(foot.x, 0.5, 1e-3);
    near(foot.y, 0.5, 1e-3);
    near(proj.lower_distance().unwrap(), 3.0, 1e-3);
}

#[test]
fn project_point_on_patch_distance_near_zero() {
    let surf = flat_patch();
    // Query a point that lies on the surface itself.
    let q = Pnt::new(0.3, 0.7, 0.0);
    let proj = GeomApiProjectPointOnSurf::new(q, &surf);
    assert!(proj.nb_points() >= 1);
    near(proj.lower_distance().unwrap(), 0.0, 1e-3);
}

#[test]
fn project_surf_parameters_in_domain() {
    let surf = flat_patch();
    let q = Pnt::new(0.5, 0.5, 1.0);
    let proj = GeomApiProjectPointOnSurf::new(q, &surf);
    assert!(proj.nb_points() >= 1);
    let (u, v) = proj.lower_distance_parameters().unwrap();
    assert!(u >= 0.0 - 1e-9 && u <= 1.0 + 1e-9, "u={u} outside [0,1]");
    assert!(v >= 0.0 - 1e-9 && v <= 1.0 + 1e-9, "v={v} outside [0,1]");
}

#[test]
fn project_surf_1based_accessors_consistent() {
    let surf = flat_patch();
    let q = Pnt::new(0.4, 0.6, 0.5);
    let proj = GeomApiProjectPointOnSurf::new(q, &surf);
    assert!(proj.nb_points() >= 1);
    let foot = proj.point(1).expect("point(1) must be valid");
    let (u, v) = proj.parameters(1).expect("parameters(1) must be valid");
    let dist = proj.distance(1).expect("distance(1) must be valid");
    pnt_near(foot, surf.value(u, v), 1e-4);
    near(dist, q.distance(foot), 1e-9);
}

#[test]
fn project_surf_all_results_sorted_ascending() {
    let surf = flat_patch();
    let q = Pnt::new(0.5, 0.5, 2.0);
    let proj = GeomApiProjectPointOnSurf::new(q, &surf);
    let results = proj.all_results();
    for w in results.windows(2) {
        assert!(
            w[0].distance <= w[1].distance + 1e-12,
            "results not sorted: {} > {}",
            w[0].distance,
            w[1].distance
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Cross-algorithm tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn interpolate_then_project_foot_on_curve() {
    // Interpolate four points, then project a query point and verify the foot
    // is closer to the curve than the query point.
    let pts = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.5, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
    ];
    let mut interp = GeomApiInterpolate::new(&pts, false);
    interp.perform();
    assert!(interp.is_done());
    let curve = interp.into_curve().unwrap();

    let q = Pnt::new(1.5, 2.0, 0.0);
    let proj = GeomApiProjectPointOnCurve::new(q, &curve);
    assert!(proj.nb_points() >= 1);
    let foot = proj.nearest_point().unwrap();
    let d = proj.lower_distance().unwrap();
    // Foot must be closer to q than any of the original data points.
    let min_data_dist = pts.iter().map(|p| q.distance(*p)).fold(f64::MAX, f64::min);
    assert!(d <= min_data_dist + 1e-6, "projection {d} should be <= nearest data dist {min_data_dist}");
    // Foot is on the curve: evaluate curve at the projection parameter.
    let u = proj.lower_distance_parameter().unwrap();
    let on_curve = curve.value(u);
    near(foot.distance(on_curve), 0.0, 1e-5);
}
