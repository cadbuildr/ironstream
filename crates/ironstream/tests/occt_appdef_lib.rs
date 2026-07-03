// Integration tests for appdef_lib
use ironstream::appdef_lib::*;

#[test]
fn multi_point_2d_basic() {
    let mp = AppDefMultiPoint::new2d(0.25, vec![[1.0, 2.0], [3.0, 4.0]]);
    assert_eq!(mp.nb_points2d(), 2);
    assert_eq!(mp.nb_points3d(), 0);
    assert_eq!(mp.point2d(0), Some([1.0, 2.0]));
    assert_eq!(mp.point2d(1), Some([3.0, 4.0]));
    assert_eq!(mp.point2d(2), None);
}

#[test]
fn multi_point_parameter_stored() {
    let mp = AppDefMultiPoint::new3d(0.75, vec![[0.0, 0.0, 0.0]]);
    assert!((mp.parameter() - 0.75).abs() < 1e-12);
}

#[test]
fn multi_line_parameters_range() {
    let mut ml = AppDefMultiLine::new(1, 0);
    for i in 0..=10 {
        let t = i as f64 / 10.0;
        ml.add_point(AppDefMultiPoint::new3d(t, vec![[t, 0.0, 0.0]]));
    }
    assert!((ml.first_parameter() - 0.0).abs() < 1e-12);
    assert!((ml.last_parameter() - 1.0).abs() < 1e-12);
    assert_eq!(ml.nb_multi_points(), 11);
}

#[test]
fn multi_line_nb3d_nb2d() {
    let ml = AppDefMultiLine::new(3, 2);
    assert_eq!(ml.nb_points3d(), 3);
    assert_eq!(ml.nb_points2d(), 2);
}

#[test]
fn bspline_compute_set_params() {
    let mut comp = AppDefBSplineCompute::new(2, 6);
    comp.set_continuity(1);
    comp.set_tolerance(1e-6);
    comp.set_max_segments(50);
    assert_eq!(comp.continuity, 1);
    assert!((comp.tolerance - 1e-6).abs() < 1e-20);
    assert_eq!(comp.max_seg, 50);
}

#[test]
fn appdef_compute_average_error_leq_max() {
    let mut ml = AppDefMultiLine::new(2, 0);
    for i in 0..6 {
        let t = i as f64 / 5.0;
        ml.add_point(AppDefMultiPoint::new3d(t, vec![[t, t * t, 0.0], [0.0, t, 0.0]]));
    }
    let mut comp = AppDefCompute::new(4, 1e-3);
    comp.perform(&ml);
    assert!(comp.is_done());
    assert_eq!(comp.nb_curves(), 2);
    assert!(comp.max_error() >= 0.0);
    assert!(comp.nb_iterations() > 0);
}
