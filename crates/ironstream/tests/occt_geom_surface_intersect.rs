extern crate ironstream;
use ironstream::geom_surface_intersect::*;

#[test]
fn intcs_plane_intersection() {
    let curve = vec![[0.0,0.0,-2.0],[0.0,0.0,0.0],[0.0,0.0,2.0]];
    let mut intcs = IntCS::new(curve);
    intcs.set_plane([0.0,0.0,1.0], 0.0);
    intcs.perform();
    assert!(intcs.is_done());
    assert_eq!(intcs.nb_points(), 1);
    let pt = intcs.point(1).unwrap();
    assert!((pt.point[2]).abs() < 1e-8);
}

#[test]
fn intcs_no_intersection() {
    let curve = vec![[0.0,0.0,1.0],[0.0,0.0,2.0]];
    let mut intcs = IntCS::new(curve);
    intcs.set_plane([0.0,0.0,1.0], 0.0);
    intcs.perform();
    assert_eq!(intcs.nb_points(), 0);
}

#[test]
fn intss_with_added_curve() {
    let mut s = IntSS::new(0, 1, 1e-6);
    s.perform();
    let c = IntSSCurve::new(vec![[0.0,0.0,0.0],[1.0,1.0,0.0],[0.0,0.0,0.0]]);
    let is_closed = c.is_closed;
    s.add_curve(c);
    assert_eq!(s.nb_lines(), 1);
    let line = s.line(1).unwrap();
    assert_eq!(line.nb_points(), 3);
    assert!(is_closed);
}

#[test]
fn quadric_construction() {
    let pl = Quadric::plane([0.0,0.0,1.0], 0.0);
    assert_eq!(pl.name(), "Plane");
    assert_eq!(pl.qtype, QuadricType::Plane);
    let sp = Quadric::sphere([1.0,2.0,3.0], 5.0);
    assert_eq!(sp.name(), "Sphere");
    assert!((sp.radius - 5.0).abs() < 1e-10);
    let cyl = Quadric::cylinder([0.0,0.0,1.0], 2.0);
    assert_eq!(cyl.name(), "Cylinder");
}
