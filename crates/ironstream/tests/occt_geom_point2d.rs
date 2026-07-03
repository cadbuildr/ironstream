extern crate ironstream;
use ironstream::geom_point2d::*;

#[test]
fn test_geom_point2d_basic() {
    let p = Pnt2d::new(3.0, 4.0);
    assert_eq!(p.x, 3.0);
    assert_eq!(p.y, 4.0);

    let o = Pnt2d::origin();
    assert_eq!(o.x, 0.0);
    assert_eq!(o.y, 0.0);

    let dist = p.distance(&o);
    assert!((dist - 5.0).abs() < 1e-10);

    let mid = midpoint_2d(&Pnt2d::new(0.0, 0.0), &Pnt2d::new(2.0, 4.0));
    assert!((mid.x - 1.0).abs() < 1e-10);
    assert!((mid.y - 2.0).abs() < 1e-10);

    let pts = [Pnt2d::new(0.0, 0.0), Pnt2d::new(4.0, 0.0)];
    let weights = [1.0, 1.0];
    let bc = barycenter_2d(&pts, &weights);
    assert!((bc.x - 2.0).abs() < 1e-10);
    assert!((bc.y - 0.0).abs() < 1e-10);

    let mut p2 = Pnt2d::new(1.0, 2.0);
    p2.translate(3.0, 4.0);
    assert!((p2.x - 4.0).abs() < 1e-10);
    assert!((p2.y - 6.0).abs() < 1e-10);

    let cp = CartesianPoint2d::new(5.0, 6.0);
    assert!((cp.x() - 5.0).abs() < 1e-10);
    assert!((cp.y() - 6.0).abs() < 1e-10);
}
