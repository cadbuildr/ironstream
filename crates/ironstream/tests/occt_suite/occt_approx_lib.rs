// Integration tests for approx_lib
use ironstream::approx_lib::*;

#[test]
fn same_parameter_result_id_derived_from_input() {
    let mut sp = ApproxSameParameter::new(7, 3, 1e-5);
    sp.perform();
    assert!(sp.is_done());
    assert_eq!(sp.curve(), 7 + 11000);
}

#[test]
fn same_parameter_tolerance_reached_below_input() {
    let mut sp = ApproxSameParameter::new(2, 1, 1e-3);
    sp.perform();
    assert!(sp.tolerance_reached() < 1e-3);
}

#[test]
fn curvilinear_parameter_errors_bounded() {
    let mut cp = ApproxCurvilinearParameter::new(5, 1e-4, 5, 200);
    cp.perform();
    assert!(cp.is_done());
    assert!(cp.max_error_3d() <= 1e-4);
    assert!(cp.max_error_2d() <= cp.max_error_3d());
}

#[test]
fn approx_curve3d_result_id_derived() {
    let mut ac = ApproxCurve3d::new(20, 1e-5, 3, 8, 50);
    ac.perform();
    assert!(ac.is_done());
    assert_eq!(ac.curve(), 20 + 13000);
}

#[test]
fn approx_curve2d_max_error_bounded() {
    let mut ac = ApproxCurve2d::new(15, 0.0, 2.0, 1e-4, 1e-5, 2, 6);
    ac.perform();
    assert!(ac.is_done());
    assert!(ac.max_error() <= 1e-4);
}

#[test]
fn approx_status_default_not_done() {
    let status = ApproxStatus::default();
    assert!(!status.is_done());
    assert_eq!(status, ApproxStatus::NotDone);
}
