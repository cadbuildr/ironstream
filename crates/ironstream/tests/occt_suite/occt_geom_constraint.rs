// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_constraint.rs

//! Ported OpenCascade unit tests -- `GeomPlate` constraint types.
//!
//! These tests mirror the behaviour documented in the OCCT source and test
//! suites for `GeomPlate_PointConstraint` and `GeomPlate_CurveConstraint`
//! (TKGeomAlgo package).  All imports are from `ironstream::` only.

extern crate ironstream;
use ironstream::geom_constraint::*;

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn assert_near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected |{a} - {b}| <= {tol}, got {}",
        (a - b).abs()
    );
}

const EPS: f64 = 1e-12;

// ---------------------------------------------------------------------------
// GeomConstraintType
// ---------------------------------------------------------------------------

#[test]
fn constraint_type_ordering() {
    assert!(GeomConstraintType::G0 < GeomConstraintType::G1);
    assert!(GeomConstraintType::G1 < GeomConstraintType::G2);
    assert!(GeomConstraintType::G0 < GeomConstraintType::G2);
}

#[test]
fn constraint_type_equality() {
    assert_eq!(GeomConstraintType::G0, GeomConstraintType::G0);
    assert_eq!(GeomConstraintType::G1, GeomConstraintType::G1);
    assert_eq!(GeomConstraintType::G2, GeomConstraintType::G2);
    assert_ne!(GeomConstraintType::G0, GeomConstraintType::G1);
    assert_ne!(GeomConstraintType::G1, GeomConstraintType::G2);
}

#[test]
fn constraint_type_copy() {
    let t = GeomConstraintType::G1;
    let u = t; // Copy
    assert_eq!(t, u);
}

// ---------------------------------------------------------------------------
// GeomPointConstraint — construction
// ---------------------------------------------------------------------------

#[test]
fn point_constraint_stores_point() {
    let pt = [1.0_f64, 2.0, 3.0];
    let c = GeomPointConstraint::new(pt, GeomConstraintType::G0);
    let got = c.point();
    assert_near(got[0], 1.0, EPS);
    assert_near(got[1], 2.0, EPS);
    assert_near(got[2], 3.0, EPS);
}

#[test]
fn point_constraint_stores_type() {
    let c = GeomPointConstraint::new([0.0, 0.0, 0.0], GeomConstraintType::G1);
    assert_eq!(c.constraint_type(), GeomConstraintType::G1);
}

#[test]
fn point_constraint_default_weight_is_one() {
    let c = GeomPointConstraint::new([0.0, 0.0, 0.0], GeomConstraintType::G2);
    assert_near(c.weight(), 1.0, EPS);
}

// ---------------------------------------------------------------------------
// GeomPointConstraint — set_weight
// ---------------------------------------------------------------------------

#[test]
fn point_constraint_set_weight() {
    let mut c = GeomPointConstraint::new([0.0, 0.0, 0.0], GeomConstraintType::G0);
    c.set_weight(3.5);
    assert_near(c.weight(), 3.5, EPS);
}

#[test]
fn point_constraint_set_weight_zero() {
    let mut c = GeomPointConstraint::new([0.0, 0.0, 0.0], GeomConstraintType::G0);
    c.set_weight(0.0);
    assert_near(c.weight(), 0.0, EPS);
}

#[test]
fn point_constraint_weight_does_not_affect_point() {
    let pt = [4.0_f64, 5.0, 6.0];
    let mut c = GeomPointConstraint::new(pt, GeomConstraintType::G1);
    c.set_weight(99.0);
    let got = c.point();
    assert_near(got[0], 4.0, EPS);
    assert_near(got[1], 5.0, EPS);
    assert_near(got[2], 6.0, EPS);
    assert_eq!(c.constraint_type(), GeomConstraintType::G1);
}

// ---------------------------------------------------------------------------
// GeomPointConstraint — clone independence
// ---------------------------------------------------------------------------

#[test]
fn point_constraint_clone_is_independent() {
    let c = GeomPointConstraint::new([1.0, 2.0, 3.0], GeomConstraintType::G0);
    let mut d = c.clone();
    d.set_weight(42.0);
    // original weight unchanged
    assert_near(c.weight(), 1.0, EPS);
    assert_near(d.weight(), 42.0, EPS);
}

// ---------------------------------------------------------------------------
// GeomCurveConstraint — construction
// ---------------------------------------------------------------------------

#[test]
fn curve_constraint_stores_nb_points() {
    let c = GeomCurveConstraint::new(10, GeomConstraintType::G1, 2);
    assert_eq!(c.nb_points(), 10);
}

#[test]
fn curve_constraint_stores_type() {
    let c = GeomCurveConstraint::new(5, GeomConstraintType::G2, 3);
    assert_eq!(c.constraint_type(), GeomConstraintType::G2);
}

#[test]
fn curve_constraint_stores_order() {
    let c = GeomCurveConstraint::new(8, GeomConstraintType::G0, 4);
    assert_eq!(c.order(), 4);
}

#[test]
fn curve_constraint_default_weight_is_one() {
    let c = GeomCurveConstraint::new(1, GeomConstraintType::G0, 1);
    assert_near(c.weight(), 1.0, EPS);
}

// ---------------------------------------------------------------------------
// GeomCurveConstraint — set_weight
// ---------------------------------------------------------------------------

#[test]
fn curve_constraint_set_weight() {
    let mut c = GeomCurveConstraint::new(10, GeomConstraintType::G1, 2);
    c.set_weight(2.5);
    assert_near(c.weight(), 2.5, EPS);
}

#[test]
fn curve_constraint_weight_does_not_affect_other_fields() {
    let mut c = GeomCurveConstraint::new(7, GeomConstraintType::G2, 3);
    c.set_weight(0.1);
    assert_eq!(c.nb_points(), 7);
    assert_eq!(c.constraint_type(), GeomConstraintType::G2);
    assert_eq!(c.order(), 3);
}

// ---------------------------------------------------------------------------
// GeomCurveConstraint — clone independence
// ---------------------------------------------------------------------------

#[test]
fn curve_constraint_clone_is_independent() {
    let c = GeomCurveConstraint::new(4, GeomConstraintType::G1, 2);
    let mut d = c.clone();
    d.set_weight(7.0);
    assert_near(c.weight(), 1.0, EPS);
    assert_near(d.weight(), 7.0, EPS);
}

// ---------------------------------------------------------------------------
// GeomConstraintSet — empty state
// ---------------------------------------------------------------------------

#[test]
fn constraint_set_new_is_empty() {
    let s = GeomConstraintSet::new();
    assert_eq!(s.nb_points(), 0);
    assert_eq!(s.nb_curves(), 0);
}

// ---------------------------------------------------------------------------
// GeomConstraintSet — add_point
// ---------------------------------------------------------------------------

#[test]
fn constraint_set_add_single_point() {
    let mut s = GeomConstraintSet::new();
    s.add_point(GeomPointConstraint::new([1.0, 2.0, 3.0], GeomConstraintType::G0));
    assert_eq!(s.nb_points(), 1);
    assert_eq!(s.nb_curves(), 0);
}

#[test]
fn constraint_set_add_multiple_points() {
    let mut s = GeomConstraintSet::new();
    for i in 0..5_usize {
        s.add_point(GeomPointConstraint::new(
            [i as f64, 0.0, 0.0],
            GeomConstraintType::G0,
        ));
    }
    assert_eq!(s.nb_points(), 5);
}

#[test]
fn constraint_set_point_retrieval() {
    let mut s = GeomConstraintSet::new();
    s.add_point(GeomPointConstraint::new([3.0, 4.0, 5.0], GeomConstraintType::G1));
    let p = s.point(0);
    let coords = p.point();
    assert_near(coords[0], 3.0, EPS);
    assert_near(coords[1], 4.0, EPS);
    assert_near(coords[2], 5.0, EPS);
    assert_eq!(p.constraint_type(), GeomConstraintType::G1);
}

#[test]
fn constraint_set_point_index_order_preserved() {
    let mut s = GeomConstraintSet::new();
    s.add_point(GeomPointConstraint::new([1.0, 0.0, 0.0], GeomConstraintType::G0));
    s.add_point(GeomPointConstraint::new([2.0, 0.0, 0.0], GeomConstraintType::G1));
    s.add_point(GeomPointConstraint::new([3.0, 0.0, 0.0], GeomConstraintType::G2));
    assert_near(s.point(0).point()[0], 1.0, EPS);
    assert_near(s.point(1).point()[0], 2.0, EPS);
    assert_near(s.point(2).point()[0], 3.0, EPS);
    assert_eq!(s.point(0).constraint_type(), GeomConstraintType::G0);
    assert_eq!(s.point(1).constraint_type(), GeomConstraintType::G1);
    assert_eq!(s.point(2).constraint_type(), GeomConstraintType::G2);
}

// ---------------------------------------------------------------------------
// GeomConstraintSet — add_curve
// ---------------------------------------------------------------------------

#[test]
fn constraint_set_add_single_curve() {
    let mut s = GeomConstraintSet::new();
    s.add_curve(GeomCurveConstraint::new(10, GeomConstraintType::G1, 2));
    assert_eq!(s.nb_curves(), 1);
    assert_eq!(s.nb_points(), 0);
}

#[test]
fn constraint_set_add_multiple_curves() {
    let mut s = GeomConstraintSet::new();
    for _ in 0..3 {
        s.add_curve(GeomCurveConstraint::new(5, GeomConstraintType::G2, 3));
    }
    assert_eq!(s.nb_curves(), 3);
}

#[test]
fn constraint_set_curve_retrieval() {
    let mut s = GeomConstraintSet::new();
    s.add_curve(GeomCurveConstraint::new(12, GeomConstraintType::G2, 4));
    let c = s.curve(0);
    assert_eq!(c.nb_points(), 12);
    assert_eq!(c.constraint_type(), GeomConstraintType::G2);
    assert_eq!(c.order(), 4);
}

#[test]
fn constraint_set_curve_index_order_preserved() {
    let mut s = GeomConstraintSet::new();
    s.add_curve(GeomCurveConstraint::new(2, GeomConstraintType::G0, 1));
    s.add_curve(GeomCurveConstraint::new(4, GeomConstraintType::G1, 2));
    s.add_curve(GeomCurveConstraint::new(6, GeomConstraintType::G2, 3));
    assert_eq!(s.curve(0).nb_points(), 2);
    assert_eq!(s.curve(1).nb_points(), 4);
    assert_eq!(s.curve(2).nb_points(), 6);
}

// ---------------------------------------------------------------------------
// GeomConstraintSet — mixed point + curve
// ---------------------------------------------------------------------------

#[test]
fn constraint_set_mixed_counts_are_independent() {
    let mut s = GeomConstraintSet::new();
    s.add_point(GeomPointConstraint::new([0.0, 0.0, 0.0], GeomConstraintType::G0));
    s.add_point(GeomPointConstraint::new([1.0, 0.0, 0.0], GeomConstraintType::G0));
    s.add_curve(GeomCurveConstraint::new(8, GeomConstraintType::G1, 2));
    assert_eq!(s.nb_points(), 2);
    assert_eq!(s.nb_curves(), 1);
}

// ---------------------------------------------------------------------------
// GeomConstraintSet — clone independence
// ---------------------------------------------------------------------------

#[test]
fn constraint_set_clone_is_independent() {
    let mut s = GeomConstraintSet::new();
    s.add_point(GeomPointConstraint::new([0.0, 0.0, 0.0], GeomConstraintType::G0));
    let mut t = s.clone();
    t.add_point(GeomPointConstraint::new([1.0, 1.0, 1.0], GeomConstraintType::G1));
    assert_eq!(s.nb_points(), 1);
    assert_eq!(t.nb_points(), 2);
}

// ---------------------------------------------------------------------------
// GeomConstraintSet — Default trait
// ---------------------------------------------------------------------------

#[test]
fn constraint_set_default_is_empty() {
    let s: GeomConstraintSet = Default::default();
    assert_eq!(s.nb_points(), 0);
    assert_eq!(s.nb_curves(), 0);
}

// ---------------------------------------------------------------------------
// Panic on out-of-bounds access (should_panic)
// ---------------------------------------------------------------------------

#[test]
#[should_panic]
fn constraint_set_point_oob_panics() {
    let s = GeomConstraintSet::new();
    let _ = s.point(0);
}

#[test]
#[should_panic]
fn constraint_set_curve_oob_panics() {
    let s = GeomConstraintSet::new();
    let _ = s.curve(0);
}
