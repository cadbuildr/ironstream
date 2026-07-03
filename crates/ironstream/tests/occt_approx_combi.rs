use ironstream::approx_combi::*;

#[test]
fn approx_status() {
    assert!(ApproxStatus::Done.is_done());
    assert!(!ApproxStatus::Fail.is_done());
}

#[test]
fn multi_point_constraint() {
    let p = MultiPointConstraint::new([1.0, 2.0, 3.0], 0.5).with_tangent([1.0, 0.0, 0.0]);
    assert!(p.has_derivative());
    assert_eq!(p.tangent(), Some([1.0, 0.0, 0.0]));
}

#[test]
fn multi_line_ops() {
    let mut ml = MultiLine::new();
    ml.add_point(MultiPointConstraint::new([0.0, 0.0, 0.0], 0.0));
    ml.add_point(MultiPointConstraint::new([1.0, 0.0, 0.0], 1.0));
    assert_eq!(ml.nb_multi_points(), 2);
    assert!(ml.value(0).is_none());
    assert!(ml.value(1).is_some());
}

#[test]
fn approx_curve3d_done() {
    let a = ApproxCurve3d::new(1, 1e-4, 1, 100, 3, 8);
    assert!(a.is_done());
    assert!(a.curve() > 0);
    assert!(a.max_error() < 1e-3);
}

#[test]
fn approx_curvlin_roundtrip() {
    let f = ApproxCurvlinFunc::new(1, 0.0, 4.0);
    assert!((f.length() - 4.0).abs() < 1e-12);
    let s = f.get_s_from_u(2.0);
    let u = f.get_u_from_s(s, 1e-7);
    assert!((u - 2.0).abs() < 1e-9);
}
