// FILE: rust/ironstream/crates/ironstream/tests/occt_math_kronrod.rs
extern crate ironstream;
use ironstream::math_kronrod::*;

use std::f64::consts::PI;

#[test]
fn kronrod_x_squared_on_0_1() {
    // integral of x^2 on [0,1] = 1/3
    let mut k = MathKronrodIntegration::new();
    let v = k.perform(&|x| x * x, 0.0, 1.0, 1e-10);
    assert!((v - 1.0 / 3.0).abs() < 1e-10, "got {v}");
    assert!(k.is_done());
    assert_eq!(k.value(), v);
}

#[test]
fn kronrod_sin_on_0_pi() {
    // integral of sin(x) on [0, pi] = 2
    let mut k = MathKronrodIntegration::new();
    let v = k.perform(&|x| x.sin(), 0.0, PI, 1e-10);
    assert!((v - 2.0).abs() < 1e-10, "got {v}");
    assert!(k.is_done());
    assert_eq!(k.value(), v);
}

#[test]
fn kronrod_new_is_zero() {
    let k = MathKronrodIntegration::new();
    assert_eq!(k.value(), 0.0);
    assert_eq!(k.error_estimate(), 0.0);
    assert_eq!(k.nb_iter(), 0);
    assert!(!k.is_done());
}

#[test]
fn kronrod_nb_iter_is_15() {
    // G7K15 uses exactly 15 function evaluations per call.
    let mut k = MathKronrodIntegration::new();
    k.perform(&|x| x, 0.0, 1.0, 1e-6);
    assert_eq!(k.nb_iter(), 15);
}

#[test]
fn kronrod_error_estimate_small_for_polynomial() {
    // x^2 is a low-degree polynomial; K15 and G7 should agree to machine precision.
    let mut k = MathKronrodIntegration::new();
    k.perform(&|x| x * x, 0.0, 1.0, 1e-12);
    assert!(k.error_estimate() < 1e-12, "error_estimate={}", k.error_estimate());
}

#[test]
fn kronrod_negative_interval() {
    // integral of x^2 on [-1,1] = 2/3
    let mut k = MathKronrodIntegration::new();
    let v = k.perform(&|x| x * x, -1.0, 1.0, 1e-12);
    assert!((v - 2.0 / 3.0).abs() < 1e-12, "got {v}");
}

#[test]
fn kronrod_constant_function() {
    // integral of 5 on [0,1] = 5
    let mut k = MathKronrodIntegration::new();
    let v = k.perform(&|_| 5.0, 0.0, 1.0, 1e-12);
    assert!((v - 5.0).abs() < 1e-12, "got {v}");
}

#[test]
fn kronrod_exp_function() {
    // integral of e^x on [0,1] = e - 1
    let expected = std::f64::consts::E - 1.0;
    let mut k = MathKronrodIntegration::new();
    let v = k.perform(&|x| x.exp(), 0.0, 1.0, 1e-12);
    assert!((v - expected).abs() < 1e-12, "got {v}");
}
