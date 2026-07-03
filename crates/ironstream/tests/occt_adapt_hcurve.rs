extern crate ironstream;
use ironstream::adapt_hcurve::*;

#[test]
fn test_adaptor3d_curve_value_at_midpoint() {
    let c = Adaptor3dCurve::new(1, CurveType::Line, 0.0, 10.0);
    let pt = c.value(5.0);
    assert!((pt[0] - 0.5).abs() < 1e-10);
}

#[test]
fn test_adaptor3d_curve_first_last_param() {
    let c = Adaptor3dCurve::new(2, CurveType::Circle, 0.0, std::f64::consts::TAU);
    assert!((c.first_parameter() - 0.0).abs() < 1e-14);
    assert!((c.last_parameter() - std::f64::consts::TAU).abs() < 1e-14);
    assert!(c.is_periodic());
}

#[test]
fn test_hcurve_wraps_adaptor_and_reports_type() {
    let c = Adaptor3dCurve::new(0, CurveType::BSplineCurve, 0.0, 1.0);
    let hc = HCurve::new(c);
    assert_eq!(hc.get_type(), CurveType::BSplineCurve);
    assert!(!hc.is_null());
}

#[test]
fn test_adaptor3d_surface_value_and_normal() {
    let s = Adaptor3dSurface::new(0, SurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
    let pt = s.value(0.5, 0.5);
    assert!((pt[0] - 0.5).abs() < 1e-10);
    let n = s.normal(0.0, 0.0);
    assert!((n[2] - 1.0).abs() < 1e-10);
}

#[test]
fn test_hsurface_get_type() {
    let s = Adaptor3dSurface::new(0, SurfaceType::Cylinder, 0.0, 1.0, 0.0, 1.0);
    let hs = HSurface::new(s);
    assert_eq!(hs.get_type(), SurfaceType::Cylinder);
}

#[test]
fn test_hcurve2d_value_and_d1() {
    let c2d = HCurve2d::new(7, 0.0, 2.0);
    let uv = c2d.value(1.0);
    assert!((uv[0] - 0.5).abs() < 1e-10);
    let (_, tang) = c2d.d1(0.0);
    assert!((tang[0] - 1.0).abs() < 1e-10);
}
