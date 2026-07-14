// Integration tests for law_func
use ironstream::law_func::*;

#[test]
fn law_linear_d1_derivative() {
    let l = LawLinear::new(0.0, 2.0, 0.0, 4.0);
    let (v, dv) = l.d1(1.0);
    assert!((v - 2.0).abs() < 1e-10);
    assert!((dv - 2.0).abs() < 1e-10);
}

#[test]
fn law_linear_bounds() {
    let l = LawLinear::new(1.0, 3.0, 10.0, 20.0);
    let (a, b) = l.bounds();
    assert!((a - 1.0).abs() < 1e-12);
    assert!((b - 3.0).abs() < 1e-12);
    assert_eq!(l.continuity(), LawContinuity::C1);
    assert!(!l.is_constant());
}

#[test]
fn law_interpol_midpoint_interpolated() {
    let mut li = LawInterpol::new();
    li.set_cubic(vec![0.0, 1.0], vec![3.0, 7.0], false);
    assert!(li.is_done());
    let mid = li.value(0.5);
    assert!((mid - 5.0).abs() < 1e-10);
    assert_eq!(li.nb_intervals(), 1);
}

#[test]
fn law_composite_bounds() {
    let mut c = LawComposite::new();
    c.add_segment(LawSegment::linear(0.0, 0.3, 1.0, 2.0));
    c.add_segment(LawSegment::constant(0.3, 0.7, 2.0));
    c.add_segment(LawSegment::linear(0.7, 1.0, 2.0, 3.0));
    assert_eq!(c.nb_laws(), 3);
    let (a, b) = c.bounds();
    assert!((a - 0.0).abs() < 1e-12);
    assert!((b - 1.0).abs() < 1e-12);
    assert!(!c.is_empty());
    assert!(c.law_at(1).is_some());
}

#[test]
fn law_bspfunc_degree_and_bounds() {
    let f = LawBSpFunc::new(
        2,
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        vec![1.0, 2.0, 1.0],
        vec![3, 3],
    );
    assert_eq!(f.degree(), 2);
    assert_eq!(f.nb_poles(), 3);
    assert_eq!(f.nb_knots(), 6);
    let (a, b) = f.bounds();
    assert!(a < b);
}

#[test]
fn law_interpol_empty_returns_zero() {
    let li = LawInterpol::new();
    assert!(!li.is_done());
    assert_eq!(li.nb_intervals(), 0);
    assert!((li.value(0.5) - 0.0).abs() < 1e-12);
}
