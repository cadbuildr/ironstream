// Integration tests for brep_prim_ext module
use ironstream::brep_prim_ext::*;
use std::f64::consts::PI;

#[test]
fn make_box_topology_and_volume() {
    let b = MakeBox::new(2.0, 3.0, 4.0);
    assert!(b.is_done());
    assert_eq!(b.nb_faces(), 6);
    assert_eq!(b.nb_edges(), 12);
    assert_eq!(b.nb_vertices(), 8);
    assert!((b.volume() - 24.0).abs() < 1e-10);
}

#[test]
fn make_box_face_access_one_based() {
    let b = MakeBox::new(1.0, 1.0, 1.0);
    assert!(b.is_done());
    // face(0) is out of range, face(1..6) are valid
    assert_eq!(b.face(0), None);
    assert!(b.face(1).is_some());
    assert!(b.face(6).is_some());
    assert_eq!(b.face(7), None);
}

#[test]
fn make_box_invalid_dimension_fails() {
    let b = MakeBox::new(0.0, 3.0, 4.0);
    assert!(!b.is_done());
    assert_eq!(b.nb_faces(), 0);
    assert_eq!(b.face(1), None);
}

#[test]
fn make_sphere_volume_and_surface_area() {
    let r = 2.0_f64;
    let s = MakeSphere::new(r);
    assert!(s.is_done());
    let expected_vol = 4.0 / 3.0 * PI * r.powi(3);
    let expected_area = 4.0 * PI * r.powi(2);
    assert!((s.volume() - expected_vol).abs() < 1e-6);
    assert!((s.surface_area() - expected_area).abs() < 1e-6);
}

#[test]
fn make_cylinder_volume() {
    let r = 1.0_f64;
    let h = 5.0_f64;
    let c = MakeCylinder::new(r, h);
    assert!(c.is_done());
    let expected = PI * r.powi(2) * h;
    assert!((c.volume() - expected).abs() < 1e-6);
}

#[test]
fn make_cone_slant_height_and_make_torus_volume() {
    // Cone: r1=3, r2=0, h=4 => slant = sqrt(9+16)=5
    let cone = MakeCone::new(3.0, 0.0, 4.0);
    assert!(cone.is_done());
    assert!((cone.slant_height() - 5.0).abs() < 1e-10);

    // Torus: r1=3, r2=1 => V=2*pi^2*3*1=~59.218
    let torus = MakeTorus::new(3.0, 1.0);
    assert!(torus.is_done());
    let expected = 2.0 * PI.powi(2) * 3.0 * 1.0_f64.powi(2);
    assert!((torus.volume() - expected).abs() < 1e-6);
}
