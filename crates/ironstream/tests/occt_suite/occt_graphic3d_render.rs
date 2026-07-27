// FILE: tests/occt_graphic3d_render.rs
extern crate ironstream;
use ironstream::graphic3d_render::*;

#[test]
fn test_rendering_params_rasterization_defaults() {
    let p = RenderingParams::default_rasterization();
    assert!(!p.is_ray_tracing());
    assert!(p.is_antialias_enabled);
    assert!((p.aspect_ratio() - 800.0_f32 / 600.0_f32).abs() < 1e-5);
    assert_eq!(p.nb_pixels(), 800 * 600);
}

#[test]
fn test_rendering_params_ray_tracing() {
    let p = RenderingParams::default_ray_tracing();
    assert!(p.is_ray_tracing());
    assert!(p.is_shadows_enabled);
    assert!(p.is_reflections_enabled);
    assert_eq!(p.ray_tracing_depth, 5);
}

#[test]
fn test_anti_aliasing_sample_counts() {
    assert_eq!(AntiAliasingMethod::None.sample_count(), 1);
    assert_eq!(AntiAliasingMethod::Msaa4.sample_count(), 4);
    assert_eq!(AntiAliasingMethod::Msaa8.sample_count(), 8);
    assert!(AntiAliasingMethod::Msaa2.is_hardware());
    assert!(!AntiAliasingMethod::Fxaa.is_hardware());
    assert!(!AntiAliasingMethod::Ssaa.is_hardware());
}

#[test]
fn test_bsdf_types() {
    let diff = Bsdf::diffuse([1.0, 0.0, 0.0]);
    assert!(!diff.is_emissive());
    assert!(!diff.is_transparent());

    let mirror = Bsdf::mirror();
    assert!(!mirror.is_transparent());

    let glass = Bsdf::glass(1.5);
    assert!(glass.is_transparent());
    assert!(!glass.is_emissive());
}

#[test]
fn test_pbr_material_presets() {
    let metal = PbrMaterial::metallic([0.9, 0.9, 0.9]);
    assert!((metal.metallic - 1.0).abs() < 1e-6);
    assert!(metal.is_opaque());
    assert!(!metal.is_emissive());
    assert!(!metal.has_clear_coat());

    let glass = PbrMaterial::glass();
    assert!(!glass.is_opaque());

    let plastic = PbrMaterial::plastic([0.2, 0.5, 0.8])
        .with_emission([0.1, 0.0, 0.0]);
    assert!(plastic.is_emissive());
}
