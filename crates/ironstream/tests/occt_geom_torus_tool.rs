extern crate ironstream;
use ironstream::geom_torus_tool::*;

#[test]
fn torus_volume_and_area() {
    let t = TorusParams::new([0.0;3], [0.0,0.0,1.0], 3.0, 1.0);
    let expected_vol = 2.0 * std::f64::consts::PI.powi(2) * 3.0;
    assert!((t.volume() - expected_vol).abs() < 1e-8);
    let expected_area = 4.0 * std::f64::consts::PI.powi(2) * 3.0;
    assert!((t.surface_area() - expected_area).abs() < 1e-8);
}

#[test]
fn torus_point_on_surface() {
    let t = TorusParams::new([0.0;3], [0.0,0.0,1.0], 3.0, 1.0);
    let p = t.point_at(0.0, 0.0);
    assert!((p[0] - 4.0).abs() < 1e-8); // major + minor at u=0, v=0
    assert!((p[1]).abs() < 1e-8);
    assert!((p[2]).abs() < 1e-8);
}

#[test]
fn torus_contains_point() {
    let t = TorusParams::new([0.0;3], [0.0,0.0,1.0], 3.0, 1.0);
    assert!(t.contains_point([3.0, 0.0, 0.0])); // on the tube center circle
    assert!(!t.contains_point([0.0, 0.0, 0.0])); // at origin (inside hole)
    assert!(!t.contains_point([10.0, 0.0, 0.0])); // far outside
}

#[test]
fn torus_discretize() {
    let t = TorusParams::new([0.0;3], [0.0,0.0,1.0], 2.0, 0.5);
    let pts = t.discretize(8, 6);
    assert_eq!(pts.len(), 48);
}

#[test]
fn torus_sections() {
    let params = TorusParams::new([0.0;3], [0.0,0.0,1.0], 4.0, 1.0);
    let full = TorusSections::full(params);
    assert!(full.is_full());
    let partial = TorusSections::partial(params, 0.0, 180.0);
    assert!(!partial.is_full());
    assert!((partial.angle_deg() - 180.0).abs() < 1e-6);
}
