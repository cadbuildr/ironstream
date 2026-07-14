// FILE: tests/occt_adapt_curve.rs
extern crate ironstream;

use ironstream::adapt_curve::{
    CurveType, CurveD1, CurveD2, Adaptor3dCurve, GeomAdaptorCurve, Adaptor2dCurve2d,
};

#[test]
fn adaptor3d_default_param_range() {
    let c = Adaptor3dCurve::new(1, CurveType::Line);
    assert!((c.first_parameter() - 0.0).abs() < 1e-9);
    assert!((c.last_parameter() - 1.0).abs() < 1e-9);
}

#[test]
fn adaptor3d_set_param_range() {
    let mut c = Adaptor3dCurve::new(2, CurveType::Circle);
    c.set_param_range(-1.0, 5.0);
    assert!((c.first_parameter() - (-1.0)).abs() < 1e-9);
    assert!((c.last_parameter() - 5.0).abs() < 1e-9);
}

#[test]
fn adaptor3d_trim_returns_new_range() {
    let c = Adaptor3dCurve::new(3, CurveType::Ellipse);
    let t = c.trim(0.25, 0.75);
    assert!((t.first_parameter() - 0.25).abs() < 1e-9);
    assert!((t.last_parameter() - 0.75).abs() < 1e-9);
    // original unchanged
    assert!((c.first_parameter() - 0.0).abs() < 1e-9);
}

#[test]
fn adaptor3d_d1_and_d2() {
    let c = Adaptor3dCurve::new(4, CurveType::Line);
    let d1 = c.d1(0.5);
    assert!((d1.point[0] - 0.5).abs() < 1e-9);
    assert!((d1.d1[0] - 1.0).abs() < 1e-9);

    let d2 = c.d2(0.5);
    assert!((d2.d2[0] - 0.0).abs() < 1e-9);
    assert!((d2.d2[1] - 0.0).abs() < 1e-9);
    assert!((d2.d2[2] - 0.0).abs() < 1e-9);
}

#[test]
fn geom_adaptor_curve_range() {
    let g = GeomAdaptorCurve::from_curve_and_range(10, 2.0, 8.0);
    assert!((g.first_parameter() - 2.0).abs() < 1e-9);
    assert!((g.last_parameter() - 8.0).abs() < 1e-9);
    assert_eq!(g.get_type(), CurveType::BSplineCurve);
}

#[test]
fn adaptor2d_value_and_type() {
    let c = Adaptor2dCurve2d::new(5, CurveType::BezierCurve);
    let p = c.value(0.0);
    assert!((p[0] - 0.0).abs() < 1e-9);
    assert!((p[1] - 0.0).abs() < 1e-9);
    let p1 = c.value(1.0);
    assert!((p1[0] - 1.0).abs() < 1e-9);
    assert_eq!(c.get_type(), CurveType::BezierCurve);
}
