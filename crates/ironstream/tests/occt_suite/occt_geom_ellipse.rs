extern crate ironstream;
use ironstream::geom_ellipse::*;
use std::f64::consts::PI;

#[test]
fn test_geom_ellipse_basic() {
    let e = GeomEllipse::new([0.0, 0.0, 0.0], 10.0, 5.0);
    assert!((e.major_radius() - 10.0).abs() < 1e-10);
    assert!((e.minor_radius() - 5.0).abs() < 1e-10);
    assert_eq!(e.center, [0.0, 0.0, 0.0]);
}

#[test]
fn test_geom_ellipse_point_at() {
    let e = GeomEllipse::new([0.0, 0.0, 0.0], 10.0, 5.0);
    let pt0 = e.point_at(0.0);
    assert!((pt0[0] - 10.0).abs() < 1e-10);
    assert!(pt0[1].abs() < 1e-10);

    let pt_half = e.point_at(PI / 2.0);
    assert!(pt_half[0].abs() < 1e-10);
    assert!((pt_half[1] - 5.0).abs() < 1e-10);
}

#[test]
fn test_geom_ellipse_eccentricity() {
    let e = GeomEllipse::new([0.0, 0.0, 0.0], 10.0, 5.0);
    let expected = (100.0_f64 - 25.0).sqrt() / 10.0;
    assert!((e.eccentricity() - expected).abs() < 1e-10);
}

#[test]
fn test_geom_ellipse_focal() {
    let e = GeomEllipse::new([0.0, 0.0, 0.0], 10.0, 5.0);
    let expected = 2.0 * (100.0_f64 - 25.0).sqrt();
    assert!((e.focal() - expected).abs() < 1e-10);
}

#[test]
fn test_geom_ellipse_parameter() {
    let e = GeomEllipse::new([0.0, 0.0, 0.0], 10.0, 5.0);
    assert!((e.parameter() - 2.5).abs() < 1e-10);
}

#[test]
fn test_geom_ellipse_closed_periodic() {
    let e = GeomEllipse::new([0.0, 0.0, 0.0], 3.0, 2.0);
    assert!(e.is_closed());
    assert!(e.is_periodic());
    assert!((e.first_parameter() - 0.0).abs() < 1e-15);
    assert!((e.last_parameter() - 2.0 * PI).abs() < 1e-15);
}

#[test]
fn test_geom_ellipse_foci() {
    let e = GeomEllipse::new([0.0, 0.0, 0.0], 10.0, 5.0);
    let f1 = e.focus1();
    let f2 = e.focus2();
    let c = (100.0_f64 - 25.0).sqrt();
    assert!((f1[0] - c).abs() < 1e-10);
    assert!((f2[0] + c).abs() < 1e-10);
}

#[test]
fn test_geom_ellipse_reverse() {
    let mut e = GeomEllipse::new([0.0, 0.0, 0.0], 5.0, 3.0);
    e.reverse();
    assert!(e.normal[2] < 0.0);
}
