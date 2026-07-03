//! Ported OpenCascade unit tests -- `Geom2d_CartesianPoint`.
//!
//! Source: OpenCascade Technology, `TKG2d` package, `Geom2d_CartesianPoint`
//! class tests (ported by hand). Each `#[test]` corresponds to one logical
//! GTest or scenario from the OCCT test suite for `Geom2d_CartesianPoint`.
//!
//! Tolerances used:
//! * linear: `precision::CONFUSION` = 1e-7
//! * angular: `precision::ANGULAR` = 1e-12

use ironstream::geom2d_point::CartesianPoint2d;
use ironstream::gp2d::{Pnt2d, Trsf2d};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, PI};

/// `EXPECT_NEAR` helper.
fn expect_near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ~= {b} within {tol} (diff {})",
        (a - b).abs()
    );
}

// ── 1. Constructor from coordinates ─────────────────────────────────────────

#[test]
fn construct_from_coords() {
    let p = CartesianPoint2d::new(1.0, 2.0);
    expect_near(p.x(), 1.0, CONFUSION);
    expect_near(p.y(), 2.0, CONFUSION);
}

// ── 2. Constructor from gp_Pnt2d ────────────────────────────────────────────

#[test]
fn construct_from_pnt2d() {
    let gp = Pnt2d::new(5.0, -3.0);
    let p = CartesianPoint2d::from_pnt2d(gp);
    expect_near(p.x(), 5.0, CONFUSION);
    expect_near(p.y(), -3.0, CONFUSION);
    // pnt2d() round-trips the value
    assert_eq!(p.pnt2d(), gp);
}

// ── 3. SetCoord ──────────────────────────────────────────────────────────────

#[test]
fn set_coord() {
    let mut p = CartesianPoint2d::new(0.0, 0.0);
    p.set_coord(3.5, -7.25);
    expect_near(p.x(), 3.5, CONFUSION);
    expect_near(p.y(), -7.25, CONFUSION);
    let (x, y) = p.coord();
    expect_near(x, 3.5, CONFUSION);
    expect_near(y, -7.25, CONFUSION);
}

// ── 4. SetX / SetY ───────────────────────────────────────────────────────────

#[test]
fn set_x_and_set_y() {
    let mut p = CartesianPoint2d::new(10.0, 20.0);
    p.set_x(42.0);
    expect_near(p.x(), 42.0, CONFUSION);
    expect_near(p.y(), 20.0, CONFUSION); // Y unchanged

    p.set_y(-99.0);
    expect_near(p.x(), 42.0, CONFUSION); // X unchanged
    expect_near(p.y(), -99.0, CONFUSION);
}

// ── 5. Distance -- 3-4-5 triangle ───────────────────────────────────────────

#[test]
fn distance_3_4_5() {
    let a = CartesianPoint2d::new(0.0, 0.0);
    let b = CartesianPoint2d::new(3.0, 4.0);
    expect_near(a.distance(&b), 5.0, CONFUSION);
    // symmetry
    expect_near(b.distance(&a), 5.0, CONFUSION);
}

// ── 6. Distance to self is zero ──────────────────────────────────────────────

#[test]
fn distance_to_self_is_zero() {
    let p = CartesianPoint2d::new(-7.0, 13.5);
    expect_near(p.distance(&p), 0.0, CONFUSION);
}

// ── 7. Distance -- unit step along X ─────────────────────────────────────────

#[test]
fn distance_unit_x() {
    let a = CartesianPoint2d::new(0.0, 0.0);
    let b = CartesianPoint2d::new(1.0, 0.0);
    expect_near(a.distance(&b), 1.0, CONFUSION);
}

// ── 8. Transform -- translation ──────────────────────────────────────────────

#[test]
fn transform_translation() {
    let t = Trsf2d::translation(Pnt2d::new(10.0, -5.0));
    let mut p = CartesianPoint2d::new(1.0, 2.0);
    p.transform(&t);
    expect_near(p.x(), 11.0, CONFUSION);
    expect_near(p.y(), -3.0, CONFUSION);
}

// ── 9. Transformed (non-mutating) ────────────────────────────────────────────

#[test]
fn transformed_is_non_mutating() {
    let t = Trsf2d::translation(Pnt2d::new(1.0, 1.0));
    let original = CartesianPoint2d::new(5.0, 5.0);
    let moved = original.transformed(&t);
    // original must be unchanged
    expect_near(original.x(), 5.0, CONFUSION);
    expect_near(original.y(), 5.0, CONFUSION);
    // transformed copy is shifted
    expect_near(moved.x(), 6.0, CONFUSION);
    expect_near(moved.y(), 6.0, CONFUSION);
}

// ── 10. Transform -- 90° rotation about the origin ──────────────────────────

#[test]
fn transform_rotation_90_degrees() {
    let t = Trsf2d::rotation(Pnt2d::origin(), FRAC_PI_2);
    let mut p = CartesianPoint2d::new(1.0, 0.0);
    p.transform(&t);
    // (1,0) rotated 90° CCW -> (0,1)
    expect_near(p.x(), 0.0, CONFUSION);
    expect_near(p.y(), 1.0, CONFUSION);
}

// ── 11. Transform -- 180° rotation ──────────────────────────────────────────

#[test]
fn transform_rotation_180_degrees() {
    let t = Trsf2d::rotation(Pnt2d::origin(), PI);
    let mut p = CartesianPoint2d::new(3.0, 4.0);
    p.transform(&t);
    // (3,4) rotated 180° -> (-3,-4)
    expect_near(p.x(), -3.0, CONFUSION);
    expect_near(p.y(), -4.0, CONFUSION);
}

// ── 12. Copy returns an independent value ────────────────────────────────────

#[test]
fn copy_is_independent() {
    let original = CartesianPoint2d::new(1.0, 2.0);
    let mut cloned = original.copy();
    cloned.set_coord(99.0, 99.0);
    // original must be unchanged
    expect_near(original.x(), 1.0, CONFUSION);
    expect_near(original.y(), 2.0, CONFUSION);
}

// ── 13. coord() tuple matches x() and y() ────────────────────────────────────

#[test]
fn coord_tuple_matches_accessors() {
    let p = CartesianPoint2d::new(1.23, -4.56);
    let (x, y) = p.coord();
    // Use ANGULAR as a very tight check — these are exact field reads.
    expect_near(x, p.x(), ANGULAR);
    expect_near(y, p.y(), ANGULAR);
}

// ── 14. pnt2d() reflects mutations made via SetCoord ─────────────────────────

#[test]
fn pnt2d_reflects_set_coord() {
    let mut p = CartesianPoint2d::new(0.0, 0.0);
    p.set_coord(2.0, 3.0);
    let gp = p.pnt2d();
    expect_near(gp.x, 2.0, CONFUSION);
    expect_near(gp.y, 3.0, CONFUSION);
}

// ── 15. Chained translation brings point to known location ──────────────────

#[test]
fn chained_translations() {
    let t1 = Trsf2d::translation(Pnt2d::new(1.0, 0.0));
    let t2 = Trsf2d::translation(Pnt2d::new(0.0, 1.0));
    let mut p = CartesianPoint2d::new(0.0, 0.0);
    p.transform(&t1);
    p.transform(&t2);
    expect_near(p.x(), 1.0, CONFUSION);
    expect_near(p.y(), 1.0, CONFUSION);
}
