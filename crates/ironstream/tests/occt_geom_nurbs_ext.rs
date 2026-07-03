extern crate ironstream;
use ironstream::geom_nurbs_ext::*;

#[test]
fn bspline_curve_to_bezier_arcs() {
    let knots = vec![0.0, 0.5, 1.0];
    let poles = vec![[0.0,0.0,0.0],[0.5,1.0,0.0],[1.0,0.0,0.0]];
    let c = BSplineCurveToBezier::new(knots, poles, 2);
    assert!(c.is_done());
    assert_eq!(c.nb_arcs(), 2);
    let arc = c.arc(1).unwrap();
    assert!((arc.t_start).abs() < 1e-10);
    assert!((arc.t_end - 0.5).abs() < 1e-10);
}

#[test]
fn bspline_surface_to_bezier_patches() {
    let u = vec![0.0, 0.5, 1.0];
    let v = vec![0.0, 1.0];
    let s = BSplineSurfaceToBezier::new(u, v, 2, 2);
    assert!(s.is_done());
    assert_eq!(s.nb_u_patches(), 2);
    assert_eq!(s.nb_v_patches(), 1);
    assert_eq!(s.nb_patches(), 2);
}

#[test]
fn bspline_cache_evaluate_midpoint() {
    let poles = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0]];
    let knots = vec![0.0, 0.5, 1.0];
    let mut cache = BSplineCurveCache::new(poles, knots, 2);
    let p = cache.evaluate(0.5);
    assert!((p[0] - 1.0).abs() < 1e-6);
    assert!(cache.is_valid_for(0.5));
}

#[test]
fn bspline_cache_invalidate() {
    let poles = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
    let mut c = BSplineCurveCache::new(poles, vec![0.0, 1.0], 1);
    c.evaluate(0.5);
    assert!(c.is_valid_for(0.5));
    c.invalidate();
    assert!(!c.is_valid_for(0.5));
}

#[test]
fn unit_converter_inch_to_mm() {
    let conv = UnitConverter::inch_to_mm();
    assert!((conv.convert(1.0) - 25.4).abs() < 1e-10);
    let p = conv.convert_point([2.0, 0.0, 0.0]);
    assert!((p[0] - 50.8).abs() < 1e-10);
}
