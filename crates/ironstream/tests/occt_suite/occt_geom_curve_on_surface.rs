// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_curve_on_surface.rs

//! Tests for `geom_curve_on_surface` — the 2-D-to-3-D lifting stub.
//!
//! Each test exercises one aspect of `CurveOnSurface` and `CurveOnSurfaceType`,
//! mirroring the style of the OCCT GTest suite used elsewhere in IronStream.

extern crate ironstream;
use ironstream::geom_curve_on_surface::*;

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// A simple linear p-curve in [0, 1] with 3 poles, degree 1.
/// Poles: (0, 0), (0.5, 0.5), (1, 1).
fn linear_curve() -> CurveOnSurface {
    CurveOnSurface::new(
        vec![0.0, 0.5, 1.0],
        vec![0.0, 0.5, 1.0],
        vec![0.0, 1.0],
        vec![2, 2],
        1,
        "Plane",
    )
}

/// A constant p-curve (single pole) for edge cases.
fn single_pole_curve() -> CurveOnSurface {
    CurveOnSurface::new(
        vec![3.0],
        vec![7.0],
        vec![0.0, 1.0],
        vec![1, 1],
        1,
        "Cylinder",
    )
}

// ---------------------------------------------------------------------------
// constructor / accessors
// ---------------------------------------------------------------------------

#[test]
fn new_sets_not_periodic() {
    let c = linear_curve();
    assert!(!c.is_periodic());
}

#[test]
fn new_sets_curve_type_pcurve2d() {
    let c = linear_curve();
    assert_eq!(c.curve_type(), CurveOnSurfaceType::Pcurve2d);
}

#[test]
fn new_sets_surface_type_string() {
    let c = linear_curve();
    assert_eq!(c.surface_type, "Plane");
}

#[test]
fn new_stores_degree() {
    let c = linear_curve();
    assert_eq!(c.degree(), 1);
}

#[test]
fn nb_poles_returns_pole_count() {
    let c = linear_curve();
    assert_eq!(c.nb_poles(), 3);
}

#[test]
fn u_pole_returns_correct_value() {
    let c = linear_curve();
    assert!((c.u_pole(0) - 0.0).abs() < 1e-12);
    assert!((c.u_pole(1) - 0.5).abs() < 1e-12);
    assert!((c.u_pole(2) - 1.0).abs() < 1e-12);
}

#[test]
fn v_pole_returns_correct_value() {
    let c = linear_curve();
    assert!((c.v_pole(0) - 0.0).abs() < 1e-12);
    assert!((c.v_pole(1) - 0.5).abs() < 1e-12);
    assert!((c.v_pole(2) - 1.0).abs() < 1e-12);
}

// ---------------------------------------------------------------------------
// first_param / last_param
// ---------------------------------------------------------------------------

#[test]
fn first_param_is_min_knot() {
    let c = linear_curve();
    assert!((c.first_param() - 0.0).abs() < 1e-12);
}

#[test]
fn last_param_is_max_knot() {
    let c = linear_curve();
    assert!((c.last_param() - 1.0).abs() < 1e-12);
}

#[test]
fn first_last_param_with_nonzero_start() {
    let c = CurveOnSurface::new(
        vec![0.0, 1.0],
        vec![0.0, 2.0],
        vec![2.0, 5.0],
        vec![2, 2],
        1,
        "Plane",
    );
    assert!((c.first_param() - 2.0).abs() < 1e-12);
    assert!((c.last_param() - 5.0).abs() < 1e-12);
}

// ---------------------------------------------------------------------------
// evaluate_uv — piecewise-linear interpolation
// ---------------------------------------------------------------------------

#[test]
fn evaluate_uv_at_first_param_returns_first_pole() {
    let c = linear_curve();
    let (u, v) = c.evaluate_uv(0.0);
    assert!((u - 0.0).abs() < 1e-10, "u={u}");
    assert!((v - 0.0).abs() < 1e-10, "v={v}");
}

#[test]
fn evaluate_uv_at_last_param_returns_last_pole() {
    let c = linear_curve();
    let (u, v) = c.evaluate_uv(1.0);
    assert!((u - 1.0).abs() < 1e-10, "u={u}");
    assert!((v - 1.0).abs() < 1e-10, "v={v}");
}

#[test]
fn evaluate_uv_at_midpoint() {
    let c = linear_curve();
    let (u, v) = c.evaluate_uv(0.5);
    // midpoint of linear diagonal → (0.5, 0.5)
    assert!((u - 0.5).abs() < 1e-10, "u={u}");
    assert!((v - 0.5).abs() < 1e-10, "v={v}");
}

#[test]
fn evaluate_uv_clamps_below_first_param() {
    let c = linear_curve();
    let (u, v) = c.evaluate_uv(-5.0);
    assert!((u - 0.0).abs() < 1e-10, "u={u}");
    assert!((v - 0.0).abs() < 1e-10, "v={v}");
}

#[test]
fn evaluate_uv_clamps_above_last_param() {
    let c = linear_curve();
    let (u, v) = c.evaluate_uv(99.0);
    assert!((u - 1.0).abs() < 1e-10, "u={u}");
    assert!((v - 1.0).abs() < 1e-10, "v={v}");
}

#[test]
fn evaluate_uv_single_pole_returns_that_pole() {
    let c = single_pole_curve();
    let (u, v) = c.evaluate_uv(0.5);
    assert!((u - 3.0).abs() < 1e-10, "u={u}");
    assert!((v - 7.0).abs() < 1e-10, "v={v}");
}

#[test]
fn evaluate_uv_two_pole_quarter_point() {
    let c = CurveOnSurface::new(
        vec![0.0, 4.0],
        vec![0.0, 8.0],
        vec![0.0, 1.0],
        vec![2, 2],
        1,
        "Plane",
    );
    let (u, v) = c.evaluate_uv(0.25);
    assert!((u - 1.0).abs() < 1e-10, "u={u}");
    assert!((v - 2.0).abs() < 1e-10, "v={v}");
}

// ---------------------------------------------------------------------------
// evaluate_3d — stub lifting
// ---------------------------------------------------------------------------

#[test]
fn evaluate_3d_at_origin_with_unit_normal() {
    let c = linear_curve();
    let pt = c.evaluate_3d(0.0, [1.0, 1.0, 1.0]);
    // u=0, v=0 → [0*1, 0*1, (0+0)*1] = [0, 0, 0]
    assert!((pt[0] - 0.0).abs() < 1e-10);
    assert!((pt[1] - 0.0).abs() < 1e-10);
    assert!((pt[2] - 0.0).abs() < 1e-10);
}

#[test]
fn evaluate_3d_at_end_with_unit_normal() {
    let c = linear_curve();
    let pt = c.evaluate_3d(1.0, [1.0, 1.0, 1.0]);
    // u=1, v=1 → [1*1, 1*1, (1+1)*1] = [1, 1, 2]
    assert!((pt[0] - 1.0).abs() < 1e-10, "x={}", pt[0]);
    assert!((pt[1] - 1.0).abs() < 1e-10, "y={}", pt[1]);
    assert!((pt[2] - 2.0).abs() < 1e-10, "z={}", pt[2]);
}

#[test]
fn evaluate_3d_normal_scales_components_independently() {
    let c = CurveOnSurface::new(
        vec![2.0, 2.0],
        vec![3.0, 3.0],
        vec![0.0, 1.0],
        vec![2, 2],
        1,
        "Plane",
    );
    // u=2, v=3 everywhere; normal = (10, 100, 1000)
    let pt = c.evaluate_3d(0.5, [10.0, 100.0, 1000.0]);
    assert!((pt[0] - 20.0).abs() < 1e-9, "x={}", pt[0]);
    assert!((pt[1] - 300.0).abs() < 1e-9, "y={}", pt[1]);
    assert!((pt[2] - 5000.0).abs() < 1e-9, "z={}", pt[2]);
}

#[test]
fn evaluate_3d_zero_normal_gives_zero_point() {
    let c = linear_curve();
    let pt = c.evaluate_3d(0.5, [0.0, 0.0, 0.0]);
    assert_eq!(pt, [0.0, 0.0, 0.0]);
}

// ---------------------------------------------------------------------------
// CurveOnSurfaceType — enum coverage
// ---------------------------------------------------------------------------

#[test]
fn curve_on_surface_type_equality() {
    assert_eq!(CurveOnSurfaceType::Pcurve2d, CurveOnSurfaceType::Pcurve2d);
    assert_ne!(CurveOnSurfaceType::Restriction, CurveOnSurfaceType::Boundary);
}

#[test]
fn curve_on_surface_type_copy() {
    let t = CurveOnSurfaceType::Restriction;
    let t2 = t;
    assert_eq!(t, t2);
}

#[test]
fn can_set_curve_type_manually() {
    let mut c = linear_curve();
    c.curve_type = CurveOnSurfaceType::Boundary;
    assert_eq!(c.curve_type(), CurveOnSurfaceType::Boundary);
}

// ---------------------------------------------------------------------------
// clone / debug
// ---------------------------------------------------------------------------

#[test]
fn curve_on_surface_clone_is_independent() {
    let c = linear_curve();
    let mut c2 = c.clone();
    c2.u_poles[0] = 99.0;
    // original is unaffected
    assert!((c.u_pole(0) - 0.0).abs() < 1e-12);
}

#[test]
fn curve_on_surface_debug_does_not_panic() {
    let c = linear_curve();
    let _ = format!("{:?}", c);
}
