// FILE: tests/occt_prs3d_drawer.rs
extern crate ironstream;
use ironstream::prs3d_drawer::*;

// --- Prs3dTypeOfHLR ---

#[test]
fn hlr_type_three_variants_distinct() {
    assert_ne!(Prs3dTypeOfHLR::NotSet, Prs3dTypeOfHLR::PolyAlgo);
    assert_ne!(Prs3dTypeOfHLR::NotSet, Prs3dTypeOfHLR::Algo);
    assert_ne!(Prs3dTypeOfHLR::PolyAlgo, Prs3dTypeOfHLR::Algo);
}

#[test]
fn hlr_type_is_copy() {
    let h = Prs3dTypeOfHLR::PolyAlgo;
    let h2 = h;
    assert_eq!(h, h2);
}

#[test]
fn hlr_type_debug_non_empty() {
    assert_eq!(format!("{:?}", Prs3dTypeOfHLR::NotSet), "NotSet");
    assert_eq!(format!("{:?}", Prs3dTypeOfHLR::PolyAlgo), "PolyAlgo");
    assert_eq!(format!("{:?}", Prs3dTypeOfHLR::Algo), "Algo");
}

#[test]
fn hlr_type_hash_in_set() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Prs3dTypeOfHLR::NotSet);
    set.insert(Prs3dTypeOfHLR::PolyAlgo);
    set.insert(Prs3dTypeOfHLR::Algo);
    set.insert(Prs3dTypeOfHLR::NotSet); // duplicate
    assert_eq!(set.len(), 3);
}

// --- Prs3dLineAspect ---

#[test]
fn line_aspect_new_stores_fields() {
    let color = [1.0f32, 0.0, 0.0, 1.0];
    let la = Prs3dLineAspect::new(color, 2.5, 1);
    assert_eq!(la.color(), color);
    assert_eq!(la.width(), 2.5f32);
    assert_eq!(la.line_type(), 1u8);
}

#[test]
fn line_aspect_solid_type_zero() {
    let la = Prs3dLineAspect::new([0.0, 0.0, 0.0, 1.0], 1.0, 0);
    assert_eq!(la.line_type(), 0);
}

#[test]
fn line_aspect_dashed_type_one() {
    let la = Prs3dLineAspect::new([0.0, 0.0, 0.0, 1.0], 1.0, 1);
    assert_eq!(la.line_type(), 1);
}

#[test]
fn line_aspect_dotted_type_two() {
    let la = Prs3dLineAspect::new([0.0, 0.0, 0.0, 1.0], 1.0, 2);
    assert_eq!(la.line_type(), 2);
}

#[test]
fn line_aspect_dashdot_type_three() {
    let la = Prs3dLineAspect::new([0.0, 0.0, 0.0, 1.0], 1.0, 3);
    assert_eq!(la.line_type(), 3);
}

#[test]
fn line_aspect_set_color_mutates() {
    let mut la = Prs3dLineAspect::new([1.0, 1.0, 1.0, 1.0], 1.0, 0);
    let new_color = [0.0f32, 0.5, 1.0, 0.8];
    la.set_color(new_color);
    assert_eq!(la.color(), new_color);
}

#[test]
fn line_aspect_set_width_mutates() {
    let mut la = Prs3dLineAspect::new([1.0, 1.0, 1.0, 1.0], 1.0, 0);
    la.set_width(4.5);
    assert_eq!(la.width(), 4.5f32);
}

#[test]
fn line_aspect_set_line_type_mutates() {
    let mut la = Prs3dLineAspect::new([1.0, 1.0, 1.0, 1.0], 1.0, 0);
    la.set_line_type(3);
    assert_eq!(la.line_type(), 3);
}

#[test]
fn line_aspect_color_is_four_channel() {
    let color = [0.1f32, 0.2, 0.3, 0.4];
    let la = Prs3dLineAspect::new(color, 1.0, 0);
    let c = la.color();
    assert_eq!(c[0], 0.1f32);
    assert_eq!(c[1], 0.2f32);
    assert_eq!(c[2], 0.3f32);
    assert_eq!(c[3], 0.4f32);
}

#[test]
fn line_aspect_clone() {
    let la = Prs3dLineAspect::new([1.0, 0.0, 0.0, 1.0], 2.0, 1);
    let la2 = la.clone();
    assert_eq!(la, la2);
}

// --- Prs3dShadingAspect ---

#[test]
fn shading_aspect_default_white_color() {
    let sa = Prs3dShadingAspect::new();
    assert_eq!(sa.color(), [1.0f32, 1.0, 1.0, 1.0]);
}

#[test]
fn shading_aspect_default_ambient_0_1() {
    let sa = Prs3dShadingAspect::new();
    assert!((sa.ambient() - 0.1f32).abs() < 1e-6);
}

#[test]
fn shading_aspect_default_diffuse_0_8() {
    let sa = Prs3dShadingAspect::new();
    assert!((sa.diffuse() - 0.8f32).abs() < 1e-6);
}

#[test]
fn shading_aspect_default_specular_0_5() {
    let sa = Prs3dShadingAspect::new();
    assert!((sa.specular() - 0.5f32).abs() < 1e-6);
}

#[test]
fn shading_aspect_default_shininess_64() {
    let sa = Prs3dShadingAspect::new();
    assert!((sa.shininess() - 64.0f32).abs() < 1e-4);
}

#[test]
fn shading_aspect_default_transparency_zero() {
    let sa = Prs3dShadingAspect::new();
    assert_eq!(sa.transparency(), 0.0f32);
}

#[test]
fn shading_aspect_set_color_mutates() {
    let mut sa = Prs3dShadingAspect::new();
    let red = [1.0f32, 0.0, 0.0, 1.0];
    sa.set_color(red);
    assert_eq!(sa.color(), red);
}

#[test]
fn shading_aspect_set_transparency_mutates() {
    let mut sa = Prs3dShadingAspect::new();
    sa.set_transparency(0.5);
    assert!((sa.transparency() - 0.5f32).abs() < 1e-6);
}

#[test]
fn shading_aspect_set_ambient_mutates() {
    let mut sa = Prs3dShadingAspect::new();
    sa.set_ambient(0.3);
    assert!((sa.ambient() - 0.3f32).abs() < 1e-6);
}

#[test]
fn shading_aspect_set_diffuse_mutates() {
    let mut sa = Prs3dShadingAspect::new();
    sa.set_diffuse(0.6);
    assert!((sa.diffuse() - 0.6f32).abs() < 1e-6);
}

#[test]
fn shading_aspect_set_specular_mutates() {
    let mut sa = Prs3dShadingAspect::new();
    sa.set_specular(0.2);
    assert!((sa.specular() - 0.2f32).abs() < 1e-6);
}

#[test]
fn shading_aspect_set_shininess_mutates() {
    let mut sa = Prs3dShadingAspect::new();
    sa.set_shininess(128.0);
    assert!((sa.shininess() - 128.0f32).abs() < 1e-4);
}

#[test]
fn shading_aspect_default_trait_matches_new() {
    let a = Prs3dShadingAspect::new();
    let b = Prs3dShadingAspect::default();
    assert_eq!(a, b);
}

#[test]
fn shading_aspect_clone() {
    let sa = Prs3dShadingAspect::new();
    let sa2 = sa.clone();
    assert_eq!(sa, sa2);
}

// --- Prs3dDrawer ---

#[test]
fn drawer_new_hlr_type_is_not_set() {
    let d = Prs3dDrawer::new();
    assert_eq!(d.hlr_type(), Prs3dTypeOfHLR::NotSet);
}

#[test]
fn drawer_new_nb_points_is_30() {
    let d = Prs3dDrawer::new();
    assert_eq!(d.nb_points(), 30);
}

#[test]
fn drawer_new_max_flyflyangle_is_20() {
    let d = Prs3dDrawer::new();
    assert!((d.max_flyflyangle() - 20.0f64).abs() < 1e-10);
}

#[test]
fn drawer_new_wire_aspect_solid() {
    let d = Prs3dDrawer::new();
    assert_eq!(d.wire_aspect().line_type(), 0);
}

#[test]
fn drawer_new_free_wire_aspect_solid() {
    let d = Prs3dDrawer::new();
    assert_eq!(d.free_wire_aspect().line_type(), 0);
}

#[test]
fn drawer_new_wire_and_free_wire_colors_differ() {
    let d = Prs3dDrawer::new();
    assert_ne!(d.wire_aspect().color(), d.free_wire_aspect().color());
}

#[test]
fn drawer_new_shading_aspect_default_white() {
    let d = Prs3dDrawer::new();
    assert_eq!(d.shading_aspect().color(), [1.0f32, 1.0, 1.0, 1.0]);
}

#[test]
fn drawer_set_hlr_type_poly_algo() {
    let mut d = Prs3dDrawer::new();
    d.set_hlr_type(Prs3dTypeOfHLR::PolyAlgo);
    assert_eq!(d.hlr_type(), Prs3dTypeOfHLR::PolyAlgo);
}

#[test]
fn drawer_set_hlr_type_algo() {
    let mut d = Prs3dDrawer::new();
    d.set_hlr_type(Prs3dTypeOfHLR::Algo);
    assert_eq!(d.hlr_type(), Prs3dTypeOfHLR::Algo);
}

#[test]
fn drawer_set_nb_points_mutates() {
    let mut d = Prs3dDrawer::new();
    d.set_nb_points(64);
    assert_eq!(d.nb_points(), 64);
}

#[test]
fn drawer_set_max_flyflyangle_mutates() {
    let mut d = Prs3dDrawer::new();
    d.set_max_flyflyangle(45.0);
    assert!((d.max_flyflyangle() - 45.0f64).abs() < 1e-10);
}

#[test]
fn drawer_wire_aspect_mut_changes_color() {
    let mut d = Prs3dDrawer::new();
    d.wire_aspect_mut().set_color([0.5f32, 0.5, 0.5, 1.0]);
    assert_eq!(d.wire_aspect().color(), [0.5f32, 0.5, 0.5, 1.0]);
}

#[test]
fn drawer_wire_aspect_mut_changes_width() {
    let mut d = Prs3dDrawer::new();
    d.wire_aspect_mut().set_width(3.0);
    assert_eq!(d.wire_aspect().width(), 3.0f32);
}

#[test]
fn drawer_wire_aspect_mut_changes_line_type() {
    let mut d = Prs3dDrawer::new();
    d.wire_aspect_mut().set_line_type(2);
    assert_eq!(d.wire_aspect().line_type(), 2);
}

#[test]
fn drawer_free_wire_aspect_mut_changes_color() {
    let mut d = Prs3dDrawer::new();
    d.free_wire_aspect_mut().set_color([0.2f32, 0.8, 0.4, 1.0]);
    assert_eq!(d.free_wire_aspect().color(), [0.2f32, 0.8, 0.4, 1.0]);
}

#[test]
fn drawer_shading_aspect_mut_changes_transparency() {
    let mut d = Prs3dDrawer::new();
    d.shading_aspect_mut().set_transparency(0.3);
    assert!((d.shading_aspect().transparency() - 0.3f32).abs() < 1e-6);
}

#[test]
fn drawer_shading_aspect_mut_changes_color() {
    let mut d = Prs3dDrawer::new();
    let blue = [0.0f32, 0.0, 1.0, 1.0];
    d.shading_aspect_mut().set_color(blue);
    assert_eq!(d.shading_aspect().color(), blue);
}

#[test]
fn drawer_default_trait_matches_new() {
    let a = Prs3dDrawer::new();
    let b = Prs3dDrawer::default();
    assert_eq!(b.hlr_type(), a.hlr_type());
    assert_eq!(b.nb_points(), a.nb_points());
    assert!((b.max_flyflyangle() - a.max_flyflyangle()).abs() < 1e-10);
}

#[test]
fn drawer_clone() {
    let d = Prs3dDrawer::new();
    let d2 = d.clone();
    assert_eq!(d2.hlr_type(), d.hlr_type());
    assert_eq!(d2.nb_points(), d.nb_points());
    assert_eq!(d2.wire_aspect(), d.wire_aspect());
    assert_eq!(d2.shading_aspect(), d.shading_aspect());
}

#[test]
fn drawer_independent_wire_and_free_wire_mutation() {
    let mut d = Prs3dDrawer::new();
    d.wire_aspect_mut().set_line_type(1);
    d.free_wire_aspect_mut().set_line_type(2);
    assert_eq!(d.wire_aspect().line_type(), 1);
    assert_eq!(d.free_wire_aspect().line_type(), 2);
}

#[test]
fn drawer_hlr_round_trip_all_variants() {
    let mut d = Prs3dDrawer::new();
    for variant in [
        Prs3dTypeOfHLR::NotSet,
        Prs3dTypeOfHLR::PolyAlgo,
        Prs3dTypeOfHLR::Algo,
    ] {
        d.set_hlr_type(variant);
        assert_eq!(d.hlr_type(), variant);
    }
}
