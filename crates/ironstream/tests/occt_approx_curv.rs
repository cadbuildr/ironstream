// FILE: tests/occt_approx_curv.rs
extern crate ironstream;
use ironstream::approx_curv::*;

#[test]
fn approx_result_failed_not_accurate() {
    let r = ApproxResult::failed();
    assert!(!r.is_done);
    assert!(!r.is_accurate(1e3));
}

#[test]
fn points_to_bspline_basic() {
    let pts = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [2.0, 0.0, 0.0], [3.0, 1.0, 0.0]];
    let result = PointsToBSpline::new(pts).build();
    assert!(result.is_done);
    assert_eq!(result.nb_poles, 4);
    assert_eq!(result.degree, 3);
}

#[test]
fn points_to_bspline_with_tolerance_is_accurate() {
    let pts = vec![[0.0, 0.0, 0.0], [5.0, 0.0, 0.0]];
    let result = PointsToBSpline::new(pts).with_tolerance(1e-6).build();
    assert!(result.is_accurate(1.0));
}

#[test]
fn curve_interpolate_builds_ok() {
    let pts = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [2.0, 0.0, 0.0]];
    let result = CurveInterpolate::new(pts, false, 1e-7).build();
    assert!(result.is_done);
    assert!(!result.poles.is_empty());
}

#[test]
fn curvlin_func_total_length_and_parameter() {
    let pts = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [3.0, 0.0, 0.0]];
    let f = CurvlinFunc::from_polyline(&pts);
    assert!((f.total_length - 3.0).abs() < 1e-10);
    assert!((f.parameter_at(0.0)).abs() < 1e-10);
    assert!((f.parameter_at(3.0) - 1.0).abs() < 1e-10);
    assert!((f.parameter_at(1.5) - 0.5).abs() < 1e-10);
}
