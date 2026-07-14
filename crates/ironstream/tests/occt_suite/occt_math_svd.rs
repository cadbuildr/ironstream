//! Ported OpenCascade unit tests — `math_SVD`.
//!
//! Covers `math_SVD` (Golub-Reinsch singular value decomposition).
//! Tolerances use `precision::CONFUSION = 1e-7` and `precision::ANGULAR = 1e-12`.

use ironstream::math_svd::{MathMatrix, MathSVD, MathVector};
use ironstream::precision::{ANGULAR, CONFUSION};

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn expect_near(a: f64, b: f64, tol: f64, msg: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{msg}: expected {b:.15e}, got {a:.15e}  (|diff|={:.3e} > tol={tol:.3e})",
        (a - b).abs()
    );
}

/// Build the system `A·X = B` and verify `||A·X - B|| < tol`.
fn verify_ax_eq_b(a: &MathMatrix, x: &MathVector, b: &MathVector, tol: f64, label: &str) {
    let m = a.row_number();
    let n = a.col_number();
    for i in 1..=m {
        let mut row_sum = 0.0_f64;
        for j in 1..=n {
            row_sum += a.get(i, j) * x.get(j);
        }
        expect_near(row_sum, b.get(i), tol, &format!("{label}: row {i}"));
    }
}

// ---------------------------------------------------------------------------
// Test 1 — identity matrix
// ---------------------------------------------------------------------------

#[test]
fn identity_3x3() {
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 1.0);
    a.set(2, 2, 1.0);
    a.set(3, 3, 1.0);

    let svd = MathSVD::new(&a);
    assert!(svd.is_done(), "SVD of identity must succeed");

    // All singular values should be 1
    let w = svd.singular_values();
    for j in 1..=3 {
        expect_near(w.get(j), 1.0, CONFUSION, &format!("identity w[{j}]"));
    }

    // Solve I·X = B → X = B
    let mut b = MathVector::new(1, 3);
    b.set(1, 2.0);
    b.set(2, 5.0);
    b.set(3, -3.0);
    let mut x = MathVector::new(1, 3);
    svd.solve(&b, &mut x, 1.0e-10);

    expect_near(x.get(1), 2.0, CONFUSION, "identity solve x[1]");
    expect_near(x.get(2), 5.0, CONFUSION, "identity solve x[2]");
    expect_near(x.get(3), -3.0, CONFUSION, "identity solve x[3]");
}

// ---------------------------------------------------------------------------
// Test 2 — diagonal matrix
// ---------------------------------------------------------------------------

#[test]
fn diagonal_3x3() {
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 2.0);
    a.set(2, 2, 3.0);
    a.set(3, 3, 4.0);

    let svd = MathSVD::new(&a);
    assert!(svd.is_done(), "SVD of diagonal must succeed");

    // Singular values are the absolute diagonal entries (possibly reordered)
    let w = svd.singular_values();
    let mut vals: Vec<f64> = (1..=3).map(|j| w.get(j)).collect();
    vals.sort_by(|a: &f64, b: &f64| a.partial_cmp(b).unwrap());
    expect_near(vals[0], 2.0, CONFUSION, "diag sv[0]");
    expect_near(vals[1], 3.0, CONFUSION, "diag sv[1]");
    expect_near(vals[2], 4.0, CONFUSION, "diag sv[2]");

    // Solve: A·X = [2, 6, 12] → X = [1, 2, 3]
    let mut b = MathVector::new(1, 3);
    b.set(1, 2.0);
    b.set(2, 6.0);
    b.set(3, 12.0);
    let mut x = MathVector::new(1, 3);
    svd.solve(&b, &mut x, 1.0e-10);

    verify_ax_eq_b(&a, &x, &b, CONFUSION, "diagonal solve");
}

// ---------------------------------------------------------------------------
// Test 3 — well-conditioned 3×3
// ---------------------------------------------------------------------------

#[test]
fn well_conditioned_3x3() {
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 2.0); a.set(1, 2, 1.0); a.set(1, 3, 1.0);
    a.set(2, 1, 1.0); a.set(2, 2, 3.0); a.set(2, 3, 2.0);
    a.set(3, 1, 1.0); a.set(3, 2, 2.0); a.set(3, 3, 3.0);

    let svd = MathSVD::new(&a);
    assert!(svd.is_done(), "SVD of well-conditioned 3×3 must succeed");

    let mut b = MathVector::new(1, 3);
    b.set(1, 4.0);
    b.set(2, 6.0);
    b.set(3, 7.0);

    let mut x = MathVector::new(1, 3);
    svd.solve(&b, &mut x, 1.0e-10);

    verify_ax_eq_b(&a, &x, &b, CONFUSION, "well_conditioned_3x3 solve");
}

// ---------------------------------------------------------------------------
// Test 4 — rectangular 4×3 (over-determined, least squares)
// ---------------------------------------------------------------------------

#[test]
fn rectangular_4x3_least_squares() {
    // Build a consistent over-determined system from a known solution [1, 2, 3].
    let x_exact = [1.0_f64, 2.0, 3.0];
    let mut a = MathMatrix::new(1, 4, 1, 3);
    a.set(1, 1, 1.0); a.set(1, 2, 0.0); a.set(1, 3, 0.0);
    a.set(2, 1, 0.0); a.set(2, 2, 1.0); a.set(2, 3, 0.0);
    a.set(3, 1, 0.0); a.set(3, 2, 0.0); a.set(3, 3, 1.0);
    a.set(4, 1, 1.0); a.set(4, 2, 1.0); a.set(4, 3, 1.0);

    let mut b = MathVector::new(1, 4);
    for i in 1..=4_i32 {
        let mut val = 0.0_f64;
        for j in 1..=3_i32 {
            val += a.get(i, j) * x_exact[(j - 1) as usize];
        }
        b.set(i, val);
    }

    let svd = MathSVD::new(&a);
    assert!(svd.is_done(), "SVD of 4×3 must succeed");

    let mut x = MathVector::new(1, 3);
    svd.solve(&b, &mut x, 1.0e-10);

    for j in 1..=3 {
        expect_near(
            x.get(j),
            x_exact[(j - 1) as usize],
            CONFUSION,
            &format!("4×3 least-squares x[{j}]"),
        );
    }
}

// ---------------------------------------------------------------------------
// Test 5 — pseudo-inverse of a square matrix
// ---------------------------------------------------------------------------

#[test]
fn pseudo_inverse_square() {
    // For a full-rank square matrix A, A⁺ = A⁻¹.
    // A = [[1, 2], [3, 4]], det = -2, A⁻¹ = [[-2, 1], [1.5, -0.5]]
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 1.0); a.set(1, 2, 2.0);
    a.set(2, 1, 3.0); a.set(2, 2, 4.0);

    let svd = MathSVD::new(&a);
    assert!(svd.is_done(), "SVD of 2×2 must succeed");

    let mut inv = MathMatrix::new(1, 2, 1, 2);
    svd.pseudo_inverse(&mut inv, 1.0e-10);

    expect_near(inv.get(1, 1), -2.0, CONFUSION, "pinv(1,1)");
    expect_near(inv.get(1, 2), 1.0, CONFUSION, "pinv(1,2)");
    expect_near(inv.get(2, 1), 1.5, CONFUSION, "pinv(2,1)");
    expect_near(inv.get(2, 2), -0.5, CONFUSION, "pinv(2,2)");

    // Verify A · A⁺ ≈ I
    for i in 1..=2 {
        for j in 1..=2 {
            let mut s = 0.0_f64;
            for k in 1..=2 {
                s += a.get(i, k) * inv.get(k, j);
            }
            let expected = if i == j { 1.0 } else { 0.0 };
            expect_near(s, expected, CONFUSION, &format!("A·A⁺=I ({i},{j})"));
        }
    }
}

// ---------------------------------------------------------------------------
// Test 6 — pseudo-inverse of a rectangular matrix (4×2)
// ---------------------------------------------------------------------------

#[test]
fn pseudo_inverse_rectangular() {
    // A = [[1, 0], [0, 1], [1, 0], [0, 1]]  (4×2, rank 2)
    let mut a = MathMatrix::new(1, 4, 1, 2);
    a.set(1, 1, 1.0); a.set(1, 2, 0.0);
    a.set(2, 1, 0.0); a.set(2, 2, 1.0);
    a.set(3, 1, 1.0); a.set(3, 2, 0.0);
    a.set(4, 1, 0.0); a.set(4, 2, 1.0);

    let svd = MathSVD::new(&a);
    assert!(svd.is_done(), "SVD of 4×2 must succeed");

    // A⁺ should be 2×4
    let mut inv = MathMatrix::new(1, 2, 1, 4);
    svd.pseudo_inverse(&mut inv, 1.0e-10);

    // A⁺·A should be I₂ (the 2×2 identity)
    for i in 1..=2 {
        for j in 1..=2 {
            let mut s = 0.0_f64;
            for k in 1..=4 {
                s += inv.get(i, k) * a.get(k, j);
            }
            let expected = if i == j { 1.0 } else { 0.0 };
            expect_near(s, expected, CONFUSION, &format!("A⁺·A=I₂ ({i},{j})"));
        }
    }

    // A·A⁺ should be the projector onto the range of A
    // For this particular A, A·A⁺ = 0.5*[[1,0,1,0],[0,1,0,1],[1,0,1,0],[0,1,0,1]]
    let expected_aainv = |r: i32, c: i32| -> f64 {
        // diagonal + off-diagonal blocks
        if (r == 1 || r == 3) && (c == 1 || c == 3) { 0.5 }
        else if (r == 2 || r == 4) && (c == 2 || c == 4) { 0.5 }
        else { 0.0 }
    };
    for i in 1..=4 {
        for j in 1..=4 {
            let mut s = 0.0_f64;
            for k in 1..=2 {
                s += a.get(i, k) * inv.get(k, j);
            }
            expect_near(s, expected_aainv(i, j), CONFUSION,
                &format!("A·A⁺ projector ({i},{j})"));
        }
    }
}

// ---------------------------------------------------------------------------
// Test 7 — SVD U·W·Vᵀ recovers A
// ---------------------------------------------------------------------------

#[test]
fn uvwt_recovers_a() {
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 4.0); a.set(1, 2, 1.0); a.set(1, 3, 2.0);
    a.set(2, 1, 3.0); a.set(2, 2, 5.0); a.set(2, 3, 1.0);
    a.set(3, 1, 1.0); a.set(3, 2, 2.0); a.set(3, 3, 3.0);

    let svd = MathSVD::new(&a);
    assert!(svd.is_done(), "SVD must succeed");

    let u = svd.u_matrix();
    let w = svd.singular_values();
    let v = svd.v_matrix();

    // Reconstruct A_rec = U · diag(W) · Vᵀ  and compare with A
    for i in 1..=3 {
        for j in 1..=3 {
            let mut s = 0.0_f64;
            for k in 1..=3 {
                s += u.get(i, k) * w.get(k) * v.get(j, k);
            }
            expect_near(s, a.get(i, j), CONFUSION,
                &format!("U·W·Vᵀ = A  ({i},{j})"));
        }
    }
}

// ---------------------------------------------------------------------------
// Test 8 — singular (rank-deficient) matrix, is_done should still succeed
// ---------------------------------------------------------------------------

#[test]
fn rank_deficient_matrix() {
    // Rank-1 matrix: A = [1, 2; 2, 4; 3, 6]  (col 2 = 2·col 1)
    let mut a = MathMatrix::new(1, 3, 1, 2);
    a.set(1, 1, 1.0); a.set(1, 2, 2.0);
    a.set(2, 1, 2.0); a.set(2, 2, 4.0);
    a.set(3, 1, 3.0); a.set(3, 2, 6.0);

    let svd = MathSVD::new(&a);
    assert!(svd.is_done(), "SVD of rank-deficient matrix should succeed");

    let w = svd.singular_values();
    // One singular value should be near zero
    let mut vals: Vec<f64> = (1..=2).map(|j| w.get(j)).collect();
    vals.sort_by(|a: &f64, b: &f64| a.partial_cmp(b).unwrap());
    // The smallest singular value should be ~0
    assert!(
        vals[0].abs() < CONFUSION,
        "smallest sv of rank-1 matrix should be ~0, got {}",
        vals[0]
    );
    // The largest singular value: Frobenius = sqrt(1+4+4+16+9+36) = sqrt(70)
    // ||A||_F = sqrt(70) = sv_max (for rank-1: ||A||_F = sv_max)
    let expected_sv_max = 70.0_f64.sqrt();
    expect_near(vals[1], expected_sv_max, 1.0e-10,
        "largest sv of rank-1 matrix");
}

// ---------------------------------------------------------------------------
// Test 9 — solve with non-trivial RHS, 4×3 over-determined
// ---------------------------------------------------------------------------

#[test]
fn solve_overdetermined_least_squares() {
    // A is 4×3 Vandermonde-like; exact solution is [1, -1, 2].
    let x_exact = [1.0_f64, -1.0, 2.0];
    let mut a = MathMatrix::new(1, 4, 1, 3);
    a.set(1, 1, 1.0); a.set(1, 2, 1.0); a.set(1, 3, 1.0);
    a.set(2, 1, 1.0); a.set(2, 2, 2.0); a.set(2, 3, 4.0);
    a.set(3, 1, 1.0); a.set(3, 2, 3.0); a.set(3, 3, 9.0);
    a.set(4, 1, 1.0); a.set(4, 2, 4.0); a.set(4, 3,16.0);

    // Build B = A · x_exact (consistent RHS)
    let mut b = MathVector::new(1, 4);
    for i in 1..=4_i32 {
        let mut val = 0.0_f64;
        for j in 1..=3_i32 {
            val += a.get(i, j) * x_exact[(j - 1) as usize];
        }
        b.set(i, val);
    }

    let svd = MathSVD::new(&a);
    assert!(svd.is_done(), "SVD of 4×3 Vandermonde must succeed");

    let mut x = MathVector::new(1, 3);
    svd.solve(&b, &mut x, 1.0e-10);

    for j in 1..=3 {
        expect_near(
            x.get(j),
            x_exact[(j - 1) as usize],
            CONFUSION,
            &format!("Vandermonde solve x[{j}]"),
        );
    }
}

// ---------------------------------------------------------------------------
// Test 10 — U is column-orthogonal: Uᵀ·U = I_n
// ---------------------------------------------------------------------------

#[test]
fn u_column_orthogonal() {
    let mut a = MathMatrix::new(1, 4, 1, 3);
    a.set(1, 1, 1.0); a.set(1, 2, 2.0); a.set(1, 3, 3.0);
    a.set(2, 1, 4.0); a.set(2, 2, 5.0); a.set(2, 3, 6.0);
    a.set(3, 1, 7.0); a.set(3, 2, 8.0); a.set(3, 3, 9.5);
    a.set(4, 1, 0.5); a.set(4, 2, 1.5); a.set(4, 3, 2.5);

    let svd = MathSVD::new(&a);
    assert!(svd.is_done(), "SVD of 4×3 must succeed");

    let u = svd.u_matrix();
    // Uᵀ · U = I_3
    for i in 1..=3 {
        for j in 1..=3 {
            let mut s = 0.0_f64;
            for k in 1..=4 {
                s += u.get(k, i) * u.get(k, j);
            }
            let expected = if i == j { 1.0 } else { 0.0 };
            expect_near(s, expected, CONFUSION,
                &format!("Uᵀ·U = I ({i},{j})"));
        }
    }
}

// ---------------------------------------------------------------------------
// Test 11 — V is orthogonal: Vᵀ·V = V·Vᵀ = I_n
// ---------------------------------------------------------------------------

#[test]
fn v_orthogonal() {
    let mut a = MathMatrix::new(1, 3, 1, 3);
    a.set(1, 1, 3.0); a.set(1, 2, 1.0); a.set(1, 3, 0.0);
    a.set(2, 1, 1.0); a.set(2, 2, 3.0); a.set(2, 3, 1.0);
    a.set(3, 1, 0.0); a.set(3, 2, 1.0); a.set(3, 3, 3.0);

    let svd = MathSVD::new(&a);
    assert!(svd.is_done(), "SVD of symmetric 3×3 must succeed");

    let v = svd.v_matrix();
    // Vᵀ · V = I
    for i in 1..=3 {
        for j in 1..=3 {
            let mut s = 0.0_f64;
            for k in 1..=3 {
                s += v.get(k, i) * v.get(k, j);
            }
            let expected = if i == j { 1.0 } else { 0.0 };
            expect_near(s, expected, ANGULAR, &format!("Vᵀ·V = I ({i},{j})"));
        }
    }
    // V · Vᵀ = I
    for i in 1..=3 {
        for j in 1..=3 {
            let mut s = 0.0_f64;
            for k in 1..=3 {
                s += v.get(i, k) * v.get(j, k);
            }
            let expected = if i == j { 1.0 } else { 0.0 };
            expect_near(s, expected, ANGULAR, &format!("V·Vᵀ = I ({i},{j})"));
        }
    }
}

// ---------------------------------------------------------------------------
// Test 12 — dump / Display
// ---------------------------------------------------------------------------

#[test]
fn dump_displays_done() {
    let mut a = MathMatrix::new(1, 2, 1, 2);
    a.set(1, 1, 1.0);
    a.set(2, 2, 1.0);

    let svd = MathSVD::new(&a);
    assert!(svd.is_done());

    let s = svd.dump();
    assert!(s.contains("Done"), "dump should contain 'Done', got: {s}");

    let display = format!("{svd}");
    assert!(display.contains("Done"), "Display should contain 'Done'");
}
