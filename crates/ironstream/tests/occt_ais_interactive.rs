// FILE: tests/occt_ais_interactive.rs
extern crate ironstream;
use ironstream::ais_interactive::*;

#[test]
fn test_ais_display_mode_default() {
    let mode = AisDisplayMode::default();
    assert_eq!(mode, AisDisplayMode::Shading);
}

#[test]
fn test_ais_priority_ordering_and_value() {
    assert!(AisPriority::HIGH > AisPriority::DEFAULT);
    assert_eq!(AisPriority::DEFAULT.value(), 5);
    assert_eq!(AisPriority::HIGH.value(), 10);
    let p = AisPriority(3);
    assert!(p < AisPriority::DEFAULT);
}

#[test]
fn test_ais_interactive_object_color_and_transparency() {
    let mut obj = AisInteractiveObject::new(1);
    assert_eq!(obj.color(), [1.0, 1.0, 1.0]);
    assert!((obj.transparency() - 0.0).abs() < 1e-9);
    assert!(!obj.is_transparent());

    obj.set_color(0.5, 0.2, 0.9);
    obj.set_transparency(0.5);
    assert_eq!(obj.color(), [0.5, 0.2, 0.9]);
    assert!((obj.transparency() - 0.5).abs() < 1e-9);
    assert!(obj.is_transparent());

    // Transparency is clamped to [0, 1]
    obj.set_transparency(2.0);
    assert!((obj.transparency() - 1.0).abs() < 1e-9);
}

#[test]
fn test_ais_interactive_object_display_state_and_mode() {
    let mut obj = AisInteractiveObject::new(42);
    assert!(!obj.is_displayed());
    obj.mark_displayed();
    assert!(obj.is_displayed());
    obj.mark_hidden();
    assert!(!obj.is_displayed());

    obj.set_display_mode(AisDisplayMode::Wireframe);
    assert_eq!(obj.display_mode(), AisDisplayMode::Wireframe);
    obj.set_infinite_state(true);
    assert!(obj.is_infinite);
}

#[test]
fn test_ais_shape_own_color_and_width() {
    let mut s = AisShape::new(100);
    assert_eq!(s.shape(), 100);
    assert!(!s.has_own_color());
    assert!(!s.has_own_width());

    s.set_own_color(0.0, 1.0, 0.0);
    assert!(s.has_own_color());
    assert_eq!(s.base.color(), [0.0, 1.0, 0.0]);

    s.set_own_width(2.5);
    assert!(s.has_own_width());

    s.set_shape(999);
    assert_eq!(s.shape(), 999);
}

#[test]
fn test_ais_shape_selection_modes() {
    let mut s = AisShape::new(200);
    // Default selection mode is 0
    assert_eq!(s.selection_mode(0), Some(0));
    assert_eq!(s.selection_mode(1), None);

    s.add_selection_mode(4);
    s.add_selection_mode(7);
    assert_eq!(s.selection_mode(1), Some(4));
    assert_eq!(s.selection_mode(2), Some(7));
}

#[test]
fn test_ais_colored_shape_face_and_edge_colors() {
    let mut cs = AisColoredShape::new(300);
    assert_eq!(cs.nb_custom_colors(), 0);
    assert_eq!(cs.face_color(1), None);

    cs.set_custom_color(1, 1.0, 0.0, 0.0, true);
    cs.set_custom_color(2, 0.0, 1.0, 0.0, true);
    cs.set_custom_color(10, 0.0, 0.0, 1.0, false); // edge
    assert_eq!(cs.face_color(1), Some([1.0, 0.0, 0.0]));
    assert_eq!(cs.face_color(2), Some([0.0, 1.0, 0.0]));
    assert_eq!(cs.face_color(10), None); // 10 is an edge, not a face
    assert_eq!(cs.nb_custom_colors(), 3);

    cs.clear_colors();
    assert_eq!(cs.nb_custom_colors(), 0);
}
