extern crate ironstream;
use ironstream::geom_catenary::*;

#[test]
fn occt_catenary_minimum() {
    let c = Catenary::new(5.0);
    let p = c.point(0.0);
    assert!((p[1] - 5.0).abs() < 1e-10);
}

#[test]
fn occt_clothoid_curvature() {
    let cl = Clothoid::new(4.0);
    assert_eq!(cl.curvature(0.0), 0.0);
}
