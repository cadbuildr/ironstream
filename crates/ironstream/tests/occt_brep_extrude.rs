extern crate ironstream;
use ironstream::brep_extrude::*;

#[test]
fn make_prism_unit_square() {
    let profile = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[1.0,1.0,0.0],[0.0,1.0,0.0]];
    let mut p = MakePrism::new(profile, [0.0, 0.0, 1.0], 2.0);
    assert!(!p.is_done());
    p.build();
    assert!(p.is_done());
    assert!((p.volume() - 2.0).abs() < 1e-6);
}

#[test]
fn make_prism_top_z() {
    let profile = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
    let mut p = MakePrism::new(profile, [0.0, 0.0, 1.0], 5.0);
    p.build();
    let result = p.result.unwrap();
    assert!((result.top[0][2] - 5.0).abs() < 1e-10);
}

#[test]
fn make_dprism_scale() {
    let profile = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[1.0,1.0,0.0]];
    let mut d = MakeDPrism::new(profile, 10.0, 0.0);
    d.build();
    assert!(d.is_done());
    assert!((d.scale_at(0.0) - 1.0).abs() < 1e-10);
}

#[test]
fn make_linear_form_spine() {
    let spine = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0]];
    let prof = vec![[0.0,0.0,0.0],[0.0,1.0,0.0]];
    let mut m = MakeLinearForm::new(spine, prof, [0.0, 0.0, 1.0]);
    m.build();
    assert!(m.is_done());
    assert_eq!(m.nb_spine_pts(), 3);
}

#[test]
fn make_revol_angle() {
    let profile = vec![[1.0,0.0,0.0],[2.0,0.0,0.0]];
    let mut r = MakeRevol::new(profile, [0.0,0.0,0.0], [0.0,0.0,1.0], 90.0);
    r.build();
    assert!(r.is_done());
    assert!((r.angle_radians() - std::f64::consts::PI / 2.0).abs() < 1e-8);
}
