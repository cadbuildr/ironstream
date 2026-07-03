//! Port of OCCT's `math_Gauss_Test.cxx` GTest suite to Rust `#[test]` fns.
//!
//! Same numeric inputs, same expected values, same tolerances as the original
//! (`src/FoundationClasses/TKMath/GTests/math_Gauss_Test.cxx`).
//! `Precision::Confusion()` maps to `ironstream::precision::CONFUSION`.

use ironstream::math_gauss_lu::{MathGauss, MathMatrix, MathVector};
use ironstream::precision::CONFUSION;

/// `EXPECT_NEAR(a, b, tol)`.
fn expect_near(a: f64, b: f64, tol: f64, msg: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{msg}: expected {b}, got {a} (|diff|={} > tol={tol})",
        (a - b).abs()
    );
}

#[test]
fn well_conditioned_matrix() {
    // Test with a simple 3x3 well-conditioned matrix
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 2.0);
    a.set(1, 2, 1.0);
    a.set(1, 3, 1.0);
    a.set(2, 1, 1.0);
    a.set(2, 2, 3.0);
    a.set(2, 3, 2.0);
    a.set(3, 1, 1.0);
    a.set(3, 2, 2.0);
    a.set(3, 3, 3.0);

    let gauss = MathGauss::new(&a);
    assert!(
        gauss.is_done(),
        "Gauss decomposition should succeed for well-conditioned matrix"
    );

    let mut b = MathVector::new(1, 3);
    b.set(1, 1.0);
    b.set(2, 2.0);
    b.set(3, 3.0);

    let mut x = MathVector::new(1, 3);
    gauss.solve(&b, &mut x);

    // Verify solution by checking A * x = b
    for i in 1..=3 {
        let mut verify = 0.0;
        for j in 1..=3 {
            verify += a.get(i, j) * x.get(j);
        }
        expect_near(verify, b.get(i), 1.0e-10, &format!("Solution verification A*x=b ({i})"));
    }
}

#[test]
fn identity_matrix() {
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

    let gauss = MathGauss::new(&a);
    assert!(
        gauss.is_done(),
        "Gauss decomposition should succeed for identity matrix"
    );

    let mut b = MathVector::new(1, 3);
    b.set(1, 5.0);
    b.set(2, 7.0);
    b.set(3, 9.0);

    let mut x = MathVector::new(1, 3);
    gauss.solve(&b, &mut x);

    expect_near(x.get(1), 5.0, CONFUSION, "Identity matrix solution X(1)");
    expect_near(x.get(2), 7.0, CONFUSION, "Identity matrix solution X(2)");
    expect_near(x.get(3), 9.0, CONFUSION, "Identity matrix solution X(3)");
}

#[test]
fn diagonal_matrix() {
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

    let gauss = MathGauss::new(&a);
    assert!(
        gauss.is_done(),
        "Gauss decomposition should succeed for diagonal matrix"
    );

    let mut b = MathVector::new(1, 3);
    b.set(1, 8.0);
    b.set(2, 15.0);
    b.set(3, 20.0); // Expected solution: [4, 5, 5]

    let mut x = MathVector::new(1, 3);
    gauss.solve(&b, &mut x);

    expect_near(x.get(1), 4.0, CONFUSION, "Diagonal matrix solution X(1)");
    expect_near(x.get(2), 5.0, CONFUSION, "Diagonal matrix solution X(2)");
    expect_near(x.get(3), 5.0, CONFUSION, "Diagonal matrix solution X(3)");
}

#[test]
fn in_place_solve() {
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(1, 3, 3.0);
    a.set(2, 1, 2.0);
    a.set(2, 2, 5.0);
    a.set(2, 3, 8.0);
    a.set(3, 1, 3.0);
    a.set(3, 2, 8.0);
    a.set(3, 3, 14.0);

    let gauss = MathGauss::new(&a);
    assert!(gauss.is_done(), "Gauss decomposition should succeed");

    let mut b = MathVector::new(1, 3);
    b.set(1, 14.0);
    b.set(2, 31.0);
    b.set(3, 53.0); // Should give solution [13, -7, 5]

    gauss.solve_in_place(&mut b); // In-place solve

    expect_near(b.get(1), 13.0, 1.0e-10, "In-place solution B(1)");
    expect_near(b.get(2), -7.0, 1.0e-10, "In-place solution B(2)");
    expect_near(b.get(3), 5.0, 1.0e-10, "In-place solution B(3)");
}

#[test]
fn determinant() {
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(1, 3, 3.0);
    a.set(2, 1, 0.0);
    a.set(2, 2, 1.0);
    a.set(2, 3, 4.0);
    a.set(3, 1, 5.0);
    a.set(3, 2, 6.0);
    a.set(3, 3, 0.0);

    let gauss = MathGauss::new(&a);
    assert!(gauss.is_done(), "Gauss decomposition should succeed");

    let det = gauss.determinant();
    // Expected determinant: 1*(1*0 - 4*6) - 2*(0*0 - 4*5) + 3*(0*6 - 1*5) = 1
    expect_near(det, 1.0, 1.0e-12, "Determinant calculation");
}

#[test]
fn matrix_inversion() {
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 4.0);
    a.set(1, 2, 7.0);
    a.set(2, 1, 2.0);
    a.set(2, 2, 6.0);

    let gauss = MathGauss::new(&a);
    assert!(gauss.is_done(), "Gauss decomposition should succeed");

    let mut inverse = MathMatrix::new(1, 2, 1, 2);
    gauss.invert_into(&mut inverse);

    // For 2x2 matrix [[4,7],[2,6]], inverse should be [[0.6,-0.7],[-0.2,0.4]]
    expect_near(inverse.get(1, 1), 0.6, 1.0e-12, "Inverse(1,1)");
    expect_near(inverse.get(1, 2), -0.7, 1.0e-12, "Inverse(1,2)");
    expect_near(inverse.get(2, 1), -0.2, 1.0e-12, "Inverse(2,1)");
    expect_near(inverse.get(2, 2), 0.4, 1.0e-12, "Inverse(2,2)");

    // Verify that A * A^(-1) = I
    for i in 1..=2 {
        for j in 1..=2 {
            let mut product = 0.0;
            for k in 1..=2 {
                product += a.get(i, k) * inverse.get(k, j);
            }
            let expected = if i == j { 1.0 } else { 0.0 };
            expect_near(
                product,
                expected,
                1.0e-12,
                &format!("A * A^(-1) should equal identity ({i},{j})"),
            );
        }
    }
}

#[test]
fn singular_matrix() {
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

    // Singular matrix should either fail decomposition or have zero determinant
    let gauss = MathGauss::new(&a);
    if gauss.is_done() {
        let det = gauss.determinant();
        expect_near(det, 0.0, 1.0e-12, "Determinant of singular matrix must be zero");
    }
}

#[test]
fn custom_min_pivot() {
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 1.0e-15);
    a.set(1, 2, 1.0); // Very small pivot
    a.set(2, 1, 1.0);
    a.set(2, 2, 1.0);

    // With default MinPivot (1e-20), should succeed
    let gauss1 = MathGauss::new(&a);
    assert!(gauss1.is_done(), "Should succeed with default MinPivot");

    // Truly singular (all zeros) which should always fail
    let mut truly_singular = MathMatrix::new(1, 2, 1, 2);
    truly_singular.set(1, 1, 0.0);
    truly_singular.set(1, 2, 0.0);
    truly_singular.set(2, 1, 0.0);
    truly_singular.set(2, 2, 0.0);

    let gauss2 = MathGauss::new(&truly_singular);
    assert!(!gauss2.is_done(), "Should fail for zero matrix");
}

#[test]
fn larger_matrix() {
    let mut a = MathMatrix::new(1, 4, 1, 4);
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(1, 3, 1.0);
    a.set(1, 4, 1.0);
    a.set(2, 1, 2.0);
    a.set(2, 2, 1.0);
    a.set(2, 3, 3.0);
    a.set(2, 4, 1.0);
    a.set(3, 1, 1.0);
    a.set(3, 2, 3.0);
    a.set(3, 3, 1.0);
    a.set(3, 4, 2.0);
    a.set(4, 1, 1.0);
    a.set(4, 2, 1.0);
    a.set(4, 3, 2.0);
    a.set(4, 4, 3.0);

    let gauss = MathGauss::new(&a);
    assert!(
        gauss.is_done(),
        "Gauss decomposition should succeed for 4x4 matrix"
    );

    let mut b = MathVector::new(1, 4);
    b.set(1, 1.0);
    b.set(2, 2.0);
    b.set(3, 3.0);
    b.set(4, 4.0);

    let mut x = MathVector::new(1, 4);
    gauss.solve(&b, &mut x);

    // Verify solution by checking A * x = b
    for i in 1..=4 {
        let mut verify = 0.0;
        for j in 1..=4 {
            verify += a.get(i, j) * x.get(j);
        }
        expect_near(
            verify,
            b.get(i),
            1.0e-10,
            &format!("4x4 matrix solution verification ({i})"),
        );
    }
}

#[test]
fn dimension_compatibility() {
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

    let gauss = MathGauss::new(&a);
    assert!(gauss.is_done());

    let mut b = MathVector::new(1, 3);
    b.set(1, 1.0);
    b.set(2, 2.0);
    b.set(3, 3.0);

    let mut x = MathVector::new(1, 3);
    gauss.solve(&b, &mut x);

    // Verify the result makes sense
    assert_eq!(x.length(), 3, "Solution vector should have correct dimension");

    let mut inv = MathMatrix::new(1, 3, 1, 3);
    gauss.invert_into(&mut inv);

    assert_eq!(inv.row_number(), 3, "Inverse matrix should have correct dimensions");
    assert_eq!(inv.col_number(), 3, "Inverse matrix should have correct dimensions");
}

#[test]
fn singular_matrix_state() {
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 1.0);
    a.set(1, 2, 2.0);
    a.set(2, 1, 2.0);
    a.set(2, 2, 4.0); // Singular matrix

    let gauss = MathGauss::new(&a);
    assert!(!gauss.is_done(), "Should fail for singular matrix");
}

#[test]
fn custom_bounds() {
    // Test with non-standard matrix bounds
    let mut a = MathMatrix::new(2, 4, 3, 5);
    a.set(2, 3, 2.0);
    a.set(2, 4, 1.0);
    a.set(2, 5, 1.0);
    a.set(3, 3, 1.0);
    a.set(3, 4, 3.0);
    a.set(3, 5, 2.0);
    a.set(4, 3, 1.0);
    a.set(4, 4, 2.0);
    a.set(4, 5, 3.0);

    let gauss = MathGauss::new(&a);
    assert!(
        gauss.is_done(),
        "Gauss decomposition should succeed with custom bounds"
    );

    let mut b = MathVector::new(2, 4);
    b.set(2, 6.0);
    b.set(3, 11.0);
    b.set(4, 13.0); // Should give solution [0.75, 1.25, 3.25]

    let mut x = MathVector::new(3, 5);
    gauss.solve(&b, &mut x);

    expect_near(x.get(3), 0.75, 1.0e-10, "Custom bounds solution X(3)");
    expect_near(x.get(4), 1.25, 1.0e-10, "Custom bounds solution X(4)");
    expect_near(x.get(5), 3.25, 1.0e-10, "Custom bounds solution X(5)");
}
