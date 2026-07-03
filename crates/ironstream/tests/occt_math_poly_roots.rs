//! Ported OCCT GTest for `math_DirectPolynomialRoots`.
//!
//! Source: `src/FoundationClasses/TKMath/GTests/math_DirectPolynomialRoots_Test.cxx`
//! (OPEN CASCADE SAS, 2025). Each OCCT `TEST_F` becomes a Rust `#[test]` with
//! the SAME numeric inputs, expected values and tolerances. `EXPECT_NEAR(a, b,
//! tol)` maps to `assert_near(a, b, tol)`; `EXPECT_EQ`/`EXPECT_TRUE`/
//! `EXPECT_FALSE` map to `assert_eq!`/`assert!`.
//!
//! Default fixture tolerance (`SetUp` sets `myTolerance = 1.0e-10`).

use ironstream::math_poly_roots::MathDirectPolynomialRoots;

const MY_TOLERANCE: f64 = 1.0e-10;

/// `EXPECT_NEAR(actual, expected, tol)`.
fn assert_near(actual: f64, expected: f64, tol: f64, msg: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{msg}: |{actual} - {expected}| = {} > {tol}",
        (actual - expected).abs()
    );
}

/// Verify polynomial evaluation at a root gives a near-zero result.
///
/// Mirrors the fixture helper `verifyRoot`: coefficients are in descending
/// degree order (`a0*x^n + a1*x^(n-1) + ... + an*x^0`).
fn verify_root(root: f64, coeffs: &[f64], solution_index: usize, tolerance: f64) {
    let mut result = 0.0_f64;
    let mut power = 1.0_f64;

    for &c in coeffs.iter().rev() {
        result += c * power;
        power *= root;
    }

    assert!(
        result.abs() <= tolerance,
        "Root {solution_index} should satisfy polynomial equation: value = {result}, tol = {tolerance}"
    );
}

#[test]
fn quadratic_roots() {
    // x^2 - 5x + 6 = 0, roots should be 2 and 3.
    let roots = MathDirectPolynomialRoots::quadratic(1.0, -5.0, 6.0);

    assert!(roots.is_done(), "Quadratic root finding should succeed");
    assert_eq!(roots.nb_solutions(), 2, "Quadratic should have 2 roots");

    let root1 = roots.value(1);
    let root2 = roots.value(2);

    assert_near(root1, 3.0, MY_TOLERANCE, "First root should be 3");
    assert_near(root2, 2.0, MY_TOLERANCE, "Second root should be 2");

    verify_root(root1, &[1.0, -5.0, 6.0], 1, 1.0e-10);
    verify_root(root2, &[1.0, -5.0, 6.0], 2, 1.0e-10);
}

#[test]
fn quadratic_no_real_roots() {
    // x^2 + x + 1 = 0, no real roots (discriminant < 0).
    let roots = MathDirectPolynomialRoots::quadratic(1.0, 1.0, 1.0);

    assert!(roots.is_done(), "Should complete even with no real roots");
    assert_eq!(roots.nb_solutions(), 0, "Should have no real roots");
}

#[test]
fn quadratic_double_root() {
    // (x-1)^2 = x^2 - 2x + 1 = 0, double root at x = 1.
    let roots = MathDirectPolynomialRoots::quadratic(1.0, -2.0, 1.0);

    assert!(roots.is_done(), "Double root case should succeed");
    assert_eq!(
        roots.nb_solutions(),
        2,
        "Double root is reported as 2 solutions in the current implementation"
    );

    let root1 = roots.value(1);
    let root2 = roots.value(2);
    assert_near(root1, 1.0, MY_TOLERANCE, "First double root should be 1.0");
    assert_near(root2, 1.0, MY_TOLERANCE, "Second double root should be 1.0");

    verify_root(root1, &[1.0, -2.0, 1.0], 1, 1.0e-10);
    verify_root(root2, &[1.0, -2.0, 1.0], 2, 1.0e-10);
}

#[test]
fn cubic_roots() {
    // (x-1)(x-2)(x-3) = x^3 - 6x^2 + 11x - 6 = 0, roots 1, 2, 3.
    let roots = MathDirectPolynomialRoots::cubic(1.0, -6.0, 11.0, -6.0);

    assert!(roots.is_done(), "Cubic root finding should succeed");
    assert_eq!(roots.nb_solutions(), 3, "Cubic should have 3 real roots");

    // Descending order: 3, 2, 1.
    let root1 = roots.value(1);
    let root2 = roots.value(2);
    let root3 = roots.value(3);

    assert_near(root1, 3.0, MY_TOLERANCE, "First root should be 3");
    assert_near(root2, 2.0, MY_TOLERANCE, "Second root should be 2");
    assert_near(root3, 1.0, MY_TOLERANCE, "Third root should be 1");

    verify_root(root1, &[1.0, -6.0, 11.0, -6.0], 1, 1.0e-10);
    verify_root(root2, &[1.0, -6.0, 11.0, -6.0], 2, 1.0e-10);
    verify_root(root3, &[1.0, -6.0, 11.0, -6.0], 3, 1.0e-10);
}

#[test]
fn cubic_one_real_root() {
    // x^3 + x + 1 = 0, one real root.
    let roots = MathDirectPolynomialRoots::cubic(1.0, 0.0, 1.0, 1.0);

    assert!(roots.is_done(), "Cubic with one real root should succeed");
    assert_eq!(roots.nb_solutions(), 1, "Should have exactly one real root");

    let root = roots.value(1);
    verify_root(root, &[1.0, 0.0, 1.0, 1.0], 1, 1.0e-10);
}

#[test]
fn quartic_roots() {
    // (x-1)(x-2)(x+1)(x+2) = x^4 - 5x^2 + 4 = 0, roots -2, -1, 1, 2.
    let roots = MathDirectPolynomialRoots::quartic(1.0, 0.0, -5.0, 0.0, 4.0);

    assert!(roots.is_done(), "Quartic root finding should succeed");
    assert_eq!(roots.nb_solutions(), 4, "Quartic should have 4 real roots");

    // Natural algorithm order from Ferrari's method: -2, -1, 2, 1.
    let root1 = roots.value(1);
    let root2 = roots.value(2);
    let root3 = roots.value(3);
    let root4 = roots.value(4);

    assert_near(root1, -2.0, MY_TOLERANCE, "First root should be -2");
    assert_near(root2, -1.0, MY_TOLERANCE, "Second root should be -1");
    assert_near(root3, 2.0, MY_TOLERANCE, "Third root should be 2");
    assert_near(root4, 1.0, MY_TOLERANCE, "Fourth root should be 1");

    let coeffs = [1.0, 0.0, -5.0, 0.0, 4.0];
    verify_root(root1, &coeffs, 1, 1.0e-10);
    verify_root(root2, &coeffs, 2, 1.0e-10);
    verify_root(root3, &coeffs, 3, 1.0e-10);
    verify_root(root4, &coeffs, 4, 1.0e-10);
}

#[test]
fn linear_case() {
    // 2x - 6 = 0, root x = 3.
    let roots = MathDirectPolynomialRoots::linear(2.0, -6.0);

    assert!(roots.is_done(), "Linear root finding should succeed");
    assert_eq!(roots.nb_solutions(), 1, "Linear should have 1 root");

    let root = roots.value(1);
    assert_near(root, 3.0, MY_TOLERANCE, "Linear root value");

    verify_root(root, &[2.0, -6.0], 1, 1.0e-10);
}

#[test]
fn degenerate_linear_case() {
    // 0x + 5 = 0 (no solution).
    let roots = MathDirectPolynomialRoots::linear(0.0, 5.0);

    assert!(roots.is_done(), "Degenerate linear case should complete");
    assert_eq!(
        roots.nb_solutions(),
        0,
        "0x + 5 = 0 should have no solutions"
    );
}

#[test]
fn polynomial_evaluation() {
    // x^2 - 3x + 2 = 0, roots 1, 2.
    let roots = MathDirectPolynomialRoots::quadratic(1.0, -3.0, 2.0);

    assert!(roots.is_done(), "Should find roots successfully");
    assert_eq!(roots.nb_solutions(), 2, "Should have 2 roots");

    for i in 1..=roots.nb_solutions() {
        let root = roots.value(i);
        verify_root(root, &[1.0, -3.0, 2.0], i, 1.0e-10);
    }
}

#[test]
fn near_zero_coefficients() {
    // Very small leading coefficient (effectively linear: x - 2 = 0).
    let roots = MathDirectPolynomialRoots::quadratic(1.0e-15, 1.0, -2.0);

    assert!(
        roots.is_done(),
        "Should handle near-zero leading coefficient"
    );
    assert_eq!(
        roots.nb_solutions(),
        2,
        "Should find 2 roots for this quadratic"
    );

    let root1 = roots.value(1);
    let root2 = roots.value(2);

    // One root should be near 2 (meaningful solution from linear approximation).
    let found_near_two = (root1 - 2.0).abs() < 1.0e-6 || (root2 - 2.0).abs() < 1.0e-6;
    assert!(found_near_two, "Should find one root near x = 2");
}

#[test]
fn bi_quadratic_polynomial() {
    // x^4 - 10x^2 + 9 = (x^2-1)(x^2-9) = 0, roots -3, -1, 1, 3.
    let roots = MathDirectPolynomialRoots::quartic(1.0, 0.0, -10.0, 0.0, 9.0);

    assert!(roots.is_done(), "Should solve biquadratic polynomial");
    assert_eq!(roots.nb_solutions(), 4, "Should find 4 real roots");

    // Natural algorithm order for biquadratic: -3, -1, 3, 1.
    let root1 = roots.value(1);
    let root2 = roots.value(2);
    let root3 = roots.value(3);
    let root4 = roots.value(4);

    assert_near(root1, -3.0, 1.0e-8, "First root should be -3");
    assert_near(root2, -1.0, 1.0e-8, "Second root should be -1");
    assert_near(root3, 3.0, 1.0e-8, "Third root should be 3");
    assert_near(root4, 1.0, 1.0e-8, "Fourth root should be 1");

    let coeffs = [1.0, 0.0, -10.0, 0.0, 9.0];
    verify_root(root1, &coeffs, 1, 1.0e-8);
    verify_root(root2, &coeffs, 2, 1.0e-8);
    verify_root(root3, &coeffs, 3, 1.0e-8);
    verify_root(root4, &coeffs, 4, 1.0e-8);
}

#[test]
fn repeated_roots() {
    // (x-1)^3 = x^3 - 3x^2 + 3x - 1 = 0.
    let roots = MathDirectPolynomialRoots::cubic(1.0, -3.0, 3.0, -1.0);

    assert!(roots.is_done(), "Should handle repeated roots");
    assert_eq!(
        roots.nb_solutions(),
        3,
        "Triple root is reported as 3 solutions in the current implementation"
    );

    let root1 = roots.value(1);
    let root2 = roots.value(2);
    let root3 = roots.value(3);
    assert_near(root1, 1.0, 1.0e-8, "First triple root should be 1.0");
    assert_near(root2, 1.0, 1.0e-8, "Second triple root should be 1.0");
    assert_near(root3, 1.0, 1.0e-8, "Third triple root should be 1.0");

    verify_root(root1, &[1.0, -3.0, 3.0, -1.0], 1, 1.0e-8);
    verify_root(root2, &[1.0, -3.0, 3.0, -1.0], 2, 1.0e-8);
    verify_root(root3, &[1.0, -3.0, 3.0, -1.0], 3, 1.0e-8);
}

#[test]
fn problematic_quartic_case_detailed() {
    // The specific problematic case that motivated the implementation rework.
    let a = 1.0000000000000004;
    let b = -2.2737367544323211e-13;
    let c = -17361733.368892364;
    let d = 28.998342463569099;
    let e = 75357446168894.516;

    let roots = MathDirectPolynomialRoots::quartic(a, b, c, d, e);

    assert!(
        roots.is_done(),
        "Problematic quartic should be solved successfully"
    );
    assert_eq!(roots.nb_solutions(), 4, "Should find all 4 real roots");
    assert!(!roots.infinite_roots(), "Should not report infinite roots");

    let actual_roots = [
        roots.value(1),
        roots.value(2),
        roots.value(3),
        roots.value(4),
    ];

    assert_near(actual_roots[0], -2946.425490, 1.0e-3, "First root");
    assert_near(actual_roots[1], -2946.236617, 1.0e-3, "Second root");
    assert_near(actual_roots[2], 2946.394276, 1.0e-3, "Third root");
    assert_near(actual_roots[3], 2946.267830, 1.0e-3, "Fourth root");

    // Verify residuals are extremely small (main goal of the rework).
    for i in 1..=4 {
        let root = roots.value(i);
        let residual =
            a * root * root * root * root + b * root * root * root + c * root * root + d * root + e;
        assert!(
            residual.abs() < 1.0,
            "Root {i} residual should be much smaller than previous ~85000: {residual}"
        );
    }
}

#[test]
fn modern_implementation_biquadratic_detection() {
    // x^4 - 5x^2 + 4 = 0 => (x^2-1)(x^2-4) = 0, roots +/-1, +/-2.
    let roots = MathDirectPolynomialRoots::quartic(1.0, 0.0, -5.0, 0.0, 4.0);

    assert!(roots.is_done(), "Biquadratic should be solved");
    assert_eq!(roots.nb_solutions(), 4, "Should find 4 real roots");

    // Natural algorithm order for biquadratic: -2, -1, 2, 1.
    let root1 = roots.value(1);
    let root2 = roots.value(2);
    let root3 = roots.value(3);
    let root4 = roots.value(4);

    assert_near(root1, -2.0, 1.0e-12, "First root should be -2");
    assert_near(root2, -1.0, 1.0e-12, "Second root should be -1");
    assert_near(root3, 2.0, 1.0e-12, "Third root should be 2");
    assert_near(root4, 1.0, 1.0e-12, "Fourth root should be 1");
}

#[test]
fn full_ferrari_method_required() {
    // x^4 + x^3 - 4x^2 - 4x = 0 => x(x+2)(x-2)(x+1), roots 0, -2, 2, -1.
    let roots = MathDirectPolynomialRoots::quartic(1.0, 1.0, -4.0, -4.0, 0.0);

    assert!(roots.is_done(), "Full Ferrari method should work");
    assert_eq!(roots.nb_solutions(), 4, "Should find 4 real roots");

    // Current implementation order for Ferrari method: -2, -1, 2, 0.
    let root1 = roots.value(1);
    let root2 = roots.value(2);
    let root3 = roots.value(3);
    let root4 = roots.value(4);

    assert_near(root1, -2.0, 1.0e-10, "First root should be -2");
    assert_near(root2, -1.0, 1.0e-10, "Second root should be -1");
    assert_near(root3, 2.0, 1.0e-10, "Third root should be 2");
    assert_near(root4, 0.0, 1.0e-10, "Fourth root should be 0");
}

#[test]
fn aggressive_refinement_effectiveness() {
    // Large coefficients to test numerical stability.
    let a = 1.0;
    let b = 0.0;
    let c = -1000000.0;
    let d = 0.0;
    let e = 999999.0;

    let roots = MathDirectPolynomialRoots::quartic(a, b, c, d, e);

    assert!(roots.is_done(), "Should handle large coefficients");

    for i in 1..=roots.nb_solutions() {
        let root = roots.value(i);
        let residual =
            a * root * root * root * root + b * root * root * root + c * root * root + d * root + e;
        assert!(
            residual.abs() < 1.0e-3,
            "Aggressive refinement should achieve good residuals even with large coefficients: {residual}"
        );
    }
}

#[test]
fn cubic_with_aggressive_refinement() {
    // x^3 - 6x^2 + 11x - 6 = 0 => (x-1)(x-2)(x-3), roots 1, 2, 3.
    let roots = MathDirectPolynomialRoots::cubic(1.0, -6.0, 11.0, -6.0);

    assert!(
        roots.is_done(),
        "Cubic with aggressive refinement should work"
    );
    assert_eq!(roots.nb_solutions(), 3, "Should find 3 real roots");

    // Descending order: 3, 2, 1.
    let root1 = roots.value(1);
    let root2 = roots.value(2);
    let root3 = roots.value(3);

    assert_near(root1, 3.0, 1.0e-12, "First root should be 3");
    assert_near(root2, 2.0, 1.0e-12, "Second root should be 2");
    assert_near(root3, 1.0, 1.0e-12, "Third root should be 1");

    for i in 1..=3 {
        let root = roots.value(i);
        let residual = root * root * root - 6.0 * root * root + 11.0 * root - 6.0;
        assert!(
            residual.abs() < 1.0e-12,
            "Cubic residual should be excellent: {residual}"
        );
    }
}

#[test]
fn extreme_coefficients_stability() {
    // Extreme coefficient ranges.
    let a = 1.0;
    let b = 1.0e-15;
    let c = -1.0e8;
    let d = 1.0e-10;
    let e = 1.0e15;

    let roots = MathDirectPolynomialRoots::quartic(a, b, c, d, e);

    assert!(roots.is_done(), "Should handle extreme coefficient ranges");

    for i in 1..=roots.nb_solutions() {
        let root = roots.value(i);
        let residual =
            a * root * root * root * root + b * root * root * root + c * root * root + d * root + e;
        assert!(
            residual.abs() < 1.0e6,
            "Even with extreme coefficients, residuals should be controlled: {residual}"
        );
    }
}

#[test]
fn close_to_zero_coefficients() {
    // x^4 + eps*x^3 + eps*x^2 + eps*x - 16 = 0, essentially x^4 = 16, real roots +/-2.
    let epsilon = 1.0e-14;
    let roots = MathDirectPolynomialRoots::quartic(1.0, epsilon, epsilon, epsilon, -16.0);

    assert!(roots.is_done(), "Should handle near-zero coefficients");
    assert_eq!(roots.nb_solutions(), 2, "Should find 2 real roots");

    // Natural algorithm order for near-zero coefficients: 2, -2.
    let root1 = roots.value(1);
    let root2 = roots.value(2);

    assert_near(root1, 2.0, 1.0e-10, "First root should be 2");
    assert_near(root2, -2.0, 1.0e-10, "Second root should be -2");
}
