use ironstream::v3d_light::*;

#[test]
fn directional_light_defaults() {
    let l = Light::directional([0.0, 0.0, -1.0]);
    assert_eq!(l.light_type, LightType::Directional);
    assert_eq!(l.direction, [0.0, 0.0, -1.0]);
    assert_eq!(l.color, [1.0, 1.0, 1.0]);
    assert!((l.intensity - 1.0).abs() < f64::EPSILON);
    assert!(l.is_enabled());
}

#[test]
fn positional_light() {
    let l = Light::positional([1.0, 2.0, 3.0]);
    assert_eq!(l.light_type, LightType::Positional);
    assert_eq!(l.position, [1.0, 2.0, 3.0]);
    assert!(l.is_enabled());
}

#[test]
fn spot_light_angle() {
    use std::f64::consts::FRAC_PI_4;
    let l = Light::spot([0.0, 5.0, 0.0], [0.0, -1.0, 0.0], FRAC_PI_4);
    assert_eq!(l.light_type, LightType::Spot);
    assert!((l.spot_angle - FRAC_PI_4).abs() < f64::EPSILON);
}

#[test]
fn ambient_light() {
    let l = Light::ambient();
    assert_eq!(l.light_type, LightType::Ambient);
    assert!(l.is_enabled());
}

#[test]
fn set_intensity_and_color() {
    let mut l = Light::directional([1.0, 0.0, 0.0]);
    l.set_intensity(0.5);
    l.set_color(0.8, 0.6, 0.4);
    assert!((l.intensity - 0.5).abs() < f64::EPSILON);
    assert_eq!(l.color, [0.8, 0.6, 0.4]);
}

#[test]
fn default_lights_rig() {
    let lights = default_lights();
    assert_eq!(lights.len(), 2);
    assert_eq!(lights[0].light_type, LightType::Directional);
    assert_eq!(lights[1].light_type, LightType::Ambient);
    assert!((lights[0].intensity - 1.0).abs() < f64::EPSILON);
    assert!((lights[1].intensity - 0.3).abs() < f64::EPSILON);
    // toggle enable/disable
    let mut l = lights[0].clone();
    l.set_enabled(false);
    assert!(!l.is_enabled());
}
