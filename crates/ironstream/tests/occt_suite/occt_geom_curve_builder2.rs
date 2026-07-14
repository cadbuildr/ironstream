extern crate ironstream;
use ironstream::geom_curve_builder2::*;

#[test]
fn test_make_line_from_origin_dir() {
    let l = MakeLine::from_origin_dir([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(l.is_done());
    let p = l.point_at(5.0);
    assert!((p[0] - 5.0).abs() < 1e-10);
    assert!(p[1].abs() < 1e-10);
}

#[test]
fn test_make_line_from_two_points_distance() {
    let l = MakeLine::from_two_points([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(l.is_done());
    // Point at (2, 3, 0) has distance 3 to the x-axis line
    assert!((l.distance_to_point([2.0, 3.0, 0.0]) - 3.0).abs() < 1e-8);
}

#[test]
fn test_make_segment_length_and_endpoints() {
    let s = MakeSegment::from_two_points([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
    assert!(s.is_done());
    assert!((s.length() - 5.0).abs() < 1e-8);
    let start = s.start_point();
    assert!(start[0].abs() < 1e-10);
}

#[test]
fn test_make_segment_discretize() {
    let s = MakeSegment::from_two_points([0.0, 0.0, 0.0], [4.0, 0.0, 0.0]);
    let pts = s.discretize(4);
    assert_eq!(pts.len(), 5);
    assert!((pts[4][0] - 4.0).abs() < 1e-10);
}

#[test]
fn test_make_hyperbola_and_parabola() {
    let h = MakeHyperbola::new([0.0; 3], [1.0, 0.0, 0.0], 2.0, 1.0);
    assert!(h.is_done());
    assert!(h.eccentricity() > 1.0);
    // At t=0, point should be at (major_radius, 0, 0)
    let p = h.point_at(0.0);
    assert!((p[0] - 2.0).abs() < 1e-8);

    let par = MakeParabola::new([0.0; 3], [1.0, 0.0, 0.0], 1.0);
    assert!(par.is_done());
}
