extern crate ironstream;
use ironstream::geom_tube::*;
use std::f64::consts::PI;

#[test]
fn occt_cylinder_smoke() {
    let c = Cylinder::new([0.0, 0.0, 0.0], 2.0, 10.0);
    assert!((c.volume() - PI * 4.0 * 10.0).abs() < 1e-8);
}

#[test]
fn occt_torus_smoke() {
    let t = Torus::new([0.0, 0.0, 0.0], 5.0, 1.0);
    let p = t.point(0.0, 0.0);
    assert!((p[0] - 6.0).abs() < 1e-10);
}
