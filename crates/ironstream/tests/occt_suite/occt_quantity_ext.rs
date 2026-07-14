use ironstream::quantity_ext::*;

#[test]
fn length_mm_m_inch() {
    let l = QuantityLength::from_m(1.0);
    assert!((l.mm() - 1000.0).abs() < 1e-10);
    assert!((l.inch() - 39.3701).abs() < 0.001);
    let l2 = QuantityLength::from_mm(25.4);
    assert!((l2.inch() - 1.0).abs() < 1e-10);
}

#[test]
fn length_from_ft() {
    let l = QuantityLength::from_ft(1.0);
    assert!((l.mm() - 304.8).abs() < 1e-6);
}

#[test]
fn angle_radians_degrees() {
    let a = QuantityAngle::from_degrees(180.0);
    assert!((a.radians() - std::f64::consts::PI).abs() < 1e-10);
    let a2 = QuantityAngle::from_radians(std::f64::consts::PI / 2.0);
    assert!((a2.degrees() - 90.0).abs() < 1e-10);
}

#[test]
fn angle_normalize() {
    let a = QuantityAngle::from_degrees(-90.0).normalize();
    assert!((a.degrees() - 270.0).abs() < 1e-8);
    let b = QuantityAngle::from_degrees(450.0).normalize();
    assert!((b.degrees() - 90.0).abs() < 1e-8);
}

#[test]
fn speed_conversions() {
    let s = QuantitySpeed::from_km_per_h(36.0);
    assert!((s.m_per_s() - 10.0).abs() < 1e-6);
    let s2 = QuantitySpeed::from_m_per_s(1.0);
    assert!((s2.km_per_h() - 3.6).abs() < 1e-6);
}

#[test]
fn color_rgba_from_hex_blend() {
    let c = QuantityColorRgba::from_hex(0xFF0000FF);
    assert!((c.r - 1.0).abs() < 0.01);
    assert!((c.g).abs() < 0.01);
    assert!(c.is_opaque());

    let fg = QuantityColorRgba::new(1.0, 0.0, 0.0, 0.5);
    let bg = QuantityColorRgba::rgb(0.0, 0.0, 1.0);
    let blended = fg.blend_alpha(bg);
    assert!((blended.r - 0.5).abs() < 1e-10);
    assert!((blended.b - 0.5).abs() < 1e-10);
    assert!(blended.is_opaque());
}
