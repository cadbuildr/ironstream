use ironstream::geom_conic::*;

#[test]
fn parabola_vertex() {
    let p = GeomParabola::new(1, 2.0);
    let v = p.value(0.0);
    assert!(v[0].abs() < 1e-10 && v[1].abs() < 1e-10);
    assert!(!p.is_closed());
}

#[test]
fn parabola_eccentricity() {
    let p = GeomParabola::new(1, 1.0);
    assert!((p.eccentricity() - 1.0).abs() < 1e-12);
}

#[test]
fn hyperbola_eccentricity() {
    let h = GeomHyperbola::new(1, 3.0, 4.0);
    assert!((h.eccentricity() - 5.0/3.0).abs() < 1e-10);
    assert!((h.focal_distance() - 5.0).abs() < 1e-10);
}

#[test]
fn hyperbola_vertex() {
    let h = GeomHyperbola::new(1, 2.0, 1.0);
    let v = h.value(0.0);
    assert!((v[0] - 2.0).abs() < 1e-10);
    assert!(v[1].abs() < 1e-10);
}

#[test]
fn trimmed_curve_range() {
    use std::f64::consts::PI;
    let t = GeomTrimmedCurve::new(5, 0.0, PI);
    assert!((t.parameter_range() - PI).abs() < 1e-10);
    assert!(t.is_valid_param(PI/2.0));
    assert!(!t.is_valid_param(PI + 1.0));
}

#[test]
fn offset_curve_ext() {
    let oc = GeomOffsetCurveExt::new(3, 1.0, [0.0, 1.0, 0.0]);
    let p = oc.value_at([0.0, 0.0, 0.0]);
    assert!((p[1] - 1.0).abs() < 1e-10);
    assert!(oc.is_done());
}
