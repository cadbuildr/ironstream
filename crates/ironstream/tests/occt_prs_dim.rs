extern crate ironstream;

use ironstream::prs_dim::{AngleDimension, DiameterDimension, LengthDimension, RadiusDimension};

#[test]
fn length_dimension_3d() {
    let d = LengthDimension::new([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
    assert!((d.length() - 5.0).abs() < 1e-10);
}

#[test]
fn length_dimension_unit_factor() {
    let mut d = LengthDimension::new([0.0, 0.0, 0.0], [10.0, 0.0, 0.0]);
    d.set_unit_factor(0.001); // mm to m
    assert!((d.display_value() - 0.01).abs() < 1e-12);
}

#[test]
fn angle_dimension_degrees() {
    let d = AngleDimension::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    assert!((d.angle_degrees() - 90.0).abs() < 1e-8);
}

#[test]
fn radius_and_diameter() {
    let r = RadiusDimension::new([0.0, 0.0, 0.0], [5.0, 0.0, 0.0]);
    assert!((r.radius() - 5.0).abs() < 1e-10);
    assert!((r.diameter() - 10.0).abs() < 1e-10);
    let diam = DiameterDimension::new([0.0, 0.0, 0.0], 7.0);
    assert!((diam.diameter() - 14.0).abs() < 1e-10);
}

#[test]
fn angle_dimension_collinear_zero() {
    // Collinear points produce angle ~0
    let d = AngleDimension::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
    assert!(d.angle_radians() < 1e-8);
}
