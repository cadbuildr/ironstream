extern crate ironstream;
use ironstream::geom_conic_intersect::*;

#[test]
fn test_line_plane_intersect() {
    // Line along z-axis hits plane z=5
    let r = IntConicQuad::line_plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 5.0);
    assert!(r.has_intersection());
    assert_eq!(r.kind, IntType::Point);
    assert!((r.points[0][2] - 5.0).abs() < 1e-10);
}

#[test]
fn test_line_plane_parallel() {
    // Line along x-axis is parallel to plane z=1
    let r = IntConicQuad::line_plane([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    assert!(!r.has_intersection());
    assert!(r.is_parallel);
}

#[test]
fn test_sphere_sphere_intersect_circle() {
    let r = QuadQuadGeo::sphere_sphere([0.0; 3], 5.0, [6.0, 0.0, 0.0], 5.0);
    assert!(r.has_intersection());
    assert_eq!(r.kind, IntType::Circle);
}

#[test]
fn test_sphere_sphere_miss() {
    let r = QuadQuadGeo::sphere_sphere([0.0; 3], 1.0, [100.0, 0.0, 0.0], 1.0);
    assert!(!r.has_intersection());
}

#[test]
fn test_int3pln_three_planes() {
    // Three axis-aligned planes: x=2, y=3, z=4
    let n = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let d = [2.0, 3.0, 4.0];
    let pt = Int3Pln::solve(n, d).unwrap();
    assert!((pt[0] - 2.0).abs() < 1e-10);
    assert!((pt[1] - 3.0).abs() < 1e-10);
    assert!((pt[2] - 4.0).abs() < 1e-10);
}
