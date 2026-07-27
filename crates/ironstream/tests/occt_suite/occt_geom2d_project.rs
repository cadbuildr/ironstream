use ironstream::geom2d_project::*;

#[test]
fn project_point_on_curve_distance_from_point_above_axis() {
    let mut p = Geom2dProjectPointOnCurve::new([4.0, 3.0], 1, 0.0, 10.0);
    p.perform();
    assert_eq!(p.nb_points(), 1);
    let d = p.lower_distance().unwrap();
    // perpendicular distance from (4,3) to x-axis = 3
    assert!((d - 3.0).abs() < 1e-10);
}

#[test]
fn project_nearest_point_on_x_axis() {
    let mut p = Geom2dProjectPointOnCurve::new([7.0, 0.0], 1, 0.0, 10.0);
    p.perform();
    let np = p.nearest_point().unwrap();
    assert!((np[0] - 7.0).abs() < 1e-10);
    assert!(np[1].abs() < 1e-10);
}

#[test]
fn project_clamp_parameter_to_u_min() {
    let mut p = Geom2dProjectPointOnCurve::new([-5.0, 2.0], 1, 0.0, 10.0);
    p.perform();
    let param = p.lower_distance_parameter().unwrap();
    // negative x clamped to u_min = 0.0
    assert!((param - 0.0).abs() < 1e-10);
}

#[test]
fn extrema_curve_curve_lower_distance() {
    let mut e = Geom2dExtremaCurveCurve::new(10, 20);
    assert!(!e.is_done());
    e.perform([0.0, 0.0], [3.0, 4.0]);
    assert!(e.is_done());
    assert_eq!(e.nb_extrema(), 1);
    let d = e.lower_distance().unwrap();
    assert!((d - 5.0).abs() < 1e-10);
}

#[test]
fn inter_curve_curve_add_and_nb_points() {
    let mut ic = Geom2dInterCurveCurve::new(1, 2, 1e-6);
    assert!(!ic.is_done());
    ic.add_intersection(0.5, 0.5, [2.0, 3.0]);
    ic.add_intersection(0.7, 0.3, [4.0, 1.0]);
    assert!(ic.is_done());
    assert_eq!(ic.nb_points(), 2);
    assert_eq!(ic.point(0), Some([2.0, 3.0]));
    assert_eq!(ic.point(1), Some([4.0, 1.0]));
}

#[test]
fn inter_curve_curve_out_of_bounds_point() {
    let ic = Geom2dInterCurveCurve::new(1, 2, 1e-6);
    assert!(ic.point(99).is_none());
}
