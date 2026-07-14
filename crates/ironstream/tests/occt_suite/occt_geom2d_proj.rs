// FILE: rust/ironstream/crates/ironstream/tests/occt_geom2d_proj.rs
//! Integration tests for `geom2d_proj` —
//! `Geom2dProjResult`, `Geom2dProjectPointOnCurve`, and `Geom2dProjLib`.

extern crate ironstream;
use ironstream::geom2d_proj::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

const TOL: f64 = 1e-10;

fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= TOL,
        "expected {a} ≈ {b} within {TOL}, diff={}",
        (a - b).abs()
    );
}

fn pt_near(a: [f64; 2], b: [f64; 2]) {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dist = (dx * dx + dy * dy).sqrt();
    assert!(
        dist <= TOL,
        "expected [{}, {}] ≈ [{}, {}] within {TOL}, dist={dist}",
        a[0], a[1], b[0], b[1]
    );
}

// ---------------------------------------------------------------------------
// Geom2dProjResult
// ---------------------------------------------------------------------------

#[test]
fn result_new_stores_fields() {
    let r = Geom2dProjResult::new([3.0, 4.0], 0.25, 2.5);
    pt_near(r.point(), [3.0, 4.0]);
    near(r.parameter(), 0.25);
    near(r.distance(), 2.5);
}

#[test]
fn result_zero_distance() {
    let r = Geom2dProjResult::new([0.0, 0.0], 0.0, 0.0);
    near(r.distance(), 0.0);
    near(r.parameter(), 0.0);
    pt_near(r.point(), [0.0, 0.0]);
}

#[test]
fn result_negative_coords_stored_correctly() {
    let r = Geom2dProjResult::new([-1.5, -2.5], 1.0, 3.5);
    pt_near(r.point(), [-1.5, -2.5]);
    near(r.parameter(), 1.0);
    near(r.distance(), 3.5);
}

// ---------------------------------------------------------------------------
// Geom2dProjectPointOnCurve
// ---------------------------------------------------------------------------

#[test]
fn project_not_done_before_perform() {
    let proj = Geom2dProjectPointOnCurve::new([1.0, 2.0]);
    assert!(!proj.is_done());
    assert_eq!(proj.nb_points(), 0);
}

#[test]
fn project_done_after_perform() {
    let mut proj = Geom2dProjectPointOnCurve::new([1.0, 2.0]);
    proj.perform();
    assert!(proj.is_done());
}

#[test]
fn project_nb_points_after_perform() {
    let mut proj = Geom2dProjectPointOnCurve::new([0.0, 0.0]);
    proj.perform();
    assert_eq!(proj.nb_points(), 1);
}

#[test]
fn project_stub_parameter_is_half() {
    let mut proj = Geom2dProjectPointOnCurve::new([5.0, 5.0]);
    proj.perform();
    near(proj.parameter(0), 0.5);
}

#[test]
fn project_stub_lower_distance_is_one() {
    let mut proj = Geom2dProjectPointOnCurve::new([7.0, -3.0]);
    proj.perform();
    near(proj.lower_distance(), 1.0);
}

#[test]
fn project_nearest_point_equals_query_point() {
    // The stub sets the foot-point equal to the query point.
    let q = [3.14, 2.72];
    let mut proj = Geom2dProjectPointOnCurve::new(q);
    proj.perform();
    pt_near(proj.nearest_point(), q);
}

#[test]
fn project_perform_accumulates_results() {
    // Calling perform twice pushes two results.
    let mut proj = Geom2dProjectPointOnCurve::new([0.0, 0.0]);
    proj.perform();
    proj.perform();
    assert_eq!(proj.nb_points(), 2);
}

// ---------------------------------------------------------------------------
// Geom2dProjLib::project_on_plane
// ---------------------------------------------------------------------------

#[test]
fn project_on_plane_point_on_line_unchanged() {
    // Point already on the line y=0 (normal = [0,1]) through origin.
    let result = Geom2dProjLib::project_on_plane([3.0, 0.0], [0.0, 0.0], [0.0, 1.0]);
    pt_near(result, [3.0, 0.0]);
}

#[test]
fn project_on_plane_above_line_drops_normal_component() {
    // Line through origin with normal [0,1] (the X-axis line y=0).
    // Point [2.0, 5.0] projects to [2.0, 0.0].
    let result = Geom2dProjLib::project_on_plane([2.0, 5.0], [0.0, 0.0], [0.0, 1.0]);
    pt_near(result, [2.0, 0.0]);
}

#[test]
fn project_on_plane_normal_x_axis() {
    // Normal [1,0] — line is the Y-axis.  Point [4.0, 7.0] → [0.0, 7.0].
    let result = Geom2dProjLib::project_on_plane([4.0, 7.0], [0.0, 0.0], [1.0, 0.0]);
    pt_near(result, [0.0, 7.0]);
}

#[test]
fn project_on_plane_offset_origin() {
    // Line through [1.0, 1.0] with normal [0.0, 1.0] (horizontal line y=1).
    // Point [3.0, 4.0] → [3.0, 1.0].
    let result = Geom2dProjLib::project_on_plane([3.0, 4.0], [1.0, 1.0], [0.0, 1.0]);
    pt_near(result, [3.0, 1.0]);
}

#[test]
fn project_on_plane_diagonal_normal() {
    // Normal [1,1]/sqrt(2) — line through origin at 45 degrees.
    // Point [1.0, 0.0]: dot with normalised normal = 1/sqrt(2).
    // Projection = [1,0] - (1/sqrt(2))*[1/sqrt(2),1/sqrt(2)] = [1-0.5, 0-0.5] = [0.5, -0.5].
    let result = Geom2dProjLib::project_on_plane([1.0, 0.0], [0.0, 0.0], [1.0, 1.0]);
    let tol = 1e-10;
    assert!((result[0] - 0.5).abs() < tol, "x={}", result[0]);
    assert!((result[1] - (-0.5)).abs() < tol, "y={}", result[1]);
}

#[test]
fn project_on_plane_degenerate_normal_returns_point() {
    // Zero normal — should return the point unchanged.
    let result = Geom2dProjLib::project_on_plane([2.0, 3.0], [0.0, 0.0], [0.0, 0.0]);
    pt_near(result, [2.0, 3.0]);
}

// ---------------------------------------------------------------------------
// Geom2dProjLib::curve_parameter_at_point
// ---------------------------------------------------------------------------

#[test]
fn curve_param_start_of_segment() {
    // Point at the start → t = 0.
    let t = Geom2dProjLib::curve_parameter_at_point([0.0, 0.0], [4.0, 0.0], [0.0, 0.0]);
    near(t, 0.0);
}

#[test]
fn curve_param_end_of_segment() {
    // Point at the end → t = 1.
    let t = Geom2dProjLib::curve_parameter_at_point([0.0, 0.0], [4.0, 0.0], [4.0, 0.0]);
    near(t, 1.0);
}

#[test]
fn curve_param_midpoint() {
    // Point at the midpoint → t = 0.5.
    let t = Geom2dProjLib::curve_parameter_at_point([0.0, 0.0], [4.0, 0.0], [2.0, 0.0]);
    near(t, 0.5);
}

#[test]
fn curve_param_quarter_point() {
    // Point at 1/4 along [0,0]->[8,0] → t = 0.25.
    let t = Geom2dProjLib::curve_parameter_at_point([0.0, 0.0], [8.0, 0.0], [2.0, 0.0]);
    near(t, 0.25);
}

#[test]
fn curve_param_diagonal_segment() {
    // Segment [0,0]->[3,4] (length 5).  Point [1.5, 2.0] is at t = 0.5.
    let t = Geom2dProjLib::curve_parameter_at_point([0.0, 0.0], [3.0, 4.0], [1.5, 2.0]);
    near(t, 0.5);
}

#[test]
fn curve_param_before_start_clamped_to_zero() {
    // Point "before" the segment — clamped to 0.
    let t = Geom2dProjLib::curve_parameter_at_point([1.0, 0.0], [5.0, 0.0], [0.0, 0.0]);
    near(t, 0.0);
}

#[test]
fn curve_param_after_end_clamped_to_one() {
    // Point "past" the end — clamped to 1.
    let t = Geom2dProjLib::curve_parameter_at_point([0.0, 0.0], [3.0, 0.0], [10.0, 0.0]);
    near(t, 1.0);
}

#[test]
fn curve_param_degenerate_segment_returns_zero() {
    // Zero-length segment — must not panic, returns 0.
    let t = Geom2dProjLib::curve_parameter_at_point([2.0, 2.0], [2.0, 2.0], [5.0, 7.0]);
    near(t, 0.0);
}

#[test]
fn curve_param_off_axis_point_uses_projection() {
    // Point not on the line — the perpendicular foot is still found via dot product.
    // Segment [0,0]->[4,0].  Point [2.0, 99.0] (above the line) → foot at [2,0] → t=0.5.
    let t = Geom2dProjLib::curve_parameter_at_point([0.0, 0.0], [4.0, 0.0], [2.0, 99.0]);
    near(t, 0.5);
}
