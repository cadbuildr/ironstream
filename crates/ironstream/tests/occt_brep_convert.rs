extern crate ironstream;
use ironstream::brep_convert::*;

#[test]
fn test_to_bspline_surface_perform_is_done() {
    let mut conv = ToBSplineSurface::new(42);
    assert!(!conv.is_done());
    conv.perform();
    assert!(conv.is_done());
}

#[test]
fn test_to_bspline_surface_result_degrees() {
    let mut conv = ToBSplineSurface::new(0)
        .with_max_degree(5)
        .with_continuity(GeomAbs_Shape::C2);
    conv.perform();
    let r = conv.result().unwrap();
    assert_eq!(r.degree_u, 3);
    assert_eq!(r.degree_v, 3);
}

#[test]
fn test_approx_surface_max_error_within_tolerance() {
    // max_degree must be <= nb_poles used internally (5) to avoid overflow in BSplineSurfaceResult::new
    let mut approx = ApproxSurface::new(0, 1e-5, GeomAbs_Shape::C1, 3, 20);
    approx.build();
    assert!(approx.is_done());
    assert!(approx.max_error() < 1e-5);
}

#[test]
fn test_project_curve_on_surface_nb_points() {
    let mut proj = ProjectCurveOnSurface::new(0, 1e-7);
    proj.perform(10);
    assert!(proj.is_done());
    assert_eq!(proj.nb_points(), 10);
}

#[test]
fn test_project_curve_endpoints() {
    let mut proj = ProjectCurveOnSurface::new(0, 1e-7);
    proj.perform(3);
    let first = proj.param_at(0).unwrap();
    let last = proj.param_at(2).unwrap();
    assert!((first[0] - 0.0).abs() < 1e-10);
    assert!((last[0] - 1.0).abs() < 1e-10);
}

#[test]
fn test_bspline_result_knot_count_formula() {
    let r = BSplineSurfaceResult::new(3, 3, 7, 5);
    // nb_knots = nb_poles - degree + 1
    assert_eq!(r.nb_knots_u, 5); // 7 - 3 + 1
    assert_eq!(r.nb_knots_v, 3); // 5 - 3 + 1
}
