use ironstream::ais_text::*;

#[test]
fn ais_text_label_set_get_text() {
    let mut t = AisTextLabel::new(1, "Hello", [0.0, 0.0, 0.0]);
    assert_eq!(t.text(), "Hello");

    t.set_text("World");
    assert_eq!(t.text(), "World");

    // empty string is valid
    t.set_text("");
    assert_eq!(t.text(), "");
}

#[test]
fn ais_text_label_position_and_height() {
    let mut t = AisTextLabel::new(2, "A", [1.0, 2.0, 3.0]);
    assert_eq!(t.position(), [1.0, 2.0, 3.0]);

    t.set_position([4.0, 5.0, 6.0]);
    assert_eq!(t.position(), [4.0, 5.0, 6.0]);

    assert!((t.height() - 1.0).abs() < 1e-12); // default
    t.set_height(2.5);
    assert!((t.height() - 2.5).abs() < 1e-12);
}

#[test]
fn ais_text_label_color_and_has_own_color() {
    let mut t = AisTextLabel::new(3, "C", [0.0, 0.0, 0.0]);
    assert!(!t.has_own_color()); // default: no own color

    t.set_color([1.0, 0.0, 0.0, 1.0]);
    assert!(t.has_own_color());
    assert!((t.color()[0] - 1.0).abs() < 1e-12);
    assert!((t.color()[1] - 0.0).abs() < 1e-12);
    assert!((t.color()[2] - 0.0).abs() < 1e-12);
    assert!((t.color()[3] - 1.0).abs() < 1e-12);
}

#[test]
fn ais_dimension_new_length_value() {
    // 3-4-5 right triangle
    let d = AisDimension::new_length(1, [0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
    assert!(d.is_valid());
    assert_eq!(d.kind(), DimensionKind::Length);
    assert!((d.value() - 5.0).abs() < 1e-10);
    assert_eq!(d.unit(), "mm");
}

#[test]
fn ais_dimension_new_angle_value_and_valid() {
    let d = AisDimension::new_angle(2, 90.0, [0.0, 0.0, 0.0]);
    assert!(d.is_valid());
    assert_eq!(d.kind(), DimensionKind::Angle);
    assert!((d.value() - 90.0).abs() < 1e-10);
    assert_eq!(d.unit(), "deg");

    // zero angle => invalid
    let bad = AisDimension::new_angle(3, 0.0, [0.0, 0.0, 0.0]);
    assert!(!bad.is_valid());

    // negative angle => invalid
    let neg = AisDimension::new_angle(4, -10.0, [0.0, 0.0, 0.0]);
    assert!(!neg.is_valid());
}

#[test]
fn ais_dimension_new_radius() {
    let d = AisDimension::new_radius(5, [0.0, 0.0, 0.0], 5.0);
    assert!(d.is_valid());
    assert_eq!(d.kind(), DimensionKind::Radius);
    assert!((d.value() - 5.0).abs() < 1e-10);

    // zero radius => invalid
    let bad = AisDimension::new_radius(6, [0.0, 0.0, 0.0], 0.0);
    assert!(!bad.is_valid());
}

#[test]
fn ais_dimension_formatted_value_contains_unit() {
    let d = AisDimension::new_length(1, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let s = d.formatted_value();
    assert!(s.contains("mm"), "expected 'mm' in '{}'", s);

    let a = AisDimension::new_angle(2, 45.0, [0.0, 0.0, 0.0]);
    let sa = a.formatted_value();
    assert!(sa.contains("deg"), "expected 'deg' in '{}'", sa);

    // when display_units is false, no unit suffix
    let mut d2 = AisDimension::new_length(3, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
    d2.set_display_units(false);
    let s2 = d2.formatted_value();
    assert!(!s2.contains("mm"), "expected no 'mm' in '{}'", s2);
}
