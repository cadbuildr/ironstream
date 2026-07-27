// Integration tests for shape_construct
use ironstream::shape_construct::*;

#[test]
fn construct_curve_fix_knots_id() {
    let cc = ShapeConstructCurve::new();
    let fixed = cc.fix_knots(7);
    assert_eq!(fixed, 7 + 20000);
}

#[test]
fn construct_curve_reverse() {
    let cc = ShapeConstructCurve::new();
    let (rid, rf, rl) = cc.reverse_curve(3, 0.0, 1.0);
    assert_eq!(rid, 3 + 30000);
    assert!((rf - (-1.0)).abs() < 1e-12);
    assert!((rl - 0.0).abs() < 1e-12);
}

#[test]
fn construct_curve_convert_2d_bspline() {
    let mut cc = ShapeConstructCurve::new();
    let id = cc.convert_tob_spline2d(9);
    assert!(cc.is_done());
    assert_eq!(id, 9 + 40000);
    assert_eq!(cc.result(), id);
}

#[test]
fn project_curve_perform_by_pts_number() {
    let mut proj = ShapeConstructProjectCurveOnSurface::new();
    proj.set_surface(5, 1e-4);
    let id = proj.perform_by_pts_number(3, 0.0, 1.0, 30);
    assert!(proj.is_done());
    assert_eq!(id, 3 + 5 + 50000);
    assert!(proj.max_deviation() > 0.0);
}

#[test]
fn project_curve_project_point() {
    let mut proj = ShapeConstructProjectCurveOnSurface::new();
    proj.set_surface(2, 1e-6);
    let pt = proj.project_point([0.0, 0.0, 0.0]).unwrap();
    assert!((pt.u - 0.5).abs() < 1e-12);
    assert!((pt.v - 0.5).abs() < 1e-12);
}

#[test]
fn make_triangulation_multi_node_triangle() {
    let mut tri = ShapeConstructMakeTriangulation::new(1e-7);
    let a = tri.add_node(0.0, 0.0, 0.0);
    let b = tri.add_node(2.0, 0.0, 0.0);
    let c = tri.add_node(0.0, 2.0, 0.0);
    let d = tri.add_node(2.0, 2.0, 0.0);
    tri.add_triangle(a, b, c);
    tri.add_triangle(b, d, c);
    tri.perform();
    assert!(tri.is_done());
    assert_eq!(tri.nb_nodes(), 4);
    assert_eq!(tri.nb_triangles(), 2);
    let t0 = tri.triangle(0).unwrap();
    assert_eq!(t0.a, a);
    assert!(tri.node(3).is_some());
}
