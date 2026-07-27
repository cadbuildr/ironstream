// FILE: tests/occt_geom2d_api_ext.rs
extern crate ironstream;

use ironstream::geom2d_api_ext::{
    Geom2dApiInterpolate, Geom2dApiProjectPointOnCurve,
    Geom2dApiPtsToBSplineParams, Geom2dApiPointsToBSpline,
};

#[test]
fn pts_bspline_default_params_values() {
    let p = Geom2dApiPtsToBSplineParams::default();
    assert_eq!(p.degree_min, 3);
    assert_eq!(p.degree_max, 8);
    assert_eq!(p.continuity, 2);
    assert_eq!(p.tolerance, 1e-6);
}

#[test]
fn pts_bspline_new_not_done() {
    let b = Geom2dApiPointsToBSpline::new(Geom2dApiPtsToBSplineParams::default());
    assert!(!b.is_done());
    assert_eq!(b.nb_points(), 0);
    assert_eq!(b.nb_poles(), 0);
    assert_eq!(b.degree(), 0);
}

#[test]
fn pts_bspline_no_points_not_done_after_perform() {
    let mut b = Geom2dApiPointsToBSpline::new(Geom2dApiPtsToBSplineParams::default());
    b.perform();
    assert!(!b.is_done());
}

#[test]
fn pts_bspline_one_point_not_done_after_perform() {
    let mut b = Geom2dApiPointsToBSpline::new(Geom2dApiPtsToBSplineParams::default());
    b.add_point(1.0, 2.0);
    b.perform();
    assert!(!b.is_done());
}

#[test]
fn pts_bspline_three_points_perform_done() {
    let mut b = Geom2dApiPointsToBSpline::new(Geom2dApiPtsToBSplineParams::default());
    b.add_point(0.0, 0.0);
    b.add_point(1.0, 1.0);
    b.add_point(2.0, 0.0);
    b.perform();
    assert!(b.is_done());
    assert_eq!(b.nb_points(), 3);
    assert_eq!(b.nb_poles(), 3);
    assert_eq!(b.degree(), 3);
}

#[test]
fn pts_bspline_custom_degree_min() {
    let params = Geom2dApiPtsToBSplineParams {
        degree_min: 2,
        degree_max: 6,
        continuity: 1,
        tolerance: 1e-5,
    };
    let mut b = Geom2dApiPointsToBSpline::new(params);
    b.add_point(0.0, 0.0);
    b.add_point(1.0, 0.0);
    b.perform();
    assert!(b.is_done());
    assert_eq!(b.degree(), 2);
}

#[test]
fn pts_bspline_set_continuity_and_tolerance() {
    let mut b = Geom2dApiPointsToBSpline::new(Geom2dApiPtsToBSplineParams::default());
    b.set_continuity(0);
    b.set_tolerance(1e-3);
    b.add_point(0.0, 1.0);
    b.add_point(2.0, 3.0);
    b.perform();
    assert!(b.is_done());
    assert_eq!(b.nb_poles(), 2);
}

#[test]
fn interpolate_initial_state() {
    let interp = Geom2dApiInterpolate::new(false, 1e-6);
    assert!(!interp.is_done());
    assert_eq!(interp.nb_poles(), 0);
    assert!(!interp.is_periodic());
}

#[test]
fn interpolate_periodic_flag_preserved() {
    let interp = Geom2dApiInterpolate::new(true, 1e-7);
    assert!(interp.is_periodic());
}

#[test]
fn interpolate_one_point_not_done() {
    let mut interp = Geom2dApiInterpolate::new(false, 1e-6);
    interp.add_point(0.0, 0.0);
    interp.perform();
    assert!(!interp.is_done());
    assert_eq!(interp.nb_poles(), 0);
}

#[test]
fn interpolate_two_points_done() {
    let mut interp = Geom2dApiInterpolate::new(false, 1e-6);
    interp.add_point(0.0, 0.0);
    interp.add_point(1.0, 1.0);
    interp.perform();
    assert!(interp.is_done());
    assert_eq!(interp.nb_poles(), 2);
}

#[test]
fn interpolate_four_points_done() {
    let mut interp = Geom2dApiInterpolate::new(false, 1e-6);
    interp.add_point(0.0, 0.0);
    interp.add_point(1.0, 2.0);
    interp.add_point(3.0, 1.5);
    interp.add_point(4.0, 0.0);
    interp.perform();
    assert!(interp.is_done());
    assert_eq!(interp.nb_poles(), 4);
}

#[test]
fn project_point_initial_state() {
    let proj = Geom2dApiProjectPointOnCurve::new([2.0, 5.0]);
    assert!(!proj.is_done());
    assert_eq!(proj.nb_points(), 0);
}

#[test]
fn project_point_perform_done() {
    let mut proj = Geom2dApiProjectPointOnCurve::new([1.0, 1.0]);
    proj.perform();
    assert!(proj.is_done());
    assert_eq!(proj.nb_points(), 1);
}

#[test]
fn project_point_lower_distance_zero_after_perform() {
    let mut proj = Geom2dApiProjectPointOnCurve::new([0.0, 0.0]);
    proj.perform();
    assert_eq!(proj.lower_distance(), 0.0);
}

#[test]
fn project_point_lower_distance_parameter_zero() {
    let mut proj = Geom2dApiProjectPointOnCurve::new([3.0, -1.0]);
    proj.perform();
    assert_eq!(proj.lower_distance_parameter(), 0.0);
}
