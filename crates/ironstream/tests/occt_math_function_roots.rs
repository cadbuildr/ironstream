//! Ported OCCT tests for `math_FunctionRoots`.
//!
//! Each `#[test]` corresponds to a scenario that exercises the
//! `ironstream::math_function_roots::FunctionRoots` API using polynomial
//! lambdas and other scalar functions. Tolerances follow the OCCT defaults:
//! `precision::CONFUSION = 1e-7`, `precision::ANGULAR = 1e-12`.

use ironstream::math_function_roots::{FunctionRoots, MathFunction};
use ironstream::precision;

// ---------------------------------------------------------------------------
// Concrete test functions (polynomial lambdas via impl MathFunction).
// ---------------------------------------------------------------------------

/// Wraps a `Fn(f64) -> f64` closure as a `MathFunction`.
struct Fn1<F: Fn(f64) -> f64>(F);
impl<F: Fn(f64) -> f64> MathFunction for Fn1<F> {
    fn value(&self, x: f64) -> Option<f64> {
        Some((self.0)(x))
    }
}

// ---------------------------------------------------------------------------
// Helper.
// ---------------------------------------------------------------------------

fn assert_near(actual: f64, expected: f64, tol: f64, msg: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{msg}: |{actual} - {expected}| = {} > {tol}",
        (actual - expected).abs()
    );
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Test 1: Linear function x - 2 = 0, one root at x = 2.
#[test]
fn linear_single_root() {
    let f = Fn1(|x: f64| x - 2.0);
    let roots = FunctionRoots::new(&f, 0.0, 5.0, 100, precision::CONFUSION, 1.0e-10);
    assert!(roots.is_done(), "linear: is_done");
    assert_eq!(roots.nb_solutions(), 1, "linear: nb_solutions");
    assert_near(roots.value(1), 2.0, 1.0e-6, "linear root at 2");
}

/// Test 2: Quadratic (x-1)(x-3) = x^2 - 4x + 3, two roots at 1 and 3.
#[test]
fn quadratic_two_roots() {
    let f = Fn1(|x: f64| x * x - 4.0 * x + 3.0);
    let roots = FunctionRoots::new(&f, 0.0, 5.0, 200, precision::CONFUSION, 1.0e-10);
    assert!(roots.is_done(), "quadratic: is_done");
    assert_eq!(roots.nb_solutions(), 2, "quadratic: nb_solutions");
    assert_near(roots.value(1), 1.0, 1.0e-6, "quadratic root 1 at 1");
    assert_near(roots.value(2), 3.0, 1.0e-6, "quadratic root 2 at 3");
}

/// Test 3: Cubic x(x-2)(x-4) = x^3 - 6x^2 + 8x, three roots at 0, 2, 4.
#[test]
fn cubic_three_roots() {
    let f = Fn1(|x: f64| x * (x - 2.0) * (x - 4.0));
    let roots = FunctionRoots::new(&f, -0.5, 5.0, 500, precision::CONFUSION, 1.0e-10);
    assert!(roots.is_done(), "cubic: is_done");
    assert_eq!(roots.nb_solutions(), 3, "cubic: nb_solutions");
    assert_near(roots.value(1), 0.0, 1.0e-6, "cubic root 1 at 0");
    assert_near(roots.value(2), 2.0, 1.0e-6, "cubic root 2 at 2");
    assert_near(roots.value(3), 4.0, 1.0e-6, "cubic root 3 at 4");
}

/// Test 4: No roots — f(x) = x^2 + 1 is always positive.
#[test]
fn no_roots_always_positive() {
    let f = Fn1(|x: f64| x * x + 1.0);
    let roots = FunctionRoots::new(&f, -10.0, 10.0, 200, precision::CONFUSION, 1.0e-10);
    assert!(roots.is_done(), "no-roots: is_done");
    assert_eq!(roots.nb_solutions(), 0, "no-roots: nb_solutions should be 0");
    assert!(!roots.is_all_zero(), "no-roots: is_all_zero should be false");
}

/// Test 5: IsDone is false only on construction failure (zero nb_sample).
/// With nb_sample = 0, the constructor must set is_done() = false.
#[test]
fn zero_sample_not_done() {
    let f = Fn1(|x: f64| x - 1.0);
    let roots = FunctionRoots::new(&f, 0.0, 2.0, 0, 1.0e-7, 1.0e-10);
    assert!(!roots.is_done(), "zero sample: is_done should be false");
}

/// Test 6: IsAllZero — identically zero function.
#[test]
fn all_zero_function() {
    // f(x) = 0 everywhere — is_all_zero() should be true.
    let f = Fn1(|_x: f64| 0.0);
    let roots = FunctionRoots::new(&f, 0.0, 1.0, 50, 1.0e-7, 1.0e-10);
    assert!(roots.is_done(), "all-zero: is_done");
    assert!(roots.is_all_zero(), "all-zero: is_all_zero");
}

/// Test 7: Root exactly at the left endpoint a.
#[test]
fn root_at_left_endpoint() {
    // f(x) = x, root at x = 0.
    let f = Fn1(|x: f64| x);
    let roots = FunctionRoots::new(&f, 0.0, 1.0, 100, 1.0e-7, 1.0e-10);
    assert!(roots.is_done(), "left-endpoint: is_done");
    // Must find the root at 0.
    assert!(roots.nb_solutions() >= 1, "left-endpoint: at least one root");
    assert_near(roots.value(1), 0.0, 1.0e-6, "left-endpoint root at 0");
}

/// Test 8: Root exactly at the right endpoint b.
#[test]
fn root_at_right_endpoint() {
    // f(x) = x - 5, root at x = 5.
    let f = Fn1(|x: f64| x - 5.0);
    let roots = FunctionRoots::new(&f, 0.0, 5.0, 100, 1.0e-7, 1.0e-10);
    assert!(roots.is_done(), "right-endpoint: is_done");
    assert!(roots.nb_solutions() >= 1, "right-endpoint: at least one root");
    // Last root should be at 5.
    let last = roots.value(roots.nb_solutions());
    assert_near(last, 5.0, 1.0e-6, "right-endpoint root at 5");
}

/// Test 9: Reversed interval [b, a] is normalised.
#[test]
fn reversed_interval() {
    // Same as quadratic test but with a > b.
    let f = Fn1(|x: f64| x * x - 4.0 * x + 3.0);
    let roots = FunctionRoots::new(&f, 5.0, 0.0, 200, precision::CONFUSION, 1.0e-10);
    assert!(roots.is_done(), "reversed: is_done");
    assert_eq!(roots.nb_solutions(), 2, "reversed: nb_solutions");
    assert_near(roots.value(1), 1.0, 1.0e-6, "reversed root 1 at 1");
    assert_near(roots.value(2), 3.0, 1.0e-6, "reversed root 2 at 3");
}

/// Test 10: StateNumber counts sign-change brackets.
#[test]
fn state_number_reflects_brackets() {
    // f(x) = (x-1)(x-3)(x-5) has 3 roots → at least 3 brackets.
    let f = Fn1(|x: f64| (x - 1.0) * (x - 3.0) * (x - 5.0));
    let roots = FunctionRoots::new(&f, 0.0, 6.0, 600, precision::CONFUSION, 1.0e-10);
    assert!(roots.is_done(), "state-number: is_done");
    assert_eq!(roots.nb_solutions(), 3, "state-number: nb_solutions");
    // state_number must be >= number of roots found.
    assert!(
        roots.state_number() >= 3,
        "state_number {} should be >= 3",
        roots.state_number()
    );
}

/// Test 11: Narrow interval containing exactly one root.
#[test]
fn narrow_interval_single_root() {
    // f(x) = x^2 - 4, root at x = 2 within [1.5, 2.5].
    let f = Fn1(|x: f64| x * x - 4.0);
    let roots = FunctionRoots::new(&f, 1.5, 2.5, 50, precision::CONFUSION, 1.0e-10);
    assert!(roots.is_done(), "narrow: is_done");
    assert_eq!(roots.nb_solutions(), 1, "narrow: nb_solutions");
    assert_near(roots.value(1), 2.0, 1.0e-6, "narrow root at 2");
}

/// Test 12: Quartic (x+2)(x+1)(x-1)(x-2) = x^4 - 5x^2 + 4, four roots.
#[test]
fn quartic_four_roots() {
    let f = Fn1(|x: f64| {
        let x2 = x * x;
        x2 * x2 - 5.0 * x2 + 4.0
    });
    let roots = FunctionRoots::new(&f, -3.0, 3.0, 600, precision::CONFUSION, 1.0e-10);
    assert!(roots.is_done(), "quartic: is_done");
    assert_eq!(roots.nb_solutions(), 4, "quartic: four roots");
    assert_near(roots.value(1), -2.0, 1.0e-6, "quartic root 1 at -2");
    assert_near(roots.value(2), -1.0, 1.0e-6, "quartic root 2 at -1");
    assert_near(roots.value(3), 1.0, 1.0e-6, "quartic root 3 at 1");
    assert_near(roots.value(4), 2.0, 1.0e-6, "quartic root 4 at 2");
}

/// Test 13: Value at each root is close to zero (residual check).
#[test]
fn residuals_near_zero() {
    // f(x) = x^3 - x = x(x-1)(x+1), roots at -1, 0, 1.
    let f = Fn1(|x: f64| x * x * x - x);
    let roots = FunctionRoots::new(&f, -2.0, 2.0, 400, precision::CONFUSION, 1.0e-10);
    assert!(roots.is_done(), "residual: is_done");
    for i in 1..=roots.nb_solutions() {
        let r = roots.value(i);
        let residual = r * r * r - r;
        assert!(
            residual.abs() < 1.0e-6,
            "root {} at {}: residual {} exceeds 1e-6",
            i,
            r,
            residual
        );
    }
}

/// Test 14: Solutions are returned in ascending order.
#[test]
fn solutions_sorted_ascending() {
    // f(x) = sin(x), roots at multiples of π.
    let f = Fn1(|x: f64| x.sin());
    let roots = FunctionRoots::new(
        &f,
        -0.1,
        3.0 * std::f64::consts::PI + 0.1,
        1000,
        precision::CONFUSION,
        1.0e-10,
    );
    assert!(roots.is_done(), "sorted: is_done");
    let n = roots.nb_solutions();
    for i in 1..n {
        assert!(
            roots.value(i) <= roots.value(i + 1),
            "solutions must be non-decreasing: value({})={} > value({})={}",
            i,
            roots.value(i),
            i + 1,
            roots.value(i + 1)
        );
    }
}

/// Test 15: High-multiplicity near-touch: (x-2)^2 barely touches zero.
/// With a double root, sign change may not be detected; the function could be
/// zero only at one point. We verify no spurious extra roots appear.
#[test]
fn double_root_touch() {
    // f(x) = (x-2)^2, touches zero at x=2 (doesn't change sign).
    // With purely sign-change detection: may or may not find the root.
    // The important contract: no wrong roots, is_done() = true.
    let f = Fn1(|x: f64| (x - 2.0) * (x - 2.0));
    let roots = FunctionRoots::new(&f, 0.0, 4.0, 200, 1.0e-7, 1.0e-10);
    assert!(roots.is_done(), "double-root: is_done");
    // If any roots found, they must all be close to 2.
    for i in 1..=roots.nb_solutions() {
        assert_near(roots.value(i), 2.0, 1.0e-4, "double-root: all roots near 2");
    }
}

/// Test 16: Large NbSample finds all five roots of a degree-5 polynomial.
#[test]
fn quintic_five_roots_large_sample() {
    // f(x) = (x+2)(x+1)(x)(x-1)(x-2) = x^5 - 5x^3 + 4x
    let f = Fn1(|x: f64| {
        let x2 = x * x;
        x * (x2 * x2 - 5.0 * x2 + 4.0)
    });
    let roots = FunctionRoots::new(&f, -3.0, 3.0, 1000, precision::CONFUSION, 1.0e-10);
    assert!(roots.is_done(), "quintic: is_done");
    assert_eq!(roots.nb_solutions(), 5, "quintic: five roots");
    let expected = [-2.0_f64, -1.0, 0.0, 1.0, 2.0];
    for (i, &exp) in expected.iter().enumerate() {
        assert_near(roots.value(i + 1), exp, 1.0e-6, &format!("quintic root {}", i + 1));
    }
}
