use ironstream::render_raytracing::*;

#[test]
fn raytracing_defaults_whitted_depth_5_not_path_tracing() {
    let p = RaytracingParams::new();
    assert_eq!(p.algorithm(), RaytracingAlgorithm::WhittedStyle);
    assert_eq!(p.max_ray_depth(), 5);
    assert!(!p.is_path_tracing());
}

#[test]
fn raytracing_enable_gi_becomes_path_tracing() {
    let mut p = RaytracingParams::new();
    p.set_global_illumination(true);
    assert!(p.is_path_tracing());
    assert!(p.is_global_illumination);
}

#[test]
fn raytracing_depth_clamped_max_50_min_1() {
    let mut p = RaytracingParams::new();
    p.set_max_depth(100);
    assert_eq!(p.max_ray_depth(), 50);
    p.set_max_depth(0);
    assert_eq!(p.max_ray_depth(), 1);
}

#[test]
fn tone_mapping_enabled_filmic() {
    let tm = ToneMappingParams::new(ToneMappingMethod::Filmic);
    assert!(tm.is_enabled());
    assert_eq!(tm.method(), ToneMappingMethod::Filmic);
}

#[test]
fn tone_mapping_disabled_default() {
    let tm = ToneMappingParams::default();
    assert!(!tm.is_enabled());
    assert_eq!(tm.method(), ToneMappingMethod::Disabled);
}
