// Integration tests for approx_param
use ironstream::approx_param::*;

#[test]
fn constraint_default_is_pass_point() {
    let c = AppParConstraint::default();
    assert_eq!(c, AppParConstraint::PassPoint);
}

#[test]
fn multi_point_3d_new_and_access() {
    let mp = AppParMultiPoint::new3d(vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    assert_eq!(mp.nb_3d_curves(), 2);
    assert_eq!(mp.nb_2d_curves(), 0);
    assert_eq!(mp.nb_curves(), 2);
    assert_eq!(mp.point3d(0), Some([1.0, 2.0, 3.0]));
    assert_eq!(mp.point3d(1), Some([4.0, 5.0, 6.0]));
    assert_eq!(mp.point3d(2), None);
}

#[test]
fn multi_point_mixed_counts() {
    let mp = AppParMultiPoint::new_mixed(
        vec![[0.0, 0.0, 0.0]],
        vec![[0.5, 0.5], [1.0, 0.0]],
    );
    assert_eq!(mp.nb_3d_curves(), 1);
    assert_eq!(mp.nb_2d_curves(), 2);
    assert_eq!(mp.nb_curves(), 3);
    assert_eq!(mp.point2d(0), Some([0.5, 0.5]));
}

#[test]
fn multi_curve_poles_and_degree() {
    let mut c = AppParMultiCurve::new(3);
    assert_eq!(c.degree(), 3);
    assert_eq!(c.nb_poles(), 0);

    c.add_pole(AppParMultiPoint::new3d(vec![[0.0, 0.0, 0.0]]));
    c.add_pole(AppParMultiPoint::new3d(vec![[1.0, 0.0, 0.0]]));
    assert_eq!(c.nb_poles(), 2);
    assert_eq!(c.nb_curves(), 1);
}

#[test]
fn multi_bsp_curve_parameters() {
    let bsp = AppParMultiBSpCurve::new(3);
    assert_eq!(bsp.degree(), 3);
    assert!((bsp.first_parameter() - 0.0).abs() < 1e-10);
    assert!((bsp.last_parameter() - 1.0).abs() < 1e-10);
    assert_eq!(bsp.nb_poles(), 0);
}

#[test]
fn fit_and_divide_perform_and_not_done_on_zero_id() {
    // Perform with valid id produces a result
    let mut f = ApproxFitAndDivide::new(1, 3, 8);
    assert!(!f.is_done());
    f.perform();
    assert!(f.is_done());
    assert_eq!(f.nb_curves(), 1);
    let curve = f.value(0).expect("should have a curve at index 0");
    assert!(curve.first_parameter() <= curve.last_parameter());

    // Zero id -> not done
    let mut f2 = ApproxFitAndDivide::new(0, 3, 8);
    f2.perform();
    assert!(!f2.is_done());
    assert!(f2.value(0).is_none());
}
