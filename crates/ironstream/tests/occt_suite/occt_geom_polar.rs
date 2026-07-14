extern crate ironstream;
use ironstream::geom_polar::*;

#[test]
fn spherical_round_trip() {
    let p = [3.0, 4.0, 5.0];
    let s = cartesian_to_spherical(p);
    let p2 = spherical_to_cartesian(s);
    assert!((p2[0]-p[0]).abs() < 1e-10);
    assert!((p2[1]-p[1]).abs() < 1e-10);
    assert!((p2[2]-p[2]).abs() < 1e-10);
}

#[test]
fn cylindrical_round_trip() {
    let p = [1.0, 1.0, 5.0];
    let c = cartesian_to_cylindrical(p);
    let p2 = cylindrical_to_cartesian(c);
    assert!((p2[0]-p[0]).abs() < 1e-10);
    assert!((p2[1]-p[1]).abs() < 1e-10);
    assert!((p2[2]-p[2]).abs() < 1e-10);
}

#[test]
fn elslib_surface_points() {
    let p_cyl = ElSLib::cylinder_point(2.0, 0.0, 3.0);
    assert!((p_cyl[0] - 2.0).abs() < 1e-10);
    assert!((p_cyl[2] - 3.0).abs() < 1e-10);
    let p_sph = ElSLib::sphere_point(5.0, 0.0, 0.0);
    assert!((p_sph[0] - 5.0).abs() < 1e-10);
    let period = ElSLib::period();
    assert!((period - 2.0 * std::f64::consts::PI).abs() < 1e-10);
}

#[test]
fn elclib_curve_points() {
    let p_circ = ElCLib::circle_point(3.0, [0.0;3], 0.0);
    assert!((p_circ[0] - 3.0).abs() < 1e-10);
    let p_ell = ElCLib::ellipse_point([0.0;3], 4.0, 2.0, 0.0);
    assert!((p_ell[0] - 4.0).abs() < 1e-10);
    let t = ElCLib::line_parameter([0.0;3], [1.0,0.0,0.0], [5.0,0.0,0.0]);
    assert!((t - 5.0).abs() < 1e-10);
}

#[test]
fn torus_surface_point() {
    let p = ElSLib::torus_point(3.0, 1.0, 0.0, 0.0);
    assert!((p[0] - 4.0).abs() < 1e-10);
    assert!((p[1]).abs() < 1e-10);
}
