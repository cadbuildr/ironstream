// FILE: tests/occt_graphic3d.rs
extern crate ironstream;
use ironstream::graphic3d::*;

#[test]
fn reflection_copy_and_eq() {
    let r = Graphic3dTypeOfReflection::Specular;
    let r2 = r;
    assert_eq!(r, r2);
    assert_ne!(r, Graphic3dTypeOfReflection::Ambient);
}

#[test]
fn reflection_all_variants_distinct() {
    assert_ne!(Graphic3dTypeOfReflection::Ambient, Graphic3dTypeOfReflection::Diffuse);
    assert_ne!(Graphic3dTypeOfReflection::Diffuse, Graphic3dTypeOfReflection::Specular);
    assert_ne!(Graphic3dTypeOfReflection::Specular, Graphic3dTypeOfReflection::Emissive);
}

#[test]
fn shading_model_copy_and_eq() {
    let s = Graphic3dTypeOfShadingModel::Phong;
    let s2 = s;
    assert_eq!(s, s2);
    assert_ne!(s, Graphic3dTypeOfShadingModel::Gouraud);
}

#[test]
fn shading_model_all_variants_distinct() {
    let all = [
        Graphic3dTypeOfShadingModel::Unlit,
        Graphic3dTypeOfShadingModel::Gouraud,
        Graphic3dTypeOfShadingModel::Phong,
        Graphic3dTypeOfShadingModel::Pbr,
        Graphic3dTypeOfShadingModel::PbrFacet,
    ];
    for i in 0..all.len() {
        for j in 0..all.len() {
            if i == j {
                assert_eq!(all[i], all[j]);
            } else {
                assert_ne!(all[i], all[j]);
            }
        }
    }
}

#[test]
fn material_new_name_and_defaults() {
    let m = Graphic3dMaterialAspect::new("Brass");
    assert_eq!(m.name(), "Brass");
    assert_eq!(m.transparency(), 0.0);
    assert!(!m.is_transparent());
    assert_eq!(m.shininess(), 0.3);
}

#[test]
fn material_set_transparency_makes_transparent() {
    let mut m = Graphic3dMaterialAspect::new("Glass");
    assert!(!m.is_transparent());
    m.set_transparency(0.8);
    assert_eq!(m.transparency(), 0.8);
    assert!(m.is_transparent());
}

#[test]
fn material_transparency_at_threshold_boundary() {
    let mut m = Graphic3dMaterialAspect::new("Tinted");
    m.set_transparency(0.01);
    assert!(!m.is_transparent());
    m.set_transparency(0.011);
    assert!(m.is_transparent());
}

#[test]
fn material_set_and_get_shininess() {
    let mut m = Graphic3dMaterialAspect::new("Chrome");
    m.set_shininess(1.0);
    assert_eq!(m.shininess(), 1.0);
    m.set_shininess(0.0);
    assert_eq!(m.shininess(), 0.0);
}

#[test]
fn material_public_color_fields_accessible() {
    let mut m = Graphic3dMaterialAspect::new("Custom");
    m.diffuse_color = [1.0, 0.0, 0.0];
    m.specular_color = [0.0, 1.0, 0.0];
    m.emissive_color = [0.0, 0.0, 1.0];
    assert_eq!(m.diffuse_color, [1.0, 0.0, 0.0]);
    assert_eq!(m.specular_color, [0.0, 1.0, 0.0]);
    assert_eq!(m.emissive_color, [0.0, 0.0, 1.0]);
}

#[test]
fn vertex_new_stores_coordinates() {
    let v = Graphic3dVertex::new(-1.5, 0.0, 42.0);
    assert_eq!(v.x, -1.5);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 42.0);
}

#[test]
fn vertex_distance_known_value() {
    let origin = Graphic3dVertex::new(0.0, 0.0, 0.0);
    let p = Graphic3dVertex::new(0.0, 0.0, 5.0);
    assert!((origin.distance(&p) - 5.0).abs() < 1e-6);
}

#[test]
fn group_starts_empty_no_material() {
    let g = Graphic3dGroup::new();
    assert!(g.is_empty());
    assert_eq!(g.nb_vertices(), 0);
    assert!(g.material().is_none());
}

#[test]
fn group_add_vertices_and_material() {
    let mut g = Graphic3dGroup::new();
    g.add_vertex(Graphic3dVertex::new(0.0, 0.0, 0.0));
    g.add_vertex(Graphic3dVertex::new(1.0, 0.0, 0.0));
    g.add_vertex(Graphic3dVertex::new(0.0, 1.0, 0.0));
    assert!(!g.is_empty());
    assert_eq!(g.nb_vertices(), 3);
    let mut mat = Graphic3dMaterialAspect::new("Silver");
    mat.set_shininess(0.7);
    mat.set_transparency(0.2);
    g.set_material(mat);
    let m = g.material().unwrap();
    assert_eq!(m.name(), "Silver");
    assert_eq!(m.shininess(), 0.7);
    assert!(m.is_transparent());
}

#[test]
fn group_clear_after_population() {
    let mut g = Graphic3dGroup::new();
    g.add_vertex(Graphic3dVertex::new(1.0, 2.0, 3.0));
    g.set_material(Graphic3dMaterialAspect::new("Tin"));
    assert!(!g.is_empty());
    assert!(g.material().is_some());
    g.clear();
    assert!(g.is_empty());
    assert_eq!(g.nb_vertices(), 0);
    assert!(g.material().is_none());
}

#[test]
fn group_replace_material() {
    let mut g = Graphic3dGroup::new();
    g.set_material(Graphic3dMaterialAspect::new("First"));
    assert_eq!(g.material().unwrap().name(), "First");
    g.set_material(Graphic3dMaterialAspect::new("Second"));
    assert_eq!(g.material().unwrap().name(), "Second");
}
