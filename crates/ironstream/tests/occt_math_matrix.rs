//! Ported OpenCascade unit tests -- `math_Matrix`.
//!
//! Faithful Rust port of OCCT's `math_Matrix` tests
//! (`src/FoundationClasses/TKMath/GTests/`). Same numeric inputs, same
//! expected values.
//!
//! `math_Matrix` → [`Matrix`], `math_Vector` → [`Vector`].
//! OCCT's 1-indexed accessors `M(i,j)` / `V(i)` map to `M.get(i,j)` /
//! `V.get(i)`.
//!
//! Tolerances: `Precision::Confusion()` = 1e-7, `Precision::Angular()` = 1e-12.

use ironstream::math_matrix::{Matrix, Vector};
use ironstream::precision::{ANGULAR, CONFUSION};

/// `EXPECT_NEAR(a, b, tol)`.
fn near(a: f64, b: f64, tol: f64, msg: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{msg}: expected {b}, got {a} (|diff|={} > tol={tol})",
        (a - b).abs()
    );
}

// ---------------------------------------------------------------------------
// 1. Constructor and basic accessors
// ---------------------------------------------------------------------------

#[test]
fn constructor_and_accessors() {
    let m = Matrix::new(1, 3, 1, 4);
    assert_eq!(m.lower_row(), 1);
    assert_eq!(m.upper_row(), 3);
    assert_eq!(m.lower_col(), 1);
    assert_eq!(m.upper_col(), 4);
    assert_eq!(m.nb_rows(), 3);
    assert_eq!(m.nb_cols(), 4);
    // All zero after construction.
    for i in 1..=3 {
        for j in 1..=4 {
            near(m.get(i, j), 0.0, CONFUSION, "element should be zero");
        }
    }
}

// ---------------------------------------------------------------------------
// 2. Custom (non-1) bounds
// ---------------------------------------------------------------------------

#[test]
fn custom_bounds() {
    let mut m = Matrix::new(2, 4, 3, 5);
    assert_eq!(m.lower_row(), 2);
    assert_eq!(m.upper_row(), 4);
    assert_eq!(m.lower_col(), 3);
    assert_eq!(m.upper_col(), 5);
    assert_eq!(m.nb_rows(), 3);
    assert_eq!(m.nb_cols(), 3);

    m.set(2, 3, 1.0);
    m.set(3, 4, 2.0);
    m.set(4, 5, 3.0);

    near(m.get(2, 3), 1.0, ANGULAR, "custom bounds get (2,3)");
    near(m.get(3, 4), 2.0, ANGULAR, "custom bounds get (3,4)");
    near(m.get(4, 5), 3.0, ANGULAR, "custom bounds get (4,5)");
    near(m.get(2, 4), 0.0, ANGULAR, "unset element should be zero");
}

// ---------------------------------------------------------------------------
// 3. with_value constructor
// ---------------------------------------------------------------------------

#[test]
fn with_value_constructor() {
    let m = Matrix::with_value(1, 2, 1, 2, 5.0);
    near(m.get(1, 1), 5.0, ANGULAR, "with_value (1,1)");
    near(m.get(1, 2), 5.0, ANGULAR, "with_value (1,2)");
    near(m.get(2, 1), 5.0, ANGULAR, "with_value (2,1)");
    near(m.get(2, 2), 5.0, ANGULAR, "with_value (2,2)");
}

// ---------------------------------------------------------------------------
// 4. Added — element-wise addition
// ---------------------------------------------------------------------------

#[test]
fn added() {
    // A + B for 2x2 matrices.
    let mut a = Matrix::new(1, 2, 1, 2);
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(2, 1, 3.0);
    a.set(2, 2, 4.0);

    let mut b = Matrix::new(1, 2, 1, 2);
    b.set(1, 1, 10.0);
    b.set(1, 2, 20.0);
    b.set(2, 1, 30.0);
    b.set(2, 2, 40.0);

    let c = a.added(&b);
    near(c.get(1, 1), 11.0, ANGULAR, "Added (1,1)");
    near(c.get(1, 2), 22.0, ANGULAR, "Added (1,2)");
    near(c.get(2, 1), 33.0, ANGULAR, "Added (2,1)");
    near(c.get(2, 2), 44.0, ANGULAR, "Added (2,2)");

    // Result preserves original index range.
    assert_eq!(c.lower_row(), 1);
    assert_eq!(c.upper_row(), 2);
}

// ---------------------------------------------------------------------------
// 5. Subtracted — element-wise subtraction
// ---------------------------------------------------------------------------

#[test]
fn subtracted() {
    let mut a = Matrix::new(1, 2, 1, 2);
    a.set(1, 1, 10.0);
    a.set(1, 2, 20.0);
    a.set(2, 1, 30.0);
    a.set(2, 2, 40.0);

    let mut b = Matrix::new(1, 2, 1, 2);
    b.set(1, 1, 1.0);
    b.set(1, 2, 2.0);
    b.set(2, 1, 3.0);
    b.set(2, 2, 4.0);

    let c = a.subtracted(&b);
    near(c.get(1, 1), 9.0, ANGULAR, "Subtracted (1,1)");
    near(c.get(1, 2), 18.0, ANGULAR, "Subtracted (1,2)");
    near(c.get(2, 1), 27.0, ANGULAR, "Subtracted (2,1)");
    near(c.get(2, 2), 36.0, ANGULAR, "Subtracted (2,2)");
}

// ---------------------------------------------------------------------------
// 6. Transposed
// ---------------------------------------------------------------------------

#[test]
fn transposed() {
    // 2×3 matrix transposed to 3×2.
    let mut a = Matrix::new(1, 2, 1, 3);
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(1, 3, 3.0);
    a.set(2, 1, 4.0);
    a.set(2, 2, 5.0);
    a.set(2, 3, 6.0);

    let t = a.transposed();
    assert_eq!(t.nb_rows(), 3);
    assert_eq!(t.nb_cols(), 2);
    near(t.get(1, 1), 1.0, ANGULAR, "Transposed (1,1)");
    near(t.get(2, 1), 2.0, ANGULAR, "Transposed (2,1)");
    near(t.get(3, 1), 3.0, ANGULAR, "Transposed (3,1)");
    near(t.get(1, 2), 4.0, ANGULAR, "Transposed (1,2)");
    near(t.get(2, 2), 5.0, ANGULAR, "Transposed (2,2)");
    near(t.get(3, 2), 6.0, ANGULAR, "Transposed (3,2)");

    // Double-transpose yields original.
    let tt = t.transposed();
    for i in 1..=2 {
        for j in 1..=3 {
            near(
                tt.get(i, j),
                a.get(i, j),
                ANGULAR,
                "double-transpose element",
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 7. Multiplied (matrix × matrix)
// ---------------------------------------------------------------------------

#[test]
fn multiplied_matrix_matrix() {
    // 2×3 × 3×2 = 2×2
    let mut a = Matrix::new(1, 2, 1, 3);
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(1, 3, 3.0);
    a.set(2, 1, 4.0);
    a.set(2, 2, 5.0);
    a.set(2, 3, 6.0);

    let mut b = Matrix::new(1, 3, 1, 2);
    b.set(1, 1, 7.0);
    b.set(1, 2, 8.0);
    b.set(2, 1, 9.0);
    b.set(2, 2, 10.0);
    b.set(3, 1, 11.0);
    b.set(3, 2, 12.0);

    let c = a.multiplied(&b);
    assert_eq!(c.nb_rows(), 2);
    assert_eq!(c.nb_cols(), 2);
    // c(1,1) = 1*7 + 2*9 + 3*11 = 7+18+33 = 58
    near(c.get(1, 1), 58.0, CONFUSION, "Multiplied (1,1)");
    // c(1,2) = 1*8 + 2*10 + 3*12 = 8+20+36 = 64
    near(c.get(1, 2), 64.0, CONFUSION, "Multiplied (1,2)");
    // c(2,1) = 4*7 + 5*9 + 6*11 = 28+45+66 = 139
    near(c.get(2, 1), 139.0, CONFUSION, "Multiplied (2,1)");
    // c(2,2) = 4*8 + 5*10 + 6*12 = 32+50+72 = 154
    near(c.get(2, 2), 154.0, CONFUSION, "Multiplied (2,2)");
}

// ---------------------------------------------------------------------------
// 8. Multiplied (matrix × vector)
// ---------------------------------------------------------------------------

#[test]
fn multiplied_matrix_vector() {
    // 3×3 matrix × 3-vector.
    let mut a = Matrix::new(1, 3, 1, 3);
    a.set(1, 1, 1.0);
    a.set(1, 2, 0.0);
    a.set(1, 3, 0.0);
    a.set(2, 1, 0.0);
    a.set(2, 2, 2.0);
    a.set(2, 3, 0.0);
    a.set(3, 1, 0.0);
    a.set(3, 2, 0.0);
    a.set(3, 3, 3.0);

    let mut v = Vector::new(1, 3);
    v.set(1, 4.0);
    v.set(2, 5.0);
    v.set(3, 6.0);

    let r = a.multiply_vector(&v);
    assert_eq!(r.length(), 3);
    near(r.get(1), 4.0, CONFUSION, "MV result (1)");
    near(r.get(2), 10.0, CONFUSION, "MV result (2)");
    near(r.get(3), 18.0, CONFUSION, "MV result (3)");
}

// ---------------------------------------------------------------------------
// 9. Determinant — 1×1, 2×2, 3×3
// ---------------------------------------------------------------------------

#[test]
fn determinant() {
    // 1×1
    let mut m1 = Matrix::new(1, 1, 1, 1);
    m1.set(1, 1, 7.0);
    near(m1.determinant(), 7.0, ANGULAR, "det 1×1");

    // 2×2: [[4,7],[2,6]]  det = 4*6 - 7*2 = 24 - 14 = 10
    let mut m2 = Matrix::new(1, 2, 1, 2);
    m2.set(1, 1, 4.0);
    m2.set(1, 2, 7.0);
    m2.set(2, 1, 2.0);
    m2.set(2, 2, 6.0);
    near(m2.determinant(), 10.0, CONFUSION, "det 2×2");

    // 3×3: known determinant
    // [[1,2,3],[0,1,4],[5,6,0]]
    // det = 1*(1*0 - 4*6) - 2*(0*0 - 4*5) + 3*(0*6 - 1*5)
    //     = 1*(-24) - 2*(-20) + 3*(-5)
    //     = -24 + 40 - 15 = 1
    let mut m3 = Matrix::new(1, 3, 1, 3);
    m3.set(1, 1, 1.0);
    m3.set(1, 2, 2.0);
    m3.set(1, 3, 3.0);
    m3.set(2, 1, 0.0);
    m3.set(2, 2, 1.0);
    m3.set(2, 3, 4.0);
    m3.set(3, 1, 5.0);
    m3.set(3, 2, 6.0);
    m3.set(3, 3, 0.0);
    near(m3.determinant(), 1.0, CONFUSION, "det 3×3");
}

// ---------------------------------------------------------------------------
// 10. Determinant — singular matrix returns 0
// ---------------------------------------------------------------------------

#[test]
fn determinant_singular() {
    // Row 2 = 2 × Row 1 ⇒ rank-deficient.
    let mut m = Matrix::new(1, 3, 1, 3);
    m.set(1, 1, 1.0);
    m.set(1, 2, 2.0);
    m.set(1, 3, 3.0);
    m.set(2, 1, 2.0);
    m.set(2, 2, 4.0);
    m.set(2, 3, 6.0);
    m.set(3, 1, 1.0);
    m.set(3, 2, 1.0);
    m.set(3, 3, 1.0);
    near(m.determinant(), 0.0, CONFUSION, "det of singular matrix");
}

// ---------------------------------------------------------------------------
// 11. Init — fill all elements
// ---------------------------------------------------------------------------

#[test]
fn init() {
    let mut m = Matrix::new(1, 3, 1, 3);
    m.init(7.0);
    for i in 1..=3 {
        for j in 1..=3 {
            near(m.get(i, j), 7.0, ANGULAR, "init element");
        }
    }
    m.init(0.0);
    for i in 1..=3 {
        for j in 1..=3 {
            near(m.get(i, j), 0.0, ANGULAR, "init-to-zero element");
        }
    }
}

// ---------------------------------------------------------------------------
// 12. A × Aᵀ is symmetric positive semidefinite
// ---------------------------------------------------------------------------

#[test]
fn multiply_and_transpose_symmetric() {
    // Build a random-looking 3×2 matrix and verify (A·Aᵀ) is symmetric.
    let mut a = Matrix::new(1, 3, 1, 2);
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(2, 1, 3.0);
    a.set(2, 2, 4.0);
    a.set(3, 1, 5.0);
    a.set(3, 2, 6.0);

    let at = a.transposed();
    let aat = a.multiplied(&at); // 3×3

    // Check symmetry: aat(i,j) == aat(j,i).
    for i in 1..=3 {
        for j in 1..=3 {
            near(
                aat.get(i, j),
                aat.get(j, i),
                ANGULAR,
                "A·Aᵀ should be symmetric",
            );
        }
    }

    // Diagonal elements should be non-negative (sum of squares of rows of A).
    // aat(1,1) = 1²+2² = 5, aat(2,2) = 9+16 = 25, aat(3,3) = 25+36 = 61.
    near(aat.get(1, 1), 5.0, CONFUSION, "A·Aᵀ (1,1)");
    near(aat.get(2, 2), 25.0, CONFUSION, "A·Aᵀ (2,2)");
    near(aat.get(3, 3), 61.0, CONFUSION, "A·Aᵀ (3,3)");
}

// ---------------------------------------------------------------------------
// 13. multiplied_scalar
// ---------------------------------------------------------------------------

#[test]
fn multiplied_scalar() {
    let mut m = Matrix::new(1, 2, 1, 2);
    m.set(1, 1, 1.0);
    m.set(1, 2, 2.0);
    m.set(2, 1, 3.0);
    m.set(2, 2, 4.0);

    let s = m.multiplied_scalar(3.0);
    near(s.get(1, 1), 3.0, ANGULAR, "scalar mult (1,1)");
    near(s.get(1, 2), 6.0, ANGULAR, "scalar mult (1,2)");
    near(s.get(2, 1), 9.0, ANGULAR, "scalar mult (2,1)");
    near(s.get(2, 2), 12.0, ANGULAR, "scalar mult (2,2)");
}

// ---------------------------------------------------------------------------
// 14. Vector accessors (sanity)
// ---------------------------------------------------------------------------

#[test]
fn vector_accessors() {
    let mut v = Vector::new(2, 5);
    assert_eq!(v.lower(), 2);
    assert_eq!(v.upper(), 5);
    assert_eq!(v.length(), 4);

    v.set(2, 10.0);
    v.set(5, 20.0);
    near(v.get(2), 10.0, ANGULAR, "vector get (2)");
    near(v.get(5), 20.0, ANGULAR, "vector get (5)");
}

// ---------------------------------------------------------------------------
// 15. Identity matrix has determinant 1 and multiplied by itself is itself
// ---------------------------------------------------------------------------

#[test]
fn identity_matrix_properties() {
    let mut id = Matrix::new(1, 3, 1, 3);
    id.set(1, 1, 1.0);
    id.set(2, 2, 1.0);
    id.set(3, 3, 1.0);

    near(id.determinant(), 1.0, CONFUSION, "det(I) = 1");

    let id2 = id.multiplied(&id);
    for i in 1..=3 {
        for j in 1..=3 {
            let expected = if i == j { 1.0 } else { 0.0 };
            near(id2.get(i, j), expected, ANGULAR, "I*I = I");
        }
    }
}
