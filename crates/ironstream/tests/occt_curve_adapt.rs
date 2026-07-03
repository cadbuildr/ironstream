extern crate ironstream;
use ironstream::curve_adapt::*;

#[test]
fn test_geom2d_adaptor_new_and_trim() {
    let c = Geom2dAdaptorCurve::with_trim(5, 0.0, 2.0);
    assert_eq!(c.curve_id(), 5);
    assert!((c.first_parameter()).abs() < 1e-10);
    assert!((c.last_parameter() - 2.0).abs() < 1e-10);
    assert_eq!(c.get_type(), GeomAbsShape::BSplineCurve);
}

#[test]
fn test_geom2d_adaptor_value_and_d1() {
    let c = Geom2dAdaptorCurve::with_trim(1, 0.0, 1.0);
    let v = c.value(0.0);
    // at t=0: cos(0)=1, sin(0)=0
    assert!((v[0] - 1.0).abs() < 1e-10);
    assert!((v[1]).abs() < 1e-10);
    let (pt, tang) = c.d1(0.0);
    // tangent should be non-zero
    let tang_len = (tang[0] * tang[0] + tang[1] * tang[1]).sqrt();
    assert!(tang_len > 1e-10);
    let _ = pt;
}

#[test]
fn test_geom2d_adaptor_nb_intervals() {
    let c = Geom2dAdaptorCurve::new(3);
    assert_eq!(c.nb_intervals(GeomAbsContinuity::C1), 1);
}

#[test]
fn test_adaptor_curve_on_surface() {
    let mut cos = AdaptorCurveOnSurface::new(1, 10);
    cos.set_trim(0.25, 0.75);
    assert!((cos.first_parameter() - 0.25).abs() < 1e-10);
    assert!((cos.last_parameter() - 0.75).abs() < 1e-10);
    // stub: value at midpoint t=0.5 => maps to [0.5,0.5,0.0]
    let v = cos.value(0.5);
    assert!((v[0] - 0.5).abs() < 1e-10);
    assert!((v[1] - 0.5).abs() < 1e-10);
    assert!((v[2]).abs() < 1e-10);
}

#[test]
fn test_brep_curve_representation() {
    let r3d = BrepCurveRepresentation::curve3d(10, 1e-6);
    assert!(r3d.is_curve3d());
    assert!(!r3d.is_curve_on_surface());
    assert_eq!(r3d.curve_id(), 10);
    assert!((r3d.tolerance() - 1e-6).abs() < 1e-12);

    let ros = BrepCurveRepresentation::curve_on_surface(5, 20, 1e-7);
    assert!(ros.is_curve_on_surface());
    assert!(!ros.is_curve3d());
    assert_eq!(ros.surface_id(), 20);
    assert_eq!(ros.rep_type(), BrepCurveRepType::CurveOnSurface);
}

#[test]
fn test_adaptor3d_hcurve() {
    // null check
    let null_h = Adaptor3dHCurve::new(0, 0.0, 1.0);
    assert!(null_h.is_null());

    // normal curve
    let h = Adaptor3dHCurve::new(3, 0.0, 4.0);
    assert!(!h.is_null());
    assert!((h.arc_length() - 4.0).abs() < 1e-10);
    // stub value at midpoint: t = (2.0-0.0)/(4.0-0.0) = 0.5 => [0.5, 0.0, 0.0]
    let v = h.value(2.0);
    assert!((v[0] - 0.5).abs() < 1e-10);
    assert!((v[1]).abs() < 1e-10);
    // intervals: [first, last]
    let iv = h.intervals(GeomAbsContinuity::C0);
    assert_eq!(iv.len(), 2);
    assert!((iv[0]).abs() < 1e-10);
    assert!((iv[1] - 4.0).abs() < 1e-10);
}
