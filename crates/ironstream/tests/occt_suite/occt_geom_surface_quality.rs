extern crate ironstream;
use ironstream::geom_surface_quality::*;

#[test]
fn surface_analysis_u_closed() {
    let s = SurfaceAnalysis::new(1, 0.0, 2.0*std::f64::consts::PI, -1.0, 1.0);
    assert!(s.is_u_closed(1e-6));
    assert!((s.parametric_area() - 4.0*std::f64::consts::PI).abs() < 1e-8);
}

#[test]
fn surface_analysis_project_uv() {
    let s = SurfaceAnalysis::new(2, 0.0, 1.0, 0.0, 1.0);
    assert!(s.project_uv(0.5, 0.5).is_some());
    assert!(s.project_uv(-0.1, 0.5).is_none());
    assert!(s.project_uv(0.5, 1.5).is_none());
}

#[test]
fn surface_analysis_resolution() {
    let s = SurfaceAnalysis::new(0, 0.0, 10.0, 0.0, 5.0);
    let ur = s.u_resolution(1.0);
    assert!((ur - 0.1).abs() < 1e-10);
    let vr = s.v_resolution(1.0);
    assert!((vr - 0.2).abs() < 1e-10);
}

#[test]
fn check_bspline_curve_c1() {
    let poles = vec![[0.0,0.0,0.0],[1.0,1.0,0.0],[2.0,0.0,0.0]];
    let knots = vec![0.0, 0.5, 1.0];
    let c = CheckBSplineCurve::new(poles, knots, 2);
    assert!(c.is_c1_continuous());
    assert!(c.is_g1_continuous());
}

#[test]
fn is_bounded_variants() {
    assert_eq!(is_bounded(0.0, 1.0), CurveBoundedness::Bounded);
    assert_eq!(is_bounded(0.0, f64::INFINITY), CurveBoundedness::BoundedOnLeft);
    assert_eq!(is_bounded(f64::NEG_INFINITY, 0.0), CurveBoundedness::BoundedOnRight);
    assert_eq!(is_bounded(f64::NEG_INFINITY, f64::INFINITY), CurveBoundedness::Unbounded);
}
