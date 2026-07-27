use ironstream::gcpnts_::*;

#[test]
fn abscissa_point_done() {
    let a = GcpntsAbscissaPoint::new(1, 0.5, 0.0, 0.0, 1.0);
    assert!(a.is_done());
    assert!((a.parameter() - 0.5).abs() < 1e-10);
}

#[test]
fn abscissa_point_null() {
    let a = GcpntsAbscissaPoint::new(0, 0.5, 0.0, 0.0, 1.0);
    assert!(!a.is_done());
}

#[test]
fn uniform_abscissa_by_nb() {
    let u = GcpntsUniformAbscissa::new_by_nb(1, 5, 0.0, 1.0);
    assert!(u.is_done());
    assert_eq!(u.nb_points(), 5);
    assert_eq!(u.parameter(1), Some(0.0));
    assert!((u.parameter(5).unwrap() - 1.0).abs() < 1e-10);
    assert!(u.parameter(0).is_none());
}

#[test]
fn uniform_deflection_basic() {
    let u = GcpntsUniformDeflection::new(1, 0.1, 0.0, 1.0);
    assert!(u.is_done());
    assert!(u.nb_points() >= 2);
}

#[test]
fn quasi_uniform_abscissa() {
    let q = GcpntsQuasiUniformAbscissa::new(1, 4, 0.0, 3.0);
    assert!(q.is_done());
    assert_eq!(q.nb_points(), 4);
    assert!((q.parameter(4).unwrap() - 3.0).abs() < 1e-10);
}
