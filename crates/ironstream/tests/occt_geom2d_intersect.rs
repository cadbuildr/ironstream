extern crate ironstream;
use ironstream::geom2d_intersect::*;

#[test]
fn test_geom2d_intersect_basic() {
    // IntersectionPoint2d construction.
    let pt = IntersectionPoint2d::new(0.5, 1.0, [1.0, 2.0]);
    assert!((pt.param1 - 0.5).abs() < 1e-12);
    assert!((pt.param2 - 1.0).abs() < 1e-12);
    assert_eq!(pt.point, [1.0, 2.0]);
    assert!((pt.tolerance - 1e-7).abs() < 1e-15);

    // CurveIntersect2d lifecycle.
    let mut ci = CurveIntersect2d::new(1e-6);
    assert!(!ci.is_done());
    ci.perform("line_1", "circle_1");
    assert!(ci.is_done());
    // Stub produces no points.
    assert_eq!(ci.num_points(), 0);
    assert!(ci.point(0).is_none());

    // ConicIntersect: two tangent circles (same point of tangency).
    // Circle 1: centre (0,0) r=1; Circle 2: centre (2,0) r=1 → tangent at (1,0).
    let result = ConicIntersect::circle_circle([0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    assert_eq!(result.points.len(), 1);
    assert!((result.points[0].point[0] - 1.0).abs() < 1e-10);
    assert!((result.points[0].point[1]).abs() < 1e-10);

    // Two intersecting circles: (0,0) r=1 and (1,0) r=1 → 2 intersection points.
    let result2 = ConicIntersect::circle_circle([0.0, 0.0, 1.0], [1.0, 0.0, 1.0]);
    assert_eq!(result2.points.len(), 2);

    // Non-intersecting circles (too far apart).
    let result3 = ConicIntersect::circle_circle([0.0, 0.0, 1.0], [10.0, 0.0, 1.0]);
    assert_eq!(result3.points.len(), 0);

    // Line–circle intersection: horizontal line y=0, circle at origin r=1.
    let line = [[-2.0, 0.0], [2.0, 0.0]];
    let circle = [0.0_f64, 0.0, 1.0];
    let lc = ConicIntersect::line_circle(line, circle);
    assert_eq!(lc.points.len(), 2);
}
