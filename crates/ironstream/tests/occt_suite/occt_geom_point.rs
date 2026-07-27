//! Ported OpenCascade unit tests -- `Geom_CartesianPoint`.
//!
//! These tests mirror the behaviour documented in the OCCT source and test
//! suites for `Geom_CartesianPoint` (TKG3d package).  All imports are from
//! `ironstream::` only; tolerances follow `precision::CONFUSION` (1e-7) and
//! `precision::ANGULAR` (1e-12).

use ironstream::geom_point::GeomCartesianPoint;
use ironstream::gp::{Ax1, Pnt, Trsf};
use ironstream::precision::{ANGULAR, CONFUSION};

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// Assert two f64 values are within `tol` of each other.
fn assert_near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected |{a} - {b}| <= {tol}, got {}",
        (a - b).abs()
    );
}

// ---------------------------------------------------------------------------
// 1. Construction from coordinates
// ---------------------------------------------------------------------------

#[test]
fn construction_from_xyz() {
    let p = GeomCartesianPoint::new(1.0, 2.0, 3.0);
    assert_near(p.x(), 1.0, CONFUSION);
    assert_near(p.y(), 2.0, CONFUSION);
    assert_near(p.z(), 3.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 2. Construction from gp_Pnt
// ---------------------------------------------------------------------------

#[test]
fn construction_from_pnt() {
    let gp = Pnt::new(4.0, 5.0, 6.0);
    let p = GeomCartesianPoint::from_pnt(gp);
    assert_near(p.x(), 4.0, CONFUSION);
    assert_near(p.y(), 5.0, CONFUSION);
    assert_near(p.z(), 6.0, CONFUSION);
    // pnt() round-trips back to the same gp_Pnt
    assert_eq!(p.pnt(), gp);
}

// ---------------------------------------------------------------------------
// 3. SetCoord changes all three coordinates
// ---------------------------------------------------------------------------

#[test]
fn set_coord() {
    let mut p = GeomCartesianPoint::new(0.0, 0.0, 0.0);
    p.set_coord(7.0, 8.0, 9.0);
    assert_near(p.x(), 7.0, CONFUSION);
    assert_near(p.y(), 8.0, CONFUSION);
    assert_near(p.z(), 9.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 4. SetX / SetY / SetZ change individual coordinates
// ---------------------------------------------------------------------------

#[test]
fn set_individual_coords() {
    let mut p = GeomCartesianPoint::new(1.0, 2.0, 3.0);
    p.set_x(10.0);
    p.set_y(20.0);
    p.set_z(30.0);
    assert_near(p.x(), 10.0, CONFUSION);
    assert_near(p.y(), 20.0, CONFUSION);
    assert_near(p.z(), 30.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 5. Coord() tuple round-trip
// ---------------------------------------------------------------------------

#[test]
fn coord_tuple() {
    let p = GeomCartesianPoint::new(-1.5, 2.5, -3.5);
    let (x, y, z) = p.coord();
    assert_near(x, -1.5, CONFUSION);
    assert_near(y, 2.5, CONFUSION);
    assert_near(z, -3.5, CONFUSION);
}

// ---------------------------------------------------------------------------
// 6. Distance and SquareDistance
// ---------------------------------------------------------------------------

#[test]
fn distance_and_square_distance() {
    let a = GeomCartesianPoint::new(0.0, 0.0, 0.0);
    let b = GeomCartesianPoint::new(3.0, 4.0, 0.0);
    // Pythagoras: distance should be 5.0
    assert_near(a.distance(&b), 5.0, CONFUSION);
    assert_near(a.square_distance(&b), 25.0, CONFUSION);
    // Symmetry
    assert_near(b.distance(&a), 5.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 7. Distance from point to itself is zero
// ---------------------------------------------------------------------------

#[test]
fn distance_self_is_zero() {
    let p = GeomCartesianPoint::new(1.0, 2.0, 3.0);
    assert_near(p.distance(&p), 0.0, CONFUSION);
    assert_near(p.square_distance(&p), 0.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 8. Transform -- pure translation
// ---------------------------------------------------------------------------

#[test]
fn transform_translation() {
    let mut p = GeomCartesianPoint::new(1.0, 2.0, 3.0);
    let t = Trsf::translation(Pnt::new(10.0, 20.0, 30.0));
    p.transform(&t);
    assert_near(p.x(), 11.0, CONFUSION);
    assert_near(p.y(), 22.0, CONFUSION);
    assert_near(p.z(), 33.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 9. Transformed() returns a new point, original is unchanged
// ---------------------------------------------------------------------------

#[test]
fn transformed_does_not_mutate_original() {
    let p = GeomCartesianPoint::new(0.0, 0.0, 0.0);
    let t = Trsf::translation(Pnt::new(5.0, 5.0, 5.0));
    let q = p.transformed(&t);
    // original untouched
    assert_near(p.x(), 0.0, CONFUSION);
    assert_near(p.y(), 0.0, CONFUSION);
    assert_near(p.z(), 0.0, CONFUSION);
    // new point carries the translation
    assert_near(q.x(), 5.0, CONFUSION);
    assert_near(q.y(), 5.0, CONFUSION);
    assert_near(q.z(), 5.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 10. Transform -- 90-degree rotation about Z axis
// ---------------------------------------------------------------------------

#[test]
fn transform_rotation_90z() {
    let mut p = GeomCartesianPoint::new(1.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let t = Trsf::rotation(axis, std::f64::consts::FRAC_PI_2);
    p.transform(&t);
    // (1,0,0) rotated 90° about Z → (0,1,0)
    assert_near(p.x(), 0.0, CONFUSION);
    assert_near(p.y(), 1.0, CONFUSION);
    assert_near(p.z(), 0.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 11. SetPnt replaces underlying gp_Pnt
// ---------------------------------------------------------------------------

#[test]
fn set_pnt() {
    let mut p = GeomCartesianPoint::new(1.0, 1.0, 1.0);
    let q = Pnt::new(99.0, 88.0, 77.0);
    p.set_pnt(q);
    assert_eq!(p.pnt(), q);
    assert_near(p.x(), 99.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 12. Copy produces an independent clone
// ---------------------------------------------------------------------------

#[test]
fn copy_is_independent() {
    let p = GeomCartesianPoint::new(1.0, 2.0, 3.0);
    let mut q = p.copy();
    q.set_x(999.0);
    // original unaffected
    assert_near(p.x(), 1.0, CONFUSION);
    assert_near(q.x(), 999.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 13. Unit-distance point along each axis (ANGULAR used for zero-diff check)
// ---------------------------------------------------------------------------

#[test]
fn unit_distances_along_axes() {
    let origin = GeomCartesianPoint::new(0.0, 0.0, 0.0);
    let px = GeomCartesianPoint::new(1.0, 0.0, 0.0);
    let py = GeomCartesianPoint::new(0.0, 1.0, 0.0);
    let pz = GeomCartesianPoint::new(0.0, 0.0, 1.0);
    assert_near(origin.distance(&px), 1.0, ANGULAR.max(CONFUSION));
    assert_near(origin.distance(&py), 1.0, ANGULAR.max(CONFUSION));
    assert_near(origin.distance(&pz), 1.0, ANGULAR.max(CONFUSION));
}

// ---------------------------------------------------------------------------
// 14. Uniform scale transform changes distance proportionally
// ---------------------------------------------------------------------------

#[test]
fn transform_scale() {
    let p = GeomCartesianPoint::new(2.0, 0.0, 0.0);
    let origin_pnt = GeomCartesianPoint::new(0.0, 0.0, 0.0);
    // scale by 3 about origin
    let mut t = Trsf::identity();
    t.set_scale(Pnt::origin(), 3.0);
    let q = p.transformed(&t);
    assert_near(origin_pnt.distance(&q), 6.0, CONFUSION);
}
