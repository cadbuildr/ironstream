// FILE: tests/occt_geom2d_adaptor.rs
extern crate ironstream;
use ironstream::geom2d_adaptor::*;
use core::f64::consts::TAU;

#[test]
fn line_adaptor_curve_type_is_line() {
    let a = Geom2dAdaptorCurve::new(CurveType2d::Line, 0.0, 1.0);
    assert_eq!(a.curve_type(), CurveType2d::Line);
}

#[test]
fn line_adaptor_parameter_range() {
    let a = Geom2dAdaptorCurve::new(CurveType2d::Line, 0.0, 1.0);
    assert!((a.first_parameter() - 0.0).abs() < f64::EPSILON);
    assert!((a.last_parameter() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn line_not_periodic() {
    let a = Geom2dAdaptorCurve::new(CurveType2d::Line, 0.0, 1.0);
    assert!(!a.is_periodic());
    assert!(a.period().is_none());
}

#[test]
fn line_not_closed() {
    let a = Geom2dAdaptorCurve::new(CurveType2d::Line, 0.0, 1.0);
    assert!(!a.is_closed());
}

#[test]
fn circle_adaptor_is_periodic() {
    let a = Geom2dAdaptorCurve::new(CurveType2d::Circle, 0.0, TAU);
    assert!(a.is_periodic());
}

#[test]
fn circle_adaptor_period_is_two_pi() {
    let a = Geom2dAdaptorCurve::new(CurveType2d::Circle, 0.0, TAU);
    let p = a.period().expect("circle should have period");
    assert!((p - TAU).abs() < 1e-15);
}

#[test]
fn circle_adaptor_is_closed() {
    let a = Geom2dAdaptorCurve::new(CurveType2d::Circle, 0.0, TAU);
    assert!(a.is_closed());
}

#[test]
fn ellipse_adaptor_is_periodic_and_closed() {
    let a = Geom2dAdaptorCurve::new(CurveType2d::Ellipse, 0.0, TAU);
    assert!(a.is_periodic());
    assert!(a.is_closed());
    let p = a.period().expect("ellipse should have period");
    assert!((p - TAU).abs() < 1e-15);
}

#[test]
fn bspline_not_periodic_no_period() {
    let a = Geom2dAdaptorCurve::new(CurveType2d::BSplineCurve, 0.0, 1.0);
    assert!(!a.is_periodic());
    assert!(a.period().is_none());
}

#[test]
fn continuity_builder_roundtrip() {
    let a = Geom2dAdaptorCurve::new(CurveType2d::Line, 0.0, 1.0).with_continuity(255);
    assert_eq!(a.continuity(), 255);
}

#[test]
fn value_stub_first_component_equals_u() {
    let a = Geom2dAdaptorCurve::new(CurveType2d::Line, 0.0, 10.0);
    for u in [0.0f64, 0.5, 1.0, 7.3, 10.0] {
        let v = a.value(u);
        assert!((v[0] - u).abs() < f64::EPSILON, "u={} v[0]={}", u, v[0]);
        assert!((v[1] - 0.0).abs() < f64::EPSILON);
    }
}

#[test]
fn hcurve_delegates_to_inner() {
    let a = Geom2dAdaptorCurve::new(CurveType2d::Circle, 0.0, TAU).with_continuity(2);
    let h = Geom2dAdaptorHCurve::new(a);
    assert_eq!(h.curve().curve_type(), CurveType2d::Circle);
    assert!(h.curve().is_periodic());
    assert_eq!(h.curve().continuity(), 2);
}

#[test]
fn curve_type2d_all_variants_eq() {
    let all = [
        CurveType2d::Line,
        CurveType2d::Circle,
        CurveType2d::Ellipse,
        CurveType2d::Hyperbola,
        CurveType2d::Parabola,
        CurveType2d::BezierCurve,
        CurveType2d::BSplineCurve,
        CurveType2d::OtherCurve,
    ];
    for (i, &a) in all.iter().enumerate() {
        for (j, &b) in all.iter().enumerate() {
            if i == j {
                assert_eq!(a, b);
            } else {
                assert_ne!(a, b);
            }
        }
    }
}
