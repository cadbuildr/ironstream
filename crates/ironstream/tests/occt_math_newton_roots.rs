//! Ported OpenCascade unit tests -- `math_NewtonFunctionRoots`.
//!
//! Exercises `ironstream::math_newton_roots::{NewtonFunctionRoots,
//! MathFunctionWithDerivative}`.
//! Each `#[test]` is derived from or inspired by the corresponding OCCT test
//! scenario.  Tolerances follow the OCCT defaults:
//! `precision::CONFUSION = 1e-7`, `precision::ANGULAR = 1e-12`.

use ironstream::math_newton_roots::{MathFunctionWithDerivative, NewtonFunctionRoots};
use ironstream::precision;

// ---------------------------------------------------------------------------
// Concrete test functions
// ---------------------------------------------------------------------------

/// `F(x) = sin(x)`, `F'(x) = cos(x)`.  Root at `x = k*π`.
struct SinFn;
impl MathFunctionWithDerivative for SinFn {
    fn value_and_derivative(&self, x: f64) -> Option<(f64, f64)> {
        Some((x.sin(), x.cos()))
    }
}

/// `F(x) = x^2 - 2`, `F'(x) = 2x`.  Positive root at `x = √2`.
struct SqrtTwoFn;
impl MathFunctionWithDerivative for SqrtTwoFn {
    fn value_and_derivative(&self, x: f64) -> Option<(f64, f64)> {
        Some((x * x - 2.0, 2.0 * x))
    }
}

/// `F(x) = x^3 - x - 2`, `F'(x) = 3x^2 - 1`.  Root ≈ 1.52138.
struct CubicFn;
impl MathFunctionWithDerivative for CubicFn {
    fn value_and_derivative(&self, x: f64) -> Option<(f64, f64)> {
        Some((x * x * x - x - 2.0, 3.0 * x * x - 1.0))
    }
}

/// `F(x) = exp(x) - 3`, `F'(x) = exp(x)`.  Root at `x = ln(3)`.
struct ExpFn;
impl MathFunctionWithDerivative for ExpFn {
    fn value_and_derivative(&self, x: f64) -> Option<(f64, f64)> {
        let ex = x.exp();
        Some((ex - 3.0, ex))
    }
}

/// `F(x) = cos(x) - x`, `F'(x) = -sin(x) - 1`.  Root (Dottie) ≈ 0.73909.
struct CosDottie;
impl MathFunctionWithDerivative for CosDottie {
    fn value_and_derivative(&self, x: f64) -> Option<(f64, f64)> {
        Some((x.cos() - x, -x.sin() - 1.0))
    }
}

/// `F(x) = x^2 + 1` — no real root (always positive).
struct NoRoot;
impl MathFunctionWithDerivative for NoRoot {
    fn value_and_derivative(&self, x: f64) -> Option<(f64, f64)> {
        Some((x * x + 1.0, 2.0 * x))
    }
}

/// `F(x) = x`, `F'(x) = 1`.  Root exactly at `x = 0`.
struct LinearFn;
impl MathFunctionWithDerivative for LinearFn {
    fn value_and_derivative(&self, x: f64) -> Option<(f64, f64)> {
        Some((x, 1.0))
    }
}

/// `F(x) = tan(x)`, `F'(x) = sec^2(x)`.  Root at `x = π`.
struct TanFn;
impl MathFunctionWithDerivative for TanFn {
    fn value_and_derivative(&self, x: f64) -> Option<(f64, f64)> {
        let c = x.cos();
        Some((x.tan(), 1.0 / (c * c)))
    }
}

/// `F(x) = ln(x) - 1`, `F'(x) = 1/x`.  Root at `x = e`.
struct LnFn;
impl MathFunctionWithDerivative for LnFn {
    fn value_and_derivative(&self, x: f64) -> Option<(f64, f64)> {
        if x <= 0.0 {
            None
        } else {
            Some((x.ln() - 1.0, 1.0 / x))
        }
    }
}

/// `F(x) = x^5 - x - 1`, `F'(x) = 5x^4 - 1`.  Root ≈ 1.1673.
struct QuinticFn;
impl MathFunctionWithDerivative for QuinticFn {
    fn value_and_derivative(&self, x: f64) -> Option<(f64, f64)> {
        Some((x.powi(5) - x - 1.0, 5.0 * x.powi(4) - 1.0))
    }
}

// ---------------------------------------------------------------------------
// Helper
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

/// Test 1: sin(x) = 0 near π.
#[test]
fn sin_root_at_pi() {
    let f = SinFn;
    let solver = NewtonFunctionRoots::new(&f, 3.0, 4.0, precision::CONFUSION);
    assert!(solver.is_done(), "sin root near π should be found");
    let root = solver.root();
    assert_near(root, std::f64::consts::PI, 1.0e-6, "sin root near π");
    assert!(
        solver.value().abs() <= precision::CONFUSION,
        "F(root) should be within CONFUSION: {}",
        solver.value()
    );
    // Derivative at π: cos(π) = -1.
    assert_near(solver.derivative(), -1.0, 1.0e-6, "F'(root) at π");
}

/// Test 2: x^2 - 2 = 0, positive root = √2.
#[test]
fn sqrt_two() {
    let f = SqrtTwoFn;
    let solver = NewtonFunctionRoots::new(&f, 1.0, 2.0, precision::CONFUSION);
    assert!(solver.is_done(), "x^2 - 2 root should be found");
    let root = solver.root();
    assert_near(root, 2.0_f64.sqrt(), 1.0e-6, "positive root of x^2 - 2");
    assert!(
        solver.value().abs() <= precision::CONFUSION,
        "F(root) = {}",
        solver.value()
    );
    // Derivative at √2: 2√2 ≈ 2.828...
    assert_near(
        solver.derivative(),
        2.0 * 2.0_f64.sqrt(),
        1.0e-6,
        "F'(root) at √2",
    );
}

/// Test 3: cubic x^3 - x - 2 = 0, root ≈ 1.52138.
#[test]
fn cubic_root() {
    let f = CubicFn;
    let solver = NewtonFunctionRoots::new(&f, 1.0, 2.0, precision::CONFUSION);
    assert!(solver.is_done(), "cubic root should be found");
    let root = solver.root();
    let expected = 1.521_379_706_804_568_f64;
    assert_near(root, expected, 1.0e-6, "cubic root");
    assert!(
        solver.value().abs() <= precision::CONFUSION,
        "F(root) = {}",
        solver.value()
    );
}

/// Test 4: exp(x) - 3 = 0, root = ln(3).
#[test]
fn exp_minus_3() {
    let f = ExpFn;
    let solver = NewtonFunctionRoots::new(&f, 1.0, 1.5, precision::CONFUSION);
    assert!(solver.is_done(), "exp(x) - 3 root should be found");
    let root = solver.root();
    assert_near(root, 3.0_f64.ln(), 1.0e-6, "root of exp(x) - 3");
    assert!(
        solver.value().abs() <= precision::CONFUSION,
        "F(root) = {}",
        solver.value()
    );
}

/// Test 5: cos(x) - x = 0 (Dottie number ≈ 0.73909).
#[test]
fn dottie_number() {
    let f = CosDottie;
    let solver = NewtonFunctionRoots::new(&f, 0.0, 1.0, precision::CONFUSION);
    assert!(solver.is_done(), "Dottie number should be found");
    let root = solver.root();
    let expected = 0.739_085_133_215_160_6_f64;
    assert_near(root, expected, 1.0e-6, "Dottie number");
    assert!(
        solver.value().abs() <= precision::CONFUSION,
        "F(root) = {}",
        solver.value()
    );
}

/// Test 6: no sign change → is_done() must be false.
#[test]
fn no_sign_change_fails() {
    let f = NoRoot;
    let solver = NewtonFunctionRoots::new(&f, -5.0, 5.0, precision::CONFUSION);
    assert!(
        !solver.is_done(),
        "x^2 + 1 has no real root — is_done must be false"
    );
}

/// Test 7: bounds swapped — algorithm must still work.
#[test]
fn bounds_reversed() {
    let f = SqrtTwoFn;
    let solver = NewtonFunctionRoots::new(&f, 2.0, 1.0, precision::CONFUSION);
    assert!(solver.is_done(), "reversed bounds should still find root");
    let root = solver.root();
    assert_near(root, 2.0_f64.sqrt(), 1.0e-6, "reversed bounds root");
}

/// Test 8: root exactly at the left bound is returned immediately.
#[test]
fn root_at_left_bound() {
    // F(x) = x, root at 0; use [0, 1].
    let f = LinearFn;
    let solver = NewtonFunctionRoots::new(&f, 0.0, 1.0, precision::CONFUSION);
    assert!(solver.is_done(), "root at left bound should be found");
    assert_near(solver.root(), 0.0, precision::CONFUSION, "root at 0");
    assert!(solver.value().abs() <= precision::CONFUSION);
}

/// Test 9: sin(x) = 0 near −π (negative interval).
#[test]
fn sin_root_at_neg_pi() {
    let f = SinFn;
    let solver = NewtonFunctionRoots::new(&f, -4.0, -3.0, precision::CONFUSION);
    assert!(solver.is_done(), "sin root near -π should be found");
    let root = solver.root();
    assert_near(root, -std::f64::consts::PI, 1.0e-6, "sin root near -π");
}

/// Test 10: tight tolerance — verify |F(root)| satisfies the tolerance.
#[test]
fn tight_tolerance_value_check() {
    let f = SqrtTwoFn;
    let tol = 1.0e-11;
    let solver = NewtonFunctionRoots::new(&f, 1.0, 2.0, tol);
    assert!(solver.is_done(), "tight-tolerance solve should succeed");
    assert!(
        solver.value().abs() <= tol,
        "value at root should be within tight tolerance: {}",
        solver.value()
    );
}

/// Test 11: ln(x) - 1 = 0, root = e.
#[test]
fn ln_root_at_e() {
    let f = LnFn;
    let solver = NewtonFunctionRoots::new(&f, 2.0, 3.0, precision::CONFUSION);
    assert!(solver.is_done(), "ln(x) - 1 root should be found");
    let root = solver.root();
    assert_near(root, std::f64::consts::E, 1.0e-6, "root of ln(x) - 1 = e");
    assert!(
        solver.value().abs() <= precision::CONFUSION,
        "F(root) = {}",
        solver.value()
    );
}

/// Test 12: quintic x^5 - x - 1 = 0, root ≈ 1.1673.
#[test]
fn quintic_root() {
    let f = QuinticFn;
    let solver = NewtonFunctionRoots::new(&f, 1.0, 2.0, precision::CONFUSION);
    assert!(solver.is_done(), "quintic root should be found");
    let root = solver.root();
    // Numerically verified: root ≈ 1.167303978154...
    assert_near(root, 1.167_303_978_154_f64, 1.0e-6, "quintic root");
    assert!(
        solver.value().abs() <= precision::CONFUSION,
        "F(root) = {}",
        solver.value()
    );
}

/// Test 13: tan(x) = 0 near π.
#[test]
fn tan_root_near_pi() {
    let f = TanFn;
    // tan(x) = 0 at x = π ≈ 3.14159... Bracket [3.0, 3.5].
    let solver = NewtonFunctionRoots::new(&f, 3.0, 3.5, precision::CONFUSION);
    assert!(solver.is_done(), "tan root near π should be found");
    let root = solver.root();
    assert_near(root, std::f64::consts::PI, 1.0e-6, "tan root near π");
}

/// Test 14: panic on root() when not done.
#[test]
#[should_panic(expected = "StdFail_NotDone")]
fn root_panics_when_not_done() {
    let f = NoRoot;
    let solver = NewtonFunctionRoots::new(&f, -5.0, 5.0, precision::CONFUSION);
    let _ = solver.root();
}

/// Test 15: panic on value() when not done.
#[test]
#[should_panic(expected = "StdFail_NotDone")]
fn value_panics_when_not_done() {
    let f = NoRoot;
    let solver = NewtonFunctionRoots::new(&f, -5.0, 5.0, precision::CONFUSION);
    let _ = solver.value();
}

/// Test 16: panic on derivative() when not done.
#[test]
#[should_panic(expected = "StdFail_NotDone")]
fn derivative_panics_when_not_done() {
    let f = NoRoot;
    let solver = NewtonFunctionRoots::new(&f, -5.0, 5.0, precision::CONFUSION);
    let _ = solver.derivative();
}
