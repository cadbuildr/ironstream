extern crate ironstream;
use ironstream::geom_curve_constraints::*;

#[test]
fn test_geom_curve_constraints_basic() {
    // MultiPointConstraint
    let c = MultiPointConstraint::new(vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]])
        .with_order(ConstraintOrder::C1)
        .with_weight(2.0);
    assert_eq!(c.point_count(), 2);
    assert_eq!(c.order, ConstraintOrder::C1);
    assert!((c.weight - 2.0).abs() < f64::EPSILON);

    // MultiCurve
    let mut mc = MultiCurve::new(1);
    mc.add_curve(vec![[0.0, 0.0, 0.0], [1.0, 2.0, 3.0]]);
    let p0 = mc.evaluate(0, 0.0);
    let p1 = mc.evaluate(0, 1.0);
    assert!((p0[0]).abs() < 1e-12);
    assert!((p1[0] - 1.0).abs() < 1e-12);
    assert_eq!(mc.curve_count(), 1);

    // build_multi_curve
    let constraints = vec![
        MultiPointConstraint::new(vec![[0.0, 0.0, 0.0]]),
        MultiPointConstraint::new(vec![[1.0, 0.0, 0.0]]),
        MultiPointConstraint::new(vec![[2.0, 0.0, 0.0]]),
    ];
    let built = build_multi_curve(&constraints, 2);
    assert_eq!(built.curve_count(), 1);
    assert_eq!(built.curves[0].len(), 3);
}
