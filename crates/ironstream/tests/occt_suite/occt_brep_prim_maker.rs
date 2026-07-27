use ironstream::brep_prim_maker::*;

#[test]
fn sphere_volume_and_surface_area() {
    use std::f64::consts::PI;
    let s = BrepPrimSphere::new(1.0);
    assert!((s.volume() - (4.0 / 3.0) * PI).abs() < 1e-10);
    assert!((s.surface_area() - 4.0 * PI).abs() < 1e-10);
}

#[test]
fn sphere_build_yields_six_faces() {
    let mut s = BrepPrimSphere::new(5.0);
    assert!(!s.is_done());
    s.build();
    assert!(s.is_done());
    let shell = s.shell.as_ref().expect("shell after build");
    assert_eq!(shell.nb_faces, 6);
}

#[test]
fn cone_apex_volume_and_two_faces() {
    use std::f64::consts::PI;
    let mut c = BrepPrimCone::new(3.0, 0.0, 4.0);
    let expected = PI * 4.0 / 3.0 * 9.0; // π*h/3*(r1²+0+0)
    assert!((c.volume() - expected).abs() < 1e-10);
    c.build();
    assert!(c.is_done());
    assert_eq!(c.shell.as_ref().unwrap().nb_faces, 2);
}

#[test]
fn cone_truncated_three_faces() {
    let mut c = BrepPrimCone::new(3.0, 1.5, 4.0);
    c.build();
    assert_eq!(c.shell.as_ref().unwrap().nb_faces, 3);
}

#[test]
fn cylinder_volume() {
    use std::f64::consts::PI;
    let mut cyl = BrepPrimCylinder::new(2.0, 5.0);
    assert!((cyl.volume() - PI * 4.0 * 5.0).abs() < 1e-10);
    cyl.build();
    assert!(cyl.is_done());
    assert!(cyl.shell.is_some());
}

#[test]
fn torus_volume() {
    use std::f64::consts::PI;
    let t = BrepPrimTorus::new(3.0, 1.0);
    let expected = 2.0 * PI * PI * 3.0 * 1.0_f64.powi(2);
    assert!((t.volume() - expected).abs() < 1e-10);
    let mut t2 = BrepPrimTorus::new(3.0, 1.0);
    t2.build();
    assert!(t2.is_done());
}
