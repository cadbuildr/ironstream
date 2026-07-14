extern crate ironstream;
use ironstream::geom_diff_geom::*;

#[test]
fn curve_props_straight_line() {
    let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0],[3.0,0.0,0.0]];
    let p = CurveProps::from_polyline(&pts, 2).unwrap();
    assert!((p.tangent[0] - 1.0).abs() < 1e-8);
    assert!(p.curvature.abs() < 1e-8);
    assert!(!p.has_normal()); // straight line has no normal
}

#[test]
fn curve_props_at_endpoint() {
    let pts = vec![[0.0,0.0,0.0],[0.0,1.0,0.0],[0.0,2.0,0.0]];
    let p = CurveProps::from_polyline(&pts, 0).unwrap();
    assert!((p.tangent[1] - 1.0).abs() < 1e-8);
}

#[test]
fn surface_props_sphere_umbilic() {
    let s = SurfaceProps::from_sphere([0.0;3], 2.0, 0.0, 0.0);
    assert!(s.is_umbilic());
    assert!((s.mean_curvature - 0.5).abs() < 1e-8);
    assert!((s.gaussian_curvature - 0.25).abs() < 1e-8);
}

#[test]
fn surface_props_flat_developable() {
    let s = SurfaceProps::from_flat_surface(1.0, 1.0);
    assert!(s.is_developable());
    assert!((s.normal[2] - 1.0).abs() < 1e-10);
}

#[test]
fn geom_surface_props_collection() {
    let mut g = GeomSurfaceProps::new();
    assert_eq!(g.nb_points(), 0);
    g.add(SurfaceProps::from_flat_surface(0.0, 0.0));
    g.add(SurfaceProps::from_sphere([0.0;3], 1.0, 0.0, 0.0));
    assert_eq!(g.nb_points(), 2);
}
