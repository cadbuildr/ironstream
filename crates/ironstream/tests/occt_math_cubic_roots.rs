// FILE: rust/ironstream/crates/ironstream/tests/occt_math_cubic_roots.rs

//! Tests for `math_cubic_roots` — cubic / quartic root solvers and the
//! shared `solve_quadratic` helper.
//!
//! Mirrors the style of `occt_math_poly_roots.rs` in this crate.

extern crate ironstream;
use ironstream::math_cubic_roots::*;

const TOL: f64 = 1.0e-10;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn assert_near(actual: f64, expected: f64, tol: f64, msg: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{msg}: |{actual} - {expected}| = {} > {tol}",
        (actual - expected).abs()
    );
}

/// Evaluate polynomial (coefficients in descending degree order) at `x`.
fn poly_eval(coeffs: &[f64], x: f64) -> f64 {
    let mut acc = 0.0;
    let mut power = 1.0_f64;
    for &c in coeffs.iter().rev() {
        acc += c * power;
        power *= x;
    }
    acc
}

/// Assert that `x` is a root of the polynomial (residual ≤ `tol`).
fn assert_root(x: f64, coeffs: &[f64], label: &str, tol: f64) {
    let residual = poly_eval(coeffs, x);
    assert!(
        residual.abs() <= tol,
        "{label}: poly({x}) = {residual}, expected |residual| <= {tol}"
    );
}

// ---------------------------------------------------------------------------
// solve_quadratic
// ---------------------------------------------------------------------------

#[test]
fn quadratic_two_roots() {
    // x² - 5x + 6 = (x-2)(x-3), roots 2 and 3.
    let roots = solve_quadratic(1.0, -5.0, 6.0);
    assert_eq!(roots.len(), 2, "should find 2 roots");
    let mut sorted = roots.clone();
    sorted.sort_by(f64::total_cmp);
    assert_near(sorted[0], 2.0, TOL, "smaller root");
    assert_near(sorted[1], 3.0, TOL, "larger root");
    for &r in &roots {
        assert_root(r, &[1.0, -5.0, 6.0], "quadratic residual", 1.0e-12);
    }
}

#[test]
fn quadratic_double_root() {
    // (x-1)² = x² - 2x + 1 = 0, double root at 1.
    let roots = solve_quadratic(1.0, -2.0, 1.0);
    assert_eq!(roots.len(), 2, "double root reported as 2 entries");
    for &r in &roots {
        assert_near(r, 1.0, TOL, "double root");
    }
}

#[test]
fn quadratic_no_real_roots() {
    // x² + x + 1 = 0 — discriminant < 0.
    let roots = solve_quadratic(1.0, 1.0, 1.0);
    assert_eq!(roots.len(), 0, "no real roots expected");
}

#[test]
fn quadratic_degenerate_to_linear() {
    // 0·x² + 2x - 4 = 0, root x = 2.
    let roots = solve_quadratic(0.0, 2.0, -4.0);
    assert_eq!(roots.len(), 1, "linear case: one root");
    assert_near(roots[0], 2.0, TOL, "linear root");
}

#[test]
fn quadratic_zero_polynomial() {
    // 0·x² + 0·x + 5 = 0 — no solution.
    let roots = solve_quadratic(0.0, 0.0, 5.0);
    assert_eq!(roots.len(), 0, "constant non-zero: no roots");
}

// ---------------------------------------------------------------------------
// MathCubicRoots
// ---------------------------------------------------------------------------

#[test]
fn cubic_three_roots_1_2_3() {
    // x³ - 6x² + 11x - 6 = (x-1)(x-2)(x-3), roots 1, 2, 3.
    let mut solver = MathCubicRoots::new();
    solver.perform(1.0, -6.0, 11.0, -6.0);

    assert!(solver.is_done(), "solver should complete");
    assert_eq!(solver.nb_roots(), 3, "should find 3 real roots");

    let mut roots: Vec<f64> = (0..3).map(|i| solver.root(i)).collect();
    roots.sort_by(f64::total_cmp);

    assert_near(roots[0], 1.0, TOL, "root 1");
    assert_near(roots[1], 2.0, TOL, "root 2");
    assert_near(roots[2], 3.0, TOL, "root 3");

    for &r in &roots {
        assert_root(r, &[1.0, -6.0, 11.0, -6.0], "cubic 1-2-3 residual", 1.0e-10);
    }
}

#[test]
fn cubic_one_real_root() {
    // x³ + x + 1 = 0 — one real root near -0.6824.
    let mut solver = MathCubicRoots::new();
    solver.perform(1.0, 0.0, 1.0, 1.0);

    assert!(solver.is_done(), "solver should complete");
    assert_eq!(solver.nb_roots(), 1, "one real root expected");

    let r = solver.root(0);
    assert_root(r, &[1.0, 0.0, 1.0, 1.0], "x^3+x+1 residual", 1.0e-10);
    assert_near(r, -0.6823278038280193, 1.0e-8, "known root value");
}

#[test]
fn cubic_triple_root() {
    // (x-2)³ = x³ - 6x² + 12x - 8 = 0, triple root at 2.
    let mut solver = MathCubicRoots::new();
    solver.perform(1.0, -6.0, 12.0, -8.0);

    assert!(solver.is_done(), "solver should complete");
    assert!(solver.nb_roots() >= 1 && solver.nb_roots() <= 3, "1 or 3 roots for triple root");

    for i in 0..solver.nb_roots() {
        assert_near(solver.root(i), 2.0, 1.0e-7, "triple root at 2");
    }
}

#[test]
fn cubic_depressed_x3_minus_x() {
    // x³ - x = x(x-1)(x+1), roots -1, 0, 1.
    let mut solver = MathCubicRoots::new();
    solver.perform(1.0, 0.0, -1.0, 0.0);

    assert!(solver.is_done(), "solver should complete");
    assert_eq!(solver.nb_roots(), 3, "three distinct roots");

    let mut roots: Vec<f64> = (0..3).map(|i| solver.root(i)).collect();
    roots.sort_by(f64::total_cmp);

    assert_near(roots[0], -1.0, TOL, "root -1");
    assert_near(roots[1], 0.0, TOL, "root 0");
    assert_near(roots[2], 1.0, TOL, "root 1");
}

#[test]
fn cubic_negative_roots() {
    // (x+1)(x+2)(x+3) = x³ + 6x² + 11x + 6 = 0, roots -1, -2, -3.
    let mut solver = MathCubicRoots::new();
    solver.perform(1.0, 6.0, 11.0, 6.0);

    assert!(solver.is_done(), "solver should complete");
    assert_eq!(solver.nb_roots(), 3, "three roots");

    let mut roots: Vec<f64> = (0..3).map(|i| solver.root(i)).collect();
    roots.sort_by(f64::total_cmp);

    assert_near(roots[0], -3.0, TOL, "root -3");
    assert_near(roots[1], -2.0, TOL, "root -2");
    assert_near(roots[2], -1.0, TOL, "root -1");
}

#[test]
fn cubic_scaled_leading_coefficient() {
    // 2x³ - 12x² + 22x - 12 = 2(x³ - 6x² + 11x - 6), same roots 1, 2, 3.
    let mut solver = MathCubicRoots::new();
    solver.perform(2.0, -12.0, 22.0, -12.0);

    assert!(solver.is_done(), "solver should complete");
    assert_eq!(solver.nb_roots(), 3, "three roots");

    let mut roots: Vec<f64> = (0..3).map(|i| solver.root(i)).collect();
    roots.sort_by(f64::total_cmp);

    assert_near(roots[0], 1.0, TOL, "root 1");
    assert_near(roots[1], 2.0, TOL, "root 2");
    assert_near(roots[2], 3.0, TOL, "root 3");
}

#[test]
fn cubic_degenerate_to_quadratic() {
    // 0·x³ + x² - 5x + 6 = 0, falls back to quadratic, roots 2 and 3.
    let mut solver = MathCubicRoots::new();
    solver.perform(0.0, 1.0, -5.0, 6.0);

    assert!(solver.is_done(), "solver should complete");
    assert_eq!(solver.nb_roots(), 2, "two roots from quadratic fallback");

    let mut roots: Vec<f64> = (0..2).map(|i| solver.root(i)).collect();
    roots.sort_by(f64::total_cmp);

    assert_near(roots[0], 2.0, TOL, "root 2");
    assert_near(roots[1], 3.0, TOL, "root 3");
}

#[test]
fn cubic_residuals_are_small() {
    // x³ - 15x - 4 = 0, three real roots (casus irreducibilis); residuals checked.
    let mut solver = MathCubicRoots::new();
    solver.perform(1.0, 0.0, -15.0, -4.0);

    assert!(solver.is_done(), "solver should complete");
    assert_eq!(solver.nb_roots(), 3, "three real roots");

    for i in 0..solver.nb_roots() {
        let r = solver.root(i);
        assert_root(r, &[1.0, 0.0, -15.0, -4.0], "x^3-15x-4 residual", 1.0e-8);
    }
}

#[test]
#[should_panic]
fn cubic_root_before_perform_panics() {
    let solver = MathCubicRoots::new();
    let _ = solver.root(0);
}

#[test]
#[should_panic]
fn cubic_root_out_of_range_panics() {
    let mut solver = MathCubicRoots::new();
    solver.perform(1.0, 0.0, 1.0, 1.0); // one root
    let _ = solver.root(1);              // index 1 is out of range
}

// ---------------------------------------------------------------------------
// MathQuarticRoots
// ---------------------------------------------------------------------------

#[test]
fn quartic_stub_full_quartic_done_zero_roots() {
    // Full quartic — stub returns is_done=true, nb_roots=0.
    let mut solver = MathQuarticRoots::new();
    solver.perform(1.0, 0.0, -5.0, 0.0, 4.0);

    assert!(solver.is_done(), "stub should set is_done=true");
    assert_eq!(solver.nb_roots(), 0, "stub returns 0 roots for true quartic");
}

#[test]
fn quartic_reduces_to_cubic() {
    // 0·x⁴ + x³ - 6x² + 11x - 6 = 0, roots 1, 2, 3.
    let mut solver = MathQuarticRoots::new();
    solver.perform(0.0, 1.0, -6.0, 11.0, -6.0);

    assert!(solver.is_done(), "solver should complete");
    assert_eq!(solver.nb_roots(), 3, "cubic fallback: three roots");

    let mut roots: Vec<f64> = (0..3).map(|i| solver.root(i)).collect();
    roots.sort_by(f64::total_cmp);

    assert_near(roots[0], 1.0, TOL, "root 1");
    assert_near(roots[1], 2.0, TOL, "root 2");
    assert_near(roots[2], 3.0, TOL, "root 3");
}

#[test]
fn quartic_reduces_to_quadratic_via_cubic() {
    // 0·x⁴ + 0·x³ + x² - 5x + 6 = 0, roots 2 and 3.
    let mut solver = MathQuarticRoots::new();
    solver.perform(0.0, 0.0, 1.0, -5.0, 6.0);

    assert!(solver.is_done(), "solver should complete");
    assert_eq!(solver.nb_roots(), 2, "quadratic via cubic fallback");

    let mut roots: Vec<f64> = (0..2).map(|i| solver.root(i)).collect();
    roots.sort_by(f64::total_cmp);

    assert_near(roots[0], 2.0, TOL, "root 2");
    assert_near(roots[1], 3.0, TOL, "root 3");
}

#[test]
fn quartic_not_done_before_perform() {
    let solver = MathQuarticRoots::new();
    assert!(!solver.is_done(), "should not be done before perform()");
    assert_eq!(solver.nb_roots(), 0, "no roots before perform()");
}

#[test]
#[should_panic]
fn quartic_root_before_perform_panics() {
    let solver = MathQuarticRoots::new();
    let _ = solver.root(0);
}
