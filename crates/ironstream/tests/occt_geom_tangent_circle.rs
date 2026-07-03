extern crate ironstream;
use ironstream::geom_tangent_circle::*;

#[test]
fn test_circumcircle_equilateral() {
    let s = 2.0f64;
    let h = s * 3.0f64.sqrt() / 2.0;
    let p1 = [0.0, 0.0];
    let p2 = [s, 0.0];
    let p3 = [s / 2.0, h];
    let c = Circ2d3Tan::circumcircle(p1, p2, p3).unwrap();
    // Circumradius = s / sqrt(3)
    assert!((c.radius - s / 3.0f64.sqrt()).abs() < 1e-8);
}

#[test]
fn test_radical_axis_equal_power() {
    let c1 = Circle2d::new([0.0, 0.0], 3.0);
    let c2 = Circle2d::new([5.0, 0.0], 2.0);
    let (pt, _dir) = Circ2dBisec::radical_axis(&c1, &c2).unwrap();
    let power1 = pt[0] * pt[0] - 9.0;
    let power2 = (pt[0] - 5.0) * (pt[0] - 5.0) - 4.0;
    assert!((power1 - power2).abs() < 1e-8);
}

#[test]
fn test_tangent_count_separate_circles() {
    let c1 = Circle2d::new([0.0, 0.0], 1.0);
    let c2 = Circle2d::new([10.0, 0.0], 1.0);
    assert_eq!(Lin2d2Tan::nb_external_tangents(&c1, &c2), 4);
}

#[test]
fn test_angle_bisectors_perpendicular() {
    let d1 = [1.0, 0.0];
    let d2 = [0.0, 1.0];
    let (b1, b2) = Lin2dBisec::angle_bisectors(d1, d2);
    let dot = b1[0] * b2[0] + b1[1] * b2[1];
    assert!(dot.abs() < 1e-10);
    // b1 should be at 45 degrees
    assert!((b1[0] - 1.0 / 2.0f64.sqrt()).abs() < 1e-10);
}

#[test]
fn test_circle_area_and_tangency() {
    let c1 = Circle2d::new([0.0, 0.0], 2.0);
    let c2 = Circle2d::new([5.0, 0.0], 3.0);
    assert!(c1.is_tangent_to(&c2, 1e-8));
    assert!((c1.area() - std::f64::consts::PI * 4.0).abs() < 1e-8);
}
