extern crate ironstream;
use ironstream::geom_cone_tool::*;
use std::f64::consts::PI;

#[test]
fn occt_cone_smoke() {
    let c = Cone::new([0.0, 0.0, 0.0], 3.0, 1.0, 4.0);
    assert!(c.volume() > 0.0);
}

#[test]
fn occt_sphere_surface_area() {
    let s = Sphere::new([0.0, 0.0, 0.0], 1.0);
    assert!((s.surface_area() - 4.0 * PI).abs() < 1e-8);
}
