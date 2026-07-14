extern crate ironstream;
use ironstream::math_complex::*;

#[test]
fn test_complex_arithmetic() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let s = a.add(b);
    assert!((s.re - 4.0).abs() < 1e-12);
    assert!((s.im - 6.0).abs() < 1e-12);
    let p = a.mul(b);
    // (1+2i)(3+4i) = 3+4i+6i+8i^2 = 3-8 + 10i = -5 + 10i
    assert!((p.re + 5.0).abs() < 1e-12);
    assert!((p.im - 10.0).abs() < 1e-12);
}

#[test]
fn test_complex_modulus_and_div() {
    let c = Complex::new(3.0, 4.0);
    assert!((c.modulus() - 5.0).abs() < 1e-12);
    let r = Complex::ONE.div(Complex::I).unwrap();
    // 1/i = -i
    assert!(r.re.abs() < 1e-12);
    assert!((r.im + 1.0).abs() < 1e-12);
}

#[test]
fn test_complex_sqrt_of_minus_one() {
    let c = Complex::new(-1.0, 0.0);
    let s = c.sqrt_c();
    // sqrt(-1) = i (principal value)
    assert!((s.im - 1.0).abs() < 1e-10);
    assert!(s.re.abs() < 1e-10);
}

#[test]
fn test_gauss_elim_2x2() {
    let solver = GaussElim::new(2);
    // 2x + y = 5, x + 3y = 10 → x=1, y=3
    let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
    let b = vec![5.0, 10.0];
    let x = solver.solve(a, b).unwrap();
    assert!((x[0] - 1.0).abs() < 1e-8);
    assert!((x[1] - 3.0).abs() < 1e-8);
}

#[test]
fn test_power_iteration_dominant_eigenvalue() {
    // Diagonal matrix with eigenvalues 3, 1
    let a = vec![vec![3.0, 0.0], vec![0.0, 1.0]];
    let (ev, _) = PowerIteration::dominant_eigenvalue(&a, 50);
    assert!((ev - 3.0).abs() < 1e-6);
}
