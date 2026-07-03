extern crate ironstream;
use ironstream::geom_curve_offset3d::*;

#[test]
fn test_geom_curve_offset3d_basic() {
    let oc = OffsetCurve3d::new("line_1", 2.0, [0.0, 1.0, 0.0]);
    assert_eq!(oc.basis_curve(), "line_1");
    assert!((oc.offset_value() - 2.0).abs() < 1e-10);

    let pt = oc.evaluate(1.0);
    assert!((pt[0] - 1.0).abs() < 1e-10);
    assert!((pt[1] - 2.0).abs() < 1e-10);
    assert!(pt[2].abs() < 1e-10);

    let d1 = oc.d1(0.0);
    assert!((d1[0] - 1.0).abs() < 1e-10);

    let d2 = oc.d2(0.0);
    assert!(d2[0].abs() < 1e-10);

    assert!((oc.first_parameter() - 0.0).abs() < 1e-10);
    assert!((oc.last_parameter() - 1.0).abs() < 1e-10);

    let mut oc2 = OffsetCurve3d::new("curve_a", 3.0, [0.0, 0.0, 1.0]);
    oc2.reverse();
    assert!((oc2.offset_value() + 3.0).abs() < 1e-10);
}
