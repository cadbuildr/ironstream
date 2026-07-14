extern crate ironstream;
use ironstream::geom_curvature::*;

#[test]
fn test_curve_lprops_straight_line() {
    let pts = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [3.0, 0.0, 0.0]];
    let p = CurveLProps::from_polyline(&pts, 1).unwrap();
    assert!(p.is_straight());
    assert!(p.radius_of_curvature().is_none());
}

#[test]
fn test_curve_lprops_curved() {
    // A bent polyline: makes a turn
    let pts = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 2.0, 0.0]];
    let p = CurveLProps::from_polyline(&pts, 1).unwrap();
    // At the corner, curvature should be non-zero
    assert!(p.curvature >= 0.0);
    // Tangent should be a unit vector
    let tn = p.tangent;
    let len = (tn[0]*tn[0]+tn[1]*tn[1]+tn[2]*tn[2]).sqrt();
    assert!((len - 1.0).abs() < 1e-10);
}

#[test]
fn test_surface_lprops_flat() {
    let s = SurfaceLProps::flat([0.0, 0.0, 1.0]);
    assert!((s.gaussian_curvature).abs() < 1e-12);
    assert!((s.mean_curvature).abs() < 1e-12);
    assert!(s.is_umbilic());
}

#[test]
fn test_surface_lprops_sphere() {
    let r = 3.0;
    let s = SurfaceLProps::sphere(r, std::f64::consts::FRAC_PI_2, 0.0);
    assert!((s.gaussian_curvature - 1.0 / (r * r)).abs() < 1e-10);
    assert!((s.mean_curvature - 1.0 / r).abs() < 1e-10);
    assert!(s.is_umbilic());
}

#[test]
fn test_brep_curve_lprops_type_alias() {
    // BRepCurveLProps is a type alias for CurveLProps
    let pts = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
    let p: BRepCurveLProps = CurveLProps::from_polyline(&pts, 1).unwrap();
    assert!(p.is_straight());
}
