// FILE: tests/occt_prs3d_ext.rs
extern crate ironstream;

use ironstream::prs3d_ext::{
    Prs3dDisplayMode, Prs3dTypeOfHLR,
    Prs3dLineAspect, Prs3dShadingAspect, Prs3dBndBox, Prs3dDrawer,
};

#[test]
fn line_aspect_constructor_and_setters() {
    let mut la = Prs3dLineAspect::new(1.0, 0.0, 0.5, 2.5);
    assert_eq!(la.color(), [1.0, 0.0, 0.5]);
    assert!((la.width() - 2.5).abs() < 1e-6);
    la.set_color(0.1, 0.2, 0.3);
    assert_eq!(la.color(), [0.1, 0.2, 0.3]);
    la.set_width(4.0);
    assert!((la.width() - 4.0).abs() < 1e-6);
}

#[test]
fn shading_aspect_transparency_clamp() {
    let mut sa = Prs3dShadingAspect::new();
    sa.set_transparency(1.5); // should clamp to 1.0
    assert!((sa.transparency() - 1.0).abs() < 1e-6);
    assert!(sa.is_transparent());
    sa.set_transparency(0.0);
    assert!(!sa.is_transparent());
}

#[test]
fn shading_aspect_material() {
    let mut sa = Prs3dShadingAspect::new();
    sa.set_material("Steel");
    assert_eq!(sa.material_name, "Steel");
    sa.set_color(0.5, 0.5, 0.5);
    assert_eq!(sa.color(), [0.5, 0.5, 0.5]);
}

#[test]
fn bnd_box_setters() {
    let mut b = Prs3dBndBox::new();
    assert!(!b.is_oriented);
    b.set_oriented(true);
    assert!(b.is_oriented);
    b.set_draw_named_texts(true);
    assert!(b.draw_named_texts);
}

#[test]
fn drawer_defaults_and_setters() {
    let mut d = Prs3dDrawer::new();
    assert!((d.deviation_coefficient() - 0.001).abs() < 1e-15);
    assert!((d.deviation_angle() - 0.04).abs() < 1e-15);
    assert_eq!(d.nb_points(), 30);
    d.set_deviation_coefficient(0.01);
    assert!((d.deviation_coefficient() - 0.01).abs() < 1e-15);
    d.set_deviation_angle(0.1);
    assert!((d.deviation_angle() - 0.1).abs() < 1e-15);
}

#[test]
fn drawer_display_mode_and_hlr() {
    let mut d = Prs3dDrawer::new();
    assert_eq!(d.display_mode, Prs3dDisplayMode::Shading);
    d.set_display_mode(Prs3dDisplayMode::Wireframe);
    assert_eq!(d.display_mode, Prs3dDisplayMode::Wireframe);
    d.set_type_of_hlr(Prs3dTypeOfHLR::PolyAlgo);
    assert_eq!(d.type_of_hlr, Prs3dTypeOfHLR::PolyAlgo);
}
