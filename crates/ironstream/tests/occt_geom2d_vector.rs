//! Ported OpenCascade unit tests -- `Geom2d_VectorWithMagnitude`.
//!
//! Source reference: OpenCascade Technology `src/ModelingData/TKG2d/GTests/`
//! (Geom2d_Vector tests, adapted).
//!
//! Each `#[test]` below corresponds to a named OCCT GTest. Inputs, expected
//! values and tolerances are kept identical to the originals:
//!   - linear tolerance  → `ironstream::precision::CONFUSION` (1e-7)
//!   - angular tolerance → `ironstream::precision::ANGULAR`   (1e-12)

use ironstream::geom2d_vector::VectorWithMagnitude;
use ironstream::gp2d::{Pnt2d, Trsf2d};
use ironstream::precision::{ANGULAR, CONFUSION};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// `EXPECT_NEAR(a, b, tol)` analogue.
fn expect_near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ~= {b} within {tol} (diff = {})",
        (a - b).abs()
    );
}

// ---------------------------------------------------------------------------
// Test: construction from coordinates
// ---------------------------------------------------------------------------

#[test]
fn construct_from_xy() {
    let v = VectorWithMagnitude::new(3.0, 4.0);
    expect_near(v.x(), 3.0, CONFUSION);
    expect_near(v.y(), 4.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// Test: magnitude (3-4-5 Pythagorean triple)
// ---------------------------------------------------------------------------

#[test]
fn magnitude_345() {
    let v = VectorWithMagnitude::new(3.0, 4.0);
    expect_near(v.magnitude(), 5.0, CONFUSION);
    expect_near(v.square_magnitude(), 25.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// Test: construct from two points (p2 - p1)
// ---------------------------------------------------------------------------

#[test]
fn construct_from_points() {
    let p1 = Pnt2d::new(1.0, 2.0);
    let p2 = Pnt2d::new(4.0, 6.0);
    let v = VectorWithMagnitude::from_points(p1, p2);
    expect_near(v.x(), 3.0, CONFUSION);
    expect_near(v.y(), 4.0, CONFUSION);
    expect_near(v.magnitude(), 5.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// Test: addition
// ---------------------------------------------------------------------------

#[test]
fn add_two_vectors() {
    let a = VectorWithMagnitude::new(1.0, 2.0);
    let b = VectorWithMagnitude::new(3.0, 5.0);
    let s = a.added(&b);
    expect_near(s.x(), 4.0, CONFUSION);
    expect_near(s.y(), 7.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// Test: subtraction
// ---------------------------------------------------------------------------

#[test]
fn subtract_two_vectors() {
    let a = VectorWithMagnitude::new(5.0, 7.0);
    let b = VectorWithMagnitude::new(3.0, 4.0);
    let d = a.subtracted(&b);
    expect_near(d.x(), 2.0, CONFUSION);
    expect_near(d.y(), 3.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// Test: 2D cross product (Crossed)
// ---------------------------------------------------------------------------

#[test]
fn crossed_orthogonal_unit_vectors() {
    // X cross Y = +1, Y cross X = -1
    let x = VectorWithMagnitude::new(1.0, 0.0);
    let y = VectorWithMagnitude::new(0.0, 1.0);
    expect_near(x.crossed(&y), 1.0, CONFUSION);
    expect_near(y.crossed(&x), -1.0, CONFUSION);
}

#[test]
fn crossed_parallel_vectors_is_zero() {
    let a = VectorWithMagnitude::new(2.0, 0.0);
    let b = VectorWithMagnitude::new(5.0, 0.0);
    expect_near(a.crossed(&b), 0.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// Test: dot product
// ---------------------------------------------------------------------------

#[test]
fn dot_orthogonal_is_zero() {
    let a = VectorWithMagnitude::new(1.0, 0.0);
    let b = VectorWithMagnitude::new(0.0, 1.0);
    expect_near(a.dot(&b), 0.0, CONFUSION);
}

#[test]
fn dot_general() {
    // (1,2)·(3,4) = 3 + 8 = 11
    let a = VectorWithMagnitude::new(1.0, 2.0);
    let b = VectorWithMagnitude::new(3.0, 4.0);
    expect_near(a.dot(&b), 11.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// Test: scalar multiplication
// ---------------------------------------------------------------------------

#[test]
fn multiplied_by_scalar() {
    let v = VectorWithMagnitude::new(2.0, 3.0);
    let w = v.multiplied(4.0);
    expect_near(w.x(), 8.0, CONFUSION);
    expect_near(w.y(), 12.0, CONFUSION);
    expect_near(w.magnitude(), v.magnitude() * 4.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// Test: normalisation
// ---------------------------------------------------------------------------

#[test]
fn normalized_has_unit_magnitude() {
    let v = VectorWithMagnitude::new(3.0, 4.0);
    let n = v.normalized();
    expect_near(n.magnitude(), 1.0, ANGULAR.max(CONFUSION));
    // Direction must be preserved
    expect_near(n.x(), 3.0 / 5.0, CONFUSION);
    expect_near(n.y(), 4.0 / 5.0, CONFUSION);
}

#[test]
fn normalize_in_place() {
    let mut v = VectorWithMagnitude::new(0.0, 7.0);
    v.normalize();
    expect_near(v.magnitude(), 1.0, CONFUSION);
    expect_near(v.x(), 0.0, CONFUSION);
    expect_near(v.y(), 1.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// Test: reversal
// ---------------------------------------------------------------------------

#[test]
fn reversed_negates_components() {
    let v = VectorWithMagnitude::new(3.0, -5.0);
    let r = v.reversed();
    expect_near(r.x(), -3.0, CONFUSION);
    expect_near(r.y(), 5.0, CONFUSION);
    // Magnitude is unchanged
    expect_near(r.magnitude(), v.magnitude(), CONFUSION);
}

#[test]
fn reverse_in_place() {
    let mut v = VectorWithMagnitude::new(1.0, 2.0);
    let mag_before = v.magnitude();
    v.reverse();
    expect_near(v.x(), -1.0, CONFUSION);
    expect_near(v.y(), -2.0, CONFUSION);
    expect_near(v.magnitude(), mag_before, CONFUSION);
}

// ---------------------------------------------------------------------------
// Test: transform — pure rotation by 90°
// ---------------------------------------------------------------------------

#[test]
fn transform_rotation_90() {
    // Rotate (1, 0) by 90° CCW → expect (0, 1)
    let t = Trsf2d::rotation(Pnt2d::origin(), std::f64::consts::FRAC_PI_2);
    let v = VectorWithMagnitude::new(1.0, 0.0);
    let w = v.transformed(&t);
    expect_near(w.x(), 0.0, CONFUSION);
    expect_near(w.y(), 1.0, CONFUSION);
    // Magnitude is preserved under rotation
    expect_near(w.magnitude(), 1.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// Test: transform — translation must NOT move a free vector
// ---------------------------------------------------------------------------

#[test]
fn transform_translation_does_not_affect_vector() {
    // A pure translation should leave the vector components unchanged.
    let t = Trsf2d::translation(Pnt2d::new(100.0, 200.0));
    let v = VectorWithMagnitude::new(3.0, 4.0);
    let w = v.transformed(&t);
    expect_near(w.x(), 3.0, CONFUSION);
    expect_near(w.y(), 4.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// Test: in-place add and subtract
// ---------------------------------------------------------------------------

#[test]
fn in_place_add_subtract() {
    let mut v = VectorWithMagnitude::new(5.0, 3.0);
    let delta = VectorWithMagnitude::new(2.0, 1.0);
    v.add(&delta);
    expect_near(v.x(), 7.0, CONFUSION);
    expect_near(v.y(), 4.0, CONFUSION);
    v.subtract(&delta);
    expect_near(v.x(), 5.0, CONFUSION);
    expect_near(v.y(), 3.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// Test: construct from gp_Vec2d
// ---------------------------------------------------------------------------

#[test]
fn construct_from_vec2d() {
    use ironstream::gp2d::Vec2d;
    let raw = Vec2d::new(-1.0, -2.0);
    let v = VectorWithMagnitude::from_vec2d(raw);
    expect_near(v.x(), -1.0, CONFUSION);
    expect_near(v.y(), -2.0, CONFUSION);
    // vec2d accessor round-trip
    let back = v.vec2d();
    expect_near(back.x, -1.0, CONFUSION);
    expect_near(back.y, -2.0, CONFUSION);
}
