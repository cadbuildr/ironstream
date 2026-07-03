use ironstream::graphic3d_light::*;

#[test]
fn light_color_white_black_scaled() {
    let w = LightColor::white();
    assert!((w.r - 1.0).abs() < 1e-6);
    let b = LightColor::black();
    assert!(b.r.abs() < 1e-6 && b.g.abs() < 1e-6);
    let s = w.scaled(0.5);
    assert!((s.r - 0.5).abs() < 1e-6);
}

#[test]
fn light_attenuation_factor() {
    let a = LightAttenuation::new(1.0, 0.5, 0.0);
    // factor at d=2: 1/(1+0.5*2) = 1/2
    let f = a.factor(2.0);
    assert!((f - 0.5).abs() < 1e-5);
    let a2 = LightAttenuation::default();
    // constant=1, linear=0, quadratic=0 → factor=1
    assert!((a2.factor(100.0) - 1.0).abs() < 1e-6);
}

#[test]
fn directional_light_type_checks() {
    let l = Graphic3dCLight::directional([0.0, -1.0, 0.0]);
    assert!(l.is_directional());
    assert!(!l.is_spot());
    assert!(!l.is_ambient());
    assert!(l.is_enabled);
}

#[test]
fn spot_light_angle_clamped() {
    // angle 10.0 radians > π/2, should be clamped
    let l = Graphic3dCLight::spot([0.0; 3], [0.0, -1.0, 0.0], 10.0);
    assert!(l.is_spot());
    assert!(l.spot_angle <= std::f32::consts::FRAC_PI_2 + 1e-6);
}

#[test]
fn light_set_has_ambient_and_remove() {
    let mut s = Graphic3dLightSet::new();
    let mut amb = Graphic3dCLight::ambient();
    amb.set_name("ambient_main");
    s.add(amb);
    assert!(s.has_ambient());
    assert_eq!(s.nb_enabled(), 1);
    let col = s.ambient_color();
    assert!((col.r - 1.0).abs() < 1e-5);
    s.remove("ambient_main");
    assert_eq!(s.nb_lights(), 0);
    assert!(!s.has_ambient());
}

#[test]
fn effective_color_scales_by_intensity() {
    let mut l = Graphic3dCLight::ambient();
    l.set_intensity(0.25);
    let c = l.effective_color();
    assert!((c.r - 0.25).abs() < 1e-6);
    // positional light with position
    let pl = Graphic3dCLight::positional([1.0, 2.0, 3.0]);
    assert!(pl.is_positional());
    assert!((pl.position[2] - 3.0).abs() < 1e-6);
}
