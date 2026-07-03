// FILE: tests/occt_geom_proj.rs
extern crate ironstream;
use ironstream::geom_proj::*;

#[test]
fn test_proj_status_is_done() {
    assert!(ProjStatus::Done.is_done());
    assert!(!ProjStatus::NotDone.is_done());
    assert!(!ProjStatus::NullGeometry.is_done());
    assert!(!ProjStatus::NoSolution.is_done());
    assert_eq!(ProjStatus::default(), ProjStatus::NotDone);
}

#[test]
fn test_project_point_on_curve_basic() {
    let mut p = ProjectPointOnCurve::new([0.5, 0.0, 0.0], 1);
    p.perform();
    assert!(p.is_done());
    assert_eq!(p.nb_points(), 1);

    let param = p.nearest_point().unwrap();
    assert!((param - 0.5).abs() < 1e-9);
    let dist = p.lower_distance().unwrap();
    assert!((dist - 0.0).abs() < 1e-9);
}

#[test]
fn test_project_point_on_curve_off_axis() {
    let mut p = ProjectPointOnCurve::new([0.3, 1.0, 0.0], 1);
    p.perform();
    assert!(p.is_done());
    let dist = p.lower_distance().unwrap();
    // Point is 1.0 away from the curve in y
    assert!((dist - 1.0).abs() < 1e-9);
    // parameter() and distance() at index 0 match lower_distance_parameter
    let param_idx = p.parameter(0).unwrap();
    assert!((param_idx - 0.3).abs() < 1e-9);
    let dist_idx = p.distance(0).unwrap();
    assert!((dist_idx - dist).abs() < 1e-9);
}

#[test]
fn test_project_point_on_curve_null_geometry() {
    let mut p = ProjectPointOnCurve::new([0.5, 0.0, 0.0], 0);
    p.perform();
    assert!(!p.is_done());
    assert_eq!(p.status, ProjStatus::NullGeometry);
    assert_eq!(p.nb_points(), 0);
    assert!(p.nearest_point().is_none());
}

#[test]
fn test_project_point_on_curve_with_bounds() {
    // Point x=5.0 should be clamped to last_param=3.0
    let mut p = ProjectPointOnCurve::new([5.0, 0.0, 0.0], 2)
        .with_bounds(0.0, 3.0);
    p.perform();
    assert!(p.is_done());
    let param = p.nearest_point().unwrap();
    assert!((param - 3.0).abs() < 1e-9);
}

#[test]
fn test_project_point_on_surface_basic() {
    let mut p = ProjectPointOnSurface::new([0.3, 0.7, 2.0], 1);
    p.perform();
    assert!(p.is_done());
    assert_eq!(p.nb_points(), 1);

    let (u, v) = p.lower_distance_parameters().unwrap();
    assert!((u - 0.3).abs() < 1e-9);
    assert!((v - 0.7).abs() < 1e-9);

    let dist = p.lower_distance().unwrap();
    assert!((dist - 2.0).abs() < 1e-9);

    let np = p.nearest_point().unwrap();
    assert!((np[0] - 0.3).abs() < 1e-9);
    assert!((np[1] - 0.7).abs() < 1e-9);
    assert!((np[2] - 0.0).abs() < 1e-9);
}

#[test]
fn test_project_point_on_surface_null_geometry() {
    let mut p = ProjectPointOnSurface::new([0.0; 3], 0);
    p.perform();
    assert!(!p.is_done());
    assert_eq!(p.status, ProjStatus::NullGeometry);
    assert_eq!(p.nb_points(), 0);
    assert!(p.nearest_point().is_none());
    assert!(p.lower_distance_parameters().is_none());
    assert!(p.lower_distance().is_none());
}
