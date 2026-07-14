extern crate ironstream;
use ironstream::geom_circle::*;
use std::f64::consts::PI;

#[test]
fn test_geom_circle_basic() {
    let c = GeomCircle::new([0.0, 0.0, 0.0], 5.0);
    assert!((c.radius - 5.0).abs() < 1e-10);
    assert_eq!(c.normal, [0.0, 0.0, 1.0]);
    assert_eq!(c.x_axis, [1.0, 0.0, 0.0]);
}

#[test]
fn test_geom_circle_point_at() {
    let c = GeomCircle::new([0.0, 0.0, 0.0], 1.0);
    let pt = c.point_at(0.0);
    assert!((pt[0] - 1.0).abs() < 1e-10);
    assert!(pt[1].abs() < 1e-10);
    assert!(pt[2].abs() < 1e-10);

    let pt2 = c.point_at(PI / 2.0);
    assert!(pt2[0].abs() < 1e-10);
    assert!((pt2[1] - 1.0).abs() < 1e-10);
}

#[test]
fn test_geom_circle_closed_periodic() {
    let c = GeomCircle::new([0.0, 0.0, 0.0], 1.0);
    assert!(c.is_closed());
    assert!(c.is_periodic());
    assert!((c.period() - 2.0 * PI).abs() < 1e-10);
    assert_eq!(c.first_parameter(), 0.0);
    assert!((c.last_parameter() - 2.0 * PI).abs() < 1e-10);
}

#[test]
fn test_geom_circle_reverse() {
    let mut c = GeomCircle::new([0.0, 0.0, 0.0], 1.0);
    c.reverse();
    assert_eq!(c.normal, [0.0, 0.0, -1.0]);
}

#[test]
fn test_circle_from_3pts() {
    let p1 = [1.0_f64, 0.0, 0.0];
    let p2 = [0.0, 1.0, 0.0];
    let p3 = [-1.0, 0.0, 0.0];
    let gc = circle_from_3pts(p1, p2, p3).expect("should succeed");
    assert!((gc.radius - 1.0).abs() < 1e-9);

    let col = circle_from_3pts([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
    assert!(col.is_none());
}
