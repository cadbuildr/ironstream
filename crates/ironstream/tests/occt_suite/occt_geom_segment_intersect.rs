// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_segment_intersect.rs

extern crate ironstream;
use ironstream::geom_segment_intersect::*;

// ---------------------------------------------------------------------------
// SegIntersect2d tests
// ---------------------------------------------------------------------------

#[test]
fn test_2d_basic_intersection() {
    // X-axis ray and Y-axis ray cross at the origin.
    let mut s = SegIntersect2d::new();
    s.intersect([0.0, 0.0], [1.0, 0.0], [0.0, 0.0], [0.0, 1.0]);
    assert_eq!(s.result_type(), SegIntersectResult::Point);
    let pt = s.point();
    assert!((pt[0]).abs() < 1e-9);
    assert!((pt[1]).abs() < 1e-9);
    assert!((s.param1()).abs() < 1e-9);
    assert!((s.param2()).abs() < 1e-9);
}

#[test]
fn test_2d_offset_intersection() {
    // ray1: starts at (0,1) going right; ray2: starts at (2,0) going up.
    // They meet at (2,1): t=2 along ray1, s=1 along ray2.
    let mut s = SegIntersect2d::new();
    s.intersect([0.0, 1.0], [1.0, 0.0], [2.0, 0.0], [0.0, 1.0]);
    assert_eq!(s.result_type(), SegIntersectResult::Point);
    let pt = s.point();
    assert!((pt[0] - 2.0).abs() < 1e-9);
    assert!((pt[1] - 1.0).abs() < 1e-9);
    assert!((s.param1() - 2.0).abs() < 1e-9);
    assert!((s.param2() - 1.0).abs() < 1e-9);
}

#[test]
fn test_2d_parallel() {
    // Two horizontal rays offset vertically — parallel, not coincident.
    let mut s = SegIntersect2d::new();
    s.intersect([0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 0.0]);
    assert_eq!(s.result_type(), SegIntersectResult::Parallel);
}

#[test]
fn test_2d_coincident() {
    // Same ray — coincident.
    let mut s = SegIntersect2d::new();
    s.intersect([0.0, 0.0], [1.0, 0.0], [3.0, 0.0], [1.0, 0.0]);
    assert_eq!(s.result_type(), SegIntersectResult::Coincident);
}

#[test]
fn test_2d_diagonal_intersection() {
    // ray1: (0,0) + t*(1,1); ray2: (1,0) + s*(-1,1).
    // Intersection: 0+t = 1-s and 0+t = 0+s => t=s, t=1-t => t=0.5.
    // Point = (0.5, 0.5).
    let mut s = SegIntersect2d::new();
    s.intersect([0.0, 0.0], [1.0, 1.0], [1.0, 0.0], [-1.0, 1.0]);
    assert_eq!(s.result_type(), SegIntersectResult::Point);
    let pt = s.point();
    assert!((pt[0] - 0.5).abs() < 1e-9);
    assert!((pt[1] - 0.5).abs() < 1e-9);
    assert!((s.param1() - 0.5).abs() < 1e-9);
    assert!((s.param2() - 0.5).abs() < 1e-9);
}

#[test]
fn test_2d_new_is_no_intersection() {
    let s = SegIntersect2d::new();
    assert_eq!(s.result_type(), SegIntersectResult::NoIntersection);
}

#[test]
fn test_2d_default_is_no_intersection() {
    let s = SegIntersect2d::default();
    assert_eq!(s.result_type(), SegIntersectResult::NoIntersection);
}

// ---------------------------------------------------------------------------
// SegIntersect3d tests
// ---------------------------------------------------------------------------

#[test]
fn test_3d_new_is_no_intersection() {
    let s = SegIntersect3d::new();
    assert_eq!(s.result_type(), SegIntersectResult::NoIntersection);
    assert!((s.distance()).abs() < 1e-15);
}

#[test]
fn test_3d_default_is_no_intersection() {
    let s = SegIntersect3d::default();
    assert_eq!(s.result_type(), SegIntersectResult::NoIntersection);
}

#[test]
fn test_3d_exact_intersection() {
    // X-axis ray and Y-axis ray in the Z=0 plane — they meet at the origin.
    let mut s = SegIntersect3d::new();
    s.intersect(
        [0.0, 0.0, 0.0], [1.0, 0.0, 0.0],
        [0.0, 0.0, 0.0], [0.0, 1.0, 0.0],
    );
    assert_eq!(s.result_type(), SegIntersectResult::Point);
    assert!(s.distance() < 1e-9);
    let cp1 = s.closest_pt1();
    let cp2 = s.closest_pt2();
    assert!((cp1[0]).abs() < 1e-9 && (cp1[1]).abs() < 1e-9 && (cp1[2]).abs() < 1e-9);
    assert!((cp2[0]).abs() < 1e-9 && (cp2[1]).abs() < 1e-9 && (cp2[2]).abs() < 1e-9);
}

#[test]
fn test_3d_skew_lines() {
    // Classic skew-line example:
    //   ray1: (0,0,0) + t*(1,0,0)  (X-axis)
    //   ray2: (0,1,1) + s*(0,1,0)  (parallel to Y, offset in Z by 1)
    // Closest approach: on ray1 the foot is (0,0,0), on ray2 the foot is (0,1,1).
    // Distance = 1 (the Z separation).
    let mut s = SegIntersect3d::new();
    s.intersect(
        [0.0, 0.0, 0.0], [1.0, 0.0, 0.0],
        [0.0, 1.0, 1.0], [0.0, 1.0, 0.0],
    );
    assert_eq!(s.result_type(), SegIntersectResult::NoIntersection);
    assert!((s.distance() - 1.0).abs() < 1e-9);
}

#[test]
fn test_3d_parallel_offset() {
    // Two parallel rays offset in the Y direction.
    let mut s = SegIntersect3d::new();
    s.intersect(
        [0.0, 0.0, 0.0], [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0], [1.0, 0.0, 0.0],
    );
    assert_eq!(s.result_type(), SegIntersectResult::Parallel);
    assert!((s.distance() - 2.0).abs() < 1e-9);
}

#[test]
fn test_3d_coincident() {
    // Same ray — coincident.
    let mut s = SegIntersect3d::new();
    s.intersect(
        [1.0, 0.0, 0.0], [1.0, 0.0, 0.0],
        [3.0, 0.0, 0.0], [1.0, 0.0, 0.0],
    );
    assert_eq!(s.result_type(), SegIntersectResult::Coincident);
    assert!(s.distance() < 1e-9);
}

#[test]
fn test_3d_exact_intersection_offset() {
    // ray1: (1,0,0) + t*(0,1,0)  (up the Y axis from x=1)
    // ray2: (0,2,0) + s*(1,0,0)  (along X at y=2)
    // They meet at (1,2,0): t=2, s=1.
    let mut s = SegIntersect3d::new();
    s.intersect(
        [1.0, 0.0, 0.0], [0.0, 1.0, 0.0],
        [0.0, 2.0, 0.0], [1.0, 0.0, 0.0],
    );
    assert_eq!(s.result_type(), SegIntersectResult::Point);
    assert!(s.distance() < 1e-9);
    let cp1 = s.closest_pt1();
    assert!((cp1[0] - 1.0).abs() < 1e-9);
    assert!((cp1[1] - 2.0).abs() < 1e-9);
    assert!((cp1[2]).abs() < 1e-9);
}

#[test]
fn test_3d_closest_pts_symmetry() {
    // closest_pt1 and closest_pt2 should be equidistant from the midpoint of the gap.
    let mut s = SegIntersect3d::new();
    s.intersect(
        [0.0, 0.0, 0.0], [1.0, 0.0, 0.0],
        [0.0, 3.0, 0.0], [0.0, 0.0, 1.0],
    );
    // ray1 X-axis, ray2 up Z from (0,3,0). Closest: (0,0,0) on ray1, (0,3,0) on ray2. dist=3.
    assert!((s.distance() - 3.0).abs() < 1e-9);
    let cp1 = s.closest_pt1();
    let cp2 = s.closest_pt2();
    assert!((cp1[0]).abs() < 1e-9 && (cp1[1]).abs() < 1e-9 && (cp1[2]).abs() < 1e-9);
    assert!((cp2[0]).abs() < 1e-9 && (cp2[1] - 3.0).abs() < 1e-9 && (cp2[2]).abs() < 1e-9);
}
