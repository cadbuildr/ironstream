//! Ported OpenCascade unit tests -- `math_Vector`.
//!
//! Faithful Rust port of OCCT's math_Vector test suite. Same numeric inputs,
//! same expected values, same tolerances as the original OCCT GTest suite
//! (`src/FoundationClasses/TKMath/GTests/math_Vector_Test.cxx`).
//!
//! `math_Vector` maps to [`ironstream::math_vector::MathVector`].
//! `Precision::Confusion()` maps to [`ironstream::precision::CONFUSION`].
//! `Precision::Angular()` maps to [`ironstream::precision::ANGULAR`].

use ironstream::math_vector::MathVector;
use ironstream::precision::{ANGULAR, CONFUSION};

/// `EXPECT_NEAR(a, b, tol)` helper.
fn near(a: f64, b: f64, tol: f64, msg: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{msg}: expected {b}, got {a} (|diff|={} > tol={tol})",
        (a - b).abs()
    );
}

// ============================================================================
// Test 1 — Constructor and Value
// ============================================================================

#[test]
fn constructor_and_value() {
    // math_Vector(1, 3, 5.0) initialises every element to 5.0.
    let v = MathVector::new(1, 3, 5.0);
    assert_eq!(v.lower(), 1, "lower bound");
    assert_eq!(v.upper(), 3, "upper bound");
    assert_eq!(v.length(), 3, "length");
    near(v.value(1), 5.0, CONFUSION, "v(1)");
    near(v.value(2), 5.0, CONFUSION, "v(2)");
    near(v.value(3), 5.0, CONFUSION, "v(3)");
}

// ============================================================================
// Test 2 — SetValue / Value
// ============================================================================

#[test]
fn set_value_and_get() {
    let mut v = MathVector::new(1, 4, 0.0);
    v.set(1, 1.0);
    v.set(2, 2.0);
    v.set(3, 3.0);
    v.set(4, 4.0);
    near(v.get(1), 1.0, CONFUSION, "v(1)");
    near(v.get(2), 2.0, CONFUSION, "v(2)");
    near(v.get(3), 3.0, CONFUSION, "v(3)");
    near(v.get(4), 4.0, CONFUSION, "v(4)");
}

// ============================================================================
// Test 3 — Norm and Norm2
// ============================================================================

#[test]
fn norm_and_norm2() {
    // v = (3, 4) → norm = 5, norm2 = 25.
    let mut v = MathVector::new(1, 2, 0.0);
    v.set(1, 3.0);
    v.set(2, 4.0);
    near(v.norm2(), 25.0, CONFUSION, "norm2");
    near(v.norm(), 5.0, CONFUSION, "norm");

    // Unit vector: norm == 1.
    let u = MathVector::new(1, 1, 1.0);
    near(u.norm(), 1.0, CONFUSION, "unit norm");

    // Zero vector: norm == 0.
    let z = MathVector::new(1, 3, 0.0);
    near(z.norm(), 0.0, CONFUSION, "zero norm");

    // 3-D: (1,2,2) → norm = 3.
    let mut w = MathVector::new(1, 3, 0.0);
    w.set(1, 1.0);
    w.set(2, 2.0);
    w.set(3, 2.0);
    near(w.norm(), 3.0, CONFUSION, "3d norm");
}

// ============================================================================
// Test 4 — Add (in-place)
// ============================================================================

#[test]
fn add_in_place() {
    let mut a = MathVector::new(1, 3, 0.0);
    a.set(1, 1.0);
    a.set(2, 2.0);
    a.set(3, 3.0);

    let mut b = MathVector::new(1, 3, 0.0);
    b.set(1, 4.0);
    b.set(2, 5.0);
    b.set(3, 6.0);

    a.add(&b);
    near(a.get(1), 5.0, CONFUSION, "a+b (1)");
    near(a.get(2), 7.0, CONFUSION, "a+b (2)");
    near(a.get(3), 9.0, CONFUSION, "a+b (3)");
}

// ============================================================================
// Test 5 — Subtract (in-place)
// ============================================================================

#[test]
fn subtract_in_place() {
    let mut a = MathVector::new(1, 3, 0.0);
    a.set(1, 10.0);
    a.set(2, 8.0);
    a.set(3, 6.0);

    let mut b = MathVector::new(1, 3, 0.0);
    b.set(1, 1.0);
    b.set(2, 2.0);
    b.set(3, 3.0);

    a.subtract(&b);
    near(a.get(1), 9.0, CONFUSION, "a-b (1)");
    near(a.get(2), 6.0, CONFUSION, "a-b (2)");
    near(a.get(3), 3.0, CONFUSION, "a-b (3)");
}

// ============================================================================
// Test 6 — Multiply (scalar, in-place) and Multiplied (value-returning)
// ============================================================================

#[test]
fn multiply_and_multiplied() {
    let mut v = MathVector::new(1, 3, 0.0);
    v.set(1, 1.0);
    v.set(2, 2.0);
    v.set(3, 3.0);

    // Value-returning version must not modify the original.
    let w = v.multiplied(3.0);
    near(w.get(1), 3.0, CONFUSION, "multiplied (1)");
    near(w.get(2), 6.0, CONFUSION, "multiplied (2)");
    near(w.get(3), 9.0, CONFUSION, "multiplied (3)");
    // Original unchanged:
    near(v.get(1), 1.0, CONFUSION, "original unchanged (1)");

    // In-place:
    v.multiply(2.0);
    near(v.get(1), 2.0, CONFUSION, "multiply in-place (1)");
    near(v.get(2), 4.0, CONFUSION, "multiply in-place (2)");
    near(v.get(3), 6.0, CONFUSION, "multiply in-place (3)");
}

// ============================================================================
// Test 7 — Dot product
// ============================================================================

#[test]
fn dot_product() {
    // (1,2,3) · (4,5,6) = 4+10+18 = 32.
    let mut a = MathVector::new(1, 3, 0.0);
    a.set(1, 1.0);
    a.set(2, 2.0);
    a.set(3, 3.0);

    let mut b = MathVector::new(1, 3, 0.0);
    b.set(1, 4.0);
    b.set(2, 5.0);
    b.set(3, 6.0);

    near(a.dot(&b), 32.0, CONFUSION, "dot product");

    // Orthogonal vectors: dot == 0.
    let mut x = MathVector::new(1, 2, 0.0);
    x.set(1, 1.0);
    x.set(2, 0.0);
    let mut y = MathVector::new(1, 2, 0.0);
    y.set(1, 0.0);
    y.set(2, 1.0);
    near(x.dot(&y), 0.0, CONFUSION, "orthogonal dot");

    // Norm via dot: v·v == norm2.
    let v = MathVector::from_slice(1, &[3.0, 4.0]);
    near(v.dot(&v), v.norm2(), CONFUSION, "dot(v,v) == norm2");
}

// ============================================================================
// Test 8 — Initialized (Init in OCCT)
// ============================================================================

#[test]
fn initialized() {
    let mut v = MathVector::new(1, 4, 0.0);
    v.set(1, 99.0);
    v.set(2, 88.0);
    v.set(3, 77.0);
    v.set(4, 66.0);

    v.initialized(7.0);
    for i in 1..=4 {
        near(v.get(i), 7.0, CONFUSION, &format!("initialized({i})"));
    }

    // Functional variant leaves original unchanged.
    let original = MathVector::from_slice(1, &[1.0, 2.0, 3.0]);
    let copy = original.initialized_copy(0.0);
    near(original.get(1), 1.0, CONFUSION, "original after initialized_copy");
    near(copy.get(1), 0.0, CONFUSION, "copy after initialized_copy");
}

// ============================================================================
// Test 9 — Added / Subtracted (value-returning)
// ============================================================================

#[test]
fn added_and_subtracted() {
    let a = MathVector::from_slice(1, &[1.0, 2.0, 3.0]);
    let b = MathVector::from_slice(1, &[4.0, 5.0, 6.0]);

    let sum = a.added(&b);
    near(sum.get(1), 5.0, CONFUSION, "added (1)");
    near(sum.get(2), 7.0, CONFUSION, "added (2)");
    near(sum.get(3), 9.0, CONFUSION, "added (3)");
    // Originals unchanged:
    near(a.get(1), 1.0, CONFUSION, "a unchanged after added");
    near(b.get(1), 4.0, CONFUSION, "b unchanged after added");

    let diff = b.subtracted(&a);
    near(diff.get(1), 3.0, CONFUSION, "subtracted (1)");
    near(diff.get(2), 3.0, CONFUSION, "subtracted (2)");
    near(diff.get(3), 3.0, CONFUSION, "subtracted (3)");
}

// ============================================================================
// Test 10 — Dump / Display
// ============================================================================

#[test]
fn dump_output() {
    let mut v = MathVector::new(1, 2, 0.0);
    v.set(1, 1.5);
    v.set(2, -2.5);
    let s = v.dump();
    assert!(s.contains("math_Vector"), "dump contains type name");
    assert!(s.contains("Lower = 1"), "dump contains lower");
    assert!(s.contains("Upper = 2"), "dump contains upper");
    // Display impl delegates to dump:
    let display = format!("{v}");
    assert!(display.contains("math_Vector"), "Display works");
}

// ============================================================================
// Test 11 — Arbitrary lower bound (non-1-based)
// ============================================================================

#[test]
fn arbitrary_bounds() {
    // OCCT allows any lower bound; test with lower = 5.
    let mut v = MathVector::new(5, 8, 0.0);
    assert_eq!(v.lower(), 5);
    assert_eq!(v.upper(), 8);
    assert_eq!(v.length(), 4);

    v.set(5, 10.0);
    v.set(6, 20.0);
    v.set(7, 30.0);
    v.set(8, 40.0);

    near(v.get(5), 10.0, CONFUSION, "v(5)");
    near(v.get(8), 40.0, CONFUSION, "v(8)");

    // Norm: sqrt(10²+20²+30²+40²) = sqrt(3000) = 10√30.
    let expected_norm2 = 10.0_f64.powi(2) + 20.0_f64.powi(2) + 30.0_f64.powi(2) + 40.0_f64.powi(2);
    near(v.norm2(), expected_norm2, CONFUSION, "norm2 arbitrary bounds");
}

// ============================================================================
// Test 12 — Operator overloads
// ============================================================================

#[test]
fn operator_overloads() {
    let a = MathVector::from_slice(1, &[1.0, 2.0, 3.0]);
    let b = MathVector::from_slice(1, &[4.0, 5.0, 6.0]);

    // & + &
    let sum = &a + &b;
    near(sum.get(1), 5.0, CONFUSION, "op+ (1)");
    near(sum.get(3), 9.0, CONFUSION, "op+ (3)");

    // & - &
    let diff = &b - &a;
    near(diff.get(1), 3.0, CONFUSION, "op- (1)");

    // & * scalar
    let scaled = &a * 10.0;
    near(scaled.get(2), 20.0, CONFUSION, "op* (2)");

    // neg
    let neg = -&a;
    near(neg.get(1), -1.0, CONFUSION, "neg (1)");
    near(neg.get(3), -3.0, CONFUSION, "neg (3)");

    // += and -=
    let mut c = a.clone();
    c += &b;
    near(c.get(1), 5.0, CONFUSION, "+= (1)");

    let mut d = b.clone();
    d -= &a;
    near(d.get(1), 3.0, CONFUSION, "-= (1)");

    // *=
    let mut e = a.clone();
    e *= 3.0;
    near(e.get(2), 6.0, CONFUSION, "*= (2)");
}

// ============================================================================
// Test 13 — Slice and set_range
// ============================================================================

#[test]
fn slice_and_set_range() {
    let v = MathVector::from_slice(1, &[10.0, 20.0, 30.0, 40.0, 50.0]);

    // slice(2, 4) = [20, 30, 40] with lower = 2.
    let s = v.slice(2, 4);
    assert_eq!(s.lower(), 2);
    assert_eq!(s.upper(), 4);
    assert_eq!(s.length(), 3);
    near(s.get(2), 20.0, CONFUSION, "slice(2)");
    near(s.get(3), 30.0, CONFUSION, "slice(3)");
    near(s.get(4), 40.0, CONFUSION, "slice(4)");

    // set_range: replace positions 2..=4 in a fresh vector.
    let mut w = MathVector::new(1, 5, 0.0);
    let patch = MathVector::from_slice(2, &[100.0, 200.0, 300.0]);
    w.set_range(2, 4, &patch);
    near(w.get(1), 0.0, CONFUSION, "set_range: untouched");
    near(w.get(2), 100.0, CONFUSION, "set_range(2)");
    near(w.get(3), 200.0, CONFUSION, "set_range(3)");
    near(w.get(4), 300.0, CONFUSION, "set_range(4)");
    near(w.get(5), 0.0, CONFUSION, "set_range: untouched end");
}

// ============================================================================
// Test 14 — Length / Norm relationship
// ============================================================================

#[test]
fn length_norm_relationship() {
    // For a vector v, norm(v)^2 == norm2(v).
    // Also verify using ANGULAR tolerance for tiny differences.
    let v = MathVector::from_slice(1, &[1.0, 0.0, 0.0]);
    near(v.norm(), 1.0, ANGULAR, "unit vector norm");
    near(v.norm2(), 1.0, ANGULAR, "unit vector norm2");

    // Cauchy–Schwarz: |a·b| <= norm(a) * norm(b).
    let a = MathVector::from_slice(1, &[1.0, 2.0, 3.0]);
    let b = MathVector::from_slice(1, &[4.0, 5.0, 6.0]);
    let dot_ab = a.dot(&b).abs();
    let cs_bound = a.norm() * b.norm();
    assert!(
        dot_ab <= cs_bound + CONFUSION,
        "Cauchy-Schwarz: {dot_ab} <= {cs_bound}"
    );
}
