// FILE: tests/occt_chord_dev.rs
extern crate ironstream;

use ironstream::chord_dev::{
    GcpntsAbscissaPoint, GcpntsUniformAbscissa, GcpntsTangentialDeflection,
};

#[test]
fn abscissa_point_result_param() {
    let a = GcpntsAbscissaPoint::new(1, 0.4, 0.1);
    assert!(a.is_done());
    // result_param = from_param + abscissa = 0.1 + 0.4 = 0.5
    let p = a.parameter().unwrap();
    assert!((p - 0.5).abs() < 1e-10);
}

#[test]
fn abscissa_point_null_curve_not_done() {
    let a = GcpntsAbscissaPoint::new(0, 0.5, 0.0);
    assert!(!a.is_done());
    assert!(a.parameter().is_none());
}

#[test]
fn abscissa_point_static_length() {
    let len = GcpntsAbscissaPoint::length(1, 1.0, 4.0);
    assert!((len - 3.0).abs() < 1e-10);
    // zero curve_id returns 0
    assert!((GcpntsAbscissaPoint::length(0, 0.0, 10.0) - 0.0).abs() < 1e-10);
}

#[test]
fn uniform_abscissa_nb_points_and_endpoints() {
    let u = GcpntsUniformAbscissa::new_nb_points(1, 6, 0.0, 1.0);
    assert!(u.is_done());
    assert_eq!(u.nb_points(), 6);
    // 1-based: param(1) = 0.0, param(6) = 1.0
    assert!((u.parameter(1).unwrap() - 0.0).abs() < 1e-10);
    assert!((u.parameter(6).unwrap() - 1.0).abs() < 1e-10);
    // index 0 returns None
    assert!(u.parameter(0).is_none());
}

#[test]
fn uniform_abscissa_null_curve_not_done() {
    let u = GcpntsUniformAbscissa::new_nb_points(0, 5, 0.0, 1.0);
    assert!(!u.is_done());
}

#[test]
fn tangential_deflection_basic() {
    let t = GcpntsTangentialDeflection::new(1, 0.0, 2.0, 0.1, 0.2);
    assert!(t.is_done());
    assert!(t.nb_points() >= 2);
    // first param should be 0.0
    assert!((t.parameter(1).unwrap() - 0.0).abs() < 1e-10);
    // last param should be 2.0
    let last = t.parameter(t.nb_points()).unwrap();
    assert!((last - 2.0).abs() < 1e-10);
}
