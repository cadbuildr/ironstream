extern crate ironstream;
use ironstream::geom_surface_ext2::*;

#[test]
fn test_surface_of_linear_extrusion() {
    let basis = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
    let s = SurfaceOfLinearExtrusion::new(basis, [0.0, 0.0, 1.0]);
    assert_eq!(s.nb_u_points(), 2);
    let p = s.point_at(0, 5.0).unwrap();
    assert!((p[2] - 5.0).abs() < 1e-10);
    assert!(s.point_at(99, 0.0).is_none());
}

#[test]
fn test_cylindrical_surface_point() {
    let s = CylindricalSurface::new([0.0; 3], [0.0, 0.0, 1.0], 3.0);
    let p = s.point_at(0.0, 0.0);
    // At u=0, v=0: point should be at (radius, 0, 0)
    assert!((p[0] - 3.0).abs() < 1e-10);
    assert!(p[1].abs() < 1e-10);
}

#[test]
fn test_spherical_surface_area_volume() {
    let s = SphericalSurface::new([0.0; 3], 2.0);
    assert!((s.area() - 4.0 * std::f64::consts::PI * 4.0).abs() < 1e-10);
    assert!((s.volume() - 4.0 / 3.0 * std::f64::consts::PI * 8.0).abs() < 1e-10);
}

#[test]
fn test_conical_surface_apex_angle() {
    let s = ConicalSurface::new([0.0; 3], [0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_4);
    assert!((s.apex_angle() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
    // At (u=0, v=1): r = tan(pi/4)*1 = 1 → point at (1, 0, 1)
    let p = s.point_at(0.0, 1.0);
    assert!((p[2] - 1.0).abs() < 1e-10);
}

#[test]
fn test_surface_of_revolution_at_zero_angle() {
    let basis = vec![[2.0, 0.0, 0.0]];
    let s = SurfaceOfRevolution::new(basis, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    // At v=0 (angle=0), the point should be back at the original position
    let p = s.point_at(0, 0.0).unwrap();
    // Point on circle of radius 2 at angle 0
    assert!((p[0] - 2.0).abs() < 1e-10);
    assert!(p[1].abs() < 1e-10);
}
