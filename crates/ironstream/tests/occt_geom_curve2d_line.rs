extern crate ironstream;
use ironstream::geom_curve2d_line::*;

#[test]
fn test_geom_curve2d_line_basic() {
    let line = Line2d::new([0.0, 0.0], [1.0, 0.0]);
    let pt = line.point_at(3.0);
    assert!((pt[0] - 3.0).abs() < 1e-10);
    assert!(pt[1].abs() < 1e-10);

    let d1 = line.d1(0.0);
    assert!((d1[0] - 1.0).abs() < 1e-10);
    assert!(d1[1].abs() < 1e-10);

    let d2 = line.d2(0.0);
    assert!(d2[0].abs() < 1e-10);
    assert!(d2[1].abs() < 1e-10);

    let t = line.project([5.0, 0.0]);
    assert!((t - 5.0).abs() < 1e-10);

    let dist = line.distance_to([0.0, 3.0]);
    assert!((dist - 3.0).abs() < 1e-10);

    let mut rev = line;
    rev.reverse();
    let d1r = rev.d1(0.0);
    assert!((d1r[0] + 1.0).abs() < 1e-10);

    let side = line.side([0.0, 1.0]);
    assert!(side > 0.0);

    let line2 = Line2d::new([0.0, 1.0], [1.0, 0.0]);
    let inter = line.intersect(&line2);
    assert!(inter.is_none());

    let line3 = Line2d::from_2pts([0.0, 0.0], [1.0, 1.0]);
    assert!((line3.direction[0] - line3.direction[1]).abs() < 1e-10);
}
