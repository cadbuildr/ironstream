// FILE: tests/occt_prs3d.rs
extern crate ironstream;
use ironstream::prs3d::*;

#[test]
fn color_new_rgb_alpha_one() {
    let c = Prs3dColor::new(0.2, 0.4, 0.6);
    assert_eq!(c.r, 0.2);
    assert_eq!(c.g, 0.4);
    assert_eq!(c.b, 0.6);
    assert_eq!(c.a, 1.0);
}

#[test]
fn color_with_alpha_stores_all_channels() {
    let c = Prs3dColor::with_alpha(0.1, 0.2, 0.3, 0.75);
    assert_eq!(c.r, 0.1);
    assert_eq!(c.g, 0.2);
    assert_eq!(c.b, 0.3);
    assert_eq!(c.a, 0.75);
}

#[test]
fn predefined_colors_are_correct() {
    assert_eq!(Prs3dColor::WHITE.r, 1.0);
    assert_eq!(Prs3dColor::WHITE.g, 1.0);
    assert_eq!(Prs3dColor::WHITE.b, 1.0);
    assert_eq!(Prs3dColor::BLACK.r, 0.0);
    assert_eq!(Prs3dColor::BLACK.g, 0.0);
    assert_eq!(Prs3dColor::BLACK.b, 0.0);
    assert_eq!(Prs3dColor::RED.r, 1.0);
    assert_eq!(Prs3dColor::RED.g, 0.0);
    assert_eq!(Prs3dColor::GREEN.g, 1.0);
    assert_eq!(Prs3dColor::GREEN.r, 0.0);
    assert_eq!(Prs3dColor::BLUE.b, 1.0);
    assert_eq!(Prs3dColor::BLUE.r, 0.0);
}

#[test]
fn predefined_colors_are_opaque() {
    assert_eq!(Prs3dColor::WHITE.a, 1.0);
    assert_eq!(Prs3dColor::BLACK.a, 1.0);
    assert_eq!(Prs3dColor::RED.a, 1.0);
    assert_eq!(Prs3dColor::GREEN.a, 1.0);
    assert_eq!(Prs3dColor::BLUE.a, 1.0);
}

#[test]
fn line_type_five_distinct_variants() {
    let types = [
        Prs3dLineType::Solid,
        Prs3dLineType::Dash,
        Prs3dLineType::Dot,
        Prs3dLineType::DashDot,
        Prs3dLineType::DashDotDot,
    ];
    assert_eq!(types.len(), 5);
    for i in 0..types.len() {
        for j in 0..types.len() {
            if i == j {
                assert_eq!(types[i], types[j]);
            } else {
                assert_ne!(types[i], types[j]);
            }
        }
    }
}

#[test]
fn line_aspect_construction_and_fields() {
    let la = Prs3dLineAspect::new(Prs3dColor::GREEN, Prs3dLineType::Dot, 1.5);
    assert_eq!(la.color, Prs3dColor::GREEN);
    assert_eq!(la.line_type, Prs3dLineType::Dot);
    assert_eq!(la.width, 1.5);
}

#[test]
fn line_aspect_setters_update_each_field_independently() {
    let mut la = Prs3dLineAspect::new(Prs3dColor::WHITE, Prs3dLineType::Solid, 1.0);
    la.set_color(Prs3dColor::RED);
    assert_eq!(la.color, Prs3dColor::RED);
    assert_eq!(la.line_type, Prs3dLineType::Solid);

    la.set_width(4.0);
    assert_eq!(la.width, 4.0);

    la.set_type(Prs3dLineType::DashDotDot);
    assert_eq!(la.line_type, Prs3dLineType::DashDotDot);
}

#[test]
fn shading_aspect_default_values() {
    let sa = Prs3dShadingAspect::new();
    assert_eq!(sa.color(), Prs3dColor::WHITE);
    assert_eq!(sa.transparency(), 0.0);
    assert_eq!(sa.shininess(), 0.3);
    assert!(sa.ambient() >= 0.0);
    assert!(sa.diffuse() >= 0.0);
    assert!(sa.specular() >= 0.0);
}

#[test]
fn shading_aspect_setters_round_trip() {
    let mut sa = Prs3dShadingAspect::new();
    sa.set_color(Prs3dColor::BLUE);
    sa.set_transparency(0.4);
    sa.set_shininess(0.9);
    sa.set_ambient(0.1);
    sa.set_diffuse(0.7);
    sa.set_specular(0.2);
    assert_eq!(sa.color(), Prs3dColor::BLUE);
    assert_eq!(sa.transparency(), 0.4);
    assert_eq!(sa.shininess(), 0.9);
    assert_eq!(sa.ambient(), 0.1);
    assert_eq!(sa.diffuse(), 0.7);
    assert_eq!(sa.specular(), 0.2);
}

#[test]
fn drawer_default_iso_counts_are_one() {
    let d = Prs3dDrawer::new();
    assert_eq!(d.nb_iso_u(), 1);
    assert_eq!(d.nb_iso_v(), 1);
}

#[test]
fn drawer_default_coefficients_are_positive() {
    let d = Prs3dDrawer::new();
    assert!(d.deviation_coefficient() > 0.0);
    assert!(d.max_param_value() > 0.0);
}

#[test]
fn drawer_iso_setters() {
    let mut d = Prs3dDrawer::new();
    d.set_nb_iso_u(8);
    d.set_nb_iso_v(4);
    assert_eq!(d.nb_iso_u(), 8);
    assert_eq!(d.nb_iso_v(), 4);
}

#[test]
fn drawer_deviation_and_max_param_setters() {
    let mut d = Prs3dDrawer::new();
    d.set_deviation_coefficient(0.0005);
    d.set_max_param_value(250_000.0);
    assert_eq!(d.deviation_coefficient(), 0.0005);
    assert_eq!(d.max_param_value(), 250_000.0);
}

#[test]
fn drawer_set_line_aspect() {
    let mut d = Prs3dDrawer::new();
    let la = Prs3dLineAspect::new(Prs3dColor::BLACK, Prs3dLineType::Dash, 2.0);
    d.set_line_aspect(la);
    assert_eq!(d.line_aspect().color, Prs3dColor::BLACK);
    assert_eq!(d.line_aspect().line_type, Prs3dLineType::Dash);
    assert_eq!(d.line_aspect().width, 2.0);
}

#[test]
fn drawer_set_shading_aspect() {
    let mut d = Prs3dDrawer::new();
    let mut sa = Prs3dShadingAspect::new();
    sa.set_color(Prs3dColor::RED);
    sa.set_transparency(0.3);
    d.set_shading_aspect(sa);
    assert_eq!(d.shading_aspect().color(), Prs3dColor::RED);
    assert_eq!(d.shading_aspect().transparency(), 0.3);
}
