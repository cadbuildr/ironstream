extern crate ironstream;
use ironstream::geom_curve2d_ellipse::*;
use std::f64::consts::PI;

#[test]
fn test_geom_curve2d_ellipse_basic() {
    let e = Ellipse2dCurve::new(0.0, 0.0, 3.0, 2.0);
    assert!((e.major_radius - 3.0).abs() < 1e-10);
    assert!((e.minor_radius - 2.0).abs() < 1e-10);

    let pt0 = e.point_at(0.0);
    assert!((pt0[0] - 3.0).abs() < 1e-10);
    assert!(pt0[1].abs() < 1e-10);

    let pt_half = e.point_at(PI / 2.0);
    assert!(pt_half[0].abs() < 1e-10);
    assert!((pt_half[1] - 2.0).abs() < 1e-10);

    let d1 = e.d1(0.0);
    assert!(d1[0].abs() < 1e-10);

    assert_eq!(Ellipse2dCurve::first_parameter(), 0.0);
    assert!((Ellipse2dCurve::last_parameter() - 2.0 * PI).abs() < 1e-10);
    assert!(Ellipse2dCurve::is_closed());
    assert!(Ellipse2dCurve::is_periodic());

    let ecc = e.eccentricity();
    assert!(ecc > 0.0 && ecc < 1.0);

    let f1 = e.focus1();
    let f2 = e.focus2();
    assert!((f1[0] + f2[0]).abs() < 1e-10);

    let er = e.with_rotation(PI / 4.0);
    assert!((er.angle - PI / 4.0).abs() < 1e-10);

    let mut e2 = Ellipse2dCurve::new(0.0, 0.0, 3.0, 2.0);
    e2.reverse();
    assert!(e2.minor_radius < 0.0);
}
