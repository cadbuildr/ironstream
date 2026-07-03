// FILE: rust/ironstream/crates/ironstream/tests/occt_math_bfgs.rs
extern crate ironstream;
use ironstream::math_bfgs::*;

fn assert_near(actual: f64, expected: f64, tol: f64, msg: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{msg}: |{actual} - {expected}| = {} > {tol}",
        (actual - expected).abs()
    );
}

/// Test 1: MathBfgsResult::new() starts with is_done=false and nb_iter=0.
#[test]
fn result_new_is_not_done() {
    let r = MathBfgsResult::new();
    assert!(!r.is_done());
    assert_eq!(r.nb_iter(), 0);
}

/// Test 2: MathBfgsResult::new() value() is infinite (not yet computed).
#[test]
fn result_new_value_is_infinite() {
    let r = MathBfgsResult::new();
    assert!(r.value().is_infinite());
}

/// Test 3: MathBfgsResult::minimum() slice has length 8.
#[test]
fn result_minimum_slice_length() {
    let r = MathBfgsResult::new();
    assert_eq!(r.minimum().len(), 8);
}

/// Test 4: MathBfgs::new constructs without panic for valid n <= 8.
#[test]
fn bfgs_new_valid_dimension() {
    let _ = MathBfgs::new(1, 1e-8, 100);
    let _ = MathBfgs::new(4, 1e-6, 50);
    let _ = MathBfgs::new(8, 1e-10, 200);
}

/// Test 5: Before perform, is_done() returns false.
#[test]
fn bfgs_not_done_before_perform() {
    let bfgs = MathBfgs::new(1, 1e-8, 100);
    assert!(!bfgs.is_done());
}

/// Test 6: Minimizing f(x) = x² from x=3.0 finds minimum near x=0.
#[test]
fn bfgs_1d_x_squared_minimum_near_zero() {
    let mut bfgs = MathBfgs::new(1, 1e-8, 300);
    let f = |x: &[f64]| x[0] * x[0];
    let g = |x: &[f64]| vec![2.0 * x[0]];
    bfgs.perform(&f, &g, &[3.0]);
    assert!(bfgs.is_done(), "is_done must be true after perform");
    assert!(
        bfgs.minimum()[0].abs() < 1e-6,
        "expected minimum near 0, got {}",
        bfgs.minimum()[0]
    );
}

/// Test 7: Minimizing f(x) = x² gives value near 0.
#[test]
fn bfgs_1d_x_squared_value_near_zero() {
    let mut bfgs = MathBfgs::new(1, 1e-8, 300);
    let f = |x: &[f64]| x[0] * x[0];
    let g = |x: &[f64]| vec![2.0 * x[0]];
    bfgs.perform(&f, &g, &[3.0]);
    assert!(bfgs.value() < 1e-12, "expected value near 0, got {}", bfgs.value());
}

/// Test 8: nb_iter() is greater than 0 when starting away from minimum.
#[test]
fn bfgs_nb_iter_positive_after_convergence() {
    let mut bfgs = MathBfgs::new(1, 1e-8, 300);
    let f = |x: &[f64]| x[0] * x[0];
    let g = |x: &[f64]| vec![2.0 * x[0]];
    bfgs.perform(&f, &g, &[5.0]);
    assert!(bfgs.nb_iter() > 0, "nb_iter must be > 0 after actual work");
}

/// Test 9: nb_iter() is 0 when starting exactly at minimum (gradient is zero).
#[test]
fn bfgs_nb_iter_zero_when_already_at_minimum() {
    let mut bfgs = MathBfgs::new(1, 1e-8, 100);
    let f = |x: &[f64]| x[0] * x[0];
    let g = |x: &[f64]| vec![2.0 * x[0]];
    bfgs.perform(&f, &g, &[0.0]);
    assert_eq!(bfgs.nb_iter(), 0, "zero iterations when gradient is already zero");
}

/// Test 10: Minimizing f(x) = x² from a negative start (-4.0) also works.
#[test]
fn bfgs_1d_x_squared_from_negative_start() {
    let mut bfgs = MathBfgs::new(1, 1e-8, 300);
    let f = |x: &[f64]| x[0] * x[0];
    let g = |x: &[f64]| vec![2.0 * x[0]];
    bfgs.perform(&f, &g, &[-4.0]);
    assert!(
        bfgs.minimum()[0].abs() < 1e-6,
        "expected minimum near 0, got {}",
        bfgs.minimum()[0]
    );
    assert!(bfgs.value() < 1e-12, "value={}", bfgs.value());
}

/// Test 11: Minimizing f(x) = (x - 5)² finds minimum near x=5.
#[test]
fn bfgs_1d_shifted_parabola_finds_minimum() {
    let mut bfgs = MathBfgs::new(1, 1e-8, 300);
    let f = |x: &[f64]| (x[0] - 5.0).powi(2);
    let g = |x: &[f64]| vec![2.0 * (x[0] - 5.0)];
    bfgs.perform(&f, &g, &[0.0]);
    assert!(bfgs.is_done());
    assert_near(bfgs.minimum()[0], 5.0, 1e-5, "minimum x");
    assert!(bfgs.value() < 1e-10, "value={}", bfgs.value());
}

/// Test 12: Minimizing 2D f(x,y) = x² + y² from (2, -3) finds (0, 0).
#[test]
fn bfgs_2d_x_squared_plus_y_squared() {
    let mut bfgs = MathBfgs::new(2, 1e-8, 500);
    let f = |x: &[f64]| x[0] * x[0] + x[1] * x[1];
    let g = |x: &[f64]| vec![2.0 * x[0], 2.0 * x[1]];
    bfgs.perform(&f, &g, &[2.0, -3.0]);
    assert!(bfgs.is_done());
    assert!(
        bfgs.minimum()[0].abs() < 1e-5,
        "x={}",
        bfgs.minimum()[0]
    );
    assert!(
        bfgs.minimum()[1].abs() < 1e-5,
        "y={}",
        bfgs.minimum()[1]
    );
    assert!(bfgs.value() < 1e-10, "value={}", bfgs.value());
}

/// Test 13: value() accessor matches f evaluated at minimum().
#[test]
fn bfgs_value_matches_f_at_minimum() {
    let mut bfgs = MathBfgs::new(1, 1e-8, 300);
    let f = |x: &[f64]| x[0] * x[0];
    let g = |x: &[f64]| vec![2.0 * x[0]];
    bfgs.perform(&f, &g, &[2.5]);
    let recomputed = f(bfgs.minimum());
    assert_near(bfgs.value(), recomputed, 1e-12, "value vs f(minimum)");
}

/// Test 14: nb_iter() does not exceed max_iter.
#[test]
fn bfgs_nb_iter_does_not_exceed_max_iter() {
    let max_iter = 10u32;
    let mut bfgs = MathBfgs::new(1, 1e-30, max_iter); // tiny tolerance to avoid early stop
    let f = |x: &[f64]| x[0] * x[0];
    let g = |x: &[f64]| vec![2.0 * x[0]];
    bfgs.perform(&f, &g, &[1.0]);
    assert!(
        bfgs.nb_iter() <= max_iter,
        "nb_iter={} > max_iter={}",
        bfgs.nb_iter(),
        max_iter
    );
}

/// Test 15: minimum() slice returned by MathBfgs has length 8.
#[test]
fn bfgs_minimum_slice_length_is_8() {
    let mut bfgs = MathBfgs::new(1, 1e-8, 100);
    let f = |x: &[f64]| x[0] * x[0];
    let g = |x: &[f64]| vec![2.0 * x[0]];
    bfgs.perform(&f, &g, &[1.0]);
    assert_eq!(bfgs.minimum().len(), 8);
}
