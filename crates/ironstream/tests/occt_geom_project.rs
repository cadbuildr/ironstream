extern crate ironstream;
use ironstream::geom_project::*;

#[test]
fn project_point_on_surf_done() {
    let p = ProjectPointOnSurf::new([3.0, 4.0, 5.0], 1);
    assert!(p.is_done());
    assert_eq!(p.nb_points(), 1);
    let (u, v) = p.lower_distance_parameters().unwrap();
    assert!((u - 3.0).abs() < 1e-10);
    assert!((v - 4.0).abs() < 1e-10);
}

#[test]
fn project_point_on_surf_distance() {
    let p = ProjectPointOnSurf::new([0.0, 0.0, 10.0], 1);
    assert!((p.lower_distance() - 10.0).abs() < 1e-10);
    let pt = p.lower_distance_point().unwrap();
    assert!((pt[2]).abs() < 1e-10);
}

#[test]
fn project_point_on_curve_closest() {
    let curve = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
    let p = ProjectPointOnCurve::new([1.0, 1.0, 0.0], curve);
    assert_eq!(p.nb_points(), 1);
    assert!((p.lower_distance() - 1.0).abs() < 1e-10);
}

#[test]
fn project_point_on_curve_parameter() {
    let curve = vec![[0.0, 0.0, 0.0], [5.0, 0.0, 0.0]];
    let p = ProjectPointOnCurve::new([5.0, 0.0, 0.0], curve);
    assert!((p.lower_distance_parameter() - 1.0).abs() < 1e-10);
}

#[test]
fn projected_curve_2d() {
    let pts3d = vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
    let c = ProjectedCurve::new(1, pts3d);
    assert!(c.is_done());
    assert_eq!(c.nb_knots(), 2);
    assert_eq!(c.curve_pts_2d[0], [1.0, 2.0]);
    assert_eq!(c.curve_pts_2d[1], [4.0, 5.0]);
}
