use ironstream::geom2d_tools_ext::*;

#[test]
fn line_line_intersect_basic_cross_at_0_5_0() {
    let i = Geom2dLineLineIntersect::new(
        [0.0, 0.0], [1.0, 0.0],
        [0.5, -1.0], [0.0, 1.0],
    );
    assert!(i.is_done());
    assert!(!i.is_parallel());
    let pt = i.point().unwrap();
    assert!((pt[0] - 0.5).abs() < 1e-10);
    assert!(pt[1].abs() < 1e-10);
}

#[test]
fn line_line_parallel_no_point() {
    let i = Geom2dLineLineIntersect::new(
        [0.0, 0.0], [1.0, 0.0],
        [0.0, 1.0], [1.0, 0.0],
    );
    assert!(i.is_done());
    assert!(i.is_parallel());
    assert!(i.point().is_none());
}

#[test]
fn project_point_on_curve_is_done_nb_points_1_distance_nonneg() {
    let p = Geom2dProjectPointOnCurve::new([1.0, 0.0], 1, 0.0, std::f64::consts::PI);
    assert!(p.is_done());
    assert_eq!(p.nb_points(), 1);
    assert!(p.lower_distance() >= 0.0);
}

#[test]
fn bspline_to_bezier_is_done_nb_arcs_ge_1_arc_has_degree_plus_1_points() {
    let b = Geom2dBSplineToBezier::new(1, 0.0, std::f64::consts::PI * 2.0, 3);
    assert!(b.is_done());
    assert!(b.nb_arcs() >= 1);
    let arc = b.bezier_arc(1);
    assert!(arc.is_some());
    assert_eq!(arc.unwrap().len(), b.degree() + 1);
}

#[test]
fn bspline_to_bezier_invalid_curve_id_0_not_done() {
    let b = Geom2dBSplineToBezier::new(0, 0.0, 1.0, 3);
    assert!(!b.is_done());
    assert_eq!(b.nb_arcs(), 0);
}
