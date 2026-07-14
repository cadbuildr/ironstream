// FILE: rust/ironstream/crates/ironstream/tests/occt_math_optimization.rs
extern crate ironstream;
use ironstream::math_optimization::*;

fn assert_near(actual: f64, expected: f64, tol: f64, msg: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{msg}: |{actual} - {expected}| = {} > {tol}",
        (actual - expected).abs()
    );
}

// ────────────────────────────────────────────────────────────
// MathGSSectionResult
// ────────────────────────────────────────────────────────────

/// Test 1: MathGSSectionResult::new() has is_done=false.
#[test]
fn gs_result_new_is_not_done() {
    let r = MathGSSectionResult::new();
    assert!(!r.is_done());
}

/// Test 2: MathGSSectionResult::new() has nb_iter=0.
#[test]
fn gs_result_new_nb_iter_is_zero() {
    let r = MathGSSectionResult::new();
    assert_eq!(r.nb_iter(), 0);
}

/// Test 3: MathGSSectionResult::new() value() is infinite.
#[test]
fn gs_result_new_value_is_infinite() {
    let r = MathGSSectionResult::new();
    assert!(r.value().is_infinite());
}

/// Test 4: MathGSSectionResult::new() minimum() slice is empty.
#[test]
fn gs_result_new_minimum_is_empty() {
    let r = MathGSSectionResult::new();
    assert!(r.minimum().is_empty());
}

// ────────────────────────────────────────────────────────────
// MathGoldenSection
// ────────────────────────────────────────────────────────────

/// Test 5: MathGoldenSection::new() is not done before perform.
#[test]
fn golden_section_new_not_done() {
    let gs = MathGoldenSection::new();
    assert!(!gs.is_done());
}

/// Test 6: Minimizing f(x) = x² on [-5, 5] finds minimum near x=0.
#[test]
fn golden_section_x_squared_minimum_near_zero() {
    let mut gs = MathGoldenSection::new();
    gs.perform(&|x| x * x, -5.0, 5.0, 1e-10, 300);
    assert!(gs.is_done());
    assert!(
        gs.result().abs() < 1e-6,
        "expected result near 0, got {}",
        gs.result()
    );
}

/// Test 7: Minimizing f(x) = x² gives value near 0.
#[test]
fn golden_section_x_squared_value_near_zero() {
    let mut gs = MathGoldenSection::new();
    gs.perform(&|x| x * x, -5.0, 5.0, 1e-10, 300);
    assert!(gs.value() < 1e-12, "expected value near 0, got {}", gs.value());
}

/// Test 8: nb_iter() is positive after a non-trivial search.
#[test]
fn golden_section_nb_iter_positive() {
    let mut gs = MathGoldenSection::new();
    gs.perform(&|x| x * x, -5.0, 5.0, 1e-10, 300);
    assert!(gs.nb_iter() > 0, "expected nb_iter > 0");
}

/// Test 9: Minimizing f(x) = (x - 3)² finds minimum near x=3.
#[test]
fn golden_section_shifted_parabola_finds_three() {
    let mut gs = MathGoldenSection::new();
    gs.perform(&|x| (x - 3.0).powi(2), 0.0, 6.0, 1e-10, 300);
    assert!(gs.is_done());
    assert_near(gs.result(), 3.0, 1e-6, "minimum of (x-3)^2");
    assert!(gs.value() < 1e-12, "value={}", gs.value());
}

/// Test 10: Golden ratio — minimizing f(x) = (x - phi)² where phi = (1+sqrt(5))/2.
/// This finds phi ≈ 1.6180339887 on [1.0, 2.0].
#[test]
fn golden_section_finds_golden_ratio() {
    let phi: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0; // ≈ 1.6180339887
    let mut gs = MathGoldenSection::new();
    gs.perform(&|x| (x - phi) * (x - phi), 1.0, 2.0, 1e-10, 300);
    assert!(gs.is_done());
    assert_near(gs.result(), phi, 1e-6, "golden ratio");
    assert!(gs.value() < 1e-12, "value={}", gs.value());
}

/// Test 11: nb_iter() does not exceed max_iter.
#[test]
fn golden_section_nb_iter_bounded_by_max_iter() {
    let max_iter = 15u32;
    let mut gs = MathGoldenSection::new();
    gs.perform(&|x| x * x, -10.0, 10.0, 1e-30, max_iter);
    assert!(
        gs.nb_iter() <= max_iter,
        "nb_iter={} > max_iter={}",
        gs.nb_iter(),
        max_iter
    );
}

/// Test 12: value() is consistent with f evaluated at result().
#[test]
fn golden_section_value_matches_f_at_result() {
    let mut gs = MathGoldenSection::new();
    gs.perform(&|x| (x - 2.0).powi(2) + 1.0, -5.0, 5.0, 1e-10, 300);
    let recomputed = (gs.result() - 2.0).powi(2) + 1.0;
    assert_near(gs.value(), recomputed, 1e-10, "value vs f(result)");
}

// ────────────────────────────────────────────────────────────
// MathPowell
// ────────────────────────────────────────────────────────────

/// Test 13: MathPowell::new(n) is not done before perform.
#[test]
fn powell_new_not_done() {
    let pw = MathPowell::new(2);
    assert!(!pw.is_done());
}

/// Test 14: MathPowell::new(n) result() slice length equals n.
#[test]
fn powell_new_result_slice_length() {
    let pw = MathPowell::new(3);
    assert_eq!(pw.result().len(), 3);
}

/// Test 15: Minimizing f(x) = x² (1D via Powell) from x=5 finds minimum near 0.
#[test]
fn powell_1d_x_squared_minimum_near_zero() {
    let mut pw = MathPowell::new(1);
    let f = |x: &[f64]| x[0] * x[0];
    pw.perform(&f, vec![5.0], 1e-8, 200);
    assert!(pw.is_done());
    assert!(
        pw.result()[0].abs() < 1e-5,
        "expected result near 0, got {}",
        pw.result()[0]
    );
    assert!(pw.value() < 1e-10, "value={}", pw.value());
}

/// Test 16: Minimizing f(x, y) = x² + y² from (3, -4) finds (0, 0).
#[test]
fn powell_2d_bowl_finds_origin() {
    let mut pw = MathPowell::new(2);
    let f = |x: &[f64]| x[0] * x[0] + x[1] * x[1];
    pw.perform(&f, vec![3.0, -4.0], 1e-8, 200);
    assert!(pw.is_done());
    assert!(
        pw.result()[0].abs() < 1e-5,
        "x={}",
        pw.result()[0]
    );
    assert!(
        pw.result()[1].abs() < 1e-5,
        "y={}",
        pw.result()[1]
    );
    assert!(pw.value() < 1e-10, "value={}", pw.value());
}

/// Test 17: nb_iter() is positive after actual work.
#[test]
fn powell_nb_iter_positive_after_work() {
    let mut pw = MathPowell::new(2);
    let f = |x: &[f64]| x[0] * x[0] + x[1] * x[1];
    pw.perform(&f, vec![5.0, 5.0], 1e-8, 200);
    assert!(pw.nb_iter() > 0, "expected nb_iter > 0");
}

/// Test 18: Minimizing a shifted 3D paraboloid finds the correct minimum.
#[test]
fn powell_3d_shifted_paraboloid() {
    let mut pw = MathPowell::new(3);
    // f(x,y,z) = (x-1)^2 + (y-2)^2 + (z-3)^2, minimum at (1,2,3)
    let f = |x: &[f64]| {
        (x[0] - 1.0).powi(2) + (x[1] - 2.0).powi(2) + (x[2] - 3.0).powi(2)
    };
    pw.perform(&f, vec![0.0, 0.0, 0.0], 1e-8, 300);
    assert!(pw.is_done());
    assert_near(pw.result()[0], 1.0, 1e-4, "x at minimum");
    assert_near(pw.result()[1], 2.0, 1e-4, "y at minimum");
    assert_near(pw.result()[2], 3.0, 1e-4, "z at minimum");
    assert!(pw.value() < 1e-8, "value={}", pw.value());
}

/// Test 19: nb_iter() does not exceed max_iter.
#[test]
fn powell_nb_iter_bounded_by_max_iter() {
    let max_iter = 5u32;
    let mut pw = MathPowell::new(2);
    let f = |x: &[f64]| x[0] * x[0] + x[1] * x[1];
    pw.perform(&f, vec![10.0, 10.0], 1e-30, max_iter);
    assert!(
        pw.nb_iter() <= max_iter,
        "nb_iter={} > max_iter={}",
        pw.nb_iter(),
        max_iter
    );
}

/// Test 20: value() matches f evaluated at result().
#[test]
fn powell_value_matches_f_at_result() {
    let mut pw = MathPowell::new(2);
    let f = |x: &[f64]| (x[0] - 1.0).powi(2) + (x[1] + 1.0).powi(2);
    pw.perform(&f, vec![5.0, -5.0], 1e-8, 200);
    let recomputed = f(pw.result());
    assert_near(pw.value(), recomputed, 1e-10, "value vs f(result)");
}
