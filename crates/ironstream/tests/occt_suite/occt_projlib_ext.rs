// Integration tests for projlib_ext
use ironstream::projlib_ext::*;

#[test]
fn projected_curve_parameter_range_stored() {
    let mut pc = ProjLibProjectedCurve::new(3, ProjSurfaceType::Sphere);
    pc.perform(8, 0.5, 2.5);
    assert!(pc.is_done());
    assert!((pc.first_parameter() - 0.5).abs() < 1e-12);
    assert!((pc.last_parameter() - 2.5).abs() < 1e-12);
    assert_eq!(pc.get_type(), ProjSurfaceType::Sphere);
}

#[test]
fn projected_curve_null_curve() {
    let mut pc = ProjLibProjectedCurve::new(5, ProjSurfaceType::Plane);
    pc.perform(0, 0.0, 1.0);
    assert_eq!(pc.status, ProjLibStatus::NullCurve);
    assert!(!pc.is_done());
}

#[test]
fn projected_curve_use_same_param() {
    let mut pc = ProjLibProjectedCurve::new(4, ProjSurfaceType::Cylinder);
    pc.set_use_same_param(true);
    assert!(pc.use_same_param);
}

#[test]
fn compute_approx_errors_bounded() {
    let mut ca = ProjLibComputeApprox::new(5, 10, 1e-5, 4);
    ca.perform();
    assert!(ca.is_done());
    assert!(ca.tolerance_reached() <= 1e-5);
    assert!(ca.average_error() <= ca.tolerance_reached());
    assert_eq!(ca.nb_poles(), 5); // degree + 1
}

#[test]
fn proj_plane_null_plane() {
    let mut pp = ProjLibPlane::new(0);
    pp.project(3);
    assert_eq!(pp.status, ProjLibStatus::NullSurface);
    assert!(!pp.is_done());
}

#[test]
fn proj_cylinder_result_id_derived() {
    let mut pc = ProjLibCylinder::new(10);
    pc.project(5);
    assert!(pc.is_done());
    assert_eq!(pc.curve2d(), 5 + 10 + 18000);
}
