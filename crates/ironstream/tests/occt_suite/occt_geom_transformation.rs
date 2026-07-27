// FILE: tests/occt_geom_transformation.rs
extern crate ironstream;
use ironstream::geom_transformation::Geom_Transformation;
use ironstream::gp_trsf::{Trsf as gp_Trsf, Form as TrsfForm};
use ironstream::gp::{Ax1, Pnt};

const EPS: f64 = 1e-9;

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

#[test]
fn test_identity_transformation() {
    let t = Geom_Transformation::new();
    assert!(matches!(t.form(), TrsfForm::Identity));
    assert!(approx_eq(t.value(1, 1), 1.0));
    assert!(approx_eq(t.value(2, 2), 1.0));
    assert!(approx_eq(t.value(3, 3), 1.0));
    assert!(approx_eq(t.value(1, 2), 0.0));
    assert!(approx_eq(t.value(1, 3), 0.0));
    assert!(approx_eq(t.value(2, 1), 0.0));
    assert!(approx_eq(t.value(2, 3), 0.0));
    assert!(approx_eq(t.value(3, 1), 0.0));
    assert!(approx_eq(t.value(3, 2), 0.0));
    assert!(approx_eq(t.value(1, 4), 0.0));
    assert!(approx_eq(t.value(2, 4), 0.0));
    assert!(approx_eq(t.value(3, 4), 0.0));
    assert!(approx_eq(t.scale_factor(), 1.0));
    assert!(!t.is_negative());
}

#[test]
fn test_translation_values() {
    let mut trsf = gp_Trsf::default();
    trsf.set_translation(Pnt::new(10.0, 20.0, 30.0));
    let t = Geom_Transformation::from_trsf(trsf);
    assert!(matches!(t.form(), TrsfForm::Translation));
    assert!(approx_eq(t.value(1, 4), 10.0));
    assert!(approx_eq(t.value(2, 4), 20.0));
    assert!(approx_eq(t.value(3, 4), 30.0));
    assert!(approx_eq(t.value(1, 1), 1.0));
    assert!(approx_eq(t.value(2, 2), 1.0));
    assert!(approx_eq(t.value(3, 3), 1.0));
    assert!(!t.is_negative());
    assert!(approx_eq(t.scale_factor(), 1.0));
}

#[test]
fn test_scale_values() {
    let mut trsf = gp_Trsf::default();
    let origin = Pnt::new(0.0, 0.0, 0.0);
    trsf.set_scale(origin, 3.0).expect("scale");
    let t = Geom_Transformation::from_trsf(trsf);
    assert!(matches!(t.form(), TrsfForm::Scale));
    assert!(approx_eq(t.scale_factor(), 3.0));
    assert!(approx_eq(t.value(1, 1), 3.0));
    assert!(approx_eq(t.value(2, 2), 3.0));
    assert!(approx_eq(t.value(3, 3), 3.0));
    assert!(approx_eq(t.value(1, 4), 0.0));
    assert!(!t.is_negative());
}

#[test]
fn test_rotation_90_around_z() {
    let mut trsf = gp_Trsf::default();
    let origin = Pnt::new(0.0, 0.0, 0.0);
    let z = Pnt::new(0.0, 0.0, 1.0);
    let ax = Ax1::new(origin, z);
    trsf.set_rotation(ax, std::f64::consts::FRAC_PI_2);
    let t = Geom_Transformation::from_trsf(trsf);
    assert!(matches!(t.form(), TrsfForm::Rotation));
    // [0,-1,0; 1,0,0; 0,0,1]
    assert!(approx_eq(t.value(1, 1), 0.0));
    assert!(approx_eq(t.value(1, 2), -1.0));
    assert!(approx_eq(t.value(1, 3), 0.0));
    assert!(approx_eq(t.value(2, 1), 1.0));
    assert!(approx_eq(t.value(2, 2), 0.0));
    assert!(approx_eq(t.value(2, 3), 0.0));
    assert!(approx_eq(t.value(3, 1), 0.0));
    assert!(approx_eq(t.value(3, 2), 0.0));
    assert!(approx_eq(t.value(3, 3), 1.0));
    assert!(approx_eq(t.value(1, 4), 0.0));
    assert!(approx_eq(t.value(2, 4), 0.0));
    assert!(approx_eq(t.value(3, 4), 0.0));
    assert!(!t.is_negative());
    assert!(approx_eq(t.scale_factor(), 1.0));
}

#[test]
fn test_rotation_180_around_x() {
    let mut trsf = gp_Trsf::default();
    let origin = Pnt::new(0.0, 0.0, 0.0);
    let x = Pnt::new(1.0, 0.0, 0.0);
    let ax = Ax1::new(origin, x);
    trsf.set_rotation(ax, std::f64::consts::PI);
    let t = Geom_Transformation::from_trsf(trsf);
    // R = [[1,0,0],[0,-1,0],[0,0,-1]]
    assert!(approx_eq(t.value(1, 1), 1.0));
    assert!(approx_eq(t.value(1, 2), 0.0));
    assert!(approx_eq(t.value(2, 2), -1.0));
    assert!(approx_eq(t.value(3, 3), -1.0));
    assert!(!t.is_negative());
}

#[test]
fn test_compose_rotation_and_translation() {
    // Rotate 90 around Z then translate (5, 0, 0)
    let origin = Pnt::new(0.0, 0.0, 0.0);
    let z = Pnt::new(0.0, 0.0, 1.0);
    let ax = Ax1::new(origin, z);

    let mut rot_trsf = gp_Trsf::default();
    rot_trsf.set_rotation(ax, std::f64::consts::FRAC_PI_2);
    let rot = Geom_Transformation::from_trsf(rot_trsf);

    let mut trans_trsf = gp_Trsf::default();
    trans_trsf.set_translation(Pnt::new(5.0, 0.0, 0.0));
    let trans = Geom_Transformation::from_trsf(trans_trsf);

    // trans * rot: apply rotation first, then translation
    let composed = trans.multiplied(&rot);
    // Rotation part is same as rot
    assert!(approx_eq(composed.value(1, 1), 0.0));
    assert!(approx_eq(composed.value(1, 2), -1.0));
    assert!(approx_eq(composed.value(2, 1), 1.0));
    assert!(approx_eq(composed.value(2, 2), 0.0));
    // Translation column: trans applies after rot, so tx=5, ty=0, tz=0
    assert!(approx_eq(composed.value(1, 4), 5.0));
    assert!(approx_eq(composed.value(2, 4), 0.0));
    assert!(approx_eq(composed.value(3, 4), 0.0));
}

#[test]
fn test_invert_then_compose_is_identity() {
    let mut trsf = gp_Trsf::default();
    let origin = Pnt::new(0.0, 0.0, 0.0);
    let y = Pnt::new(0.0, 1.0, 0.0);
    let ax = Ax1::new(origin, y);
    trsf.set_rotation(ax, 0.7854); // ~45 degrees
    let t = Geom_Transformation::from_trsf(trsf);
    let inv = t.invert();
    let result = t.multiplied(&inv);
    assert!(approx_eq(result.value(1, 1), 1.0));
    assert!(approx_eq(result.value(2, 2), 1.0));
    assert!(approx_eq(result.value(3, 3), 1.0));
    assert!(approx_eq(result.value(1, 2), 0.0));
    assert!(approx_eq(result.value(1, 3), 0.0));
    assert!(approx_eq(result.value(2, 1), 0.0));
    assert!(approx_eq(result.value(1, 4), 0.0));
    assert!(approx_eq(result.value(2, 4), 0.0));
    assert!(approx_eq(result.value(3, 4), 0.0));
}

#[test]
fn test_invert_translation_numeric() {
    let mut trsf = gp_Trsf::default();
    trsf.set_translation(Pnt::new(-5.0, 3.0, 7.5));
    let t = Geom_Transformation::from_trsf(trsf);
    let inv = t.invert();
    assert!(approx_eq(inv.value(1, 4), 5.0));
    assert!(approx_eq(inv.value(2, 4), -3.0));
    assert!(approx_eq(inv.value(3, 4), -7.5));
}

#[test]
fn test_invert_scale_numeric() {
    let mut trsf = gp_Trsf::default();
    let origin = Pnt::new(0.0, 0.0, 0.0);
    trsf.set_scale(origin, 5.0).expect("scale");
    let t = Geom_Transformation::from_trsf(trsf);
    let inv = t.invert();
    assert!(approx_eq(inv.scale_factor(), 0.2));
    assert!(approx_eq(inv.value(1, 1), 0.2));
    assert!(approx_eq(inv.value(2, 2), 0.2));
    assert!(approx_eq(inv.value(3, 3), 0.2));
}

#[test]
fn test_point_mirror_is_negative() {
    let mut trsf = gp_Trsf::default();
    let pt = Pnt::new(0.0, 0.0, 0.0);
    trsf.set_mirror_point(pt);
    let t = Geom_Transformation::from_trsf(trsf);
    assert!(matches!(t.form(), TrsfForm::PntMirror));
    assert!(t.is_negative());
    assert!(approx_eq(t.value(1, 1), -1.0));
    assert!(approx_eq(t.value(2, 2), -1.0));
    assert!(approx_eq(t.value(3, 3), -1.0));
    assert!(approx_eq(t.value(1, 4), 0.0));
    assert!(approx_eq(t.value(2, 4), 0.0));
    assert!(approx_eq(t.value(3, 4), 0.0));
}

#[test]
fn test_point_mirror_at_non_origin() {
    let mut trsf = gp_Trsf::default();
    let pt = Pnt::new(2.0, 3.0, 1.0);
    trsf.set_mirror_point(pt);
    let t = Geom_Transformation::from_trsf(trsf);
    assert!(matches!(t.form(), TrsfForm::PntMirror));
    assert!(t.is_negative());
    // Translation = 2 * pt
    assert!(approx_eq(t.value(1, 4), 4.0));
    assert!(approx_eq(t.value(2, 4), 6.0));
    assert!(approx_eq(t.value(3, 4), 2.0));
}

#[test]
fn test_scale_at_non_origin() {
    let mut trsf = gp_Trsf::default();
    let center = Pnt::new(1.0, 0.0, 0.0);
    trsf.set_scale(center, 2.0).expect("scale");
    let t = Geom_Transformation::from_trsf(trsf);
    assert!(approx_eq(t.scale_factor(), 2.0));
    // Point (1,0,0) should map to itself; point (2,0,0) -> (3,0,0)
    // Matrix diagonal = 2, translation = (1-2, 0, 0) = (-1, 0, 0)
    assert!(approx_eq(t.value(1, 1), 2.0));
    assert!(approx_eq(t.value(1, 4), -1.0));
    assert!(approx_eq(t.value(2, 4), 0.0));
    assert!(approx_eq(t.value(3, 4), 0.0));
}

#[test]
fn test_compose_three_transformations() {
    // t1: translate (1,0,0), t2: translate (0,1,0), t3: translate (0,0,1)
    // composed = t3 * t2 * t1 should translate (1,1,1)
    let mut t1_trsf = gp_Trsf::default();
    t1_trsf.set_translation(Pnt::new(1.0, 0.0, 0.0));
    let t1 = Geom_Transformation::from_trsf(t1_trsf);

    let mut t2_trsf = gp_Trsf::default();
    t2_trsf.set_translation(Pnt::new(0.0, 1.0, 0.0));
    let t2 = Geom_Transformation::from_trsf(t2_trsf);

    let mut t3_trsf = gp_Trsf::default();
    t3_trsf.set_translation(Pnt::new(0.0, 0.0, 1.0));
    let t3 = Geom_Transformation::from_trsf(t3_trsf);

    let c = t3.multiplied(&t2).multiplied(&t1);
    assert!(approx_eq(c.value(1, 4), 1.0));
    assert!(approx_eq(c.value(2, 4), 1.0));
    assert!(approx_eq(c.value(3, 4), 1.0));
}

#[test]
fn test_get_set_trsf_roundtrip() {
    let t_orig = Geom_Transformation::new();
    let mut t = t_orig.clone();

    let mut new_trsf = gp_Trsf::default();
    new_trsf.set_translation(Pnt::new(100.0, 200.0, 300.0));
    t.set_trsf(new_trsf);

    assert!(approx_eq(t.value(1, 4), 100.0));
    assert!(approx_eq(t.value(2, 4), 200.0));
    assert!(approx_eq(t.value(3, 4), 300.0));

    // original should be unchanged
    assert!(approx_eq(t_orig.value(1, 4), 0.0));
}

#[test]
fn test_scale_non_positive_handled() {
    // Negative scale should produce a negative transformation
    let mut trsf = gp_Trsf::default();
    let origin = Pnt::new(0.0, 0.0, 0.0);
    trsf.set_scale(origin, -1.0).expect("scale");
    let t = Geom_Transformation::from_trsf(trsf);
    assert!(approx_eq(t.scale_factor(), -1.0));
    assert!(t.is_negative());
}

#[test]
fn test_rotation_full_circle_is_identity() {
    let mut trsf = gp_Trsf::default();
    let origin = Pnt::new(0.0, 0.0, 0.0);
    let z = Pnt::new(0.0, 0.0, 1.0);
    let ax = Ax1::new(origin, z);
    trsf.set_rotation(ax, 2.0 * std::f64::consts::PI);
    let t = Geom_Transformation::from_trsf(trsf);
    assert!(approx_eq(t.value(1, 1), 1.0));
    assert!(approx_eq(t.value(2, 2), 1.0));
    assert!(approx_eq(t.value(3, 3), 1.0));
    assert!(approx_eq(t.value(1, 2), 0.0));
    assert!(approx_eq(t.value(2, 1), 0.0));
}

#[test]
fn test_default_is_identity() {
    let t: Geom_Transformation = Default::default();
    assert!(matches!(t.form(), TrsfForm::Identity));
    assert!(approx_eq(t.scale_factor(), 1.0));
}
