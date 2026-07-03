//! Ported OpenCascade unit tests — `gp_XYZ`.
//!
//! Source: OpenCascade Technology, `TKMath` package, `gp_XYZ` class tests
//! (ported by hand). Each `#[test]` corresponds to one logical scenario from
//! the OCCT test suite for `gp_XYZ`.
//!
//! Tolerances used:
//! * linear:  `precision::CONFUSION` = 1e-7
//! * angular: `precision::ANGULAR`   = 1e-12

use ironstream::gp_xyz::XYZ;
use ironstream::precision::{ANGULAR, CONFUSION};

/// `EXPECT_NEAR` helper — panics if `|a - b| > tol`.
fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ~= {b} within {tol} (diff {})",
        (a - b).abs()
    );
}

// ── 1. Constructor: stores coordinates without modification ───────────────────

#[test]
fn construct_and_accessors() {
    let v = XYZ::new(1.0, 2.0, 3.0);
    near(v.x(), 1.0, ANGULAR);
    near(v.y(), 2.0, ANGULAR);
    near(v.z(), 3.0, ANGULAR);
}

// ── 2. coord() tuple matches individual accessors ─────────────────────────────

#[test]
fn coord_tuple_matches_accessors() {
    let v = XYZ::new(4.0, 5.0, 6.0);
    let (x, y, z) = v.coord();
    near(x, v.x(), ANGULAR);
    near(y, v.y(), ANGULAR);
    near(z, v.z(), ANGULAR);
}

// ── 3. SetCoord replaces all three components ─────────────────────────────────

#[test]
fn set_coord_replaces_all() {
    let mut v = XYZ::new(1.0, 2.0, 3.0);
    v.set_coord(7.0, 8.0, 9.0);
    near(v.x(), 7.0, ANGULAR);
    near(v.y(), 8.0, ANGULAR);
    near(v.z(), 9.0, ANGULAR);
}

// ── 4. SetX / SetY / SetZ replace individual components ──────────────────────

#[test]
fn set_xyz_individual() {
    let mut v = XYZ::new(0.0, 0.0, 0.0);
    v.set_x(10.0);
    v.set_y(20.0);
    v.set_z(30.0);
    near(v.x(), 10.0, ANGULAR);
    near(v.y(), 20.0, ANGULAR);
    near(v.z(), 30.0, ANGULAR);
}

// ── 5. Modulus: (3, 4, 0) → 5 ────────────────────────────────────────────────

#[test]
fn modulus_3_4_0() {
    let v = XYZ::new(3.0, 4.0, 0.0);
    near(v.modulus(), 5.0, CONFUSION);
}

// ── 6. SquareModulus: (3, 4, 0) → 25 ─────────────────────────────────────────

#[test]
fn square_modulus_3_4_0() {
    let v = XYZ::new(3.0, 4.0, 0.0);
    near(v.square_modulus(), 25.0, CONFUSION);
}

// ── 7. Modulus: (1, 1, 1) → sqrt(3) ─────────────────────────────────────────

#[test]
fn modulus_unit_diagonal() {
    let v = XYZ::new(1.0, 1.0, 1.0);
    near(v.modulus(), 3.0_f64.sqrt(), CONFUSION);
}

// ── 8. Add (in-place) ────────────────────────────────────────────────────────

#[test]
fn add_in_place() {
    let mut a = XYZ::new(1.0, 2.0, 3.0);
    let b = XYZ::new(10.0, 20.0, 30.0);
    a.add(&b);
    near(a.x(), 11.0, ANGULAR);
    near(a.y(), 22.0, ANGULAR);
    near(a.z(), 33.0, ANGULAR);
}

// ── 9. Added (non-mutating) ───────────────────────────────────────────────────

#[test]
fn added_non_mutating() {
    let a = XYZ::new(1.0, 2.0, 3.0);
    let b = XYZ::new(4.0, 5.0, 6.0);
    let c = a.added(&b);
    near(c.x(), 5.0, ANGULAR);
    near(c.y(), 7.0, ANGULAR);
    near(c.z(), 9.0, ANGULAR);
    // originals unchanged
    near(a.x(), 1.0, ANGULAR);
    near(b.x(), 4.0, ANGULAR);
}

// ── 10. Subtract (in-place) ───────────────────────────────────────────────────

#[test]
fn subtract_in_place() {
    let mut a = XYZ::new(10.0, 20.0, 30.0);
    let b = XYZ::new(1.0, 2.0, 3.0);
    a.subtract(&b);
    near(a.x(), 9.0, ANGULAR);
    near(a.y(), 18.0, ANGULAR);
    near(a.z(), 27.0, ANGULAR);
}

// ── 11. Subtracted (non-mutating) ────────────────────────────────────────────

#[test]
fn subtracted_non_mutating() {
    let a = XYZ::new(5.0, 7.0, 9.0);
    let b = XYZ::new(1.0, 2.0, 3.0);
    let c = a.subtracted(&b);
    near(c.x(), 4.0, ANGULAR);
    near(c.y(), 5.0, ANGULAR);
    near(c.z(), 6.0, ANGULAR);
    near(a.x(), 5.0, ANGULAR); // original unchanged
}

// ── 12. Multiplied by scalar ──────────────────────────────────────────────────

#[test]
fn multiplied_by_scalar() {
    let v = XYZ::new(1.0, 2.0, 3.0);
    let m = v.multiplied(3.0);
    near(m.x(), 3.0, ANGULAR);
    near(m.y(), 6.0, ANGULAR);
    near(m.z(), 9.0, ANGULAR);
    near(v.x(), 1.0, ANGULAR); // original unchanged
}

// ── 13. Divided by scalar ─────────────────────────────────────────────────────

#[test]
fn divided_by_scalar() {
    let v = XYZ::new(2.0, 4.0, 6.0);
    let d = v.divided(2.0);
    near(d.x(), 1.0, ANGULAR);
    near(d.y(), 2.0, ANGULAR);
    near(d.z(), 3.0, ANGULAR);
    near(v.x(), 2.0, ANGULAR); // original unchanged
}

// ── 14. Dot product: perpendicular axes → 0 ──────────────────────────────────

#[test]
fn dot_perpendicular_is_zero() {
    let x = XYZ::new(1.0, 0.0, 0.0);
    let y = XYZ::new(0.0, 1.0, 0.0);
    near(x.dot(&y), 0.0, ANGULAR);
}

// ── 15. Dot product: (1,2,3)·(4,5,6) = 32 ────────────────────────────────────

#[test]
fn dot_product_value() {
    let a = XYZ::new(1.0, 2.0, 3.0);
    let b = XYZ::new(4.0, 5.0, 6.0);
    near(a.dot(&b), 32.0, ANGULAR);
}

// ── 16. Dot: self dot self = square modulus ───────────────────────────────────

#[test]
fn dot_self_is_square_modulus() {
    let v = XYZ::new(3.0, 4.0, 0.0);
    near(v.dot(&v), v.square_modulus(), ANGULAR);
}

// ── 17. Cross product: X × Y = Z ─────────────────────────────────────────────

#[test]
fn crossed_x_y_gives_z() {
    let x = XYZ::new(1.0, 0.0, 0.0);
    let y = XYZ::new(0.0, 1.0, 0.0);
    let z = x.crossed(&y);
    near(z.x(), 0.0, ANGULAR);
    near(z.y(), 0.0, ANGULAR);
    near(z.z(), 1.0, ANGULAR);
}

// ── 18. Cross: Y × X = -Z ────────────────────────────────────────────────────

#[test]
fn crossed_y_x_gives_neg_z() {
    let x = XYZ::new(1.0, 0.0, 0.0);
    let y = XYZ::new(0.0, 1.0, 0.0);
    let neg_z = y.crossed(&x);
    near(neg_z.x(), 0.0, ANGULAR);
    near(neg_z.y(), 0.0, ANGULAR);
    near(neg_z.z(), -1.0, ANGULAR);
}

// ── 19. Cross (in-place) ──────────────────────────────────────────────────────

#[test]
fn cross_in_place() {
    let mut x = XYZ::new(1.0, 0.0, 0.0);
    let y = XYZ::new(0.0, 1.0, 0.0);
    x.cross(&y);
    near(x.x(), 0.0, ANGULAR);
    near(x.y(), 0.0, ANGULAR);
    near(x.z(), 1.0, ANGULAR);
}

// ── 20. CrossMagnitude: perpendicular unit vectors → 1 ───────────────────────

#[test]
fn cross_magnitude_perpendicular() {
    let x = XYZ::new(1.0, 0.0, 0.0);
    let y = XYZ::new(0.0, 1.0, 0.0);
    near(x.cross_magnitude(&y), 1.0, CONFUSION);
    near(x.cross_square_magnitude(&y), 1.0, CONFUSION);
}

// ── 21. CrossMagnitude: parallel vectors → 0 ─────────────────────────────────

#[test]
fn cross_magnitude_parallel_is_zero() {
    let v = XYZ::new(1.0, 0.0, 0.0);
    let same = XYZ::new(5.0, 0.0, 0.0);
    near(v.cross_magnitude(&same), 0.0, CONFUSION);
    near(v.cross_square_magnitude(&same), 0.0, CONFUSION);
}

// ── 22. Normalized: (0, 3, 4) → (0, 0.6, 0.8) ───────────────────────────────

#[test]
fn normalized_gives_unit_vector() {
    let v = XYZ::new(0.0, 3.0, 4.0);
    let n = v.normalized();
    near(n.x(), 0.0, CONFUSION);
    near(n.y(), 0.6, CONFUSION);
    near(n.z(), 0.8, CONFUSION);
    near(n.modulus(), 1.0, CONFUSION);
}

// ── 23. Normalize (in-place) ──────────────────────────────────────────────────

#[test]
fn normalize_in_place() {
    let mut v = XYZ::new(2.0, 0.0, 0.0);
    v.normalize();
    near(v.x(), 1.0, CONFUSION);
    near(v.y(), 0.0, CONFUSION);
    near(v.z(), 0.0, CONFUSION);
    near(v.modulus(), 1.0, CONFUSION);
}

// ── 24. Normalized: already-unit vector is unchanged ─────────────────────────

#[test]
fn normalized_unit_vector_unchanged() {
    let v = XYZ::new(1.0, 0.0, 0.0);
    let n = v.normalized();
    near(n.x(), 1.0, ANGULAR);
    near(n.y(), 0.0, ANGULAR);
    near(n.z(), 0.0, ANGULAR);
}

// ── 25. Reversed (non-mutating) ──────────────────────────────────────────────

#[test]
fn reversed_non_mutating() {
    let v = XYZ::new(1.0, -2.0, 3.0);
    let r = v.reversed();
    near(r.x(), -1.0, ANGULAR);
    near(r.y(), 2.0, ANGULAR);
    near(r.z(), -3.0, ANGULAR);
    // original unchanged
    near(v.x(), 1.0, ANGULAR);
}

// ── 26. Reverse (in-place) ────────────────────────────────────────────────────

#[test]
fn reverse_in_place() {
    let mut v = XYZ::new(1.0, 2.0, 3.0);
    v.reverse();
    near(v.x(), -1.0, ANGULAR);
    near(v.y(), -2.0, ANGULAR);
    near(v.z(), -3.0, ANGULAR);
}

// ── 27. Double reverse is identity ───────────────────────────────────────────

#[test]
fn double_reverse_is_identity() {
    let original = XYZ::new(1.0, 2.0, 3.0);
    let mut v = original;
    v.reverse();
    v.reverse();
    near(v.x(), original.x(), ANGULAR);
    near(v.y(), original.y(), ANGULAR);
    near(v.z(), original.z(), ANGULAR);
}

// ── 28. IsEqual: within tolerance ────────────────────────────────────────────

#[test]
fn is_equal_within_tolerance() {
    let a = XYZ::new(1.0, 2.0, 3.0);
    let b = XYZ::new(1.0 + 1e-8, 2.0, 3.0);
    assert!(a.is_equal(&b, 1e-7));
    assert!(!a.is_equal(&b, 1e-9));
}

// ── 29. IsEqual: identical vectors ────────────────────────────────────────────

#[test]
fn is_equal_identical() {
    let v = XYZ::new(5.0, -3.0, 1.5);
    assert!(v.is_equal(&v, 0.0));
}

// ── 30. IsEqual: far-apart vectors fail ──────────────────────────────────────

#[test]
fn is_equal_far_apart_fails() {
    let a = XYZ::new(0.0, 0.0, 0.0);
    let b = XYZ::new(1.0, 0.0, 0.0);
    assert!(!a.is_equal(&b, CONFUSION));
}

// ── 31. Operator+ and operator- ──────────────────────────────────────────────

#[test]
fn ops_add_sub() {
    let a = XYZ::new(1.0, 2.0, 3.0);
    let b = XYZ::new(4.0, 5.0, 6.0);
    let s = a + b;
    near(s.x(), 5.0, ANGULAR);
    near(s.y(), 7.0, ANGULAR);
    near(s.z(), 9.0, ANGULAR);
    let d = a - b;
    near(d.x(), -3.0, ANGULAR);
    near(d.y(), -3.0, ANGULAR);
    near(d.z(), -3.0, ANGULAR);
}

// ── 32. Operator* and operator/ ──────────────────────────────────────────────

#[test]
fn ops_mul_div() {
    let v = XYZ::new(2.0, 4.0, 6.0);
    let m = v * 2.0;
    near(m.x(), 4.0, ANGULAR);
    near(m.y(), 8.0, ANGULAR);
    near(m.z(), 12.0, ANGULAR);
    let d = v / 2.0;
    near(d.x(), 1.0, ANGULAR);
    near(d.y(), 2.0, ANGULAR);
    near(d.z(), 3.0, ANGULAR);
}

// ── 33. Operator- (negation) ─────────────────────────────────────────────────

#[test]
fn ops_neg() {
    let v = XYZ::new(1.0, -2.0, 3.0);
    let r = -v;
    near(r.x(), -1.0, ANGULAR);
    near(r.y(), 2.0, ANGULAR);
    near(r.z(), -3.0, ANGULAR);
}

// ── 34. Origin helper ────────────────────────────────────────────────────────

#[test]
fn origin_is_zero() {
    let o = XYZ::origin();
    near(o.x(), 0.0, ANGULAR);
    near(o.y(), 0.0, ANGULAR);
    near(o.z(), 0.0, ANGULAR);
    near(o.modulus(), 0.0, ANGULAR);
}

// ── 35. Multiply (in-place) and Divide (in-place) ────────────────────────────

#[test]
fn multiply_divide_in_place() {
    let mut v = XYZ::new(1.0, 2.0, 3.0);
    v.multiply(4.0);
    near(v.x(), 4.0, ANGULAR);
    near(v.y(), 8.0, ANGULAR);
    near(v.z(), 12.0, ANGULAR);
    v.divide(4.0);
    near(v.x(), 1.0, ANGULAR);
    near(v.y(), 2.0, ANGULAR);
    near(v.z(), 3.0, ANGULAR);
}

// ── 36. Cross antisymmetry ────────────────────────────────────────────────────

#[test]
fn cross_antisymmetric() {
    let a = XYZ::new(1.0, 2.0, 3.0);
    let b = XYZ::new(4.0, 5.0, 6.0);
    let ab = a.crossed(&b);
    let ba = b.crossed(&a);
    near(ab.x(), -ba.x(), ANGULAR);
    near(ab.y(), -ba.y(), ANGULAR);
    near(ab.z(), -ba.z(), ANGULAR);
}

// ── 37. CrossMagnitude 2D vectors at 45° ─────────────────────────────────────

#[test]
fn cross_magnitude_45_degrees() {
    // (1,0,0) and (1,1,0)/√2: sin(45°) = 1/√2
    let x = XYZ::new(1.0, 0.0, 0.0);
    let d45 = XYZ::new(1.0, 1.0, 0.0).normalized();
    let expected_sin = std::f64::consts::FRAC_PI_4.sin();
    near(x.cross_magnitude(&d45), expected_sin, CONFUSION);
}

// ── 38. Modulus of zero vector ────────────────────────────────────────────────

#[test]
fn modulus_zero_vector() {
    let v = XYZ::origin();
    near(v.modulus(), 0.0, ANGULAR);
    near(v.square_modulus(), 0.0, ANGULAR);
}
