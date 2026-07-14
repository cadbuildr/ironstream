//! Ported OpenCascade unit tests -- `math_BissecNewton`.
//!
//! Exercises `ironstream::math_bissec_newton::{BissecNewton, MathFunction}`.
//! Each `#[test]` is derived from or inspired by the corresponding OCCT test
//! scenario for `math_BissecNewton`.  Tolerances follow the OCCT defaults:
//! `precision::CONFUSION = 1e-7`, `precision::ANGULAR = 1e-12`.

use ironstream::math_bissec_newton::{BissecNewton, MathFunction};
use ironstream::precision;

// ---------------------------------------------------------------------------
// Concrete test functions
// ---------------------------------------------------------------------------

/// `F(x) = sin(x)`, `F'(x) = cos(x)`.  Root at `x = k*π`.
struct SinFn;
impl MathFunction for SinFn {
    fn value(&self, x: f64) -> f64 {
        x.sin()
    }
    fn derivative(&self, x: f64) -> f64 {
        x.cos()
    }
}

/// `F(x) = x^2 - 2`, `F'(x) = 2x`.  Positive root at `x = √2`.
struct SqrtTwoFn;
impl MathFunction for SqrtTwoFn {
    fn value(&self, x: f64) -> f64 {
        x * x - 2.0
    }
    fn derivative(&self, x: f64) -> f64 {
        2.0 * x
    }
}

/// `F(x) = x^3 - x - 2`, `F'(x) = 3x^2 - 1`.  Root near `x ≈ 1.5214`.
struct CubicFn;
impl MathFunction for CubicFn {
    fn value(&self, x: f64) -> f64 {
        x * x * x - x - 2.0
    }
    fn derivative(&self, x: f64) -> f64 {
        3.0 * x * x - 1.0
    }
}

/// `F(x) = exp(x) - 3`, `F'(x) = exp(x)`.  Root at `x = ln(3)`.
struct ExpFn;
impl MathFunction for ExpFn {
    fn value(&self, x: f64) -> f64 {
        x.exp() - 3.0
    }
    fn derivative(&self, x: f64) -> f64 {
        x.exp()
    }
}

/// `F(x) = cos(x) - x`, `F'(x) = -sin(x) - 1`.  Root (Dottie number) ≈ 0.7391.
struct CosDottie;
impl MathFunction for CosDottie {
    fn value(&self, x: f64) -> f64 {
        x.cos() - x
    }
    fn derivative(&self, x: f64) -> f64 {
        -x.sin() - 1.0
    }
}

/// `F(x) = x^2 + 1` — no real root (always positive). Used to test failure.
/// The derivative is `2x` but the algorithm never calls it when there is no
/// sign change between the bounds — sign-change check fails first.
struct NoRoot;
impl MathFunction for NoRoot {
    fn value(&self, x: f64) -> f64 {
        x * x + 1.0
    }
    fn derivative(&self, x: f64) -> f64 {
        2.0 * x
    }
}


/// `F(x) = x`, `F'(x) = 1`.  Root exactly at `x = 0`.
struct LinearFn;
impl MathFunction for LinearFn {
    fn value(&self, x: f64) -> f64 {
        x
    }
    fn derivative(&self, _x: f64) -> f64 {
        1.0
    }
}

/// `F(x) = tan(x)`, `F'(x) = 1/cos^2(x)`.  Root at `x = 0` and `x = π`, etc.
struct TanFn;
impl MathFunction for TanFn {
    fn value(&self, x: f64) -> f64 {
        x.tan()
    }
    fn derivative(&self, x: f64) -> f64 {
        let c = x.cos();
        1.0 / (c * c)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Assert two floats are within `tol` of each other.
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
    let solver = BissecNewton::new(&f, 3.0, 4.0, precision::CONFUSION);
    assert!(solver.is_done(), "sin root near π should be found");
    let root = solver.root();
    assert_near(root, std::f64::consts::PI, 1.0e-6, "sin root near π");
    assert!(
        solver.value().abs() <= precision::CONFUSION,
        "F(root) should be within CONFUSION: {}",
        solver.value()
    );
}

/// Test 2: x^2 - 2 = 0, positive root = √2.
#[test]
fn sqrt_two() {
    let f = SqrtTwoFn;
    let solver = BissecNewton::new(&f, 1.0, 2.0, precision::CONFUSION);
    assert!(solver.is_done(), "x^2 - 2 root should be found");
    let root = solver.root();
    assert_near(root, 2.0_f64.sqrt(), 1.0e-6, "positive root of x^2 - 2");
    assert!(
        solver.value().abs() <= precision::CONFUSION,
        "F(root) = {}",
        solver.value()
    );
}

/// Test 3: cubic x^3 - x - 2 = 0, root ≈ 1.5214.
#[test]
fn cubic_root() {
    let f = CubicFn;
    let solver = BissecNewton::new(&f, 1.0, 2.0, precision::CONFUSION);
    assert!(solver.is_done(), "cubic root should be found");
    let root = solver.root();
    // True root: x^3 - x - 2 = 0 → x ≈ 1.52137971642...
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
    let solver = BissecNewton::new(&f, 1.0, 1.5, precision::CONFUSION);
    assert!(solver.is_done(), "exp(x)-3 root should be found");
    let root = solver.root();
    assert_near(root, 3.0_f64.ln(), 1.0e-6, "root of exp(x) - 3");
    assert!(
        solver.value().abs() <= precision::CONFUSION,
        "F(root) = {}",
        solver.value()
    );
}

/// Test 5: cos(x) - x = 0 (Dottie number ≈ 0.7390851332).
#[test]
fn dottie_number() {
    let f = CosDottie;
    let solver = BissecNewton::new(&f, 0.0, 1.0, precision::CONFUSION);
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
    // x^2 + 1 is always positive; no root in any real interval.
    let solver = BissecNewton::new(&f, -5.0, 5.0, precision::CONFUSION);
    assert!(
        !solver.is_done(),
        "x^2 + 1 has no real root — is_done must be false"
    );
}

/// Test 7: bounds swapped — algorithm must still work.
#[test]
fn bounds_reversed() {
    let f = SqrtTwoFn;
    // Reverse the bounds: [2.0, 1.0].
    let solver = BissecNewton::new(&f, 2.0, 1.0, precision::CONFUSION);
    assert!(solver.is_done(), "reversed bounds should still find root");
    let root = solver.root();
    assert_near(root, 2.0_f64.sqrt(), 1.0e-6, "reversed bounds root");
}

/// Test 8: root exactly at a bound is found immediately.
#[test]
fn root_at_bound() {
    // F(x) = x, root at 0; use [0, 1].
    let f = LinearFn;
    let solver = BissecNewton::new(&f, 0.0, 1.0, precision::CONFUSION);
    assert!(solver.is_done(), "root at bound should be found");
    assert_near(solver.root(), 0.0, precision::CONFUSION, "root at bound = 0");
    assert!(solver.value().abs() <= precision::CONFUSION);
}

/// Test 9: sin(x) = 0 near −π (negative interval).
#[test]
fn sin_root_at_neg_pi() {
    let f = SinFn;
    let solver = BissecNewton::new(&f, -4.0, -3.0, precision::CONFUSION);
    assert!(solver.is_done(), "sin root near -π should be found");
    let root = solver.root();
    assert_near(root, -std::f64::consts::PI, 1.0e-6, "sin root near -π");
}

/// Test 10: tight tolerance — verify |F(root)| satisfies the tolerance.
#[test]
fn tight_tolerance_value_check() {
    let f = SqrtTwoFn;
    let tol = 1.0e-11;
    let solver = BissecNewton::new(&f, 1.0, 2.0, tol);
    assert!(solver.is_done(), "tight-tolerance solve should succeed");
    assert!(
        solver.value().abs() <= tol,
        "value at root should be within tight tolerance: {}",
        solver.value()
    );
}

/// Test 11: tan(x) = 0 near x = π (use interval just below π+ε to avoid
/// the asymptote at π/2).
#[test]
fn tan_root_near_pi() {
    let f = TanFn;
    // tan(x) = 0 at x = π ≈ 3.14159... Bracket [3.0, 3.5].
    let solver = BissecNewton::new(&f, 3.0, 3.5, precision::CONFUSION);
    assert!(solver.is_done(), "tan root near π should be found");
    let root = solver.root();
    assert_near(root, std::f64::consts::PI, 1.0e-6, "tan root near π");
}

/// Test 12: panic on Root() when not done.
#[test]
#[should_panic(expected = "StdFail_NotDone")]
fn root_panics_when_not_done() {
    let f = NoRoot;
    let solver = BissecNewton::new(&f, -5.0, 5.0, precision::CONFUSION);
    // This must panic.
    let _ = solver.root();
}
