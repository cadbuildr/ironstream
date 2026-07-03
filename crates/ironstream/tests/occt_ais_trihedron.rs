// FILE: tests/occt_ais_trihedron.rs
extern crate ironstream;
use ironstream::ais_trihedron::*;

#[test]
fn trihedron_default_is_orthonormal() {
    let t = Trihedron::new([0.0; 3], 1.0);
    assert!(t.axes_are_orthonormal());
}

#[test]
fn trihedron_axis_endpoints() {
    let t = Trihedron::new([1.0, 2.0, 3.0], 5.0);
    let xe = t.x_end();
    let ye = t.y_end();
    let ze = t.z_end();
    assert!((xe[0] - 6.0).abs() < 1e-10);
    assert!((ye[1] - 7.0).abs() < 1e-10);
    assert!((ze[2] - 8.0).abs() < 1e-10);
}

#[test]
fn trihedron_set_size() {
    let mut t = Trihedron::new([0.0; 3], 1.0);
    t.set_size(10.0);
    assert!((t.x_end()[0] - 10.0).abs() < 1e-10);
}

#[test]
fn trihedron_display_mode() {
    let mut t = Trihedron::new([0.0; 3], 2.0);
    t.set_display_mode(TrihedronDispMode::Shaded);
    assert_eq!(t.display_mode, TrihedronDispMode::Shaded);
}

#[test]
fn ais_axis_end_point() {
    let ax = AisAxis::new([0.0; 3], [0.0, 1.0, 0.0], 7.0);
    let ep = ax.end_point();
    assert!((ep[1] - 7.0).abs() < 1e-10);
    assert!(ep[0].abs() < 1e-10);
}

#[test]
fn plane_trihedron_u_v_perpendicular_to_normal() {
    let pt = PlaneTrihedron::new([0.0; 3], [0.0, 0.0, 1.0], 3.0);
    let u = pt.u_axis;
    let v = pt.v_axis();
    // Both should be perpendicular to z-normal
    assert!(u[2].abs() < 1e-10);
    assert!(v[2].abs() < 1e-10);
}
