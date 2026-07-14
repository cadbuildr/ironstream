// FILE: tests/occt_law_s_interpolate.rs
extern crate ironstream;
use ironstream::law_s_interpolate::*;

#[test]
fn law_s_endpoints() {
    let s = LawS::new(0.0, 10.0);
    assert!((s.value(0.0)).abs() < 1e-10);
    assert!((s.value(1.0) - 10.0).abs() < 1e-10);
}

#[test]
fn law_s_midpoint_symmetric() {
    let s = LawS::new(0.0, 1.0);
    assert!((s.value(0.5) - 0.5).abs() < 1e-10);
}

#[test]
fn law_s_zero_derivative_at_bounds() {
    let s = LawS::new(0.0, 5.0);
    assert!(s.d1(0.0).abs() < 1e-10);
    assert!(s.d1(1.0).abs() < 1e-10);
}

#[test]
fn law_linear_interpolation() {
    let l = LawLinear::new(vec![0.0, 1.0, 2.0], vec![0.0, 5.0, 10.0]).unwrap();
    assert!((l.value(0.5) - 2.5).abs() < 1e-10);
    assert!((l.value(1.5) - 7.5).abs() < 1e-10);
    assert!((l.d1(0.5) - 5.0).abs() < 1e-10);
}

#[test]
fn law_linear_constant() {
    let l = LawLinear::constant(7.0);
    assert!(l.is_constant());
    assert!((l.value(0.0) - 7.0).abs() < 1e-10);
    assert!((l.value(1.0) - 7.0).abs() < 1e-10);
}

#[test]
fn law_interpol_endpoints() {
    let l = LawInterpol::new(vec![0.0, 0.5, 1.0], vec![1.0, 3.0, 2.0]).unwrap();
    assert!((l.value(0.0) - 1.0).abs() < 1e-10);
    assert!((l.value(1.0) - 2.0).abs() < 1e-10);
}
