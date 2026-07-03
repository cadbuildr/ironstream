// FILE: tests/occt_adaptor3d.rs
extern crate ironstream;
use ironstream::adaptor3d::{
    Adaptor3dContinuity, Adaptor3dCurve, Adaptor3dCurveType, Adaptor3dSurface,
    Adaptor3dSurfaceType,
};
use core::f64::consts::PI;

// ---- curve: line ---------------------------------------------------------------

#[test]
fn line_adaptor_curve_type() {
    let c = Adaptor3dCurve::new(Adaptor3dCurveType::Line, 0.0, 10.0);
    assert_eq!(c.curve_type(), Adaptor3dCurveType::Line);
}

#[test]
fn line_adaptor_not_closed_not_periodic() {
    let c = Adaptor3dCurve::new(Adaptor3dCurveType::Line, 0.0, 10.0);
    assert!(!c.is_closed());
    assert!(!c.is_periodic());
}

#[test]
fn line_adaptor_period_is_none() {
    let c = Adaptor3dCurve::new(Adaptor3dCurveType::Line, 0.0, 10.0);
    assert!(c.period().is_none());
}

// ---- curve: circle -------------------------------------------------------------

#[test]
fn circle_adaptor_is_closed_and_periodic() {
    let c = Adaptor3dCurve::new(Adaptor3dCurveType::Circle, 0.0, 2.0 * PI);
    assert!(c.is_closed());
    assert!(c.is_periodic());
}

#[test]
fn circle_adaptor_period_is_two_pi() {
    let c = Adaptor3dCurve::new(Adaptor3dCurveType::Circle, 0.0, 2.0 * PI);
    let p = c.period().expect("circle must have a period");
    assert!((p - 2.0 * PI).abs() < 1e-15);
}

#[test]
fn circle_adaptor_parameter_range() {
    let c = Adaptor3dCurve::new(Adaptor3dCurveType::Circle, -1.0, 5.0);
    assert!((c.first_parameter() - (-1.0)).abs() < 1e-15);
    assert!((c.last_parameter() - 5.0).abs() < 1e-15);
}

// ---- curve: ellipse ------------------------------------------------------------

#[test]
fn ellipse_adaptor_is_closed_and_periodic() {
    let c = Adaptor3dCurve::new(Adaptor3dCurveType::Ellipse, 0.0, 2.0 * PI);
    assert!(c.is_closed());
    assert!(c.is_periodic());
    let p = c.period().unwrap();
    assert!((p - 2.0 * PI).abs() < 1e-15);
}

// ---- curve: bspline (non-periodic) ---------------------------------------------

#[test]
fn bspline_adaptor_not_closed() {
    let c = Adaptor3dCurve::new(Adaptor3dCurveType::BSplineCurve, 0.0, 1.0);
    assert!(!c.is_closed());
    assert!(!c.is_periodic());
    assert!(c.period().is_none());
}

// ---- curve: continuity builder -------------------------------------------------

#[test]
fn curve_with_continuity_overrides_default() {
    let c = Adaptor3dCurve::new(Adaptor3dCurveType::BSplineCurve, 0.0, 1.0)
        .with_continuity(Adaptor3dContinuity::C2);
    assert_eq!(c.continuity(), Adaptor3dContinuity::C2);
}

#[test]
fn curve_default_continuity_is_cn() {
    let c = Adaptor3dCurve::new(Adaptor3dCurveType::Line, 0.0, 1.0);
    assert_eq!(c.continuity(), Adaptor3dContinuity::CN);
}

// ---- curve: value stub ---------------------------------------------------------

#[test]
fn curve_value_stub_encodes_u() {
    let c = Adaptor3dCurve::new(Adaptor3dCurveType::Line, 0.0, 10.0);
    let p = c.value(3.7);
    assert!((p[0] - 3.7).abs() < 1e-15);
    assert_eq!(p[1], 0.0);
    assert_eq!(p[2], 0.0);
}

// ---- surface: plane ------------------------------------------------------------

#[test]
fn plane_surface_type() {
    let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
    assert_eq!(s.surface_type(), Adaptor3dSurfaceType::Plane);
}

#[test]
fn plane_not_u_closed_not_v_closed() {
    let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
    assert!(!s.is_u_closed());
    assert!(!s.is_v_closed());
    assert!(!s.is_u_periodic());
    assert!(!s.is_v_periodic());
}

// ---- surface: cylinder ---------------------------------------------------------

#[test]
fn cylinder_u_closed_not_v_closed() {
    let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 10.0);
    assert!(s.is_u_closed());
    assert!(s.is_u_periodic());
    assert!(!s.is_v_closed());
    assert!(!s.is_v_periodic());
}

// ---- surface: sphere -----------------------------------------------------------

#[test]
fn sphere_u_and_v_closed() {
    let s = Adaptor3dSurface::new(
        Adaptor3dSurfaceType::Sphere,
        0.0,
        2.0 * PI,
        -PI / 2.0,
        PI / 2.0,
    );
    assert!(s.is_u_closed());
    assert!(s.is_v_closed());
    assert!(s.is_u_periodic());
    assert!(s.is_v_periodic());
}

// ---- surface: torus ------------------------------------------------------------

#[test]
fn torus_u_and_v_closed() {
    let s =
        Adaptor3dSurface::new(Adaptor3dSurfaceType::Torus, 0.0, 2.0 * PI, 0.0, 2.0 * PI);
    assert!(s.is_u_closed());
    assert!(s.is_v_closed());
}

// ---- surface: cone -------------------------------------------------------------

#[test]
fn cone_u_closed_not_v_closed() {
    let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::Cone, 0.0, 2.0 * PI, 0.0, 5.0);
    assert!(s.is_u_closed());
    assert!(!s.is_v_closed());
}

// ---- surface: parameter ranges -------------------------------------------------

#[test]
fn surface_parameter_ranges_returned_correctly() {
    let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::Plane, -2.0, 3.0, -4.0, 5.0);
    assert!((s.u_first() - (-2.0)).abs() < 1e-15);
    assert!((s.u_last() - 3.0).abs() < 1e-15);
    assert!((s.v_first() - (-4.0)).abs() < 1e-15);
    assert!((s.v_last() - 5.0).abs() < 1e-15);
}

// ---- surface: value stub -------------------------------------------------------

#[test]
fn surface_value_stub_encodes_uv() {
    let s = Adaptor3dSurface::new(Adaptor3dSurfaceType::BSplineSurface, 0.0, 1.0, 0.0, 1.0);
    let p = s.value(0.3, 0.7);
    assert!((p[0] - 0.3).abs() < 1e-15);
    assert!((p[1] - 0.7).abs() < 1e-15);
    assert_eq!(p[2], 0.0);
}

// ---- continuity ordering -------------------------------------------------------

#[test]
fn continuity_enum_ordering() {
    assert!(Adaptor3dContinuity::C0 < Adaptor3dContinuity::G1);
    assert!(Adaptor3dContinuity::G1 < Adaptor3dContinuity::C1);
    assert!(Adaptor3dContinuity::C1 < Adaptor3dContinuity::G2);
    assert!(Adaptor3dContinuity::G2 < Adaptor3dContinuity::C2);
    assert!(Adaptor3dContinuity::C2 < Adaptor3dContinuity::C3);
    assert!(Adaptor3dContinuity::C3 < Adaptor3dContinuity::CN);
}
