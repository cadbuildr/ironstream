//! Ported OCCT GTest for `math_Householder`.
//!
//! Source: `src/FoundationClasses/TKMath/GTests/math_Householder_Test.cxx`
//! (OPEN CASCADE SAS, 2025). Each `TEST(MathHouseholderTest, ...)` is ported as
//! a Rust `#[test]` with the SAME numeric inputs, expected values, and
//! tolerances. `math_Matrix`/`math_Vector` map to the module's
//! `MathMatrix`/`MathVector`; `Precision::Confusion()` maps to
//! `ironstream::precision::CONFUSION`.

use ironstream::math_householder::{MathHouseholder, MathMatrix, MathVector};
use ironstream::precision::CONFUSION;

/// Assert that `actual` is within `tol` of `expected` (mirrors EXPECT_NEAR).
fn expect_near(actual: f64, expected: f64, tol: f64, msg: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{msg}: |{actual} - {expected}| = {} > {tol}",
        (actual - expected).abs()
    );
}

#[test]
fn exactly_determined_system() {
    // Test with a square matrix (exact solution exists)
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(1, 3, 3.0);
    a.set(2, 1, 4.0);
    a.set(2, 2, 5.0);
    a.set(2, 3, 6.0);
    a.set(3, 1, 7.0);
    a.set(3, 2, 8.0);
    a.set(3, 3, 10.0); // Note: 9 would make it singular

    let mut b = MathVector::new(1, 3);
    b.set(1, 14.0);
    b.set(2, 32.0);
    b.set(3, 55.0); // Should give solution approximately [1, 2, 3]

    let householder = MathHouseholder::from_vector(&a, &b, MathHouseholder::DEFAULT_EPS);
    assert!(
        householder.is_done(),
        "Householder should succeed for well-conditioned system"
    );

    let sol = householder.value(1);

    // Verify solution by checking A * x = b
    let mut verify = MathVector::new(1, 3);
    for i in 1..=3 {
        let mut acc = 0.0;
        for j in 1..=3 {
            acc += a.get(i, j) * sol.get(j);
        }
        verify.set(i, acc);
    }

    expect_near(verify.get(1), b.get(1), 1.0e-10, "Solution verification A*x=b (1)");
    expect_near(verify.get(2), b.get(2), 1.0e-10, "Solution verification A*x=b (2)");
    expect_near(verify.get(3), b.get(3), 1.0e-10, "Solution verification A*x=b (3)");
}

#[test]
fn overdetermined_system() {
    // Test with an overdetermined system (more equations than unknowns)
    let mut a = MathMatrix::new(1, 4, 1, 2); // 4 equations, 2 unknowns
    a.set(1, 1, 1.0);
    a.set(1, 2, 1.0); // x + y = 2 (approximately)
    a.set(2, 1, 1.0);
    a.set(2, 2, 2.0); // x + 2y = 3
    a.set(3, 1, 2.0);
    a.set(3, 2, 1.0); // 2x + y = 3
    a.set(4, 1, 1.0);
    a.set(4, 2, 3.0); // x + 3y = 4

    let mut b = MathVector::new(1, 4);
    b.set(1, 2.0);
    b.set(2, 3.0);
    b.set(3, 3.0);
    b.set(4, 4.0);

    let householder = MathHouseholder::from_vector(&a, &b, MathHouseholder::DEFAULT_EPS);
    assert!(
        householder.is_done(),
        "Householder should succeed for overdetermined system"
    );

    let sol = householder.value(1);

    // For least squares, solution should minimize ||A*x - b||^2
    // Expected solution is approximately [1, 1]
    expect_near(sol.get(1), 1.0, 1.0e-6, "Least squares solution X(1)");
    expect_near(sol.get(2), 1.0, 1.0e-6, "Least squares solution X(2)");
}

#[test]
fn multiple_right_hand_sides() {
    // Test solving A*X = B where B has multiple columns
    let mut a = MathMatrix::new(1, 3, 1, 2);
    a.set(1, 1, 1.0);
    a.set(1, 2, 0.0);
    a.set(2, 1, 0.0);
    a.set(2, 2, 1.0);
    a.set(3, 1, 1.0);
    a.set(3, 2, 1.0);

    let mut b = MathMatrix::new(1, 3, 1, 2); // Two right-hand sides
    b.set(1, 1, 1.0);
    b.set(1, 2, 2.0); // First RHS: [1, 3, 4]
    b.set(2, 1, 3.0);
    b.set(2, 2, 4.0); // Second RHS: [2, 4, 6]
    b.set(3, 1, 4.0);
    b.set(3, 2, 6.0);

    let householder = MathHouseholder::from_matrix(&a, &b, MathHouseholder::DEFAULT_EPS);
    assert!(
        householder.is_done(),
        "Householder should succeed for multiple RHS"
    );

    // Get first solution
    let sol1 = householder.value(1);
    // Get second solution
    let sol2 = householder.value(2);

    // Expected solutions: first RHS gives approximately [1, 3], second gives [2, 4]
    expect_near(sol1.get(1), 1.0, 1.0e-6, "First solution X(1)");
    expect_near(sol1.get(2), 3.0, 1.0e-6, "First solution X(2)");

    expect_near(sol2.get(1), 2.0, 1.0e-6, "Second solution X(1)");
    expect_near(sol2.get(2), 4.0, 1.0e-6, "Second solution X(2)");
}

#[test]
fn near_singular_matrix() {
    // Test with nearly singular matrix
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(1, 3, 3.0);
    a.set(2, 1, 2.0);
    a.set(2, 2, 4.0);
    a.set(2, 3, 6.0 + 1.0e-15); // Nearly dependent
    a.set(3, 1, 3.0);
    a.set(3, 2, 6.0);
    a.set(3, 3, 9.0 + 2.0e-15); // Nearly dependent

    let mut b = MathVector::new(1, 3);
    b.set(1, 1.0);
    b.set(2, 2.0);
    b.set(3, 3.0);

    // With default EPS, should handle near-singularity
    let householder = MathHouseholder::from_vector(&a, &b, MathHouseholder::DEFAULT_EPS);

    // Near-singular matrix should fail with default EPS
    assert!(
        !householder.is_done(),
        "Should fail for near-singular matrix with default EPS"
    );
}

#[test]
fn custom_epsilon() {
    // Test with custom epsilon threshold using well-conditioned values
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 1.0e-3);
    a.set(1, 2, 1.0);
    a.set(2, 1, 2.0e-3);
    a.set(2, 2, 2.0);

    let mut b = MathVector::new(1, 2);
    b.set(1, 1.0);
    b.set(2, 2.0);

    // Test that default EPS works with this matrix
    let householder1 = MathHouseholder::from_vector(&a, &b, MathHouseholder::DEFAULT_EPS);
    assert!(householder1.is_done(), "Should succeed with default EPS");

    // Test with very restrictive EPS that should fail
    let householder2 = MathHouseholder::from_vector(&a, &b, 1.0e-2);
    assert!(
        !householder2.is_done(),
        "Should fail with very restrictive EPS"
    );
}

#[test]
fn identity_matrix() {
    // Test with identity matrix (trivial case)
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

    let mut b = MathVector::new(1, 3);
    b.set(1, 5.0);
    b.set(2, 7.0);
    b.set(3, 9.0);

    let householder = MathHouseholder::from_vector(&a, &b, MathHouseholder::DEFAULT_EPS);
    assert!(
        householder.is_done(),
        "Householder should succeed for identity matrix"
    );

    let sol = householder.value(1);

    // For identity matrix, solution should equal RHS
    expect_near(sol.get(1), 5.0, CONFUSION, "Identity matrix solution X(1)");
    expect_near(sol.get(2), 7.0, CONFUSION, "Identity matrix solution X(2)");
    expect_near(sol.get(3), 9.0, CONFUSION, "Identity matrix solution X(3)");
}

// Removed RangeConstructor test due to unknown exception issues (mirrors OCCT)
// TODO: Investigate math_Householder range constructor compatibility

#[test]
fn dimension_compatibility() {
    // Test dimension compatibility handling
    let mut a = MathMatrix::new(1, 3, 1, 2);
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(2, 1, 3.0);
    a.set(2, 2, 4.0);
    a.set(3, 1, 5.0);
    a.set(3, 2, 6.0);

    // Test with correctly sized B vector
    let mut b_correct = MathVector::new(1, 3); // Correct size 3
    b_correct.set(1, 1.0);
    b_correct.set(2, 2.0);
    b_correct.set(3, 3.0);

    // Test with correctly sized B vector - should work
    let householder1 = MathHouseholder::from_vector(&a, &b_correct, MathHouseholder::DEFAULT_EPS);
    assert!(
        householder1.is_done(),
        "Should work with correct B vector size"
    );

    // Test with correctly sized B matrix
    let mut b_matrix_correct = MathMatrix::new(1, 3, 1, 2); // Correct row count 3
    b_matrix_correct.set(1, 1, 1.0);
    b_matrix_correct.set(1, 2, 4.0);
    b_matrix_correct.set(2, 1, 2.0);
    b_matrix_correct.set(2, 2, 5.0);
    b_matrix_correct.set(3, 1, 3.0);
    b_matrix_correct.set(3, 2, 6.0);

    let householder2 =
        MathHouseholder::from_matrix(&a, &b_matrix_correct, MathHouseholder::DEFAULT_EPS);
    assert!(
        householder2.is_done(),
        "Should work with correct B matrix size"
    );
}

#[test]
fn near_zero_matrix_state() {
    // Create a scenario where Householder fails
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 1.0e-25);
    a.set(1, 2, 0.0); // Extremely small values
    a.set(2, 1, 0.0);
    a.set(2, 2, 1.0e-25);

    let mut b = MathVector::new(1, 2);
    b.set(1, 1.0);
    b.set(2, 2.0);

    let householder = MathHouseholder::from_vector(&a, &b, 1.0e-10); // Large EPS
    assert!(
        !householder.is_done(),
        "Should fail for nearly zero matrix"
    );
}

#[test]
fn valid_index_range() {
    // Test valid index range handling
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 1.0);
    a.set(1, 2, 0.0);
    a.set(2, 1, 0.0);
    a.set(2, 2, 1.0);

    let mut b = MathMatrix::new(1, 2, 1, 2); // Two columns
    b.set(1, 1, 1.0);
    b.set(1, 2, 2.0);
    b.set(2, 1, 3.0);
    b.set(2, 2, 4.0);

    let householder = MathHouseholder::from_matrix(&a, &b, MathHouseholder::DEFAULT_EPS);
    assert!(householder.is_done());

    // Test valid indices
    let sol1 = householder.value(1); // Should work
    assert_eq!(sol1.length(), 2, "Solution vector should have correct size");

    let sol2 = householder.value(2); // Should work
    assert_eq!(sol2.length(), 2, "Solution vector should have correct size");

    // Verify we have the expected number of columns to work with
    assert_eq!(b.col_number(), 2, "Matrix should have 2 columns available");
}

#[test]
fn regression_test() {
    // Regression test with known data
    let mut a = MathMatrix::new(1, 3, 1, 2);
    a.set(1, 1, 2.0);
    a.set(1, 2, 1.0); // 2x + y = 5
    a.set(2, 1, 1.0);
    a.set(2, 2, 1.0); // x + y = 3
    a.set(3, 1, 1.0);
    a.set(3, 2, 2.0); // x + 2y = 4

    let mut b = MathVector::new(1, 3);
    b.set(1, 5.0);
    b.set(2, 3.0);
    b.set(3, 4.0);

    let householder = MathHouseholder::from_vector(&a, &b, MathHouseholder::DEFAULT_EPS);
    assert!(householder.is_done(), "Regression test should succeed");

    let sol = householder.value(1);

    // Expected least squares solution
    expect_near(sol.get(1), 2.0, 1.0e-10, "Regression solution X(1)");
    expect_near(sol.get(2), 1.0, 1.0e-10, "Regression solution X(2)");
}
