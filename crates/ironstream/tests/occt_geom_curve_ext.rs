// Integration tests for geom_curve_ext module
extern crate ironstream;

use ironstream::geom_curve_ext::*;

#[test]
fn curve_ext_status_predicates() {
    assert!(CurveExtStatus::Ok.is_ok());
    assert!(!CurveExtStatus::NullCurve.is_ok());
    assert!(!CurveExtStatus::InvalidTrim.is_ok());
    assert!(!CurveExtStatus::SingularTransform.is_ok());
    assert!(!CurveExtStatus::Error.is_ok());
}

#[test]
fn geom_transformation_identity() {
    let t = GeomTransformation::identity();
    assert!(t.is_identity());
    let p = t.transform_point([1.0, 2.0, 3.0]);
    assert!((p[0] - 1.0).abs() < 1e-9);
    assert!((p[1] - 2.0).abs() < 1e-9);
    assert!((p[2] - 3.0).abs() < 1e-9);
    // diagonal elements are 1, off-diagonal 0
    assert!((t.value(0, 0) - 1.0).abs() < 1e-9);
    assert!((t.value(0, 1)).abs() < 1e-9);
}

#[test]
fn geom_transformation_translation_and_multiply() {
    let tx = GeomTransformation::translation(5.0, 0.0, 0.0);
    let ty = GeomTransformation::translation(0.0, 3.0, 0.0);
    let p = tx.transform_point([0.0, 0.0, 0.0]);
    assert!((p[0] - 5.0).abs() < 1e-9);

    let combined = tx.multiplied(&ty);
    let q = combined.transform_point([0.0, 0.0, 0.0]);
    assert!((q[0] - 5.0).abs() < 1e-9);
    assert!((q[1] - 3.0).abs() < 1e-9);
    assert!((q[2]).abs() < 1e-9);
}

#[test]
fn geom_transformation_scale() {
    let s = GeomTransformation::scale(0.0, 0.0, 0.0, 2.0);
    let p = s.transform_point([3.0, 4.0, 5.0]);
    assert!((p[0] - 6.0).abs() < 1e-9);
    assert!((p[1] - 8.0).abs() < 1e-9);
    assert!((p[2] - 10.0).abs() < 1e-9);
    // inverted stub returns identity
    let inv = s.inverted();
    assert!(inv.is_identity());
}

#[test]
fn geom_trimmed_curve_valid_and_invalid() {
    let tc = GeomTrimmedCurve::new(5, 0.2, 0.8);
    assert!(tc.is_ok());
    assert!((tc.first_parameter() - 0.2).abs() < 1e-9);
    assert!((tc.last_parameter() - 0.8).abs() < 1e-9);
    assert_eq!(tc.basis_curve(), 5);

    // reversed parameter
    let rev = tc.reversed_parameter(0.5);
    assert!((rev - (0.2 + 0.8 - 0.5)).abs() < 1e-9);

    // null curve
    let tc_null = GeomTrimmedCurve::new(0, 0.0, 1.0);
    assert_eq!(tc_null.status, CurveExtStatus::NullCurve);

    // invalid trim (first >= last)
    let tc_inv = GeomTrimmedCurve::new(5, 0.8, 0.2);
    assert_eq!(tc_inv.status, CurveExtStatus::InvalidTrim);

    // with_sense variant
    let tc_s = GeomTrimmedCurve::new_with_sense(3, 0.0, 1.0, false);
    assert!(tc_s.is_ok());
    assert!(!tc_s.sense);
}

#[test]
fn geom_offset_curve_value_and_set_trim() {
    let mut oc = GeomOffsetCurve::new(3, 0.5, [0.0, 0.0, 1.0]);
    assert!(oc.is_ok());
    assert!((oc.offset_value() - 0.5).abs() < 1e-9);
    assert_eq!(oc.basis_curve(), 3);

    let v = oc.value(1.0);
    // direction is [0,0,1], offset 0.5: z component should be 0.5
    assert!((v[2] - 0.5).abs() < 1e-9);

    oc.set_offset_value(1.0);
    assert!((oc.offset_value() - 1.0).abs() < 1e-9);

    // reversed parameter
    assert!((oc.reversed_parameter(2.0) - (-2.0)).abs() < 1e-9);

    // null basis curve
    let oc_null = GeomOffsetCurve::new(0, 0.5, [1.0, 0.0, 0.0]);
    assert_eq!(oc_null.status, CurveExtStatus::NullCurve);

    // set_trim on trimmed curve
    let mut tc = GeomTrimmedCurve::new(7, 0.0, 1.0);
    tc.set_trim(0.1, 0.9);
    assert!((tc.first_parameter() - 0.1).abs() < 1e-9);
    tc.set_trim(0.9, 0.1); // invalid
    assert_eq!(tc.status, CurveExtStatus::InvalidTrim);
}
