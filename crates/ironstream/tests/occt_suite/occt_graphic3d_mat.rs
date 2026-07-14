extern crate ironstream;
use ironstream::graphic3d_mat::*;

#[test]
fn test_graphic3d_mat_basic() {
    let brass = MaterialAspect::new(MaterialName::Brass);
    assert_eq!(brass.name, MaterialName::Brass);
    assert!(!brass.is_transparent());
    assert!(brass.shininess > 0.0);

    let gold = MaterialAspect::default_brass();
    assert_eq!(gold.name, MaterialName::Brass);

    let mut m = MaterialAspect::new(MaterialName::Silver);
    m.set_transparency(0.5);
    assert!(m.is_transparent());
    assert!((m.transparency - 0.5).abs() < 1e-10);

    m.set_shininess(0.8);
    assert!((m.shininess - 0.8).abs() < 1e-10);

    // clamp check
    m.set_transparency(2.0);
    assert!((m.transparency - 1.0).abs() < 1e-10);
}

#[test]
fn test_bsdf_basic() {
    let b = Bsdf::new();
    assert_eq!(b.kd, [0.0, 0.0, 0.0]);

    let d = Bsdf::diffuse([0.8, 0.2, 0.1]);
    assert_eq!(d.kd, [0.8, 0.2, 0.1]);
    assert_eq!(d.ks, [0.0, 0.0, 0.0]);

    let m = Bsdf::metal([0.9, 0.9, 0.9], 0.3);
    assert_eq!(m.ks, [0.9, 0.9, 0.9]);

    let def = Bsdf::default();
    assert_eq!(def.kd, [0.0, 0.0, 0.0]);
}
