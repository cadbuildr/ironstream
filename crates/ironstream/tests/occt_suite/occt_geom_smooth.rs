extern crate ironstream;
use ironstream::geom_smooth::*;

#[test]
fn occt_smooth_params_default() {
    let p = SmoothParams::default();
    assert_eq!(p.continuity, 1);
    assert!(p.tolerance > 0.0);
}

#[test]
fn occt_smooth_params_new() {
    let p = SmoothParams::new(2, 1e-5, 4, 8);
    assert_eq!(p.continuity, 2);
    assert_eq!(p.max_degree, 4);
}

#[test]
fn occt_curve_smooth_evaluate() {
    let mut cs = CurveSmooth::new(3);
    cs.add_point([0.0, 0.0, 0.0]);
    cs.add_point([1.0, 0.0, 0.0]);
    let p = cs.evaluate(0.5);
    assert!((p[0] - 0.5).abs() < 1e-10);
}
