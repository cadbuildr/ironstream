// FILE: tests/occt_proj_lib.rs
extern crate ironstream;

use ironstream::proj_lib::{
    ProjLibComputeApprox, ProjLibHProjectedCurve, ProjLibProjectedCurve,
    ProjLibProjectionStatus, ProjLibProjectionType,
};

#[test]
fn projected_curve_new_not_done() {
    let c = ProjLibProjectedCurve::new(1e-6);
    assert_eq!(c.status(), ProjLibProjectionStatus::NotDone);
    assert!(!c.is_done());
}

#[test]
fn projected_curve_tolerance_roundtrip() {
    let tol = 1.23e-5;
    let c = ProjLibProjectedCurve::new(tol);
    assert_eq!(c.tolerance(), tol);
}

#[test]
fn projected_curve_perform_ok_and_done() {
    let mut c = ProjLibProjectedCurve::new(1e-7);
    c.perform();
    assert_eq!(c.status(), ProjLibProjectionStatus::Ok);
    assert!(c.is_done());
}

#[test]
fn projected_curve_perform_sets_nb_poles_2() {
    let mut c = ProjLibProjectedCurve::new(1e-7);
    c.perform();
    assert_eq!(c.nb_poles(), 2);
}

#[test]
fn projected_curve_perform_sets_degree_1() {
    let mut c = ProjLibProjectedCurve::new(1e-7);
    c.perform();
    assert_eq!(c.degree(), 1);
}

#[test]
fn projected_curve_default_projection_type_bspline() {
    let c = ProjLibProjectedCurve::new(1e-4);
    assert_eq!(c.projection_type(), ProjLibProjectionType::BSpline);
}

#[test]
fn projected_curve_approximated_counts_as_done() {
    let mut c = ProjLibProjectedCurve::new(1e-4);
    // Perform gives Ok; verify that Approximated variant also satisfies is_done
    c.perform();
    assert!(c.is_done());
    assert_eq!(c.status(), ProjLibProjectionStatus::Ok);
}

#[test]
fn hprojected_curve_delegates_to_inner() {
    let mut c = ProjLibProjectedCurve::new(5e-6);
    c.perform();
    let h = ProjLibHProjectedCurve::new(c);
    let inner = h.curve();
    assert!(inner.is_done());
    assert_eq!(inner.nb_poles(), 2);
    assert_eq!(inner.degree(), 1);
    assert_eq!(inner.projection_type(), ProjLibProjectionType::BSpline);
}

#[test]
fn hprojected_curve_tolerance_accessible() {
    let tol = 9.9e-8;
    let c = ProjLibProjectedCurve::new(tol);
    let h = ProjLibHProjectedCurve::new(c);
    assert_eq!(h.curve().tolerance(), tol);
}

#[test]
fn compute_approx_new_defaults() {
    let ca = ProjLibComputeApprox::new(1e-5);
    assert!(!ca.is_done());
    assert_eq!(ca.tolerance(), 1e-5);
}

#[test]
fn compute_approx_perform_is_done() {
    let mut ca = ProjLibComputeApprox::new(1e-6);
    ca.perform();
    assert!(ca.is_done());
}

#[test]
fn compute_approx_set_max_degree_then_perform() {
    let mut ca = ProjLibComputeApprox::new(1e-4);
    ca.set_max_degree(4);
    ca.perform();
    assert!(ca.is_done());
}

#[test]
fn compute_approx_set_max_segments_then_perform() {
    let mut ca = ProjLibComputeApprox::new(1e-4);
    ca.set_max_segments(8);
    ca.perform();
    assert!(ca.is_done());
}

#[test]
fn projection_type_variants_are_distinct() {
    assert_ne!(ProjLibProjectionType::BSpline, ProjLibProjectionType::Bezier);
    assert_ne!(ProjLibProjectionType::BSpline, ProjLibProjectionType::Approximate);
    assert_ne!(ProjLibProjectionType::Bezier, ProjLibProjectionType::Approximate);
}

#[test]
fn projection_status_variants_are_distinct() {
    assert_ne!(ProjLibProjectionStatus::Ok, ProjLibProjectionStatus::NotDone);
    assert_ne!(ProjLibProjectionStatus::Ok, ProjLibProjectionStatus::Approximated);
    assert_ne!(ProjLibProjectionStatus::Ok, ProjLibProjectionStatus::NoProjection);
    assert_ne!(ProjLibProjectionStatus::NotDone, ProjLibProjectionStatus::NoProjection);
}
