extern crate ironstream;
use ironstream::geom_hyperbola_tool::*;

#[test]
fn test_hyperbola3d_point_at_zero() {
    let h = Hyperbola3d::standard(3.0, 4.0);
    let p = h.point_at(0.0);
    // At t=0: cosh(0)=1, sinh(0)=0 → point = (major_radius, 0, 0)
    assert!((p[0] - 3.0).abs() < 1e-10);
    assert!(p[1].abs() < 1e-10);
}

#[test]
fn test_hyperbola3d_eccentricity() {
    let h = Hyperbola3d::standard(3.0, 4.0);
    // e = sqrt(a^2+b^2)/a = 5/3
    assert!((h.eccentricity() - 5.0 / 3.0).abs() < 1e-10);
    assert!(h.eccentricity() > 1.0);
}

#[test]
fn test_hyperbola3d_foci() {
    let h = Hyperbola3d::standard(3.0, 4.0);
    let (f1, f2) = h.foci();
    // focal distance = sqrt(9+16) = 5
    assert!((f1[0] - 5.0).abs() < 1e-10);
    assert!((f2[0] + 5.0).abs() < 1e-10);
}

#[test]
fn test_hyperbola3d_arc_length() {
    let h = Hyperbola3d::standard(2.0, 1.0);
    let l = h.arc_length(0.0, 1.0, 200);
    assert!(l > 0.0);
    // Conjugate branch at t=0 is at (-major_radius, 0, 0)
    let cj = h.conjugate_point_at(0.0);
    assert!((cj[0] + 2.0).abs() < 1e-10);
}

#[test]
fn test_hyperbola2d_contains_point() {
    let h = Hyperbola2d::new([0.0, 0.0], 2.0, 1.0, 0.0);
    let p = h.point_at(0.0);
    assert!(h.contains_point_approx(p, 1e-8));
    let e = h.eccentricity();
    assert!(e > 1.0);
}
