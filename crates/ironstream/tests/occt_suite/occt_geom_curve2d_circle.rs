extern crate ironstream;
use ironstream::geom_curve2d_circle::*;
use std::f64::consts::PI;

#[test]
fn test_geom_curve2d_circle_basic() {
    let c = Circle2dCurve::new(0.0, 0.0, 1.0);
    assert!((c.radius - 1.0).abs() < 1e-10);

    let pt = c.point_at(0.0);
    assert!((pt[0] - 1.0).abs() < 1e-10);
    assert!(pt[1].abs() < 1e-10);

    let pt2 = c.point_at(PI / 2.0);
    assert!(pt2[0].abs() < 1e-10);
    assert!((pt2[1] - 1.0).abs() < 1e-10);

    let d1 = c.d1(0.0);
    assert!(d1[0].abs() < 1e-10);
    assert!((d1[1] - 1.0).abs() < 1e-10);

    let d2 = c.d2(0.0);
    assert!((d2[0] + 1.0).abs() < 1e-10);
    assert!(d2[1].abs() < 1e-10);

    assert_eq!(Circle2dCurve::first_parameter(), 0.0);
    assert!((Circle2dCurve::last_parameter() - 2.0 * PI).abs() < 1e-10);
    assert!(Circle2dCurve::is_closed());
    assert!(Circle2dCurve::is_periodic());
    assert!((Circle2dCurve::period() - 2.0 * PI).abs() < 1e-10);

    let dist = c.distance_to([0.0, 0.0]);
    assert!((dist - 1.0).abs() < 1e-10);

    let mut c2 = Circle2dCurve::new(0.0, 2.0, 3.0);
    c2.reverse();
    assert!((c2.center[1] + 2.0).abs() < 1e-10);
}
