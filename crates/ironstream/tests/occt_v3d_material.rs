// FILE: tests/occt_v3d_material.rs
extern crate ironstream;
use ironstream::v3d_material::*;

#[test]
fn material_defaults() {
    let m = Graphic3dMaterialAspect::new();
    assert!(!m.is_transparent());
    assert!((m.shininess() - 0.5).abs() < 1e-10);
}

#[test]
fn material_preset_gold() {
    let mut m = Graphic3dMaterialAspect::new();
    m.set_name(MaterialName::Gold);
    assert!(m.ambient_color[0] > 0.0);
    assert!(m.diffuse_color[0] > 0.5);
}

#[test]
fn material_glass_transparent() {
    let mut m = Graphic3dMaterialAspect::new();
    m.set_name(MaterialName::Glass);
    assert!(m.is_transparent());
    assert!((m.refractive_index - 1.5).abs() < 1e-10);
}

#[test]
fn pbr_metallic_roughness() {
    let mut p = Graphic3dPbrMaterial::new();
    p.set_metallic(0.8);
    p.set_roughness(0.2);
    assert!(p.is_metallic());
    assert!(!p.is_transparent());
}

#[test]
fn pbr_f0_dielectric() {
    let mut p = Graphic3dPbrMaterial::new();
    p.set_metallic(0.0);
    p.set_ior(1.5);
    let f = p.f0();
    // Fresnel for glass ≈ 0.04 — in range (0, 0.1)
    assert!(f[0] > 0.0 && f[0] < 0.1);
}

#[test]
fn pbr_transparent() {
    let mut p = Graphic3dPbrMaterial::new();
    p.set_base_color([0.8, 0.8, 0.8, 0.3]);
    assert!(p.is_transparent());
}
