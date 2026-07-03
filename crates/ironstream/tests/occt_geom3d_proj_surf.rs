use ironstream::geom3d_proj_surf::*;

#[test]
fn project_point_on_surface_basic() {
    let mut p = GeomApiProjectPointOnSurface::new(1, 0.0, 1.0, 0.0, 1.0);
    p.perform([0.5, 0.5, 3.0]);
    assert!(p.is_done());
    assert_eq!(p.nb_points(), 1);
    // distance is purely from z=3 projected to z=0
    assert!((p.lower_distance().unwrap() - 3.0).abs() < 1e-10);
    let (u, v) = p.lower_distance_parameter().unwrap();
    assert!((u - 0.5).abs() < 1e-14);
    assert!((v - 0.5).abs() < 1e-14);
}

#[test]
fn project_point_on_surface_clamped() {
    let mut p = GeomApiProjectPointOnSurface::new(1, 0.0, 1.0, 0.0, 1.0);
    p.perform([2.0, 3.0, 0.0]);
    let (u, v) = p.lower_distance_parameter().unwrap();
    assert!((u - 1.0).abs() < 1e-14);
    assert!((v - 1.0).abs() < 1e-14);
}

#[test]
fn project_point_on_surface_nearest_point() {
    let mut p = GeomApiProjectPointOnSurface::new(1, 0.0, 5.0, 0.0, 5.0);
    p.perform([2.0, 3.0, 10.0]);
    let np = p.nearest_point().unwrap();
    assert!((np[0] - 2.0).abs() < 1e-14);
    assert!((np[1] - 3.0).abs() < 1e-14);
    assert!(np[2].abs() < 1e-14);
}

#[test]
fn project_point_on_curve_basic() {
    let mut p = GeomApiProjectPointOnCurve::new(1, 0.0, 10.0);
    p.perform([5.0, 3.0, 4.0]);
    assert!(p.is_done());
    let t = p.lower_distance_parameter().unwrap();
    assert!((t - 5.0).abs() < 1e-14);
    // dist = sqrt(0 + 9 + 16) = 5
    let d = p.lower_distance().unwrap();
    assert!((d - 5.0).abs() < 1e-10);
}

#[test]
fn project_point_on_curve_clamped() {
    let mut p = GeomApiProjectPointOnCurve::new(1, 0.0, 3.0);
    p.perform([10.0, 0.0, 0.0]);
    let t = p.lower_distance_parameter().unwrap();
    assert!((t - 3.0).abs() < 1e-14); // clamped to u_max
}

#[test]
fn extrema_curve_curve_lower_distance() {
    let mut e = GeomApiExtremaCurveCurve::new(3, 7);
    e.perform();
    assert!(e.is_done);
    assert_eq!(e.nb_extrema(), 1);
    assert!((e.lower_distance().unwrap() - 4.0).abs() < 1e-14);

    let mut e2 = GeomApiExtremaCurveSurface::new(1, 1);
    e2.perform();
    assert!(e2.is_done);
    assert!((e2.lower_distance().unwrap()).abs() < 1e-14);
}
