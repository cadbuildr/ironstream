extern crate ironstream;
use ironstream::geom_adapt_surface::*;

#[test]
fn test_geom_adapt_surface_basic() {
    // BrepAdaptorSurface - Plane
    let surf = BrepAdaptorSurface::new("MyFace");
    assert_eq!(surf.face, "MyFace");
    assert_eq!(surf.surface_type, "Plane");

    let p = surf.value(0.5, 0.5);
    assert!((p[0] - 0.5).abs() < 1e-10);
    assert!((p[1] - 0.5).abs() < 1e-10);
    assert!((p[2]).abs() < 1e-10);

    let (pt, du, dv) = surf.d1(0.0, 0.0);
    assert!((pt[2]).abs() < 1e-10);
    // plane: du = (1,0,0), dv = (0,1,0)
    assert!((du[0] - 1.0).abs() < 1e-10);
    assert!((dv[1] - 1.0).abs() < 1e-10);

    let n = surf.normal(0.0, 0.0);
    // plane normal = (0,0,1)
    assert!((n[2] - 1.0).abs() < 1e-9);

    assert!(!surf.is_u_closed());
    assert!(!surf.is_v_closed());

    // BrepAdaptorSurface - Cylinder
    let mut cyl = BrepAdaptorSurface::new("CylFace");
    cyl.surface_type = "Cylinder".to_string();
    assert!(cyl.is_u_closed());
    assert!(!cyl.is_v_closed());

    // BrepAdaptorCurve
    let curve = BrepAdaptorCurve::new("MyEdge");
    assert_eq!(curve.edge, "MyEdge");
    assert!((curve.first - 0.0).abs() < 1e-12);
    assert!((curve.last - 1.0).abs() < 1e-12);

    let pv = curve.value(0.5);
    assert!((pv[0] - 0.5).abs() < 1e-10);

    let (pt2, tangent) = curve.d1(0.0);
    assert!((pt2[0]).abs() < 1e-10);
    assert!((tangent[0] - 1.0).abs() < 1e-10);

    let len = curve.length();
    assert!(len > 0.0);
}
