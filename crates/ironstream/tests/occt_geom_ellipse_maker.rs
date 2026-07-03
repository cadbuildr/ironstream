extern crate ironstream;
use ironstream::geom_ellipse_maker::*;

#[test]
fn test_geom_ellipse_maker_basic() {
    let em = EllipseMaker::from_axes([0.0, 0.0, 0.0], 3.0, 2.0);
    assert!(em.done);
    assert!((em.major - 3.0).abs() < 1e-10);
    assert!((em.minor - 2.0).abs() < 1e-10);

    // evaluate at t=0 -> on major axis
    let p = em.evaluate(0.0);
    let mag = (p[0]*p[0] + p[1]*p[1] + p[2]*p[2]).sqrt();
    assert!((mag - 3.0).abs() < 1e-9, "point at t=0 should be at major axis distance, got {}", mag);

    // d1 should be perpendicular to evaluate
    let d = em.d1(0.0);
    let dot = p[0]*d[0] + p[1]*d[1] + p[2]*d[2];
    assert!(dot.abs() < 1e-9, "d1 should be perpendicular to position at t=0");

    assert!(em.is_done());
}

#[test]
fn test_ellipse_maker_degenerate() {
    // major < minor should set done=false
    let em = EllipseMaker::from_axes([0.0, 0.0, 0.0], 1.0, 2.0);
    assert!(!em.done, "major < minor should be invalid");
}

#[test]
fn test_circle_maker_basic() {
    let cm = CircleMaker::from_center_radius([0.0, 0.0, 0.0], 5.0, [0.0, 0.0, 1.0]);
    assert!(cm.done);
    assert!(cm.is_done());

    let p = cm.evaluate(0.0);
    let r = (p[0]*p[0] + p[1]*p[1] + p[2]*p[2]).sqrt();
    assert!((r - 5.0).abs() < 1e-9, "should be on circle of radius 5");
}

#[test]
fn test_circle_maker_from_3_points() {
    let cm = CircleMaker::from_3_points(
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [-1.0, 0.0, 0.0],
    );
    assert!(cm.is_some());
    let cm = cm.unwrap();
    assert!(cm.done);
    assert!((cm.radius - 1.0).abs() < 1e-9, "radius should be 1.0, got {}", cm.radius);
}
