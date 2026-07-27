// FILE: tests/occt_geom_plate.rs
extern crate ironstream;

use ironstream::geom_plate::*;

// TEST: Default PlateBuildParams have the correct OCCT defaults
#[test]
fn default_params_values() {
    let p = PlateBuildParams::default();
    assert_eq!(p.degree, 3);
    assert_eq!(p.max_degree, 8);
    assert_eq!(p.max_segments, 10);
    assert!((p.tolerance - 1e-4).abs() < 1e-12);
    assert_eq!(p.nb_iter, 3);
}

// TEST: New builder has no constraints and is not done
#[test]
fn new_builder_empty_not_done() {
    let b = GeomPlateBuildPlateSurface::new(PlateBuildParams::default());
    assert_eq!(b.nb_curve_constraints(), 0);
    assert_eq!(b.nb_point_constraints(), 0);
    assert!(!b.is_done());
}

// TEST: perform sets is_done to true
#[test]
fn perform_sets_is_done() {
    let mut b = GeomPlateBuildPlateSurface::new(PlateBuildParams::default());
    assert!(!b.is_done());
    b.perform();
    assert!(b.is_done());
}

// TEST: Adding curve constraints updates the count
#[test]
fn add_curve_constraints_count() {
    let mut b = GeomPlateBuildPlateSurface::new(PlateBuildParams::default());
    b.add_curve_constraint(PlateCurveConstraint::new(PlateContinuity::G0, 5));
    assert_eq!(b.nb_curve_constraints(), 1);
    b.add_curve_constraint(PlateCurveConstraint::new(PlateContinuity::G1, 10));
    assert_eq!(b.nb_curve_constraints(), 2);
    b.add_curve_constraint(PlateCurveConstraint::new(PlateContinuity::G2, 15));
    assert_eq!(b.nb_curve_constraints(), 3);
}

// TEST: Adding point constraints updates the count
#[test]
fn add_point_constraints_count() {
    let mut b = GeomPlateBuildPlateSurface::new(PlateBuildParams::default());
    b.add_point_constraint(PlatePointConstraint::new([0.0, 0.0, 0.0], PlateContinuity::G0));
    assert_eq!(b.nb_point_constraints(), 1);
    b.add_point_constraint(PlatePointConstraint::new([1.0, 0.0, 0.0], PlateContinuity::G0));
    b.add_point_constraint(PlatePointConstraint::new([0.0, 1.0, 0.0], PlateContinuity::G0));
    assert_eq!(b.nb_point_constraints(), 3);
}

// TEST: Mixed curve and point constraints are counted independently
#[test]
fn mixed_constraints_independent_counts() {
    let mut b = GeomPlateBuildPlateSurface::new(PlateBuildParams::default());
    b.add_curve_constraint(PlateCurveConstraint::new(PlateContinuity::G1, 8));
    b.add_point_constraint(PlatePointConstraint::new([2.0, 3.0, 0.0], PlateContinuity::G0));
    b.add_point_constraint(PlatePointConstraint::new([4.0, 0.0, 1.0], PlateContinuity::G0));
    assert_eq!(b.nb_curve_constraints(), 1);
    assert_eq!(b.nb_point_constraints(), 2);
}

// TEST: g0_error and g1_error return 0.0 (stub) after perform
#[test]
fn error_stubs_after_perform() {
    let mut b = GeomPlateBuildPlateSurface::new(PlateBuildParams::default());
    b.add_curve_constraint(PlateCurveConstraint::new(PlateContinuity::G0, 10));
    b.perform();
    assert_eq!(b.g0_error(), 0.0);
    assert_eq!(b.g1_error(), 0.0);
}

// TEST: PlateContinuity ordering matches G0 < G1 < G2
#[test]
fn continuity_ordering() {
    assert!(PlateContinuity::G0 < PlateContinuity::G1);
    assert!(PlateContinuity::G1 < PlateContinuity::G2);
    assert!(PlateContinuity::G0 < PlateContinuity::G2);
}

// TEST: PlateContinuity equality and copy
#[test]
fn continuity_equality_and_copy() {
    let a = PlateContinuity::G1;
    let b = a;
    assert_eq!(a, b);
    assert_ne!(a, PlateContinuity::G0);
    assert_ne!(a, PlateContinuity::G2);
}

// TEST: PlateCurveConstraint stores fields correctly
#[test]
fn curve_constraint_fields() {
    let c = PlateCurveConstraint::new(PlateContinuity::G2, 42);
    assert_eq!(c.continuity, PlateContinuity::G2);
    assert_eq!(c.nb_points, 42);
}

// TEST: PlatePointConstraint stores fields correctly
#[test]
fn point_constraint_fields() {
    let pt = [1.5, -2.3, 0.7];
    let c = PlatePointConstraint::new(pt, PlateContinuity::G1);
    assert_eq!(c.continuity, PlateContinuity::G1);
    assert!((c.point[0] - 1.5).abs() < 1e-15);
    assert!((c.point[1] - (-2.3)).abs() < 1e-15);
    assert!((c.point[2] - 0.7).abs() < 1e-15);
}

// TEST: Full workflow — add constraints, perform, check state
#[test]
fn full_workflow() {
    let mut b = GeomPlateBuildPlateSurface::new(PlateBuildParams::default());
    b.add_curve_constraint(PlateCurveConstraint::new(PlateContinuity::G0, 10));
    b.add_curve_constraint(PlateCurveConstraint::new(PlateContinuity::G1, 20));
    b.add_point_constraint(PlatePointConstraint::new([0.0, 0.0, 1.0], PlateContinuity::G0));
    b.add_point_constraint(PlatePointConstraint::new([1.0, 0.0, 0.0], PlateContinuity::G0));
    b.add_point_constraint(PlatePointConstraint::new([0.0, 1.0, 0.0], PlateContinuity::G0));

    assert!(!b.is_done());
    b.perform();
    assert!(b.is_done());
    assert_eq!(b.nb_curve_constraints(), 2);
    assert_eq!(b.nb_point_constraints(), 3);
    assert_eq!(b.g0_error(), 0.0);
    assert_eq!(b.g1_error(), 0.0);
}

// TEST: Custom PlateBuildParams are stored with correct values
#[test]
fn custom_params_stored() {
    let params = PlateBuildParams {
        degree: 5,
        max_degree: 12,
        max_segments: 20,
        tolerance: 1e-6,
        nb_iter: 10,
    };
    assert_eq!(params.degree, 5);
    assert_eq!(params.max_degree, 12);
    assert_eq!(params.max_segments, 20);
    assert!((params.tolerance - 1e-6).abs() < 1e-18);
    assert_eq!(params.nb_iter, 10);
}
