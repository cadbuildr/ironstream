// FILE: tests/occt_fair_curve.rs
extern crate ironstream;

use ironstream::fair_curve::{
    FairCurveBSpline, FairCurveBattenCurve, FairCurveConstraint,
    FairCurveMinimalVariation,
};

#[test]
fn minimal_variation_is_not_done_before_compute() {
    let mv = FairCurveMinimalVariation::new([0.0, 0.0], [1.0, 0.0]);
    assert!(!mv.is_done());
}

#[test]
fn minimal_variation_compute_returns_ok() {
    let mut mv = FairCurveMinimalVariation::new([0.0, 0.0], [1.0, 0.0]);
    assert_eq!(mv.compute(), FairCurveConstraint::Ok);
}

#[test]
fn minimal_variation_is_done_after_compute() {
    let mut mv = FairCurveMinimalVariation::new([0.0, 0.0], [1.0, 0.0]);
    mv.compute();
    assert!(mv.is_done());
}

#[test]
fn minimal_variation_nb_poles_is_four() {
    let mut mv = FairCurveMinimalVariation::new([0.0, 0.0], [4.0, 0.0]);
    mv.compute();
    assert_eq!(mv.nb_poles(), 4);
}

#[test]
fn minimal_variation_degree_is_three() {
    let mut mv = FairCurveMinimalVariation::new([0.0, 0.0], [4.0, 0.0]);
    mv.compute();
    assert_eq!(mv.degree(), 3);
}

#[test]
fn minimal_variation_code_is_ok_after_compute() {
    let mut mv = FairCurveMinimalVariation::new([1.0, 2.0], [5.0, 3.0]);
    mv.compute();
    assert_eq!(mv.code(), FairCurveConstraint::Ok);
}

#[test]
fn minimal_variation_sliding_setter_does_not_prevent_compute() {
    let mut mv = FairCurveMinimalVariation::new([0.0, 0.0], [1.0, 1.0]);
    mv.set_sliding(true);
    assert_eq!(mv.compute(), FairCurveConstraint::Ok);
}

#[test]
fn minimal_variation_nb_iter_setter_accepted() {
    let mut mv = FairCurveMinimalVariation::new([0.0, 0.0], [1.0, 0.0]);
    mv.set_nb_iter(200);
    mv.compute();
    assert!(mv.is_done());
}

#[test]
fn minimal_variation_tolerance_setter_accepted() {
    let mut mv = FairCurveMinimalVariation::new([0.0, 0.0], [1.0, 0.0]);
    mv.set_tolerance(1e-10);
    mv.compute();
    assert!(mv.is_done());
}

#[test]
fn bspline_knot_count_formula() {
    let poles = vec![[0.0, 0.0], [1.0, 0.5], [2.0, 0.5], [3.0, 0.0]];
    let bs = FairCurveBSpline::new(poles, 3);
    assert_eq!(bs.nb_knots, 8);
}

#[test]
fn bspline_stores_poles_and_degree() {
    let poles = vec![[0.0, 0.0], [1.0, 1.0], [2.0, 0.0]];
    let bs = FairCurveBSpline::new(poles, 2);
    assert_eq!(bs.degree, 2);
    assert_eq!(bs.poles.len(), 3);
    assert_eq!(bs.poles[1], [1.0, 1.0]);
}

#[test]
fn batten_curve_not_done_before_compute() {
    let bc = FairCurveBattenCurve::new([0.0, 0.0], [1.0, 0.0], 0.5);
    assert!(!bc.is_done());
}

#[test]
fn batten_curve_compute_ok_and_done() {
    let mut bc = FairCurveBattenCurve::new([0.0, 0.0], [3.0, 0.0], 1.0);
    let code = bc.compute();
    assert_eq!(code, FairCurveConstraint::Ok);
    assert!(bc.is_done());
}

#[test]
fn batten_curve_height_roundtrip() {
    let mut bc = FairCurveBattenCurve::new([0.0, 0.0], [1.0, 0.0], 0.25);
    assert_eq!(bc.height(), 0.25);
    bc.set_height(1.5);
    assert_eq!(bc.height(), 1.5);
}

#[test]
fn batten_curve_sliding_does_not_prevent_compute() {
    let mut bc = FairCurveBattenCurve::new([0.0, 0.0], [2.0, 0.0], 0.5);
    bc.set_sliding(true);
    assert_eq!(bc.compute(), FairCurveConstraint::Ok);
}
