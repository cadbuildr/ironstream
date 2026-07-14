use ironstream::ais_shape::*;

#[test]
fn ais_shape_defaults() {
    let s = AisShape::new("box");
    assert_eq!(s.shape(), "box");
    assert_eq!(s.color, [1.0, 1.0, 1.0]);
    assert_eq!(s.transparency, 0.0);
    assert_eq!(s.display_mode, DisplayMode::Shaded);
    assert!(s.is_visible());
}

#[test]
fn ais_shape_set_color() {
    let mut s = AisShape::new("cone");
    s.set_color(0.2, 0.4, 0.6);
    assert_eq!(s.color, [0.2, 0.4, 0.6]);
}

#[test]
fn ais_shape_set_transparency() {
    let mut s = AisShape::new("sphere");
    s.set_transparency(0.75);
    assert!((s.transparency - 0.75).abs() < 1e-12);
}

#[test]
fn ais_shape_display_modes() {
    let mut s = AisShape::new("torus");
    s.set_display_mode(DisplayMode::Wireframe);
    assert_eq!(s.display_mode, DisplayMode::Wireframe);
    s.set_display_mode(DisplayMode::ShadedWithEdges);
    assert_eq!(s.display_mode, DisplayMode::ShadedWithEdges);
}

#[test]
fn ais_shape_visibility_toggle() {
    let mut s = AisShape::new("cylinder");
    assert!(s.is_visible());
    s.visible = false;
    assert!(!s.is_visible());
}

#[test]
fn colored_shape_face_colors_override() {
    let mut cs = ColoredShape::new("body");
    // Use clearly distinct names to avoid hash collisions.
    cs.set_custom_color("alpha", [1.0, 0.0, 0.0]);
    cs.set_custom_color("zeta_face_zz", [0.0, 1.0, 0.0]);
    assert_eq!(cs.face_colors.len(), 2);
    // Overwrite same face should not grow the vec.
    cs.set_custom_color("alpha", [0.0, 0.0, 1.0]);
    assert_eq!(cs.face_colors.len(), 2);
}
