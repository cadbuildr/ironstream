// FILE: tests/occt_pnt_surf.rs
// Integration tests for pnt_surf module
extern crate ironstream;

use ironstream::pnt_surf::{
    ExtremaCurveCurve, ExtremaSurfaceSurface, ProjectPointOnCurve, ProjectPointOnSurf,
};

#[test]
fn project_point_on_curve_basic_projection() {
    let p = ProjectPointOnCurve::new([0.5, 0.0, 0.0], 1, 0.0, 1.0);
    assert!(p.is_done());
    assert_eq!(p.nb_points(), 1);
    let t = p.nearest_point().unwrap();
    assert!((t - 0.5).abs() < 1e-6);
    // parameter(1) is 1-based
    assert!((p.parameter(1).unwrap() - 0.5).abs() < 1e-6);
    // distance(1)
    assert!(p.distance(1).is_some());
}

#[test]
fn project_point_on_curve_no_curve_not_done() {
    let p = ProjectPointOnCurve::new([0.3, 0.0, 0.0], 0, 0.0, 1.0);
    assert!(!p.is_done());
    assert_eq!(p.nb_points(), 0);
    assert!(p.nearest_point().is_none());
    assert!(p.lower_distance().is_none());
}

#[test]
fn project_point_on_surf_basic_and_clamping() {
    // Point on XY plane, within bounds
    let p = ProjectPointOnSurf::new([0.5, 0.3, 2.0], 1, 0.0, 1.0, 0.0, 1.0);
    assert!(p.is_done());
    assert_eq!(p.nb_points(), 1);
    let (u, v) = p.nearest_point().unwrap();
    assert!((u - 0.5).abs() < 1e-10);
    assert!((v - 0.3).abs() < 1e-10);
    assert!((p.lower_distance().unwrap() - 2.0).abs() < 1e-10);

    // Point outside bounds → clamped
    let p2 = ProjectPointOnSurf::new([5.0, -1.0, 0.0], 1, 0.0, 1.0, 0.0, 1.0);
    assert!(p2.is_done());
    let (u2, v2) = p2.nearest_point().unwrap();
    assert!((u2 - 1.0).abs() < 1e-10);
    assert!((v2 - 0.0).abs() < 1e-10);

    // parameters(1) and distance(1) — 1-based indexing
    let params = p.lower_distance_parameters().unwrap();
    assert!((params.0 - 0.5).abs() < 1e-10);
    assert!((params.1 - 0.3).abs() < 1e-10);
}

#[test]
fn project_point_on_surf_no_surface_not_done() {
    let p = ProjectPointOnSurf::new([0.5, 0.5, 1.0], 0, 0.0, 1.0, 0.0, 1.0);
    assert!(!p.is_done());
    assert_eq!(p.nb_points(), 0);
    assert!(p.nearest_point().is_none());
}

#[test]
fn extrema_curve_curve_done_and_result() {
    // Both curve_ids > 0 → is_done
    let e = ExtremaCurveCurve::new(1, 2, 0.0, 1.0, 0.0, 1.0);
    assert!(e.is_done());
    assert_eq!(e.nb_extrema(), 1);
    // Both param ranges are [0,1], midpoints are both 0.5 → distance = 0
    let dist = e.lower_distance().unwrap();
    assert!(dist.abs() < 1e-10);

    // extrema(1) is 1-based
    let pair = e.extrema(1).unwrap();
    assert!((pair.param1 - 0.5).abs() < 1e-10);
    assert!((pair.param2 - 0.5).abs() < 1e-10);

    // At least one curve_id == 0 → not done
    let e_bad = ExtremaCurveCurve::new(1, 0, 0.0, 1.0, 0.0, 1.0);
    assert!(!e_bad.is_done());
    assert_eq!(e_bad.nb_extrema(), 0);
    assert!(e_bad.lower_distance().is_none());
}

#[test]
fn extrema_surface_surface_done() {
    let e = ExtremaSurfaceSurface::new(1, 2);
    assert!(e.is_done());
    assert_eq!(e.nb_extrema(), 1);
    assert!(e.lower_distance().unwrap().abs() < 1e-10);
}
