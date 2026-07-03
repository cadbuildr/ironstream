extern crate ironstream;
use ironstream::geom_curve_offset2d::*;

#[test]
fn test_geom_curve_offset2d_basic() {
    let oc = OffsetCurve2d::new("line_1", 2.0);
    assert_eq!(oc.basis_curve(), "line_1");
    assert!((oc.offset_value() - 2.0).abs() < 1e-10);

    let pt = oc.evaluate(1.0);
    assert!((pt[0] - 1.0).abs() < 1e-10);
    assert!((pt[1] - 2.0).abs() < 1e-10);

    let d1 = oc.d1(0.0);
    assert!((d1[0] - 1.0).abs() < 1e-10);
    assert!(d1[1].abs() < 1e-10);

    assert!((oc.first_parameter() - 0.0).abs() < 1e-10);
    assert!((oc.last_parameter() - 1.0).abs() < 1e-10);

    let mut oc2 = OffsetCurve2d::new("curve_a", 3.0);
    oc2.reverse();
    assert!((oc2.offset_value() + 3.0).abs() < 1e-10);

    let pts = vec![[0.0_f64, 0.0], [1.0, 0.0], [1.0, 1.0]];
    let off_pts = offset_polyline_2d(&pts, 1.0);
    assert!(!off_pts.is_empty());
}
