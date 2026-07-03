//! Ported OpenCascade unit tests — `math_Crout`.
//!
//! Faithful Rust port of OCCT's `math_Crout_Test.cxx`
//! (Open-Cascade-SAS/OCCT, `src/FoundationClasses/TKMath/GTests/`). Same
//! numeric inputs, same expected values, same tolerances.
//!
//! `math_Matrix` → [`MathMatrix`], `math_Vector` → [`MathVector`],
//! `math_Crout` → [`Crout`]. OCCT's 1-indexed accessors `M(i,j)` / `V(i)` map
//! to `M.set(i,j,..)` / `M.get(i,j)` and `V.set(i,..)` / `V.get(i)`.

use ironstream::math_crout::{Crout, MathMatrix, MathVector};

/// EXPECT_NEAR(a, b, tol).
fn near(a: f64, b: f64, tol: f64, msg: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{msg}: expected {a} ≈ {b} (tol {tol})"
    );
}

// ====================== MathCroutTest.SimpleSymmetricMatrix ======================

#[test]
fn simple_symmetric_matrix() {
    // Test with a simple 3x3 symmetric positive definite matrix
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 4.0);
    a.set(1, 2, 2.0);
    a.set(1, 3, 1.0);
    a.set(2, 1, 2.0);
    a.set(2, 2, 3.0);
    a.set(2, 3, 0.5);
    a.set(3, 1, 1.0);
    a.set(3, 2, 0.5);
    a.set(3, 3, 2.0);

    let crout = Crout::new(&a);

    assert!(crout.is_done(), "Crout decomposition should succeed");

    // Test solving a system
    let mut b = MathVector::new(1, 3);
    b.set(1, 7.0);
    b.set(2, 5.5);
    b.set(3, 3.5); // Expected solution: [1, 1, 1]

    let mut x = MathVector::new(1, 3);
    crout.solve(&b, &mut x);

    near(x.get(1), 1.0, 1.0e-10, "Solution X(1) should be 1");
    near(x.get(2), 1.0, 1.0e-10, "Solution X(2) should be 1");
    near(x.get(3), 1.0, 1.0e-10, "Solution X(3) should be 1");
}

// ====================== MathCroutTest.IdentityMatrix ======================

#[test]
fn identity_matrix() {
    // Test with identity matrix
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 1.0);
    a.set(1, 2, 0.0);
    a.set(1, 3, 0.0);
    a.set(2, 1, 0.0);
    a.set(2, 2, 1.0);
    a.set(2, 3, 0.0);
    a.set(3, 1, 0.0);
    a.set(3, 2, 0.0);
    a.set(3, 3, 1.0);

    let crout = Crout::new(&a);

    assert!(
        crout.is_done(),
        "Identity matrix decomposition should succeed"
    );

    // Test solving with identity matrix
    let mut b = MathVector::new(1, 3);
    b.set(1, 5.0);
    b.set(2, 7.0);
    b.set(3, 9.0);

    let mut x = MathVector::new(1, 3);
    crout.solve(&b, &mut x);

    near(x.get(1), 5.0, 1.0e-12, "Identity matrix solution X(1)");
    near(x.get(2), 7.0, 1.0e-12, "Identity matrix solution X(2)");
    near(x.get(3), 9.0, 1.0e-12, "Identity matrix solution X(3)");

    // Check inverse matrix
    let inverse = crout.inverse();
    near(
        inverse.get(1, 1),
        1.0,
        1.0e-12,
        "Inverse should be identity",
    );
    near(
        inverse.get(2, 2),
        1.0,
        1.0e-12,
        "Inverse should be identity",
    );
    near(
        inverse.get(3, 3),
        1.0,
        1.0e-12,
        "Inverse should be identity",
    );
}

// ====================== MathCroutTest.DiagonalMatrix ======================

#[test]
fn diagonal_matrix() {
    // Test with diagonal matrix
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 2.0);
    a.set(1, 2, 0.0);
    a.set(1, 3, 0.0);
    a.set(2, 1, 0.0);
    a.set(2, 2, 3.0);
    a.set(2, 3, 0.0);
    a.set(3, 1, 0.0);
    a.set(3, 2, 0.0);
    a.set(3, 3, 4.0);

    let crout = Crout::new(&a);

    assert!(
        crout.is_done(),
        "Diagonal matrix decomposition should succeed"
    );

    let mut b = MathVector::new(1, 3);
    b.set(1, 4.0);
    b.set(2, 9.0);
    b.set(3, 12.0); // Expected solution: [2, 3, 3]

    let mut x = MathVector::new(1, 3);
    crout.solve(&b, &mut x);

    near(x.get(1), 2.0, 1.0e-12, "Diagonal solution X(1)");
    near(x.get(2), 3.0, 1.0e-12, "Diagonal solution X(2)");
    near(x.get(3), 3.0, 1.0e-12, "Diagonal solution X(3)");
}

// ====================== MathCroutTest.LowerTriangularInput ======================

#[test]
fn lower_triangular_input() {
    // Test providing only the lower triangular part (as mentioned in documentation)
    let mut a = MathMatrix::new(1, 3, 1, 3);
    // Only fill lower triangular part
    a.set(1, 1, 4.0);
    a.set(1, 2, 0.0);
    a.set(1, 3, 0.0);
    a.set(2, 1, 2.0);
    a.set(2, 2, 3.0);
    a.set(2, 3, 0.0);
    a.set(3, 1, 1.0);
    a.set(3, 2, 0.5);
    a.set(3, 3, 2.0);

    let crout = Crout::new(&a);

    assert!(crout.is_done(), "Lower triangular input should work");
}

// ====================== MathCroutTest.CustomMinPivot ======================

#[test]
fn custom_min_pivot() {
    // Test with custom minimum pivot threshold
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 1.0e-15);
    a.set(1, 2, 1.0);
    a.set(2, 1, 1.0);
    a.set(2, 2, 1.0);

    // With large MinPivot, should fail
    let crout1 = Crout::with_min_pivot(&a, 1.0e-10);
    assert!(!crout1.is_done(), "Should fail with large MinPivot");

    // With small MinPivot, should succeed
    let crout2 = Crout::with_min_pivot(&a, 1.0e-20);
    assert!(crout2.is_done(), "Should succeed with small MinPivot");
}

// ====================== MathCroutTest.SingularMatrix ======================

#[test]
fn singular_matrix() {
    // Test with singular matrix (rank deficient)
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(1, 3, 3.0);
    a.set(2, 1, 2.0);
    a.set(2, 2, 4.0);
    a.set(2, 3, 6.0); // Row 2 = 2 * Row 1
    a.set(3, 1, 3.0);
    a.set(3, 2, 6.0);
    a.set(3, 3, 9.0); // Row 3 = 3 * Row 1

    let crout = Crout::new(&a);

    assert!(!crout.is_done(), "Should fail for singular matrix");
}

// ====================== MathCroutTest.NonSquareMatrixCheck ======================

#[test]
fn non_square_matrix_check() {
    // Test detection of non-square matrix
    let mut a = MathMatrix::new(1, 2, 1, 3); // 2x3 matrix
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(1, 3, 3.0);
    a.set(2, 1, 4.0);
    a.set(2, 2, 5.0);
    a.set(2, 3, 6.0);

    // Crout decomposition requires square matrices.
    assert_ne!(
        a.row_number(),
        a.col_number(),
        "Matrix should be non-square for this test"
    );
}

// ====================== MathCroutTest.DimensionCompatibilityInSolve ======================

#[test]
fn dimension_compatibility_in_solve() {
    // Test dimension compatibility in Solve method
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 1.0);
    a.set(1, 2, 0.0);
    a.set(1, 3, 0.0);
    a.set(2, 1, 0.0);
    a.set(2, 2, 1.0);
    a.set(2, 3, 0.0);
    a.set(3, 1, 0.0);
    a.set(3, 2, 0.0);
    a.set(3, 3, 1.0);

    let crout = Crout::new(&a);
    assert!(crout.is_done(), "Decomposition should succeed");

    // Test with correctly sized vectors
    let mut b_correct = MathVector::new(1, 3);
    b_correct.set(1, 1.0);
    b_correct.set(2, 2.0);
    b_correct.set(3, 3.0);

    let mut x = MathVector::new(1, 3);
    crout.solve(&b_correct, &mut x);

    // Verify the solution is reasonable
    assert_eq!(
        x.length(),
        3,
        "Solution vector should have correct dimension"
    );
}

// ====================== MathCroutTest.SingularMatrixState ======================

#[test]
fn singular_matrix_state() {
    // Test state handling for singular matrix decomposition
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(1, 3, 3.0);
    a.set(2, 1, 2.0);
    a.set(2, 2, 4.0);
    a.set(2, 3, 6.0); // Singular
    a.set(3, 1, 3.0);
    a.set(3, 2, 6.0);
    a.set(3, 3, 9.0);

    let crout = Crout::new(&a);
    assert!(!crout.is_done(), "Should fail for singular matrix");
}

// ====================== MathCroutTest.LargerMatrix ======================

#[test]
fn larger_matrix() {
    // Test with larger 4x4 symmetric matrix
    let mut a = MathMatrix::new(1, 4, 1, 4);
    a.set(1, 1, 10.0);
    a.set(1, 2, 1.0);
    a.set(1, 3, 2.0);
    a.set(1, 4, 3.0);
    a.set(2, 1, 1.0);
    a.set(2, 2, 10.0);
    a.set(2, 3, 1.0);
    a.set(2, 4, 4.0);
    a.set(3, 1, 2.0);
    a.set(3, 2, 1.0);
    a.set(3, 3, 10.0);
    a.set(3, 4, 1.0);
    a.set(4, 1, 3.0);
    a.set(4, 2, 4.0);
    a.set(4, 3, 1.0);
    a.set(4, 4, 10.0);

    let crout = Crout::new(&a);

    assert!(crout.is_done(), "4x4 matrix decomposition should succeed");

    // Test solving
    let mut b = MathVector::new(1, 4);
    b.set(1, 16.0);
    b.set(2, 16.0);
    b.set(3, 14.0);
    b.set(4, 18.0); // Should give solution approximately [1, 1, 1, 1]

    let mut x = MathVector::new(1, 4);
    crout.solve(&b, &mut x);

    // Verify solution by checking residual
    let mut residual = MathVector::new(1, 4);
    for i in 1..=4 {
        let mut r = 0.0;
        for j in 1..=4 {
            r += a.get(i, j) * x.get(j);
        }
        r -= b.get(i);
        residual.set(i, r);
    }

    let mut residual_norm = 0.0;
    for i in 1..=4 {
        residual_norm += residual.get(i) * residual.get(i);
    }

    assert!(
        residual_norm < 1.0e-20,
        "Residual should be very small (got {residual_norm})"
    );
}

// ====================== MathCroutTest.CustomBounds ======================

#[test]
fn custom_bounds() {
    // Test with custom matrix bounds
    let mut a = MathMatrix::new(2, 4, 3, 5);
    a.set(2, 3, 4.0);
    a.set(2, 4, 2.0);
    a.set(2, 5, 1.0);
    a.set(3, 3, 2.0);
    a.set(3, 4, 3.0);
    a.set(3, 5, 0.5);
    a.set(4, 3, 1.0);
    a.set(4, 4, 0.5);
    a.set(4, 5, 2.0);

    let crout = Crout::new(&a);

    assert!(crout.is_done(), "Custom bounds matrix should work");

    let mut b = MathVector::new(2, 4);
    b.set(2, 7.0);
    b.set(3, 5.5);
    b.set(4, 3.5);

    let mut x = MathVector::new(3, 5);
    crout.solve(&b, &mut x);

    // Verify the solution makes sense
    assert!(x.get(3) > 0.0, "Solution should be reasonable");
    assert!(x.get(4) > 0.0, "Solution should be reasonable");
    assert!(x.get(5) > 0.0, "Solution should be reasonable");
}
