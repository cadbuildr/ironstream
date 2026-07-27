//! Ported OpenCascade unit tests -- `math_BracketMinimum`.
//!
//! Exercises `ironstream::math_bracket_min::{BracketMinimum, MathFunctionBracket}`.
//! Each `#[test]` is derived from or inspired by the corresponding OCCT test
//! scenario for `math_BracketMinimum`. Tolerances follow the OCCT defaults:
//! `precision::CONFUSION = 1e-7`, `precision::ANGULAR = 1e-12`.

use ironstream::math_bracket_min::{BracketMinimum, MathFunctionBracket};
use ironstream::precision;

// ---------------------------------------------------------------------------
// Concrete test functions
// ---------------------------------------------------------------------------

/// `F(x) = (x - 3)^2` — parabola with minimum at x = 3.
struct Parabola;
impl MathFunctionBracket for Parabola {
    fn value(&self, x: f64) -> f64 {
        (x - 3.0) * (x - 3.0)
    }
}

/// `F(x) = (x + 2)^2` — parabola with minimum at x = -2.
struct NegParabola;
impl MathFunctionBracket for NegParabola {
    fn value(&self, x: f64) -> f64 {
        (x + 2.0) * (x + 2.0)
    }
}

/// `F(x) = sin(x)` — periodic, minimum at -π/2 + 2kπ.
struct SinFn;
impl MathFunctionBracket for SinFn {
    fn value(&self, x: f64) -> f64 {
        x.sin()
    }
}

/// `F(x) = x^4 - 4x^2` — quartic with two local minima near ±√2.
struct QuarticFn;
impl MathFunctionBracket for QuarticFn {
    fn value(&self, x: f64) -> f64 {
        let x2 = x * x;
        x2 * x2 - 4.0 * x2
    }
}

/// `F(x) = exp(x) - x` — convex, minimum at x = 0.
struct ExpMinusX;
impl MathFunctionBracket for ExpMinusX {
    fn value(&self, x: f64) -> f64 {
        x.exp() - x
    }
}

/// `F(x) = x^2 - 4*cos(x)` — minimum near x = 0.
struct CosWellFn;
impl MathFunctionBracket for CosWellFn {
    fn value(&self, x: f64) -> f64 {
        x * x - 4.0 * x.cos()
    }
}

/// `F(x) = |x - 1.5|` — absolute value, non-smooth minimum at x = 1.5.
struct AbsValue;
impl MathFunctionBracket for AbsValue {
    fn value(&self, x: f64) -> f64 {
        (x - 1.5).abs()
    }
}

/// `F(x) = (x - 10)^2 + 5` — minimum far from origin at x = 10.
struct FarMinimum;
impl MathFunctionBracket for FarMinimum {
    fn value(&self, x: f64) -> f64 {
        (x - 10.0) * (x - 10.0) + 5.0
    }
}

/// `F(x) = cos(x)` — minimum at x = π.
struct CosFn;
impl MathFunctionBracket for CosFn {
    fn value(&self, x: f64) -> f64 {
        x.cos()
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Verify that the bracket is valid:
/// - `F(b) < F(a)` and `F(b) < F(c)`.
fn assert_valid_bracket(
    bm: &BracketMinimum,
    f: &dyn MathFunctionBracket,
    label: &str,
) {
    assert!(bm.is_done(), "{label}: is_done() must be true");
    let fa = bm.val_a();
    let fb = bm.val_b();
    let fc = bm.val_c();
    assert!(
        fb < fa,
        "{label}: F(b)={fb} must be < F(a)={fa}"
    );
    assert!(
        fb < fc,
        "{label}: F(b)={fb} must be < F(c)={fc}"
    );
    // Also verify that the stored values match re-evaluation.
    let fa2 = f.value(bm.par_a());
    let fb2 = f.value(bm.par_b());
    let fc2 = f.value(bm.par_c());
    assert!(
        (fa - fa2).abs() < precision::CONFUSION,
        "{label}: val_a mismatch: stored {fa}, recomputed {fa2}"
    );
    assert!(
        (fb - fb2).abs() < precision::CONFUSION,
        "{label}: val_b mismatch: stored {fb}, recomputed {fb2}"
    );
    assert!(
        (fc - fc2).abs() < precision::CONFUSION,
        "{label}: val_c mismatch: stored {fc}, recomputed {fc2}"
    );
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Test 1: Simple parabola (x-3)^2 — two-point constructor starting near origin.
#[test]
fn parabola_two_point_constructor() {
    let f = Parabola;
    // Start from (0, 1) — both values above the minimum; algorithm should
    // expand until it brackets the minimum near x=3.
    let bm = BracketMinimum::new(&f, 0.0, 1.0);
    assert_valid_bracket(&bm, &f, "parabola_two_point");
}

/// Test 2: Parabola with starting points trending downhill toward the minimum.
#[test]
fn parabola_straddling_start() {
    let f = Parabola;
    // a=1 (F=4), b=4 (F=1): trending downhill toward minimum at x=3;
    // algorithm expands further and must locate the bracket.
    let bm = BracketMinimum::new(&f, 1.0, 4.0);
    assert_valid_bracket(&bm, &f, "parabola_straddling");
}

/// Test 3: Negative-side parabola (x+2)^2 — minimum at x = -2.
#[test]
fn neg_parabola_two_point() {
    let f = NegParabola;
    // Start from (-1, 0) — algorithm must go left.
    let bm = BracketMinimum::new(&f, -1.0, 0.0);
    assert_valid_bracket(&bm, &f, "neg_parabola");
    // The minimum-bracketing point par_b should be near x = -2.
    assert!(
        bm.par_b() < -1.0,
        "bracket middle should be left of -1, got {}",
        bm.par_b()
    );
}

/// Test 4: sin(x) — two-point constructor, bracket should contain a minimum.
#[test]
fn sin_two_point_bracket() {
    let f = SinFn;
    // sin is decreasing at x=0 → x=π/2, so start from (0, 0.5).
    let bm = BracketMinimum::new(&f, 0.0, 0.5);
    assert_valid_bracket(&bm, &f, "sin_two_point");
}

/// Test 5: Three-point constructor — pre-existing valid bracket is accepted immediately.
#[test]
fn three_point_valid_bracket() {
    let f = Parabola;
    // (1, 3, 5): F(1)=4, F(3)=0, F(5)=4 — already a valid bracket.
    let bm = BracketMinimum::with_c(&f, 1.0, 3.0, 5.0);
    assert_valid_bracket(&bm, &f, "three_point_valid");
    // b should remain at 3 since it was already the minimum.
    let tol = 1.0e-12;
    assert!(
        (bm.par_b() - 3.0).abs() < tol,
        "par_b should be 3.0, got {}",
        bm.par_b()
    );
    assert!(
        bm.val_b() < tol,
        "val_b should be ~0, got {}",
        bm.val_b()
    );
}

/// Test 6: Three-point constructor where initial (a, b, c) is not yet a bracket.
#[test]
fn three_point_needs_expansion() {
    let f = Parabola;
    // (0, 1, 2): F(0)=9, F(1)=4, F(2)=1 — monotone decreasing; not a bracket.
    // Algorithm should expand to find the minimum.
    let bm = BracketMinimum::with_c(&f, 0.0, 1.0, 2.0);
    assert_valid_bracket(&bm, &f, "three_point_needs_expansion");
}

/// Test 7: with_values constructor (pre-computed fa, fb).
#[test]
fn with_values_constructor() {
    let f = Parabola;
    let a = 0.0_f64;
    let b = 1.0_f64;
    let fa = f.value(a);
    let fb = f.value(b);
    let bm = BracketMinimum::with_values(&f, a, b, fa, fb);
    assert_valid_bracket(&bm, &f, "with_values");
}

/// Test 8: exp(x)-x — convex function, minimum at x=0.
#[test]
fn exp_minus_x_bracket() {
    let f = ExpMinusX;
    // F(-1)=1/e+1≈1.37, F(0.5)=e^0.5-0.5≈1.15, both above min F(0)=1.
    let bm = BracketMinimum::new(&f, -1.0, 0.5);
    assert_valid_bracket(&bm, &f, "exp_minus_x");
}

/// Test 9: quartic x^4 - 4x^2 — find a bracket near the positive minimum.
#[test]
fn quartic_bracket() {
    let f = QuarticFn;
    // Start from (0.5, 1.0) — algorithm should find a bracket near √2 ≈ 1.414.
    let bm = BracketMinimum::new(&f, 0.5, 1.0);
    assert_valid_bracket(&bm, &f, "quartic");
}

/// Test 10: absolute value |x - 1.5| — non-smooth minimum.
#[test]
fn abs_value_bracket() {
    let f = AbsValue;
    // Start from (0, 0.5).
    let bm = BracketMinimum::new(&f, 0.0, 0.5);
    assert_valid_bracket(&bm, &f, "abs_value");
    // The bracket middle should be near x = 1.5.
    assert!(
        bm.par_b() > 0.5,
        "bracket middle should be past 0.5, got {}",
        bm.par_b()
    );
}

/// Test 11: far minimum at x=10 — algorithm must expand far from origin.
#[test]
fn far_minimum_bracket() {
    let f = FarMinimum;
    // Start from (0, 1).
    let bm = BracketMinimum::new(&f, 0.0, 1.0);
    assert_valid_bracket(&bm, &f, "far_minimum");
    // The minimum-value point should be substantially to the right.
    assert!(
        bm.par_b() > 1.0,
        "bracket middle should be > 1, got {}",
        bm.par_b()
    );
}

/// Test 12: cos(x) — verify bracket contains a region with cos descending to -1.
#[test]
fn cos_bracket() {
    let f = CosFn;
    // cos is decreasing on (0, π); start from (0.5, 1.5).
    let bm = BracketMinimum::new(&f, 0.5, 1.5);
    assert_valid_bracket(&bm, &f, "cos");
}

/// Test 13: par_a / par_b / par_c and val_a / val_b / val_c self-consistency.
#[test]
fn pars_and_vals_self_consistent() {
    let f = Parabola;
    let bm = BracketMinimum::new(&f, 0.0, 1.0);
    assert!(bm.is_done());
    // val_X must equal F(par_X).
    let tol = precision::CONFUSION;
    assert!(
        (bm.val_a() - f.value(bm.par_a())).abs() < tol,
        "val_a must equal F(par_a)"
    );
    assert!(
        (bm.val_b() - f.value(bm.par_b())).abs() < tol,
        "val_b must equal F(par_b)"
    );
    assert!(
        (bm.val_c() - f.value(bm.par_c())).abs() < tol,
        "val_c must equal F(par_c)"
    );
    // val_b is strictly less than val_a and val_c.
    assert!(bm.val_b() < bm.val_a(), "val_b < val_a");
    assert!(bm.val_b() < bm.val_c(), "val_b < val_c");
}

/// Test 14: cos-well x^2 - 4*cos(x) — minimum near x=0.
#[test]
fn cos_well_bracket() {
    let f = CosWellFn;
    // F is symmetric around 0; start from (0.5, 1.0) going away from minimum.
    let bm = BracketMinimum::new(&f, 0.5, 1.0);
    // The function is decreasing for x in (0.5, 1.0) toward 0, so the two-point
    // start should trigger expansion back toward x = 0.
    // Rather than test exact position, just verify the bracket is valid.
    assert_valid_bracket(&bm, &f, "cos_well");
}

/// Test 15: panic on par_a() when not done.
#[test]
#[should_panic(expected = "StdFail_NotDone")]
fn par_a_panics_when_not_done() {
    // Feed a function that returns NaN immediately so bracketing fails.
    struct NanFn;
    impl MathFunctionBracket for NanFn {
        fn value(&self, _x: f64) -> f64 {
            f64::NAN
        }
    }
    let bm = BracketMinimum::new(&NanFn, 0.0, 1.0);
    assert!(!bm.is_done(), "should not be done");
    let _ = bm.par_a(); // must panic
}

/// Test 16: panic on par_b() when not done.
#[test]
#[should_panic(expected = "StdFail_NotDone")]
fn par_b_panics_when_not_done() {
    struct NanFn;
    impl MathFunctionBracket for NanFn {
        fn value(&self, _x: f64) -> f64 {
            f64::NAN
        }
    }
    let bm = BracketMinimum::new(&NanFn, 0.0, 1.0);
    let _ = bm.par_b();
}

/// Test 17: three-point constructor with negative coordinates.
#[test]
fn three_point_negative_coords() {
    let f = NegParabola;
    // (x+2)^2: F(-4)=4, F(-2)=0, F(0)=4 — already a valid bracket.
    let bm = BracketMinimum::with_c(&f, -4.0, -2.0, 0.0);
    assert_valid_bracket(&bm, &f, "three_point_neg");
    let tol = 1.0e-12;
    assert!(
        (bm.par_b() - (-2.0)).abs() < tol,
        "par_b should be -2, got {}",
        bm.par_b()
    );
}

/// Test 18: with_values constructor matches new() for the same inputs.
#[test]
fn with_values_matches_new() {
    let f = Parabola;
    let a = 0.0_f64;
    let b = 2.0_f64;
    let fa = f.value(a);
    let fb = f.value(b);

    let bm_new = BracketMinimum::new(&f, a, b);
    let bm_vals = BracketMinimum::with_values(&f, a, b, fa, fb);

    assert_eq!(bm_new.is_done(), bm_vals.is_done());
    if bm_new.is_done() && bm_vals.is_done() {
        // Both should produce valid brackets.
        assert_valid_bracket(&bm_new, &f, "with_values_matches_new (new)");
        assert_valid_bracket(&bm_vals, &f, "with_values_matches_new (vals)");
    }
}
