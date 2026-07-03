//! Ported OpenCascade unit tests — `math_Jacobi`.
//!
//! Faithful Rust port of OCCT's `math_Jacobi_Test.cxx`
//! (Open-Cascade-SAS/OCCT, `src/FoundationClasses/TKMath/GTests/`). Same
//! numeric inputs, same expected values, same tolerances.
//!
//! `math_Matrix` → [`MathMatrix`], `math_Vector` → [`MathVector`],
//! `math_Jacobi` → [`MathJacobi`]. OCCT's 1-indexed accessors `M(i,j)` / `V(i)`
//! map to `M.set(i,j,..)` / `M.get(i,j)` and `V.set(i,..)` / `V.get(i)`.

use ironstream::math_jacobi::{MathJacobi, MathMatrix, MathVector};

/// EXPECT_NEAR(a, b, tol).
fn near(a: f64, b: f64, tol: f64, msg: &str) {
    assert!((a - b).abs() <= tol, "{msg}: expected {a} ≈ {b} (tol {tol})");
}

// ====================== MathJacobiTest.IdentityMatrix ======================

#[test]
fn identity_matrix() {
    // Test with identity matrix - all eigenvalues should be 1
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

    let jacobi = MathJacobi::new(&a);

    assert!(jacobi.is_done(), "Jacobi should succeed for identity matrix");

    let values = jacobi.values();
    assert_eq!(values.length(), 3, "Should have 3 eigenvalues");

    // All eigenvalues should be 1
    near(values.get(1), 1.0, 1.0e-10, "First eigenvalue should be 1");
    near(values.get(2), 1.0, 1.0e-10, "Second eigenvalue should be 1");
    near(values.get(3), 1.0, 1.0e-10, "Third eigenvalue should be 1");

    // Test individual value access
    near(jacobi.value(1), 1.0, 1.0e-10, "Value(1) should be 1");
    near(jacobi.value(2), 1.0, 1.0e-10, "Value(2) should be 1");
    near(jacobi.value(3), 1.0, 1.0e-10, "Value(3) should be 1");
}

// ====================== MathJacobiTest.DiagonalMatrix ======================

#[test]
fn diagonal_matrix() {
    // Test with diagonal matrix - eigenvalues should be diagonal elements
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 5.0);
    a.set(1, 2, 0.0);
    a.set(1, 3, 0.0);
    a.set(2, 1, 0.0);
    a.set(2, 2, 3.0);
    a.set(2, 3, 0.0);
    a.set(3, 1, 0.0);
    a.set(3, 2, 0.0);
    a.set(3, 3, 7.0);

    let jacobi = MathJacobi::new(&a);

    assert!(jacobi.is_done(), "Jacobi should succeed for diagonal matrix");

    let values = jacobi.values();

    // Eigenvalues might be in different order, so collect them
    let mut eigenvals = [values.get(1), values.get(2), values.get(3)];
    eigenvals.sort_by(|x, y| x.partial_cmp(y).unwrap());

    near(eigenvals[0], 3.0, 1.0e-10, "Smallest eigenvalue should be 3");
    near(eigenvals[1], 5.0, 1.0e-10, "Middle eigenvalue should be 5");
    near(eigenvals[2], 7.0, 1.0e-10, "Largest eigenvalue should be 7");
}

// ====================== MathJacobiTest.SimpleSymmetricMatrix ======================

#[test]
fn simple_symmetric_matrix() {
    // Test with simple 2x2 symmetric matrix
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 3.0);
    a.set(1, 2, 1.0);
    a.set(2, 1, 1.0);
    a.set(2, 2, 3.0);

    let jacobi = MathJacobi::new(&a);

    assert!(
        jacobi.is_done(),
        "Jacobi should succeed for 2x2 symmetric matrix"
    );

    let values = jacobi.values();

    // For this matrix, eigenvalues are 2 and 4
    let mut eigenvals = [values.get(1), values.get(2)];
    eigenvals.sort_by(|x, y| x.partial_cmp(y).unwrap());

    near(eigenvals[0], 2.0, 1.0e-10, "First eigenvalue should be 2");
    near(eigenvals[1], 4.0, 1.0e-10, "Second eigenvalue should be 4");
}

// ====================== MathJacobiTest.EigenvectorVerification ======================

#[test]
fn eigenvector_verification() {
    // Test eigenvector computation and verification
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 4.0);
    a.set(1, 2, 2.0);
    a.set(2, 1, 2.0);
    a.set(2, 2, 1.0);

    let jacobi = MathJacobi::new(&a);

    assert!(jacobi.is_done(), "Jacobi should succeed");

    // Verify A * v = lambda * v for each eigenpair
    for i in 1..=2 {
        let mut v = MathVector::new(1, 2);
        jacobi.vector(i, &mut v);

        let lambda = jacobi.value(i);

        // Compute A * v
        let mut av = MathVector::new(1, 2);
        av.set(1, a.get(1, 1) * v.get(1) + a.get(1, 2) * v.get(2));
        av.set(2, a.get(2, 1) * v.get(1) + a.get(2, 2) * v.get(2));

        // Compute lambda * v
        let mut lambda_v = MathVector::new(1, 2);
        lambda_v.set(1, lambda * v.get(1));
        lambda_v.set(2, lambda * v.get(2));

        // Check A*v = lambda*v
        near(
            av.get(1),
            lambda_v.get(1),
            1.0e-10,
            "Eigenvector equation should hold for component 1",
        );
        near(
            av.get(2),
            lambda_v.get(2),
            1.0e-10,
            "Eigenvector equation should hold for component 2",
        );
    }
}

// ====================== MathJacobiTest.LargerSymmetricMatrix ======================

#[test]
fn larger_symmetric_matrix() {
    // Test with larger 4x4 symmetric matrix
    let mut a = MathMatrix::new(1, 4, 1, 4);
    // Create a symmetric positive definite matrix
    a.set(1, 1, 4.0);
    a.set(1, 2, 1.0);
    a.set(1, 3, 0.5);
    a.set(1, 4, 0.2);
    a.set(2, 1, 1.0);
    a.set(2, 2, 5.0);
    a.set(2, 3, 1.5);
    a.set(2, 4, 0.3);
    a.set(3, 1, 0.5);
    a.set(3, 2, 1.5);
    a.set(3, 3, 6.0);
    a.set(3, 4, 2.0);
    a.set(4, 1, 0.2);
    a.set(4, 2, 0.3);
    a.set(4, 3, 2.0);
    a.set(4, 4, 3.0);

    let jacobi = MathJacobi::new(&a);

    assert!(jacobi.is_done(), "Jacobi should succeed for 4x4 matrix");

    let values = jacobi.values();
    assert_eq!(values.length(), 4, "Should have 4 eigenvalues");

    // All eigenvalues should be real and positive (positive definite matrix)
    for i in 1..=4 {
        assert!(
            jacobi.value(i) > 0.0,
            "Eigenvalue {i} should be positive"
        );
    }

    // Verify trace preservation (sum of eigenvalues = sum of diagonal elements)
    let trace_original = a.get(1, 1) + a.get(2, 2) + a.get(3, 3) + a.get(4, 4);
    let trace_eigenvalues = values.get(1) + values.get(2) + values.get(3) + values.get(4);

    near(
        trace_eigenvalues,
        trace_original,
        1.0e-10,
        "Sum of eigenvalues should equal trace",
    );
}

// ====================== MathJacobiTest.SingleElementMatrix ======================

#[test]
fn single_element_matrix() {
    // Test with 1x1 matrix
    let mut a = MathMatrix::new(1, 1, 1, 1);
    a.set(1, 1, 42.0);

    let jacobi = MathJacobi::new(&a);

    assert!(jacobi.is_done(), "Jacobi should succeed for 1x1 matrix");

    let values = jacobi.values();
    assert_eq!(values.length(), 1, "Should have 1 eigenvalue");
    near(
        values.get(1),
        42.0,
        1.0e-12,
        "Eigenvalue should be the matrix element",
    );

    let mut v = MathVector::new(1, 1);
    jacobi.vector(1, &mut v);
    near(v.get(1), 1.0, 1.0e-12, "Eigenvector should be [1]");
}

// ====================== MathJacobiTest.SquareMatrixRequirement ======================

#[test]
fn square_matrix_requirement() {
    // Test square matrix requirement for Jacobi eigenvalue decomposition
    let mut non_square = MathMatrix::new(1, 2, 1, 3); // 2x3 matrix
    non_square.set(1, 1, 1.0);
    non_square.set(1, 2, 2.0);
    non_square.set(1, 3, 3.0);
    non_square.set(2, 1, 4.0);
    non_square.set(2, 2, 5.0);
    non_square.set(2, 3, 6.0);

    // Verify matrix is indeed non-square
    assert_ne!(
        non_square.row_number(),
        non_square.col_number(),
        "Matrix should be non-square for this test"
    );

    // Jacobi eigenvalue decomposition requires square matrices
    // Test with a proper square matrix instead
    let mut square = MathMatrix::new(1, 2, 1, 2);
    square.set(1, 1, 1.0);
    square.set(1, 2, 0.5);
    square.set(2, 1, 0.5);
    square.set(2, 2, 2.0);

    let jacobi = MathJacobi::new(&square);
    assert!(jacobi.is_done(), "Should work with square matrix");
}

// ====================== MathJacobiTest.NotDoneExceptions ======================

#[test]
fn not_done_exceptions() {
    // Create a scenario where Jacobi might fail (though it usually succeeds)
    // Test exception handling for accessing results before computation
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 1.0);
    a.set(1, 2, 0.0);
    a.set(2, 1, 0.0);
    a.set(2, 2, 1.0);

    let jacobi = MathJacobi::new(&a);

    if !jacobi.is_done() {
        // Only test exceptions if computation actually failed.
        // In OCCT these methods raise StdFail_NotDone; here they panic. We
        // verify that by catching the panic for each accessor.
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| jacobi.values())).is_err(),
            "Should panic (StdFail_NotDone) for Values()"
        );
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| jacobi.value(1))).is_err(),
            "Should panic (StdFail_NotDone) for Value()"
        );
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| jacobi.vectors())).is_err(),
            "Should panic (StdFail_NotDone) for Vectors()"
        );
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut v = MathVector::new(1, 2);
                jacobi.vector(1, &mut v);
            }))
            .is_err(),
            "Should panic (StdFail_NotDone) for Vector()"
        );
    }
}

// ====================== MathJacobiTest.OrthogonalityOfEigenvectors ======================

#[test]
fn orthogonality_of_eigenvectors() {
    // Test orthogonality of eigenvectors for symmetric matrix
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 6.0);
    a.set(1, 2, 2.0);
    a.set(1, 3, 1.0);
    a.set(2, 1, 2.0);
    a.set(2, 2, 3.0);
    a.set(2, 3, 1.0);
    a.set(3, 1, 1.0);
    a.set(3, 2, 1.0);
    a.set(3, 3, 1.0);

    let jacobi = MathJacobi::new(&a);

    assert!(jacobi.is_done(), "Jacobi should succeed");

    // Get eigenvectors
    let mut v1 = MathVector::new(1, 3);
    let mut v2 = MathVector::new(1, 3);
    let mut v3 = MathVector::new(1, 3);
    jacobi.vector(1, &mut v1);
    jacobi.vector(2, &mut v2);
    jacobi.vector(3, &mut v3);

    // Check orthogonality (dot products should be zero for different eigenvalues)
    let dot12 = v1.get(1) * v2.get(1) + v1.get(2) * v2.get(2) + v1.get(3) * v2.get(3);
    let dot13 = v1.get(1) * v3.get(1) + v1.get(2) * v3.get(2) + v1.get(3) * v3.get(3);
    let dot23 = v2.get(1) * v3.get(1) + v2.get(2) * v3.get(2) + v2.get(3) * v3.get(3);

    let values = jacobi.values();

    // Only check orthogonality for distinct eigenvalues
    if (values.get(1) - values.get(2)).abs() > 1.0e-10 {
        near(dot12, 0.0, 1.0e-10, "Eigenvectors 1 and 2 should be orthogonal");
    }
    if (values.get(1) - values.get(3)).abs() > 1.0e-10 {
        near(dot13, 0.0, 1.0e-10, "Eigenvectors 1 and 3 should be orthogonal");
    }
    if (values.get(2) - values.get(3)).abs() > 1.0e-10 {
        near(dot23, 0.0, 1.0e-10, "Eigenvectors 2 and 3 should be orthogonal");
    }
}

// ====================== MathJacobiTest.NormalizationOfEigenvectors ======================

#[test]
fn normalization_of_eigenvectors() {
    // Test that eigenvectors are normalized
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 2.0);
    a.set(1, 2, 1.0);
    a.set(1, 3, 0.0);
    a.set(2, 1, 1.0);
    a.set(2, 2, 2.0);
    a.set(2, 3, 1.0);
    a.set(3, 1, 0.0);
    a.set(3, 2, 1.0);
    a.set(3, 3, 2.0);

    let jacobi = MathJacobi::new(&a);

    assert!(jacobi.is_done(), "Jacobi should succeed");

    // Check that each eigenvector has unit norm
    for i in 1..=3 {
        let mut v = MathVector::new(1, 3);
        jacobi.vector(i, &mut v);

        let norm = (v.get(1) * v.get(1) + v.get(2) * v.get(2) + v.get(3) * v.get(3)).sqrt();
        near(norm, 1.0, 1.0e-10, &format!("Eigenvector {i} should be normalized"));
    }
}

// ====================== MathJacobiTest.CustomBounds ======================

#[test]
fn custom_bounds() {
    // Test with custom matrix bounds
    let mut a = MathMatrix::new(2, 3, 2, 3);
    a.set(2, 2, 5.0);
    a.set(2, 3, 1.0);
    a.set(3, 2, 1.0);
    a.set(3, 3, 3.0);

    let jacobi = MathJacobi::new(&a);

    assert!(jacobi.is_done(), "Jacobi should work with custom bounds");

    let values = jacobi.values();
    assert_eq!(values.length(), 2, "Should have 2 eigenvalues for 2x2 matrix");

    // Both eigenvalues should be positive
    assert!(values.get(1) > 0.0, "First eigenvalue should be positive");
    assert!(values.get(2) > 0.0, "Second eigenvalue should be positive");
}
