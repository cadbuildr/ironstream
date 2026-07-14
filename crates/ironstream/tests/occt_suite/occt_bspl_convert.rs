// Integration tests for bspl_convert module
use ironstream::bspl_convert::*;

#[test]
fn check_knots_multiplicities_valid() {
    // degree 2, knots [0,1], mults [3,3] => sum=6, n_poles=6-3=3 > 0 => Ok
    let err = BsplCLib::check_knots_multiplicities(2, &[0.0, 1.0], &[3, 3]);
    assert!(err.is_ok());
}

#[test]
fn check_knots_multiplicities_decreasing_knots() {
    let err = BsplCLib::check_knots_multiplicities(2, &[1.0, 0.0], &[3, 3]);
    assert_eq!(err, BsplError::BadKnots);
}

#[test]
fn flatten_knots_basic() {
    let flat = BsplCLib::flatten_knots(&[0.0, 0.5, 1.0], &[2, 1, 2]);
    assert_eq!(flat, vec![0.0, 0.0, 0.5, 1.0, 1.0]);
}

#[test]
fn nb_poles_calculation() {
    // degree 3, mults [4,4] => sum=8, nb_poles=8-4=4
    assert_eq!(BsplCLib::nb_poles(3, &[4, 4]), 4);
}

#[test]
fn eval_curve_linear_interpolation() {
    // degree-1 B-spline with 2 poles should interpolate linearly
    let poles = [[0.0_f64, 0.0, 0.0], [1.0, 2.0, 0.0]];
    let flat = vec![0.0, 0.0, 1.0, 1.0];
    let p = BsplCLib::eval_curve(0.5, 1, &poles, &flat);
    assert!((p[0] - 0.5).abs() < 1e-10);
    assert!((p[1] - 1.0).abs() < 1e-10);
    assert!((p[2]).abs() < 1e-10);
}

#[test]
fn plib_bernstein_and_poly_eval() {
    // Bernstein linear: (1-u)*a + u*b at u=0.3, coeffs=[1,3] => 0.7+0.9=1.6
    let b = PLib::eval_bernstein(0.3, &[1.0, 3.0]);
    assert!((b - (0.7 * 1.0 + 0.3 * 3.0)).abs() < 1e-10);
    // Power basis: 1 + 2u + 3u^2 at u=2 => 1+4+12=17
    let p = PLib::eval_poly(2.0, &[1.0, 2.0, 3.0]);
    assert!((p - 17.0).abs() < 1e-10);
}
