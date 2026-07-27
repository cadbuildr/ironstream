// FILE: tests/occt_v3d_viewer.rs
extern crate ironstream;
use ironstream::v3d_viewer::*;

#[test]
fn test_background_solid_color_roundtrip() {
    let bg = V3dBackground::new([0.3, 0.5, 0.7]);
    assert_eq!(bg.color(), [0.3, 0.5, 0.7]);
    assert!(!bg.use_gradient());
}

#[test]
fn test_background_gradient_top_and_bottom() {
    let bg = V3dBackground::with_gradient([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    assert!(bg.use_gradient());
    assert_eq!(bg.gradient_top(), [1.0, 0.0, 0.0]);
    assert_eq!(bg.gradient_bottom(), [0.0, 0.0, 1.0]);
}

#[test]
fn test_background_gradient_color_field_is_zero() {
    let bg = V3dBackground::with_gradient([0.5, 0.5, 0.5], [0.1, 0.1, 0.1]);
    assert_eq!(bg.color(), [0.0, 0.0, 0.0]);
}

#[test]
fn test_background_solid_does_not_set_use_gradient() {
    let bg = V3dBackground::new([1.0, 1.0, 1.0]);
    assert!(!bg.use_gradient());
}

#[test]
fn test_light_type_all_variants_distinct() {
    let variants = [
        V3dLightType::Ambient,
        V3dLightType::Directional,
        V3dLightType::Positional,
        V3dLightType::Spot,
    ];
    for i in 0..variants.len() {
        for j in (i + 1)..variants.len() {
            assert_ne!(variants[i], variants[j], "variants {} and {} must differ", i, j);
        }
    }
}

#[test]
fn test_light_type_is_copy() {
    let t = V3dLightType::Spot;
    let t2 = t;
    assert_eq!(t, t2);
}

#[test]
fn test_light_new_color() {
    let l = V3dLight::new(V3dLightType::Ambient, [0.8, 0.9, 1.0]);
    assert_eq!(l.color(), [0.8, 0.9, 1.0]);
}

#[test]
fn test_light_new_default_intensity_one() {
    let l = V3dLight::new(V3dLightType::Directional, [1.0, 1.0, 1.0]);
    assert_eq!(l.intensity(), 1.0);
}

#[test]
fn test_light_new_enabled_by_default() {
    let l = V3dLight::new(V3dLightType::Positional, [1.0, 0.5, 0.0]);
    assert!(l.is_enabled());
}

#[test]
fn test_light_default_direction() {
    let l = V3dLight::new(V3dLightType::Directional, [1.0, 1.0, 1.0]);
    assert_eq!(l.direction(), [0.0, 0.0, -1.0]);
}

#[test]
fn test_light_set_direction_roundtrip() {
    let mut l = V3dLight::new(V3dLightType::Directional, [1.0, 1.0, 1.0]);
    l.set_direction([0.577, 0.577, 0.577]);
    let d = l.direction();
    assert!((d[0] - 0.577f32).abs() < 1e-6);
    assert!((d[1] - 0.577f32).abs() < 1e-6);
    assert!((d[2] - 0.577f32).abs() < 1e-6);
}

#[test]
fn test_light_set_intensity_roundtrip() {
    let mut l = V3dLight::new(V3dLightType::Spot, [1.0, 1.0, 1.0]);
    l.set_intensity(3.14);
    assert!((l.intensity() - 3.14f32).abs() < 1e-5);
}

#[test]
fn test_light_enable_disable_toggle() {
    let mut l = V3dLight::new(V3dLightType::Ambient, [1.0, 1.0, 1.0]);
    l.disable();
    assert!(!l.is_enabled());
    l.enable();
    assert!(l.is_enabled());
    l.disable();
    assert!(!l.is_enabled());
}

#[test]
fn test_light_type_field_accessible() {
    let l = V3dLight::new(V3dLightType::Spot, [0.5, 0.5, 0.5]);
    assert_eq!(l.light_type, V3dLightType::Spot);
}

#[test]
fn test_viewer_new_has_no_lights() {
    let v = V3dViewer::new();
    assert_eq!(v.nb_lights(), 0);
}

#[test]
fn test_viewer_default_background_not_gradient() {
    let v = V3dViewer::new();
    assert!(!v.background().use_gradient());
}

#[test]
fn test_viewer_add_single_light() {
    let mut v = V3dViewer::new();
    v.add_light(V3dLight::new(V3dLightType::Ambient, [1.0, 1.0, 1.0]));
    assert_eq!(v.nb_lights(), 1);
}

#[test]
fn test_viewer_add_multiple_lights() {
    let mut v = V3dViewer::new();
    for _ in 0..4 {
        v.add_light(V3dLight::new(V3dLightType::Positional, [0.5, 0.5, 0.5]));
    }
    assert_eq!(v.nb_lights(), 4);
}

#[test]
fn test_viewer_light_index_type() {
    let mut v = V3dViewer::new();
    v.add_light(V3dLight::new(V3dLightType::Directional, [1.0, 0.0, 0.0]));
    v.add_light(V3dLight::new(V3dLightType::Spot, [0.0, 1.0, 0.0]));
    assert_eq!(v.light(0).light_type, V3dLightType::Directional);
    assert_eq!(v.light(1).light_type, V3dLightType::Spot);
}

#[test]
fn test_viewer_light_index_color() {
    let mut v = V3dViewer::new();
    v.add_light(V3dLight::new(V3dLightType::Ambient, [0.1, 0.2, 0.3]));
    assert_eq!(v.light(0).color(), [0.1, 0.2, 0.3]);
}

#[test]
fn test_viewer_set_default_lights_count() {
    let mut v = V3dViewer::new();
    v.set_default_lights();
    assert_eq!(v.nb_lights(), 2);
}

#[test]
fn test_viewer_set_default_lights_first_is_ambient() {
    let mut v = V3dViewer::new();
    v.set_default_lights();
    assert_eq!(v.light(0).light_type, V3dLightType::Ambient);
}

#[test]
fn test_viewer_set_default_lights_second_is_directional() {
    let mut v = V3dViewer::new();
    v.set_default_lights();
    assert_eq!(v.light(1).light_type, V3dLightType::Directional);
}

#[test]
fn test_viewer_set_default_lights_are_white() {
    let mut v = V3dViewer::new();
    v.set_default_lights();
    assert_eq!(v.light(0).color(), [1.0, 1.0, 1.0]);
    assert_eq!(v.light(1).color(), [1.0, 1.0, 1.0]);
}

#[test]
fn test_viewer_set_default_lights_replaces_previous() {
    let mut v = V3dViewer::new();
    v.add_light(V3dLight::new(V3dLightType::Spot, [0.0, 0.0, 0.0]));
    v.add_light(V3dLight::new(V3dLightType::Spot, [0.0, 0.0, 0.0]));
    v.add_light(V3dLight::new(V3dLightType::Spot, [0.0, 0.0, 0.0]));
    assert_eq!(v.nb_lights(), 3);
    v.set_default_lights();
    assert_eq!(v.nb_lights(), 2);
}

#[test]
fn test_viewer_set_default_lights_idempotent() {
    let mut v = V3dViewer::new();
    v.set_default_lights();
    v.set_default_lights();
    assert_eq!(v.nb_lights(), 2);
}

#[test]
fn test_viewer_set_background_solid() {
    let mut v = V3dViewer::new();
    v.set_background(V3dBackground::new([0.2, 0.3, 0.4]));
    assert_eq!(v.background().color(), [0.2, 0.3, 0.4]);
    assert!(!v.background().use_gradient());
}

#[test]
fn test_viewer_set_background_gradient() {
    let mut v = V3dViewer::new();
    v.set_background(V3dBackground::with_gradient([1.0, 1.0, 0.0], [0.0, 0.5, 1.0]));
    assert!(v.background().use_gradient());
    assert_eq!(v.background().gradient_top(), [1.0, 1.0, 0.0]);
    assert_eq!(v.background().gradient_bottom(), [0.0, 0.5, 1.0]);
}

#[test]
fn test_viewer_default_impl() {
    let v = V3dViewer::default();
    assert_eq!(v.nb_lights(), 0);
}

#[test]
fn test_directional_light_default_direction_downward() {
    let mut v = V3dViewer::new();
    v.set_default_lights();
    assert_eq!(v.light(1).direction(), [0.0, 0.0, -1.0]);
}

#[test]
fn test_ambient_light_is_enabled_after_set_default_lights() {
    let mut v = V3dViewer::new();
    v.set_default_lights();
    assert!(v.light(0).is_enabled());
    assert!(v.light(1).is_enabled());
}
