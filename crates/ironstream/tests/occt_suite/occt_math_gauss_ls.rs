//! Ported OpenCascade unit tests -- `math_GaussLeastSquare`.
//!
//! Same numeric inputs and expected values as the OCCT test suite.
//! `Precision::Confusion()` = 1e-7, `Precision::Angular()` = 1e-12.

use ironstream::math_gauss_ls::{MathGaussLeastSquare, MathMatrix, MathVector};
use ironstream::precision::{ANGULAR, CONFUSION};

/// Assert that `actual` is within `tol` of `expected` (mirrors EXPECT_NEAR).
fn expect_near(actual: f64, expected: f64, tol: f64, msg: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{msg}: |{actual} - {expected}| = {} > {tol}",
        (actual - expected).abs()
    );
}

// ---------------------------------------------------------------------------
// 1. Square matrix (exactly determined system)
// ---------------------------------------------------------------------------

#[test]
fn square_well_conditioned() {
    // A 3×3 well-conditioned matrix — least-squares degenerates to exact solution.
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

    let solver = MathGaussLeastSquare::new(&a);
    assert!(solver.is_done(), "Decomposition should succeed");

    let mut b = MathVector::new(1, 3);
    b.set(1, 1.0);
    b.set(2, 2.0);
    b.set(3, 3.0);

    let mut x = MathVector::new(1, 3);
    solver.solve(&b, &mut x);

    // Verify A·X ≈ B
    for i in 1..=3 {
        let mut ax = 0.0_f64;
        for j in 1..=3 {
            ax += a.get(i, j) * x.get(j);
        }
        expect_near(ax, b.get(i), 1.0e-10, &format!("A*x=b row {i}"));
    }
}

// ---------------------------------------------------------------------------
// 2. Overdetermined system (more equations than unknowns)
// ---------------------------------------------------------------------------

#[test]
fn overdetermined_4x2() {
    // 4 equations, 2 unknowns — classic least-squares.
    let mut a = MathMatrix::new(1, 4, 1, 2);
    a.set(1, 1, 1.0);
    a.set(1, 2, 1.0);
    a.set(2, 1, 1.0);
    a.set(2, 2, 2.0);
    a.set(3, 1, 2.0);
    a.set(3, 2, 1.0);
    a.set(4, 1, 1.0);
    a.set(4, 2, 3.0);

    let solver = MathGaussLeastSquare::new(&a);
    assert!(solver.is_done(), "Decomposition should succeed for overdetermined system");

    let mut b = MathVector::new(1, 4);
    b.set(1, 2.0);
    b.set(2, 3.0);
    b.set(3, 3.0);
    b.set(4, 4.0);

    let mut x = MathVector::new(1, 2);
    solver.solve(&b, &mut x);

    // Expected least-squares solution ≈ [1, 1].
    expect_near(x.get(1), 1.0, 1.0e-6, "LS solution X(1)");
    expect_near(x.get(2), 1.0, 1.0e-6, "LS solution X(2)");
}

// ---------------------------------------------------------------------------
// 3. Identity matrix (trivial case)
// ---------------------------------------------------------------------------

#[test]
fn identity_matrix() {
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 1.0);
    a.set(2, 2, 1.0);
    a.set(3, 3, 1.0);

    let solver = MathGaussLeastSquare::new(&a);
    assert!(solver.is_done(), "Decomposition should succeed for identity");

    let mut b = MathVector::new(1, 3);
    b.set(1, 5.0);
    b.set(2, 7.0);
    b.set(3, 9.0);

    let mut x = MathVector::new(1, 3);
    solver.solve(&b, &mut x);

    // Solution of I·X = B is X = B.
    expect_near(x.get(1), 5.0, CONFUSION, "Identity solution X(1)");
    expect_near(x.get(2), 7.0, CONFUSION, "Identity solution X(2)");
    expect_near(x.get(3), 9.0, CONFUSION, "Identity solution X(3)");
}

// ---------------------------------------------------------------------------
// 4. Diagonal scaling
// ---------------------------------------------------------------------------

#[test]
fn diagonal_matrix() {
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 2.0);
    a.set(2, 2, 3.0);
    a.set(3, 3, 4.0);

    let solver = MathGaussLeastSquare::new(&a);
    assert!(solver.is_done(), "Decomposition should succeed for diagonal");

    let mut b = MathVector::new(1, 3);
    b.set(1, 8.0);
    b.set(2, 15.0);
    b.set(3, 20.0);

    let mut x = MathVector::new(1, 3);
    solver.solve(&b, &mut x);

    // Expected: [4, 5, 5].
    expect_near(x.get(1), 4.0, CONFUSION, "Diagonal solution X(1)");
    expect_near(x.get(2), 5.0, CONFUSION, "Diagonal solution X(2)");
    expect_near(x.get(3), 5.0, CONFUSION, "Diagonal solution X(3)");
}

// ---------------------------------------------------------------------------
// 5. Singular / rank-deficient normal matrix → IsDone() = false
// ---------------------------------------------------------------------------

#[test]
fn singular_column_dependent() {
    // Two columns are identical → AtA is singular.
    let mut a = MathMatrix::new(1, 3, 1, 2);
    a.set(1, 1, 1.0);
    a.set(1, 2, 1.0); // col 2 = col 1
    a.set(2, 1, 2.0);
    a.set(2, 2, 2.0);
    a.set(3, 1, 3.0);
    a.set(3, 2, 3.0);

    let solver = MathGaussLeastSquare::new(&a);
    // AtA = [[14,14],[14,14]] which is singular.
    assert!(!solver.is_done(), "Should fail for rank-deficient matrix");
}

// ---------------------------------------------------------------------------
// 6. Custom min_pivot threshold
// ---------------------------------------------------------------------------

#[test]
fn custom_min_pivot_accept() {
    // Well-conditioned 2×2 — should always succeed with default or relaxed pivot.
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 3.0);
    a.set(1, 2, 0.0);
    a.set(2, 1, 0.0);
    a.set(2, 2, 4.0);

    let solver = MathGaussLeastSquare::with_min_pivot(&a, 1.0e-20);
    assert!(solver.is_done(), "Should succeed with generous min_pivot");

    let mut b = MathVector::new(1, 2);
    b.set(1, 9.0);
    b.set(2, 16.0);

    let mut x = MathVector::new(1, 2);
    solver.solve(&b, &mut x);

    // AtA = [[9,0],[0,16]], Atb = [[27],[64]], solution = [3, 4].
    expect_near(x.get(1), 3.0, CONFUSION, "custom pivot X(1)");
    expect_near(x.get(2), 4.0, CONFUSION, "custom pivot X(2)");
}

#[test]
fn custom_min_pivot_reject() {
    // Matrix whose AtA has a small but non-zero pivot; strict threshold rejects it.
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 1.0e-6);
    a.set(1, 2, 0.0);
    a.set(2, 1, 0.0);
    a.set(2, 2, 1.0e-6);

    // AtA = [[1e-12, 0],[0, 1e-12]] — pivots < 1e-10.
    let solver = MathGaussLeastSquare::with_min_pivot(&a, 1.0e-10);
    assert!(!solver.is_done(), "Should fail with strict min_pivot");
}

// ---------------------------------------------------------------------------
// 7. Non-standard (non-1-based) index bounds
// ---------------------------------------------------------------------------

#[test]
fn custom_index_bounds() {
    // A with row range [2,4], col range [3,4].
    let mut a = MathMatrix::new(2, 4, 3, 4);
    a.set(2, 3, 1.0);
    a.set(2, 4, 0.0);
    a.set(3, 3, 0.0);
    a.set(3, 4, 2.0);
    a.set(4, 3, 1.0);
    a.set(4, 4, 2.0);

    let solver = MathGaussLeastSquare::new(&a);
    assert!(solver.is_done(), "Custom bounds should succeed");

    // B with lower=2, length=3.
    let mut b = MathVector::new(2, 4);
    b.set(2, 3.0);
    b.set(3, 8.0);
    b.set(4, 7.0);

    // X with lower=5, length=2.
    let mut x = MathVector::new(5, 6);
    solver.solve(&b, &mut x);

    // Verify by computing AtA·X ≈ Atb.
    // AtA = [[2,2],[2,8]], Atb = [3+7, 16+14] = [10,30] → X=[2,3.5].
    // But let's just verify A·X residual is small.
    let ax1 = a.get(2, 3) * x.get(5) + a.get(2, 4) * x.get(6);
    let ax2 = a.get(3, 3) * x.get(5) + a.get(3, 4) * x.get(6);
    let ax3 = a.get(4, 3) * x.get(5) + a.get(4, 4) * x.get(6);
    // Residual = b - Ax; for LS the residual is Aᵀ(Ax-b)=0.
    // Check that Aᵀ(Ax-b) ≈ 0:
    let r1 = ax1 - b.get(2);
    let r2 = ax2 - b.get(3);
    let r3 = ax3 - b.get(4);
    let normal_r1 = a.get(2, 3) * r1 + a.get(3, 3) * r2 + a.get(4, 3) * r3;
    let normal_r2 = a.get(2, 4) * r1 + a.get(3, 4) * r2 + a.get(4, 4) * r3;
    expect_near(normal_r1, 0.0, 1.0e-10, "normal residual component 1");
    expect_near(normal_r2, 0.0, 1.0e-10, "normal residual component 2");
}

// ---------------------------------------------------------------------------
// 8. Larger overdetermined system (5×3)
// ---------------------------------------------------------------------------

#[test]
fn overdetermined_5x3() {
    // Fit a plane z = ax + by + c through 5 points in 3D (least squares).
    // Points: (1,0,1), (0,1,2), (1,1,3), (2,0,2), (0,2,4).
    // True coefficients: a=1, b=2, c=0  →  z = x + 2y.
    let mut a = MathMatrix::new(1, 5, 1, 3);
    // Row: [x, y, 1]
    a.set(1, 1, 1.0); a.set(1, 2, 0.0); a.set(1, 3, 1.0);
    a.set(2, 1, 0.0); a.set(2, 2, 1.0); a.set(2, 3, 1.0);
    a.set(3, 1, 1.0); a.set(3, 2, 1.0); a.set(3, 3, 1.0);
    a.set(4, 1, 2.0); a.set(4, 2, 0.0); a.set(4, 3, 1.0);
    a.set(5, 1, 0.0); a.set(5, 2, 2.0); a.set(5, 3, 1.0);

    let solver = MathGaussLeastSquare::new(&a);
    assert!(solver.is_done(), "5×3 overdetermined should succeed");

    // B = [z values]
    let mut b = MathVector::new(1, 5);
    b.set(1, 1.0);
    b.set(2, 2.0);
    b.set(3, 3.0);
    b.set(4, 2.0);
    b.set(5, 4.0);

    let mut x = MathVector::new(1, 3);
    solver.solve(&b, &mut x);

    // Solution should be [1, 2, 0] (exact fit because data is consistent).
    expect_near(x.get(1), 1.0, CONFUSION, "5x3 LS solution a=1");
    expect_near(x.get(2), 2.0, CONFUSION, "5x3 LS solution b=2");
    expect_near(x.get(3), 0.0, CONFUSION, "5x3 LS solution c=0");
}

// ---------------------------------------------------------------------------
// 9. Dump / Display
// ---------------------------------------------------------------------------

#[test]
fn dump_done() {
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 1.0);
    a.set(2, 2, 1.0);
    let solver = MathGaussLeastSquare::new(&a);
    assert!(solver.is_done());
    let s = solver.dump();
    assert!(s.contains("Done"), "dump() should report Done: {s}");
}

#[test]
fn dump_not_done() {
    // All-zero matrix → singular.
    let a = MathMatrix::new(1, 2, 1, 2);
    let solver = MathGaussLeastSquare::new(&a);
    assert!(!solver.is_done());
    let s = solver.dump();
    assert!(s.contains("not Done"), "dump() should report not Done: {s}");
}

// ---------------------------------------------------------------------------
// 10. Regression: precision::ANGULAR tolerance on residuals
// ---------------------------------------------------------------------------

#[test]
fn regression_angular_precision() {
    // Very well-conditioned 2×2; residual of normal equations should be
    // essentially machine-zero — verify it falls below ANGULAR = 1e-12.
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 1.0);
    a.set(1, 2, 0.0);
    a.set(2, 1, 0.0);
    a.set(2, 2, 1.0);

    let solver = MathGaussLeastSquare::new(&a);
    assert!(solver.is_done());

    let mut b = MathVector::new(1, 2);
    b.set(1, 3.0);
    b.set(2, 7.0);

    let mut x = MathVector::new(1, 2);
    solver.solve(&b, &mut x);

    // Normal-equation residual: Aᵀ(Ax - b) = (AtA)x - Atb.
    // For identity, AtA=I, Atb=b, so (AtA)x-Atb = x-b.
    expect_near(x.get(1) - 3.0, 0.0, ANGULAR, "normal residual row 1 within ANGULAR");
    expect_near(x.get(2) - 7.0, 0.0, ANGULAR, "normal residual row 2 within ANGULAR");
}
