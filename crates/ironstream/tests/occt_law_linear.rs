use ironstream::law_linear::*;

#[test]
fn law_linear_endpoints() {
    let l = LawLinear::new(0.0, 2.0, 1.0, 4.0);
    assert!((l.value(0.0) - 2.0).abs() < 1e-10);
    assert!((l.value(1.0) - 4.0).abs() < 1e-10);
    assert!((l.value(0.5) - 3.0).abs() < 1e-10);
}

#[test]
fn law_constant_d1() {
    let l = LawConstant::new(0.0, 1.0, 5.0);
    let (v, dv) = l.d1(0.5);
    assert!((v - 5.0).abs() < 1e-12);
    assert!(dv.abs() < 1e-12);
}

#[test]
fn law_interpol_three_points() {
    let mut l = LawInterpol::new();
    l.set(vec![0.0, 0.5, 1.0], vec![0.0, 5.0, 10.0]);
    assert!((l.value(0.25) - 2.5).abs() < 1e-9);
    assert_eq!(l.nb_points(), 3);
}

#[test]
fn law_s_midpoint() {
    let l = LawS::new(0.0, 0.0, 1.0, 10.0);
    assert!((l.value(0.5) - 5.0).abs() < 1e-9);
}

#[test]
fn law_composite_segments() {
    let mut c = LawComposite::new();
    c.add_constant_segment(0.0, 0.5, 1.0);
    c.add_constant_segment(0.5, 1.0, 2.0);
    assert_eq!(c.nb_laws(), 2);
    assert!((c.value(0.3) - 1.0).abs() < 1e-12);
    assert!((c.value(0.8) - 2.0).abs() < 1e-12);
}
