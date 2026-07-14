use ironstream::brep_surf::*;

#[test]
fn brep_curve3d_is_valid() {
    let c = BrepCurve3D::new(5, 0.0, 1.0);
    assert!(c.is_valid());
    assert_eq!(c.curve_id(), 5);
    assert!((c.first() - 0.0).abs() < 1e-12);
    assert!((c.last() - 1.0).abs() < 1e-12);

    // curve_id == 0 => invalid
    let bad_id = BrepCurve3D::new(0, 0.0, 1.0);
    assert!(!bad_id.is_valid());

    // first >= last => invalid
    let bad_range = BrepCurve3D::new(3, 2.0, 1.0);
    assert!(!bad_range.is_valid());
}

#[test]
fn brep_curve_on_surface_is_valid() {
    let c = BrepCurveOnSurface::new(3, 7, 0.0, 2.0);
    assert!(c.is_valid());
    assert_eq!(c.pcurve_id(), 3);
    assert_eq!(c.surface_id(), 7);
    assert!((c.first() - 0.0).abs() < 1e-12);
    assert!((c.last() - 2.0).abs() < 1e-12);

    // invalid: pcurve_id == 0
    let bad = BrepCurveOnSurface::new(0, 7, 0.0, 1.0);
    assert!(!bad.is_valid());

    // invalid: surface_id == 0
    let bad2 = BrepCurveOnSurface::new(3, 0, 0.0, 1.0);
    assert!(!bad2.is_valid());
}

#[test]
fn brep_surface_tolerance_and_natural_restriction() {
    let mut s = BrepSurface::new(10).with_tolerance(1e-6);
    assert_eq!(s.surface_id(), 10);
    assert!((s.tolerance() - 1e-6).abs() < 1e-15);
    assert!(!s.natural_restriction());

    s.set_natural_restriction(true);
    assert!(s.natural_restriction());

    // builder pattern for location
    let s2 = BrepSurface::new(2).with_location(42);
    assert_eq!(s2.surface_id(), 2);
    assert_eq!(s2.location_id, 42);
}

#[test]
fn brep_point_on_curve_params() {
    let p = BrepPointOnCurve::new(0.5, 3);
    assert!((p.parameter() - 0.5).abs() < 1e-12);
    assert_eq!(p.curve_id(), 3);
}

#[test]
fn brep_point_on_surface_params() {
    let p = BrepPointOnSurface::new(0.3, 0.7, 5);
    assert!((p.u() - 0.3).abs() < 1e-12);
    assert!((p.v() - 0.7).abs() < 1e-12);
    assert_eq!(p.surface_id(), 5);
}

#[test]
fn brep_point_on_curve_on_surface_params() {
    let p = BrepPointOnCurveOnSurface::new(0.25, 2, 9);
    assert!((p.parameter() - 0.25).abs() < 1e-12);
    assert_eq!(p.pcurve_id(), 2);
    assert_eq!(p.surface_id(), 9);
}

#[test]
fn brep_polygon_on_triangulation_nodes() {
    let mut p = BrepPolygonOnTriangulation::new(1);
    assert_eq!(p.nb_nodes(), 0);
    assert_eq!(p.triangulation_id(), 1);

    p.add_node(10);
    p.add_node(20);
    p.add_node(30);
    assert_eq!(p.nb_nodes(), 3);

    // 1-based access
    assert_eq!(p.node(1), Some(10));
    assert_eq!(p.node(2), Some(20));
    assert_eq!(p.node(3), Some(30));

    // index 0 returns None
    assert_eq!(p.node(0), None);

    // out-of-range returns None
    assert_eq!(p.node(99), None);
}
