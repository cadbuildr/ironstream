// FILE: tests/occt_ais_colored_shape.rs
extern crate ironstream;
use ironstream::ais_colored_shape::*;

#[test]
fn color_rgba_constants() {
    assert_eq!(ColorRgba::WHITE.r, 1.0);
    assert_eq!(ColorRgba::BLACK.r, 0.0);
    assert_eq!(ColorRgba::RED.r, 1.0);
    assert_eq!(ColorRgba::GREEN.g, 1.0);
    assert_eq!(ColorRgba::BLUE.b, 1.0);
}

#[test]
fn color_rgba_from_rgb_opaque() {
    let c = ColorRgba::from_rgb(0.5, 0.25, 0.1);
    assert!((c.a - 1.0).abs() < 1e-6);
    assert!(!c.is_transparent());
}

#[test]
fn color_rgba_lerp_midpoint() {
    let c = ColorRgba::lerp(ColorRgba::BLACK, ColorRgba::WHITE, 0.5);
    assert!((c.r - 0.5).abs() < 1e-6);
    assert!((c.g - 0.5).abs() < 1e-6);
    assert!((c.b - 0.5).abs() < 1e-6);
}

#[test]
fn color_rgba_brightness() {
    let b_white = ColorRgba::WHITE.brightness();
    let b_black = ColorRgba::BLACK.brightness();
    assert!(b_white > b_black);
    assert!((b_black).abs() < 1e-6);
}

#[test]
fn colored_shape_set_and_get_color() {
    let mut s = ColoredShape::new(1);
    let face = ShapeRef(42);
    s.set_custom_color(face.clone(), ColorRgba::RED);
    assert_eq!(s.color_of(&face), ColorRgba::RED);
    assert_eq!(s.nb_custom_shapes(), 1);
}

#[test]
fn colored_shape_default_color_fallback() {
    let s = ColoredShape::new(7);
    let unknown_face = ShapeRef(999);
    // Should return the default color (white) if no custom color is set
    let c = s.color_of(&unknown_face);
    assert_eq!(c, ColorRgba::WHITE);
}

#[test]
fn color_scale_heatmap_length() {
    let colors = ColorScale::heatmap_colors(10);
    assert_eq!(colors.len(), 10);
}

#[test]
fn color_scale_for_value_clamped() {
    let cs = ColorScale::new(0.0, 100.0, 5);
    // Values outside range should clamp to endpoints
    let c_below = cs.color_for_value(-10.0);
    let c_min = cs.color_for_value(0.0);
    let c_above = cs.color_for_value(200.0);
    let c_max = cs.color_for_value(100.0);
    assert_eq!(c_below.r, c_min.r);
    assert_eq!(c_above.r, c_max.r);
}

#[test]
fn colored_drawer_is_overriding() {
    let d_plain = ColoredDrawer::new();
    assert!(!d_plain.is_overriding());
    let d_colored = ColoredDrawer::new().with_color(ColorRgba::BLUE);
    assert!(d_colored.is_overriding());
}
