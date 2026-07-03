extern crate ironstream;
use ironstream::geom_circ_arc_maker::*;

#[test]
fn test_geom_circ_arc_maker_basic() {
    let arc = ArcOfCircle2d::from_3_points(
        [1.0, 0.0],
        [0.0, 1.0],
        [-1.0, 0.0],
    );
    assert!(arc.is_some(), "should produce an arc from 3 non-collinear 2D points");
    let arc = arc.unwrap();
    assert!(arc.radius > 0.0);
    assert!(arc.length() > 0.0);

    let p = arc.evaluate(arc.start_param);
    let _ = p;

    let d = arc.d1(arc.start_param);
    let mag = (d[0]*d[0] + d[1]*d[1]).sqrt();
    assert!((mag - arc.radius).abs() < 1e-9, "d1 magnitude should equal radius");
}

#[test]
fn test_arc_2d_collinear_returns_none() {
    let arc = ArcOfCircle2d::from_3_points([0.0, 0.0], [1.0, 0.0], [2.0, 0.0]);
    assert!(arc.is_none());
}

#[test]
fn test_segment_2d_basic() {
    let seg = Segment2d::new([0.0, 0.0], [3.0, 4.0]);
    assert!((seg.length() - 5.0).abs() < 1e-10);

    let mid = seg.midpoint();
    assert!((mid[0] - 1.5).abs() < 1e-10);
    assert!((mid[1] - 2.0).abs() < 1e-10);

    let p = seg.evaluate(0.0);
    assert_eq!(p, [0.0, 0.0]);

    let p2 = seg.evaluate(1.0);
    assert_eq!(p2, [3.0, 4.0]);

    let d = seg.d1();
    assert_eq!(d, [3.0, 4.0]);
}

#[test]
fn test_arc_2d_through_3pts_convenience() {
    let arc = arc_2d_through_3pts([1.0, 0.0], [0.0, 1.0], [-1.0, 0.0]);
    assert!(arc.is_some());
}
