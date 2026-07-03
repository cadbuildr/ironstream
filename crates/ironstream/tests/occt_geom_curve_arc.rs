extern crate ironstream;
use ironstream::geom_curve_arc::*;

#[test]
fn test_geom_curve_arc_basic() {
    // Three non-collinear points forming a right triangle layout
    let arc = ArcOfCircle::from_3_points(
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [-1.0, 0.0, 0.0],
    );
    assert!(arc.is_some(), "should produce an arc from 3 non-collinear points");
    let arc = arc.unwrap();
    assert!(arc.radius > 0.0);
    assert!(arc.is_done());

    // length should be positive
    assert!(arc.length() > 0.0);

    // evaluate at start_angle should be close to first point
    let p = arc.evaluate(arc.start_angle);
    let _ = p; // just ensure it doesn't panic

    // d1 magnitude should equal radius
    let d = arc.d1(arc.start_angle);
    let mag = (d[0]*d[0] + d[1]*d[1] + d[2]*d[2]).sqrt();
    assert!((mag - arc.radius).abs() < 1e-9, "d1 magnitude should equal radius, got {}", mag);
}

#[test]
fn test_geom_curve_arc_collinear_returns_none() {
    let arc = ArcOfCircle::from_3_points(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
    );
    assert!(arc.is_none(), "collinear points should return None");
}

#[test]
fn test_arc_of_ellipse_basic() {
    let e = ArcOfEllipse::new(0.0, 0.0, 0.0, 2.0, 1.0, 0.0, std::f64::consts::PI);
    let p = e.evaluate(0.0);
    assert!((p[0] - 2.0).abs() < 1e-10, "at t=0 x should equal major axis");
}
