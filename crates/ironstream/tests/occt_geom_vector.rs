//! Ported OpenCascade unit tests -- `Geom_VectorWithMagnitude`.
//!
//! These tests mirror the behaviour documented in the OCCT source and test
//! suites for `Geom_VectorWithMagnitude` (TKG3d package).  All imports are
//! from `ironstream::` only; tolerances follow `precision::CONFUSION` (1e-7)
//! and `precision::ANGULAR` (1e-12).

use ironstream::geom_vector::GeomVectorWithMagnitude;
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
// 1. Construction from XYZ coordinates
// ---------------------------------------------------------------------------

#[test]
fn construction_from_xyz() {
    let v = GeomVectorWithMagnitude::new(1.0, 2.0, 3.0);
    assert_near(v.x(), 1.0, CONFUSION);
    assert_near(v.y(), 2.0, CONFUSION);
    assert_near(v.z(), 3.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 2. Construction from gp_Vec
// ---------------------------------------------------------------------------

#[test]
fn construction_from_gp_vec() {
    let gp = Pnt::new(4.0, 5.0, 6.0);
    let v = GeomVectorWithMagnitude::from_vec(gp);
    assert_near(v.x(), 4.0, CONFUSION);
    assert_near(v.y(), 5.0, CONFUSION);
    assert_near(v.z(), 6.0, CONFUSION);
    assert_eq!(v.vec(), gp);
}

// ---------------------------------------------------------------------------
// 3. Magnitude and SquareMagnitude
// ---------------------------------------------------------------------------

#[test]
fn magnitude() {
    // (3, 4, 0) → magnitude = 5
    let v = GeomVectorWithMagnitude::new(3.0, 4.0, 0.0);
    assert_near(v.magnitude(), 5.0, CONFUSION);
    assert_near(v.square_magnitude(), 25.0, CONFUSION);
}

#[test]
fn magnitude_unit_axes() {
    // Each unit-axis vector should have magnitude 1.
    let vx = GeomVectorWithMagnitude::new(1.0, 0.0, 0.0);
    let vy = GeomVectorWithMagnitude::new(0.0, 1.0, 0.0);
    let vz = GeomVectorWithMagnitude::new(0.0, 0.0, 1.0);
    assert_near(vx.magnitude(), 1.0, CONFUSION);
    assert_near(vy.magnitude(), 1.0, CONFUSION);
    assert_near(vz.magnitude(), 1.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 4. Add / Subtract (returning new vector)
// ---------------------------------------------------------------------------

#[test]
fn add_and_subtract() {
    let a = GeomVectorWithMagnitude::new(1.0, 2.0, 3.0);
    let b = GeomVectorWithMagnitude::new(4.0, 5.0, 6.0);

    let sum = a.added(&b);
    assert_near(sum.x(), 5.0, CONFUSION);
    assert_near(sum.y(), 7.0, CONFUSION);
    assert_near(sum.z(), 9.0, CONFUSION);

    let diff = b.subtracted(&a);
    assert_near(diff.x(), 3.0, CONFUSION);
    assert_near(diff.y(), 3.0, CONFUSION);
    assert_near(diff.z(), 3.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 5. In-place Add / Subtract
// ---------------------------------------------------------------------------

#[test]
fn add_subtract_in_place() {
    let mut v = GeomVectorWithMagnitude::new(10.0, 10.0, 10.0);
    let delta = GeomVectorWithMagnitude::new(1.0, 2.0, 3.0);

    v.add(&delta);
    assert_near(v.x(), 11.0, CONFUSION);
    assert_near(v.y(), 12.0, CONFUSION);
    assert_near(v.z(), 13.0, CONFUSION);

    v.subtract(&delta);
    assert_near(v.x(), 10.0, CONFUSION);
    assert_near(v.y(), 10.0, CONFUSION);
    assert_near(v.z(), 10.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 6. Multiplied (scalar)
// ---------------------------------------------------------------------------

#[test]
fn multiplied_by_scalar() {
    let v = GeomVectorWithMagnitude::new(1.0, 2.0, 3.0);
    let w = v.multiplied(3.0);
    assert_near(w.x(), 3.0, CONFUSION);
    assert_near(w.y(), 6.0, CONFUSION);
    assert_near(w.z(), 9.0, CONFUSION);
    // Original unchanged
    assert_near(v.x(), 1.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 7. Dot product
// ---------------------------------------------------------------------------

#[test]
fn dot_product() {
    let a = GeomVectorWithMagnitude::new(1.0, 0.0, 0.0);
    let b = GeomVectorWithMagnitude::new(0.0, 1.0, 0.0);
    // Perpendicular vectors → dot = 0
    assert_near(a.dot(&b), 0.0, CONFUSION);

    let c = GeomVectorWithMagnitude::new(2.0, 3.0, 4.0);
    let d = GeomVectorWithMagnitude::new(1.0, 1.0, 1.0);
    // dot = 2 + 3 + 4 = 9
    assert_near(c.dot(&d), 9.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 8. Cross product (Crossed)
// ---------------------------------------------------------------------------

#[test]
fn crossed() {
    let x = GeomVectorWithMagnitude::new(1.0, 0.0, 0.0);
    let y = GeomVectorWithMagnitude::new(0.0, 1.0, 0.0);
    // X × Y = Z
    let z = x.crossed(&y);
    assert_near(z.x(), 0.0, CONFUSION);
    assert_near(z.y(), 0.0, CONFUSION);
    assert_near(z.z(), 1.0, CONFUSION);

    // Y × X = -Z
    let neg_z = y.crossed(&x);
    assert_near(neg_z.x(), 0.0, CONFUSION);
    assert_near(neg_z.y(), 0.0, CONFUSION);
    assert_near(neg_z.z(), -1.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 9. CrossedMagnitude = |a||b|sin(θ)
// ---------------------------------------------------------------------------

#[test]
fn crossed_magnitude() {
    let a = GeomVectorWithMagnitude::new(3.0, 0.0, 0.0);
    let b = GeomVectorWithMagnitude::new(0.0, 4.0, 0.0);
    // Perpendicular → |a × b| = 3 * 4 = 12
    assert_near(a.crossed_magnitude(&b), 12.0, CONFUSION);
    assert_near(a.crossed_square_magnitude(&b), 144.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 10. Normalized returns unit vector
// ---------------------------------------------------------------------------

#[test]
fn normalized() {
    let v = GeomVectorWithMagnitude::new(3.0, 4.0, 0.0);
    let u = v.normalized();
    assert_near(u.magnitude(), 1.0, CONFUSION);
    assert_near(u.x(), 3.0 / 5.0, CONFUSION);
    assert_near(u.y(), 4.0 / 5.0, CONFUSION);
    assert_near(u.z(), 0.0, CONFUSION);
    // Original vector unchanged
    assert_near(v.magnitude(), 5.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 11. Reversed and Reverse (in-place)
// ---------------------------------------------------------------------------

#[test]
fn reversed_and_reverse() {
    let v = GeomVectorWithMagnitude::new(1.0, -2.0, 3.0);

    let r = v.reversed();
    assert_near(r.x(), -1.0, CONFUSION);
    assert_near(r.y(), 2.0, CONFUSION);
    assert_near(r.z(), -3.0, CONFUSION);
    // Same magnitude
    assert_near(r.magnitude(), v.magnitude(), CONFUSION);

    let mut w = v;
    w.reverse();
    assert_near(w.x(), -1.0, CONFUSION);
    assert_near(w.y(), 2.0, CONFUSION);
    assert_near(w.z(), -3.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 12. Transform -- pure translation has no effect on a vector
// ---------------------------------------------------------------------------

#[test]
fn transform_translation_no_effect() {
    let v = GeomVectorWithMagnitude::new(1.0, 2.0, 3.0);
    // A pure translation should NOT change a free vector.
    let t = Trsf::translation(Pnt::new(100.0, 200.0, 300.0));
    let w = v.transformed(&t);
    assert_near(w.x(), 1.0, CONFUSION);
    assert_near(w.y(), 2.0, CONFUSION);
    assert_near(w.z(), 3.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 13. Transform -- 90-degree rotation about Z
// ---------------------------------------------------------------------------

#[test]
fn transform_rotation_90z() {
    let v = GeomVectorWithMagnitude::new(1.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let t = Trsf::rotation(axis, std::f64::consts::FRAC_PI_2);
    let w = v.transformed(&t);
    // (1,0,0) rotated 90° about Z → (0,1,0)
    assert_near(w.x(), 0.0, CONFUSION);
    assert_near(w.y(), 1.0, CONFUSION);
    assert_near(w.z(), 0.0, CONFUSION);
    // Magnitude is preserved under rotation
    assert_near(w.magnitude(), 1.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 14. Transform -- uniform scale changes magnitude
// ---------------------------------------------------------------------------

#[test]
fn transform_scale() {
    let v = GeomVectorWithMagnitude::new(1.0, 0.0, 0.0);
    let mut t = Trsf::identity();
    t.set_scale(Pnt::origin(), 5.0);
    let w = v.transformed(&t);
    assert_near(w.magnitude(), 5.0, CONFUSION);
    assert_near(w.x(), 5.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 15. In-place transform modifies the vector
// ---------------------------------------------------------------------------

#[test]
fn transform_in_place() {
    let mut v = GeomVectorWithMagnitude::new(2.0, 0.0, 0.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let t = Trsf::rotation(axis, std::f64::consts::PI);
    v.transform(&t);
    // (2,0,0) rotated 180° about Z → (-2, 0, 0)
    assert_near(v.x(), -2.0, CONFUSION);
    assert_near(v.y(), 0.0, CONFUSION);
    assert_near(v.z(), 0.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 16. Dot with self = square_magnitude (identity check)
// ---------------------------------------------------------------------------

#[test]
fn dot_self_equals_square_magnitude() {
    let v = GeomVectorWithMagnitude::new(2.0, 3.0, 6.0);
    let dot_self = v.dot(&v);
    assert_near(dot_self, v.square_magnitude(), ANGULAR.max(CONFUSION));
}

// ---------------------------------------------------------------------------
// 17. Copy produces independent clone
// ---------------------------------------------------------------------------

#[test]
fn copy_is_independent() {
    let v = GeomVectorWithMagnitude::new(1.0, 2.0, 3.0);
    let mut w = v.copy();
    w.multiply(10.0);
    // Original untouched
    assert_near(v.x(), 1.0, CONFUSION);
    assert_near(w.x(), 10.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 18. Normalize in-place produces unit vector
// ---------------------------------------------------------------------------

#[test]
fn normalize_in_place() {
    let mut v = GeomVectorWithMagnitude::new(0.0, 0.0, 7.0);
    v.normalize();
    assert_near(v.magnitude(), 1.0, CONFUSION);
    assert_near(v.x(), 0.0, CONFUSION);
    assert_near(v.y(), 0.0, CONFUSION);
    assert_near(v.z(), 1.0, CONFUSION);
}
