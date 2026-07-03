// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_bound_surface.rs
//! Tests for `GeomBoundBox`, `GeomSurfaceProperties`, and
//! `GeomCurveProperties` from the `geom_bound_surface` module.

extern crate ironstream;
use ironstream::geom_bound_surface::*;

// ─────────────────────────── GeomBoundBox ────────────────────────────────────

#[test]
fn bound_box_getters() {
    let bb = GeomBoundBox::new(0.0, 1.0, -2.0, 3.0);
    assert_eq!(bb.u_min(), 0.0);
    assert_eq!(bb.u_max(), 1.0);
    assert_eq!(bb.v_min(), -2.0);
    assert_eq!(bb.v_max(), 3.0);
}

#[test]
fn bound_box_ranges() {
    let bb = GeomBoundBox::new(1.0, 4.0, -1.0, 2.0);
    assert!((bb.u_range() - 3.0).abs() < 1e-15);
    assert!((bb.v_range() - 3.0).abs() < 1e-15);
}

#[test]
fn bound_box_u_range_zero() {
    let bb = GeomBoundBox::new(5.0, 5.0, 0.0, 1.0);
    assert!((bb.u_range()).abs() < 1e-15);
}

#[test]
fn bound_box_contains_uv_inside() {
    let bb = GeomBoundBox::new(0.0, 2.0, 0.0, 2.0);
    assert!(bb.contains_uv(1.0, 1.0));
}

#[test]
fn bound_box_contains_uv_on_boundary() {
    let bb = GeomBoundBox::new(0.0, 1.0, 0.0, 1.0);
    assert!(bb.contains_uv(0.0, 0.0));
    assert!(bb.contains_uv(1.0, 1.0));
    assert!(bb.contains_uv(0.0, 1.0));
    assert!(bb.contains_uv(1.0, 0.0));
}

#[test]
fn bound_box_contains_uv_outside() {
    let bb = GeomBoundBox::new(0.0, 1.0, 0.0, 1.0);
    assert!(!bb.contains_uv(-0.1, 0.5));
    assert!(!bb.contains_uv(1.1, 0.5));
    assert!(!bb.contains_uv(0.5, -0.1));
    assert!(!bb.contains_uv(0.5, 1.1));
}

#[test]
fn bound_box_is_bounded_finite() {
    let bb = GeomBoundBox::new(-1.0, 1.0, -1.0, 1.0);
    assert!(bb.is_bounded());
}

#[test]
fn bound_box_is_bounded_infinite_u() {
    let bb = GeomBoundBox::new(f64::NEG_INFINITY, f64::INFINITY, 0.0, 1.0);
    assert!(!bb.is_bounded());
}

#[test]
fn bound_box_is_bounded_infinite_v() {
    let bb = GeomBoundBox::new(0.0, 1.0, f64::NEG_INFINITY, f64::INFINITY);
    assert!(!bb.is_bounded());
}

#[test]
fn bound_box_is_bounded_partial_infinite() {
    let bb = GeomBoundBox::new(0.0, f64::INFINITY, 0.0, 1.0);
    assert!(!bb.is_bounded());
}

#[test]
fn bound_box_negative_range_roundtrip() {
    // Caller may supply inverted bounds; we store and report faithfully.
    let bb = GeomBoundBox::new(3.0, 1.0, 2.0, 0.0);
    assert_eq!(bb.u_min(), 3.0);
    assert_eq!(bb.u_max(), 1.0);
    assert!((bb.u_range() - (-2.0)).abs() < 1e-15);
}

// ─────────────────────────── GeomSurfaceProperties ───────────────────────────

#[test]
fn surface_props_default_not_done() {
    let sp = GeomSurfaceProperties::new(0.5, 0.5);
    assert!(!sp.is_done());
    assert_eq!(sp.u, 0.5);
    assert_eq!(sp.v, 0.5);
}

#[test]
fn surface_props_set_and_get_point() {
    let mut sp = GeomSurfaceProperties::new(0.0, 0.0);
    sp.set_point([1.0, 2.0, 3.0]);
    assert_eq!(sp.point(), [1.0, 2.0, 3.0]);
}

#[test]
fn surface_props_set_and_get_normal() {
    let mut sp = GeomSurfaceProperties::new(0.0, 0.0);
    sp.set_normal([0.0, 0.0, 1.0]);
    assert_eq!(sp.normal(), [0.0, 0.0, 1.0]);
}

#[test]
fn surface_props_curvatures() {
    let mut sp = GeomSurfaceProperties::new(0.0, 0.0);
    sp.set_curvatures(-1.0, 2.0);
    assert!((sp.mean_curvature() - 0.5).abs() < 1e-15);
    assert!((sp.gaussian_curvature() - (-2.0)).abs() < 1e-15);
}

#[test]
fn surface_props_mean_curvature_symmetric() {
    let mut sp = GeomSurfaceProperties::new(0.0, 0.0);
    sp.set_curvatures(3.0, 3.0);
    assert!((sp.mean_curvature() - 3.0).abs() < 1e-15);
    assert!((sp.gaussian_curvature() - 9.0).abs() < 1e-15);
}

#[test]
fn surface_props_mark_done() {
    let mut sp = GeomSurfaceProperties::new(1.0, 2.0);
    assert!(!sp.is_done());
    sp.mark_done();
    assert!(sp.is_done());
}

#[test]
fn surface_props_full_workflow() {
    let mut sp = GeomSurfaceProperties::new(0.25, 0.75);
    sp.set_point([4.0, 5.0, 6.0]);
    sp.set_normal([0.0, 1.0, 0.0]);
    sp.set_curvatures(0.5, 1.5);
    sp.mark_done();

    assert!(sp.is_done());
    assert_eq!(sp.point(), [4.0, 5.0, 6.0]);
    assert_eq!(sp.normal(), [0.0, 1.0, 0.0]);
    assert!((sp.mean_curvature() - 1.0).abs() < 1e-15);
    assert!((sp.gaussian_curvature() - 0.75).abs() < 1e-15);
}

#[test]
fn surface_props_zero_curvature_flat_surface() {
    let mut sp = GeomSurfaceProperties::new(0.0, 0.0);
    sp.set_curvatures(0.0, 0.0);
    assert!((sp.mean_curvature()).abs() < 1e-15);
    assert!((sp.gaussian_curvature()).abs() < 1e-15);
}

// ─────────────────────────── GeomCurveProperties ─────────────────────────────

#[test]
fn curve_props_default_not_done() {
    let cp = GeomCurveProperties::new(0.0);
    assert!(!cp.is_done());
    assert_eq!(cp.parameter, 0.0);
}

#[test]
fn curve_props_set_and_get_point() {
    let mut cp = GeomCurveProperties::new(1.0);
    cp.set_point([7.0, 8.0, 9.0]);
    assert_eq!(cp.point(), [7.0, 8.0, 9.0]);
}

#[test]
fn curve_props_set_and_get_tangent() {
    let mut cp = GeomCurveProperties::new(0.0);
    cp.set_tangent([1.0, 0.0, 0.0]);
    assert_eq!(cp.tangent(), [1.0, 0.0, 0.0]);
}

#[test]
fn curve_props_set_and_get_curvature() {
    let mut cp = GeomCurveProperties::new(0.0);
    cp.set_curvature(0.25);
    assert!((cp.curvature() - 0.25).abs() < 1e-15);
}

#[test]
fn curve_props_mark_done() {
    let mut cp = GeomCurveProperties::new(0.5);
    assert!(!cp.is_done());
    cp.mark_done();
    assert!(cp.is_done());
}

#[test]
fn curve_props_full_workflow() {
    let mut cp = GeomCurveProperties::new(std::f64::consts::PI / 2.0);
    cp.set_point([0.0, 1.0, 0.0]);
    cp.set_tangent([-1.0, 0.0, 0.0]);
    cp.set_curvature(1.0); // unit circle has curvature 1
    cp.mark_done();

    assert!(cp.is_done());
    assert_eq!(cp.point(), [0.0, 1.0, 0.0]);
    assert_eq!(cp.tangent(), [-1.0, 0.0, 0.0]);
    assert!((cp.curvature() - 1.0).abs() < 1e-15);
}

#[test]
fn curve_props_zero_curvature_straight_line() {
    let mut cp = GeomCurveProperties::new(0.0);
    cp.set_point([0.0, 0.0, 0.0]);
    cp.set_tangent([1.0, 0.0, 0.0]);
    cp.set_curvature(0.0);
    cp.mark_done();

    assert!((cp.curvature()).abs() < 1e-15);
}

#[test]
fn curve_props_negative_parameter() {
    let cp = GeomCurveProperties::new(-3.14);
    assert!((cp.parameter - (-3.14)).abs() < 1e-15);
}
