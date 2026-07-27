// FILE: tests/occt_shape_upgrade_ext.rs
extern crate ironstream;
use ironstream::shape_upgrade_ext::*;

#[test]
fn test_upgrade_result_statuses() {
    let ok = UpgradeResult::ok(4);
    assert!(ok.is_done());
    assert_eq!(ok.nb_modified, 4);
    assert_eq!(ok.total_changed(), 4);

    let fail = UpgradeResult::fail();
    assert!(!fail.is_done());
}

#[test]
fn test_face_divide_perform() {
    let mut fd = FaceDivide::new(0)
        .with_tolerance(1e-6)
        .with_strategy(SplitStrategy::SplitAll);
    assert!(!fd.is_done());
    fd.perform();
    assert!(fd.is_done());
}

#[test]
fn test_shape_divide_closed() {
    let mut sd = ShapeDivideClosed::new(2, 2);
    sd.perform(&[0, 1, 2, 3]);
    assert!(sd.is_done());
    assert_eq!(sd.nb_faces_processed(), 4);
}

#[test]
fn test_unify_same_domain() {
    let mut u = UnifySameDomain::new().with_concat_bsplines();
    assert!(u.concat_b_splines);
    u.build(5, 8);
    assert!(u.is_done);
    assert_eq!(u.total_unified(), 13);
}

#[test]
fn test_shape_convert_to_bezier() {
    let mut conv = ShapeConvertToBezier::new().with_surfaces();
    assert!(conv.convert_surfaces);
    conv.perform(7);
    assert!(conv.is_done());
}

#[test]
fn test_shape_divide_angle() {
    let mut sda = ShapeDivideAngle::with_max_angle_deg(90.0);
    assert!((sda.max_angle - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
    sda.perform(3);
    assert!(sda.is_done);
    assert_eq!(sda.nb_faces_divided, 3);
}
