extern crate ironstream;
use ironstream::geom_circ3d::*;

#[test]
fn make_circle_center_normal_radius() {
    let c = MakeCircle::from_center_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0);
    assert!(c.is_done());
    assert!((c.radius() - 5.0).abs() < 1e-10);
    let center = c.center();
    assert!((center[0]).abs() < 1e-10);
}

#[test]
fn make_circle_from_3_points() {
    let p1 = [1.0, 0.0, 0.0];
    let p2 = [0.0, 1.0, 0.0];
    let p3 = [-1.0, 0.0, 0.0];
    let c = MakeCircle::from_3_points(p1, p2, p3);
    assert!(c.is_done());
    assert!((c.radius() - 1.0).abs() < 1e-6);
}

#[test]
fn make_circle_discretize_radius() {
    let c = MakeCircle::from_center_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0);
    let pts = c.discretize(12);
    assert_eq!(pts.len(), 13);
    for p in &pts {
        let r = (p[0]*p[0] + p[1]*p[1] + p[2]*p[2]).sqrt();
        assert!((r - 3.0).abs() < 1e-8, "radius mismatch: {}", r);
    }
}

#[test]
fn make_arc_length() {
    let arc = MakeArcOfCircle::new([0.0,0.0,0.0], [0.0,0.0,1.0], 1.0, 0.0, 180.0);
    assert!(arc.is_done());
    assert!((arc.arc_length() - std::f64::consts::PI).abs() < 1e-8);
}

#[test]
fn make_arc_from_3_points_discretize() {
    let arc = MakeArcOfCircle::from_3_points(
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [-1.0, 0.0, 0.0],
    );
    assert!(arc.is_done());
    let pts = arc.discretize(4);
    assert_eq!(pts.len(), 5);
}
