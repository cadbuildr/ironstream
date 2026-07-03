use ironstream::v3d_render_params::*;

#[test]
fn default_params_not_ray_tracing_shadow_off_msaa_1() {
    let p = Graphic3dRenderingParams::new();
    assert!(!p.is_ray_tracing());
    assert_eq!(p.shadow_map_size(), 0);
    assert_eq!(p.msaa_samples(), 1);
    assert_eq!(p.anti_aliasing, AntiAliasingMode::None);
}

#[test]
fn enable_ray_tracing_and_depth_then_rasterization() {
    let mut p = Graphic3dRenderingParams::new();
    p.enable_ray_tracing(6);
    assert!(p.is_ray_tracing());
    assert_eq!(p.ray_tracing_depth, 6);
    p.enable_rasterization();
    assert!(!p.is_ray_tracing());
}

#[test]
fn set_anti_aliasing_msaa_samples() {
    let mut p = Graphic3dRenderingParams::new();
    p.set_anti_aliasing(AntiAliasingMode::Msaa4x);
    assert_eq!(p.msaa_samples(), 4);
    p.set_anti_aliasing(AntiAliasingMode::Msaa8x);
    assert_eq!(p.msaa_samples(), 8);
    p.set_anti_aliasing(AntiAliasingMode::Fxaa);
    assert_eq!(p.msaa_samples(), 1);
}

#[test]
fn shadow_quality_shadow_map_size() {
    let mut p = Graphic3dRenderingParams::new();
    p.set_shadow_quality(ShadowQuality::Low);
    assert_eq!(p.shadow_map_size(), 512);
    p.set_shadow_quality(ShadowQuality::Medium);
    assert_eq!(p.shadow_map_size(), 1024);
    p.set_shadow_quality(ShadowQuality::High);
    assert_eq!(p.shadow_map_size(), 2048);
}

#[test]
fn ssao_enable_and_set_radius() {
    let mut p = Graphic3dRenderingParams::new();
    assert!(!p.ssao.is_enabled);
    p.ssao.enable();
    assert!(p.ssao.is_enabled);
    p.ssao.set_radius(1.2);
    assert!((p.ssao.radius - 1.2).abs() < 1e-6);
    p.ssao.disable();
    assert!(!p.ssao.is_enabled);
}

#[test]
fn ray_tracing_depth_clamped_to_max_10() {
    let mut p = Graphic3dRenderingParams::new();
    p.enable_ray_tracing(50); // should clamp to 10
    assert_eq!(p.ray_tracing_depth, 10);
}
