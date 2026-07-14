extern crate ironstream;
use ironstream::geom_plates::*;

#[test]
fn test_geom_plates_basic() {
    // PointConstraint
    let pc = PointConstraint::new([1.0, 2.0, 3.0]).with_order(PlateOrder::G1);
    assert_eq!(pc.point, [1.0, 2.0, 3.0]);
    assert_eq!(pc.order, PlateOrder::G1);

    // CurveConstraint
    let cc = CurveConstraint::new("boundary_curve").with_order(PlateOrder::G2);
    assert_eq!(cc.curve_name, "boundary_curve");
    assert_eq!(cc.order, PlateOrder::G2);

    // PlateSurfaceBuilder
    let mut builder = PlateSurfaceBuilder::new(1e-3);
    builder.add_point(PointConstraint::new([0.0, 0.0, 0.0]));
    builder.add_point(PointConstraint::new([1.0, 0.0, 0.5]));
    builder.add_curve(CurveConstraint::new("edge_1"));
    assert!(!builder.is_done());
    let result = builder.build();
    assert!(builder.is_done());
    assert!(!result.is_empty());
}
