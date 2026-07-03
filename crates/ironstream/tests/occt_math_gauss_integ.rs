//! Ported OpenCascade unit tests -- `math_GaussSingleIntegration`.
//!
//! Exercises `ironstream::math_gauss_integ::{GaussSingleIntegration, MathFunction}`.
//! Each `#[test]` is derived from or inspired by OCCT's integration test scenarios.
//! Tolerances follow OCCT defaults:
//!   `precision::CONFUSION = 1e-7`, `precision::ANGULAR = 1e-12`.

use ironstream::math_gauss_integ::{GaussSingleIntegration, MathFunction};
use ironstream::precision;

// ---------------------------------------------------------------------------
// Concrete test functions
// ---------------------------------------------------------------------------

/// `F(x) = x` — integral on [a,b] = (b^2 - a^2) / 2.
struct LinearFn;
impl MathFunction for LinearFn {
    fn value(&self, x: f64) -> Option<f64> {
        Some(x)
    }
}

/// `F(x) = x^2` — integral on [a,b] = (b^3 - a^3) / 3.
struct QuadraticFn;
impl MathFunction for QuadraticFn {
    fn value(&self, x: f64) -> Option<f64> {
        Some(x * x)
    }
}

/// `F(x) = sin(x)` — integral on [a,b] = -cos(b) + cos(a).
struct SinFn;
impl MathFunction for SinFn {
    fn value(&self, x: f64) -> Option<f64> {
        Some(x.sin())
    }
}

/// `F(x) = cos(x)` — integral on [a,b] = sin(b) - sin(a).
struct CosFn;
impl MathFunction for CosFn {
    fn value(&self, x: f64) -> Option<f64> {
        Some(x.cos())
    }
}

/// `F(x) = exp(x)` — integral on [a,b] = exp(b) - exp(a).
struct ExpFn;
impl MathFunction for ExpFn {
    fn value(&self, x: f64) -> Option<f64> {
        Some(x.exp())
    }
}

/// `F(x) = 1 / (1 + x^2)` — integral on [a,b] = arctan(b) - arctan(a).
struct RationalFn;
impl MathFunction for RationalFn {
    fn value(&self, x: f64) -> Option<f64> {
        Some(1.0 / (1.0 + x * x))
    }
}

/// `F(x) = x^4 - 2x + 1` — integral on [0,1] = 1/5 - 1 + 1 = 1/5.
struct PolynomialFn;
impl MathFunction for PolynomialFn {
    fn value(&self, x: f64) -> Option<f64> {
        Some(x * x * x * x - 2.0 * x + 1.0)
    }
}

/// `F(x) = 1.0` (constant 1) — integral on [a,b] = b - a.
struct ConstantFn;
impl MathFunction for ConstantFn {
    fn value(&self, _x: f64) -> Option<f64> {
        Some(1.0)
    }
}

/// A function that always returns `None` — should make `is_done()` false.
struct FailFn;
impl MathFunction for FailFn {
    fn value(&self, _x: f64) -> Option<f64> {
        None
    }
}

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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Test 1: Integral of x on [0, 1] = 0.5. Order-1 GL is exact for linear fns.
#[test]
fn linear_on_unit_interval() {
    let f = LinearFn;
    let integ = GaussSingleIntegration::new(&f, 0.0, 1.0, 1);
    assert!(integ.is_done(), "integration of linear fn should succeed");
    // GL order 1 is exact for polynomials of degree <= 1.
    assert_near(integ.value(), 0.5, precision::CONFUSION, "integral of x on [0,1]");
}

/// Test 2: Integral of x^2 on [0, 1] = 1/3.
#[test]
fn quadratic_on_unit_interval() {
    let f = QuadraticFn;
    // Order 2 is exact for degree <= 3; should be exact here.
    let integ = GaussSingleIntegration::new(&f, 0.0, 1.0, 2);
    assert!(integ.is_done(), "integration of x^2 should succeed");
    assert_near(integ.value(), 1.0 / 3.0, precision::CONFUSION, "integral of x^2 on [0,1]");
}

/// Test 3: Integral of sin(x) on [0, pi] = 2.
#[test]
fn sin_on_zero_to_pi() {
    let f = SinFn;
    let pi = std::f64::consts::PI;
    // Order 8 gives better than 1e-10 accuracy for sin on [0, pi].
    let integ = GaussSingleIntegration::new(&f, 0.0, pi, 8);
    assert!(integ.is_done(), "integration of sin should succeed");
    assert_near(integ.value(), 2.0, precision::CONFUSION, "integral of sin on [0,pi]");
}

/// Test 4: Integral of cos(x) on [0, pi/2] = 1.
#[test]
fn cos_on_zero_to_pi_half() {
    let f = CosFn;
    let half_pi = std::f64::consts::FRAC_PI_2;
    let integ = GaussSingleIntegration::new(&f, 0.0, half_pi, 5);
    assert!(integ.is_done(), "integration of cos should succeed");
    assert_near(integ.value(), 1.0, precision::CONFUSION, "integral of cos on [0,pi/2]");
}

/// Test 5: Integral of exp(x) on [0, 1] = e - 1 ≈ 1.718281828.
#[test]
fn exp_on_unit_interval() {
    let f = ExpFn;
    let expected = std::f64::consts::E - 1.0;
    // Order 10 gives high accuracy for exp.
    let integ = GaussSingleIntegration::new(&f, 0.0, 1.0, 10);
    assert!(integ.is_done(), "integration of exp should succeed");
    assert_near(integ.value(), expected, precision::CONFUSION, "integral of exp on [0,1]");
}

/// Test 6: Integral of 1/(1+x^2) on [0, 1] = pi/4.
#[test]
fn rational_function_arctan() {
    let f = RationalFn;
    let expected = std::f64::consts::FRAC_PI_4;
    let integ = GaussSingleIntegration::new(&f, 0.0, 1.0, 10);
    assert!(integ.is_done(), "integration of 1/(1+x^2) should succeed");
    // This function is smooth, so high-order GL converges quickly.
    assert_near(integ.value(), expected, 1.0e-10, "integral of 1/(1+x^2) on [0,1] = pi/4");
}

/// Test 7: Integral of constant 1 on [2, 5] = 3. Exact for any order.
#[test]
fn constant_fn_arbitrary_interval() {
    let f = ConstantFn;
    let integ = GaussSingleIntegration::new(&f, 2.0, 5.0, 3);
    assert!(integ.is_done(), "integration of constant should succeed");
    assert_near(integ.value(), 3.0, precision::CONFUSION, "integral of 1 on [2,5] = 3");
}

/// Test 8: Integral of x^4 - 2x + 1 on [0, 1] = 1/5 - 1 + 1 = 1/5.
#[test]
fn polynomial_degree_4() {
    let f = PolynomialFn;
    // Order 3 is exact for degree <= 5.
    let integ = GaussSingleIntegration::new(&f, 0.0, 1.0, 3);
    assert!(integ.is_done(), "integration of degree-4 polynomial should succeed");
    assert_near(integ.value(), 0.2, precision::CONFUSION, "integral of x^4-2x+1 on [0,1]");
}

/// Test 9: `is_done()` is false when the function evaluation fails.
#[test]
fn function_failure_sets_not_done() {
    let f = FailFn;
    let integ = GaussSingleIntegration::new(&f, 0.0, 1.0, 5);
    assert!(!integ.is_done(), "integration with failing function should be not done");
}

/// Test 10: `value()` panics with StdFail_NotDone when not done.
#[test]
#[should_panic(expected = "StdFail_NotDone")]
fn value_panics_when_not_done() {
    let f = FailFn;
    let integ = GaussSingleIntegration::new(&f, 0.0, 1.0, 5);
    let _ = integ.value(); // must panic
}

/// Test 11: with_tol constructor, relaxed tolerance, sin on [0, pi].
#[test]
fn with_tol_sin_converges() {
    let f = SinFn;
    let pi = std::f64::consts::PI;
    // Use order 8 so that order-8 and order-16 estimates agree to 1e-6.
    let integ = GaussSingleIntegration::with_tol(&f, 0.0, pi, 8, 1.0e-6);
    // with_tol checks convergence; the integral is exactly 2.
    assert!(integ.is_done(), "with_tol integration of sin should converge");
    assert_near(integ.value(), 2.0, 1.0e-6, "with_tol integral of sin on [0,pi]");
}

/// Test 12: Integral on reversed interval [1, 0] = -0.5 for linear fn.
#[test]
fn reversed_interval() {
    let f = LinearFn;
    let integ = GaussSingleIntegration::new(&f, 1.0, 0.0, 3);
    assert!(integ.is_done(), "reversed interval integration should succeed");
    assert_near(integ.value(), -0.5, precision::CONFUSION, "integral of x on [1,0] = -0.5");
}

/// Test 13: Higher order integration (order 10) of exp on [0, 2].
/// Expected: e^2 - 1 ≈ 6.389056099.
#[test]
fn exp_order_10_wider_interval() {
    let f = ExpFn;
    let expected = (2.0_f64).exp() - 1.0;
    let integ = GaussSingleIntegration::new(&f, 0.0, 2.0, 10);
    assert!(integ.is_done(), "order-10 integration of exp on [0,2] should succeed");
    assert_near(integ.value(), expected, precision::CONFUSION, "integral of exp on [0,2]");
}

/// Test 14: with_tol with tol <= 0 behaves like plain new().
#[test]
fn with_tol_zero_tolerance_behaves_like_new() {
    let f = QuadraticFn;
    let integ_new = GaussSingleIntegration::new(&f, 0.0, 1.0, 5);
    let integ_tol = GaussSingleIntegration::with_tol(&f, 0.0, 1.0, 5, 0.0);
    assert!(integ_new.is_done());
    assert!(integ_tol.is_done());
    assert_near(
        integ_tol.value(),
        integ_new.value(),
        precision::ANGULAR,
        "with_tol(tol=0) should match new()",
    );
}
