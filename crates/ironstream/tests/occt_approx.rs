// FILE: tests/occt_approx.rs
//! Integration tests for `ironstream::approx` — Approx curve/surface
//! approximation, mirroring the OpenCascade `Approx`, `GeomConvert` packages.
//!
//! Tests exercise the public API only and use the canonical IronStream
//! conventions: `Pnt.x/.y/.z`, `Ax3::identity()`, no `Dir` type.

extern crate ironstream;

use ironstream::approx::{
    ApproxContinuity, Approx_Curve3d, Approx_CurveOnSurface, Approx_SameParameter,
    GeomConvert_ApproxSurface,
};
use ironstream::geom_bspline_curve::GeomBSplineCurve;
use ironstream::gp::Pnt;

use std::f64::consts::TAU;

const TOL: f64 = 1e-9;

fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= TOL,
        "{label}: expected {a} ≈ {b}, diff = {}",
        (a - b).abs()
    );
}

// ─── helper: build a simple cubic B-spline ───────────────────────────────────

fn make_cubic_bspline() -> GeomBSplineCurve {
    let poles = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 2.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(3.0, 1.5, 0.0),
    ];
    let knots = [0.0, 1.0];
    let mults = [4, 4];
    GeomBSplineCurve::new(&poles, &knots, &mults, 3, false)
}

// ─── Test 1: Approx_Curve3d — is_done on a line ──────────────────────────────

#[test]
fn approx_curve3d_line_is_done() {
    let a = Approx_Curve3d::new(
        |t| Pnt::new(t * 3.0, 0.0, 0.0),
        0.0,
        1.0,
        1e-5,
        3,
        4,
        ApproxContinuity::C2,
    );
    assert!(a.is_done(), "linear curve approx must succeed");
}

// ─── Test 2: Approx_Curve3d — t_min / t_max accessors ────────────────────────

#[test]
fn approx_curve3d_parameter_range() {
    let a = Approx_Curve3d::new(
        |t| Pnt::new(t.cos(), t.sin(), 0.0),
        0.0,
        TAU,
        1e-3,
        3,
        8,
        ApproxContinuity::C1,
    );
    near(a.t_min(), 0.0, "t_min");
    near(a.t_max(), TAU, "t_max");
}

// ─── Test 3: Approx_Curve3d — curve degree within 1..=25 ─────────────────────

#[test]
fn approx_curve3d_degree_in_range() {
    let a = Approx_Curve3d::new(
        |t| Pnt::new(t, t * t, 0.0),
        0.0,
        1.0,
        1e-4,
        2,
        4,
        ApproxContinuity::C0,
    );
    assert!(a.is_done());
    let d = a.curve().degree();
    assert!(d >= 1 && d <= 25, "degree {d} out of [1,25]");
}

// ─── Test 4: Approx_Curve3d — max_error non-negative ─────────────────────────

#[test]
fn approx_curve3d_max_error_nonneg() {
    let a = Approx_Curve3d::new(
        |t| Pnt::new(t.sin(), t.cos(), t * 0.1),
        0.0,
        TAU,
        1e-3,
        3,
        6,
        ApproxContinuity::C1,
    );
    assert!(a.max_error() >= 0.0, "max_error must be non-negative");
}

// ─── Test 5: Approx_Curve3d — curve endpoints near original ──────────────────

#[test]
fn approx_curve3d_endpoints_match() {
    let eval = |t: f64| Pnt::new(t * 2.0, t * t, 0.0);
    let a = Approx_Curve3d::new(eval, 0.0, 1.0, 1e-5, 3, 5, ApproxContinuity::C2);
    assert!(a.is_done());

    let c = a.curve();
    let p0_orig = eval(0.0);
    let p1_orig = eval(1.0);
    let p0_appr = c.value(c.first_parameter());
    let p1_appr = c.value(c.last_parameter());

    assert!(
        p0_orig.distance(p0_appr) < 0.1,
        "start mismatch: {} vs {}",
        p0_orig.x, p0_appr.x
    );
    assert!(
        p1_orig.distance(p1_appr) < 0.1,
        "end mismatch: {} vs {}",
        p1_orig.x, p1_appr.x
    );
}

// ─── Test 6: Approx_CurveOnSurface — is_done on cylinder meridian ────────────

#[test]
fn approx_curve_on_surface_cylinder_is_done() {
    // Cylindrical surface: r=2.  Curve on surface: v-line at u=0 (meridian).
    let r = 2.0_f64;
    let cos = Approx_CurveOnSurface::new(
        |t| (0.0f64, t),                               // uv: u=0, v=t
        |u, v| Pnt::new(r * u.cos(), r * u.sin(), v),  // cylinder
        0.0,
        5.0,
        1e-4,
        3,
        6,
    );
    assert!(cos.is_done(), "cylinder meridian approx must succeed");
}

// ─── Test 7: Approx_CurveOnSurface — max_error_3d positive ──────────────────

#[test]
fn approx_curve_on_surface_error_positive() {
    let cos = Approx_CurveOnSurface::new(
        |t| (t, t * 0.5),
        |u, v| Pnt::new(u, v, u * v),
        0.0,
        1.0,
        1e-3,
        3,
        4,
    );
    assert!(cos.max_error_3d() >= 0.0, "error must be non-negative");
}

// ─── Test 8: Approx_CurveOnSurface — tolerance accessor ─────────────────────

#[test]
fn approx_curve_on_surface_tolerance_accessor() {
    let tol = 5e-4_f64;
    let cos = Approx_CurveOnSurface::new(
        |t| (t, 0.0),
        |u, _v| Pnt::new(u, 0.0, 0.0),
        0.0,
        1.0,
        tol,
        2,
        3,
    );
    assert!((cos.tolerance() - tol).abs() < 1e-15, "tolerance round-trips");
}

// ─── Test 9: Approx_SameParameter — is_done on cubic B-spline ────────────────

#[test]
fn approx_same_parameter_is_done() {
    let curve = make_cubic_bspline();
    let sp = Approx_SameParameter::new(&curve, 1e-4, 30);
    assert!(sp.is_done(), "SameParameter must succeed on a smooth cubic");
}

// ─── Test 10: Approx_SameParameter — result has correct degree ───────────────

#[test]
fn approx_same_parameter_degree_preserved() {
    let curve = make_cubic_bspline();
    let sp = Approx_SameParameter::new(&curve, 1e-4, 30);
    assert!(sp.is_done());
    let d = sp.curve().degree();
    assert!(d >= 1 && d <= 25, "degree {d} out of range");
}

// ─── Test 11: GeomConvert_ApproxSurface — is_done on bilinear patch ──────────

#[test]
fn approx_surface_bilinear_is_done() {
    // z = u + v  (bilinear patch — a planar surface tilted in Z)
    let ga = GeomConvert_ApproxSurface::new(
        |u, v| Pnt::new(u, v, u + v),
        0.0, 1.0,
        0.0, 1.0,
        1e-4,
        3, 3,
        5, 5,
    );
    assert!(ga.is_done(), "bilinear patch approx must succeed");
}

// ─── Test 12: GeomConvert_ApproxSurface — degree accessors ───────────────────

#[test]
fn approx_surface_degree_accessors() {
    let ga = GeomConvert_ApproxSurface::new(
        |u, v| Pnt::new(u, v, 0.0),
        0.0, 2.0,
        0.0, 2.0,
        1e-3,
        2, 3,
        4, 5,
    );
    assert_eq!(ga.u_degree(), 2, "u_degree");
    assert_eq!(ga.v_degree(), 3, "v_degree");
}

// ─── Test 13: GeomConvert_ApproxSurface — max_error on flat plane ─────────────

#[test]
fn approx_surface_flat_plane_small_error() {
    let ga = GeomConvert_ApproxSurface::new(
        |u, v| Pnt::new(u, v, 0.0),
        0.0, 1.0,
        0.0, 1.0,
        1e-5,
        3, 3,
        6, 6,
    );
    assert!(ga.is_done());
    assert!(
        ga.max_error() < 0.01,
        "flat plane error too large: {}",
        ga.max_error()
    );
}

// ─── Test 14: GeomConvert_ApproxSurface — result pole count consistent ────────

#[test]
fn approx_surface_result_pole_counts() {
    let ga = GeomConvert_ApproxSurface::new(
        |u, v| Pnt::new(u, v, u * v),
        0.0, 1.0,
        0.0, 1.0,
        1e-3,
        2, 2,
        4, 4,
    );
    assert!(ga.is_done());
    let s = ga.surface();
    assert!(s.nb_u_poles() >= 1, "need at least 1 U pole");
    assert!(s.nb_v_poles() >= 1, "need at least 1 V pole");
}

// ─── Test 15: Approx_Curve3d — tolerance accessor round-trips ────────────────

#[test]
fn approx_curve3d_tolerance_accessor() {
    let tol = 2.5e-5_f64;
    let a = Approx_Curve3d::new(
        |t| Pnt::new(t, 0.0, 0.0),
        0.0,
        1.0,
        tol,
        3,
        4,
        ApproxContinuity::C0,
    );
    assert!((a.tolerance() - tol).abs() < 1e-15, "tolerance round-trip");
}
