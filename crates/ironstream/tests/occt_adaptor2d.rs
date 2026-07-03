// FILE: tests/occt_adaptor2d.rs
extern crate ironstream;
use ironstream::adaptor2d::{Adaptor2dCurve2d, Adaptor2dCurveType, Adaptor2dHCurve2d};
use core::f64::consts::TAU;

#[test]
fn line_adaptor_curve_type() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Line, 0.0, 10.0);
    assert_eq!(c.curve_type(), Adaptor2dCurveType::Line);
}

#[test]
fn circle_adaptor_curve_type() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Circle, 0.0, TAU);
    assert_eq!(c.curve_type(), Adaptor2dCurveType::Circle);
}

#[test]
fn line_adaptor_parameter_range() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Line, 2.0, 8.5);
    assert!((c.first_parameter() - 2.0).abs() < f64::EPSILON);
    assert!((c.last_parameter() - 8.5).abs() < f64::EPSILON);
}

#[test]
fn line_not_closed_not_periodic_no_period() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Line, 0.0, 5.0);
    assert!(!c.is_closed());
    assert!(!c.is_periodic());
    assert!(c.period().is_none());
}

#[test]
fn circle_is_closed() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Circle, 0.0, TAU);
    assert!(c.is_closed());
}

#[test]
fn circle_is_periodic_with_tau_period() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Circle, 0.0, TAU);
    assert!(c.is_periodic());
    let p = c.period().unwrap();
    assert!((p - TAU).abs() < 1e-15);
}

#[test]
fn ellipse_is_closed_and_periodic() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Ellipse, 0.0, TAU);
    assert!(c.is_closed());
    assert!(c.is_periodic());
    assert!((c.period().unwrap() - TAU).abs() < 1e-15);
}

#[test]
fn parabola_not_periodic() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Parabola, -10.0, 10.0);
    assert!(!c.is_periodic());
    assert!(c.period().is_none());
}

#[test]
fn bspline_default_continuity_zero() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::BSplineCurve, 0.0, 1.0);
    assert_eq!(c.continuity(), 0);
}

#[test]
fn with_continuity_sets_value() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::BSplineCurve, 0.0, 1.0)
        .with_continuity(2);
    assert_eq!(c.continuity(), 2);
}

#[test]
fn value_stub_returns_u_and_zero() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Line, 0.0, 10.0);
    let v = c.value(4.0);
    assert!((v[0] - 4.0).abs() < f64::EPSILON);
    assert!((v[1] - 0.0).abs() < f64::EPSILON);
}

#[test]
fn d1_stub_point_and_unit_tangent() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Circle, 0.0, TAU);
    let (pt, tan) = c.d1(1.5);
    assert!((pt[0] - 1.5).abs() < f64::EPSILON);
    assert!((pt[1] - 0.0).abs() < f64::EPSILON);
    assert!((tan[0] - 1.0).abs() < f64::EPSILON);
    assert!((tan[1] - 0.0).abs() < f64::EPSILON);
}

#[test]
fn d2_stub_zero_second_derivative() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::BezierCurve, 0.0, 1.0);
    let (pt, d1, d2) = c.d2(0.5);
    assert!((pt[0] - 0.5).abs() < f64::EPSILON);
    assert!((d1[0] - 1.0).abs() < f64::EPSILON);
    assert!((d2[0] - 0.0).abs() < f64::EPSILON);
    assert!((d2[1] - 0.0).abs() < f64::EPSILON);
}

#[test]
fn hcurve_wraps_curve_and_exposes_inner() {
    let c = Adaptor2dCurve2d::new(Adaptor2dCurveType::Ellipse, 0.0, TAU);
    let h = Adaptor2dHCurve2d::new(c);
    assert_eq!(h.curve().curve_type(), Adaptor2dCurveType::Ellipse);
    assert!(h.curve().is_closed());
}

#[test]
fn hcurve_set_curve_replaces_inner() {
    let c1 = Adaptor2dCurve2d::new(Adaptor2dCurveType::Circle, 0.0, TAU);
    let c2 = Adaptor2dCurve2d::new(Adaptor2dCurveType::BSplineCurve, 0.0, 1.0);
    let mut h = Adaptor2dHCurve2d::new(c1);
    h.set_curve(c2);
    assert_eq!(h.curve().curve_type(), Adaptor2dCurveType::BSplineCurve);
    assert!(!h.curve().is_periodic());
}
