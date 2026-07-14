use ironstream::quant_dist::*;

#[test]
fn name_of_color_red() {
    let [r, g, b] = NameOfColor::Red.to_rgb();
    assert!((r - 1.0).abs() < 1e-10 && g.abs() < 1e-10 && b.abs() < 1e-10);
}

#[test]
fn quantity_color_ops() {
    let red = QuantityColor::from_name(NameOfColor::Red);
    assert!((red.red() - 1.0).abs() < 1e-10);
    let [r8, _, _] = red.to_rgb8();
    assert_eq!(r8, 255);
}

#[test]
fn color_distance() {
    let c1 = QuantityColor::new(1.0, 0.0, 0.0);
    let c2 = QuantityColor::new(0.0, 0.0, 0.0);
    assert!((c1.distance(&c2) - 1.0).abs() < 1e-10);
    assert!(c1.is_equal(&c1, 0.0));
}

#[test]
fn color_rgba_alpha() {
    let mut c = QuantityColorRGBA::new(1.0, 0.0, 0.0, 0.5);
    assert!(!c.is_opaque());
    c.set_alpha(1.0);
    assert!(c.is_opaque());
}

#[test]
fn angle_conversion() {
    use std::f64::consts::PI;
    assert!((QuantityAngle::to_radians(180.0) - PI).abs() < 1e-10);
    assert!((QuantityAngle::to_degrees(PI) - 180.0).abs() < 1e-10);
}
