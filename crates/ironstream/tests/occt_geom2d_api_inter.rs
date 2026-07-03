// FILE: rust/ironstream/crates/ironstream/tests/occt_geom2d_api_inter.rs

extern crate ironstream;
use ironstream::geom2d_api_inter::*;

// --- Geom2dIntersectionPoint tests ---

#[test]
fn intersection_point_stores_coordinates() {
    let p = Geom2dIntersectionPoint::new([3.0, 4.0], 0.5, 0.75);
    assert_eq!(p.point(), [3.0, 4.0]);
}

#[test]
fn intersection_point_stores_param1() {
    let p = Geom2dIntersectionPoint::new([0.0, 0.0], 1.23, 4.56);
    assert!((p.param1() - 1.23).abs() < 1e-15);
}

#[test]
fn intersection_point_stores_param2() {
    let p = Geom2dIntersectionPoint::new([0.0, 0.0], 1.23, 4.56);
    assert!((p.param2() - 4.56).abs() < 1e-15);
}

#[test]
fn intersection_point_negative_coords() {
    let p = Geom2dIntersectionPoint::new([-1.0, -2.5], -0.1, 0.9);
    assert_eq!(p.point(), [-1.0, -2.5]);
    assert!((p.param1() - (-0.1)).abs() < 1e-15);
}

// --- Geom2dIntersectionSegment tests ---

#[test]
fn intersection_segment_first_point() {
    let pt1 = Geom2dIntersectionPoint::new([0.0, 0.0], 0.0, 0.0);
    let pt2 = Geom2dIntersectionPoint::new([1.0, 1.0], 1.0, 1.0);
    let seg = Geom2dIntersectionSegment::new(pt1, pt2);
    assert_eq!(seg.first_point().point(), [0.0, 0.0]);
}

#[test]
fn intersection_segment_last_point() {
    let pt1 = Geom2dIntersectionPoint::new([0.0, 0.0], 0.0, 0.0);
    let pt2 = Geom2dIntersectionPoint::new([1.0, 1.0], 1.0, 1.0);
    let seg = Geom2dIntersectionSegment::new(pt1, pt2);
    assert_eq!(seg.last_point().point(), [1.0, 1.0]);
}

#[test]
fn intersection_segment_first_point_params() {
    let pt1 = Geom2dIntersectionPoint::new([2.0, 3.0], 0.25, 0.5);
    let pt2 = Geom2dIntersectionPoint::new([4.0, 5.0], 0.75, 1.0);
    let seg = Geom2dIntersectionSegment::new(pt1, pt2);
    assert!((seg.first_point().param1() - 0.25).abs() < 1e-15);
    assert!((seg.first_point().param2() - 0.5).abs() < 1e-15);
}

#[test]
fn intersection_segment_last_point_params() {
    let pt1 = Geom2dIntersectionPoint::new([2.0, 3.0], 0.25, 0.5);
    let pt2 = Geom2dIntersectionPoint::new([4.0, 5.0], 0.75, 1.0);
    let seg = Geom2dIntersectionSegment::new(pt1, pt2);
    assert!((seg.last_point().param1() - 0.75).abs() < 1e-15);
    assert!((seg.last_point().param2() - 1.0).abs() < 1e-15);
}

// --- Geom2dInterCurveCurve tests ---

#[test]
fn inter_curve_curve_new_is_not_done() {
    let inter = Geom2dInterCurveCurve::new();
    assert!(!inter.is_done());
}

#[test]
fn inter_curve_curve_new_empty_points() {
    let inter = Geom2dInterCurveCurve::new();
    assert_eq!(inter.nb_points(), 0);
}

#[test]
fn inter_curve_curve_new_empty_segments() {
    let inter = Geom2dInterCurveCurve::new();
    assert_eq!(inter.nb_segments(), 0);
}

#[test]
fn inter_curve_curve_mark_done() {
    let mut inter = Geom2dInterCurveCurve::new();
    inter.mark_done();
    assert!(inter.is_done());
}

#[test]
fn inter_curve_curve_add_one_point() {
    let mut inter = Geom2dInterCurveCurve::new();
    inter.add_point(Geom2dIntersectionPoint::new([1.0, 2.0], 0.5, 0.3));
    assert_eq!(inter.nb_points(), 1);
}

#[test]
fn inter_curve_curve_add_multiple_points() {
    let mut inter = Geom2dInterCurveCurve::new();
    inter.add_point(Geom2dIntersectionPoint::new([0.0, 0.0], 0.0, 0.0));
    inter.add_point(Geom2dIntersectionPoint::new([1.0, 1.0], 1.0, 1.0));
    inter.add_point(Geom2dIntersectionPoint::new([2.0, 2.0], 2.0, 2.0));
    assert_eq!(inter.nb_points(), 3);
}

#[test]
fn inter_curve_curve_point_retrieval() {
    let mut inter = Geom2dInterCurveCurve::new();
    inter.add_point(Geom2dIntersectionPoint::new([5.0, 6.0], 0.1, 0.9));
    assert_eq!(inter.point(0).point(), [5.0, 6.0]);
    assert!((inter.point(0).param1() - 0.1).abs() < 1e-15);
    assert!((inter.point(0).param2() - 0.9).abs() < 1e-15);
}

#[test]
fn inter_curve_curve_point_order_preserved() {
    let mut inter = Geom2dInterCurveCurve::new();
    inter.add_point(Geom2dIntersectionPoint::new([1.0, 0.0], 0.1, 0.2));
    inter.add_point(Geom2dIntersectionPoint::new([2.0, 0.0], 0.3, 0.4));
    assert_eq!(inter.point(0).point(), [1.0, 0.0]);
    assert_eq!(inter.point(1).point(), [2.0, 0.0]);
}

#[test]
fn inter_curve_curve_add_one_segment() {
    let mut inter = Geom2dInterCurveCurve::new();
    let pt1 = Geom2dIntersectionPoint::new([0.0, 0.0], 0.0, 0.0);
    let pt2 = Geom2dIntersectionPoint::new([1.0, 0.0], 1.0, 1.0);
    inter.add_segment(Geom2dIntersectionSegment::new(pt1, pt2));
    assert_eq!(inter.nb_segments(), 1);
}

#[test]
fn inter_curve_curve_add_multiple_segments() {
    let mut inter = Geom2dInterCurveCurve::new();
    for i in 0..3 {
        let f = i as f64;
        let pt1 = Geom2dIntersectionPoint::new([f, 0.0], f, f);
        let pt2 = Geom2dIntersectionPoint::new([f + 1.0, 0.0], f + 1.0, f + 1.0);
        inter.add_segment(Geom2dIntersectionSegment::new(pt1, pt2));
    }
    assert_eq!(inter.nb_segments(), 3);
}

#[test]
fn inter_curve_curve_segment_retrieval() {
    let mut inter = Geom2dInterCurveCurve::new();
    let pt1 = Geom2dIntersectionPoint::new([3.0, 4.0], 0.2, 0.6);
    let pt2 = Geom2dIntersectionPoint::new([7.0, 8.0], 0.8, 1.2);
    inter.add_segment(Geom2dIntersectionSegment::new(pt1, pt2));
    assert_eq!(inter.segment(0).first_point().point(), [3.0, 4.0]);
    assert_eq!(inter.segment(0).last_point().point(), [7.0, 8.0]);
}

#[test]
fn inter_curve_curve_segment_order_preserved() {
    let mut inter = Geom2dInterCurveCurve::new();
    let make_seg = |x: f64| {
        let pt1 = Geom2dIntersectionPoint::new([x, 0.0], x, x);
        let pt2 = Geom2dIntersectionPoint::new([x + 1.0, 0.0], x + 1.0, x + 1.0);
        Geom2dIntersectionSegment::new(pt1, pt2)
    };
    inter.add_segment(make_seg(10.0));
    inter.add_segment(make_seg(20.0));
    assert_eq!(inter.segment(0).first_point().point(), [10.0, 0.0]);
    assert_eq!(inter.segment(1).first_point().point(), [20.0, 0.0]);
}

#[test]
fn inter_curve_curve_mixed_points_and_segments() {
    let mut inter = Geom2dInterCurveCurve::new();
    inter.add_point(Geom2dIntersectionPoint::new([0.0, 0.0], 0.0, 0.0));
    let pt1 = Geom2dIntersectionPoint::new([1.0, 0.0], 1.0, 1.0);
    let pt2 = Geom2dIntersectionPoint::new([2.0, 0.0], 2.0, 2.0);
    inter.add_segment(Geom2dIntersectionSegment::new(pt1, pt2));
    inter.mark_done();
    assert_eq!(inter.nb_points(), 1);
    assert_eq!(inter.nb_segments(), 1);
    assert!(inter.is_done());
}

#[test]
fn inter_curve_curve_default_equals_new() {
    let inter: Geom2dInterCurveCurve = Default::default();
    assert!(!inter.is_done());
    assert_eq!(inter.nb_points(), 0);
    assert_eq!(inter.nb_segments(), 0);
}
