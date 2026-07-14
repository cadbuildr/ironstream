extern crate ironstream;

use ironstream::ais_extra::{
    AisAxis, AisDimension, AisDimensionType, AisPlane, AisTrihedron, AisTypeOfAxis,
};

#[test]
fn trihedron_new_defaults() {
    let t = AisTrihedron::new([0.0, 0.0, 0.0], 1.0);
    assert_eq!(t.position, [0.0, 0.0, 0.0]);
    assert_eq!(t.size(), 1.0);
    assert_eq!(t.x_color, [1.0, 0.0, 0.0]);
    assert_eq!(t.y_color, [0.0, 1.0, 0.0]);
    assert_eq!(t.z_color, [0.0, 0.0, 1.0]);
    assert!(!t.is_displayed());
}

#[test]
fn trihedron_set_size() {
    let mut t = AisTrihedron::new([1.0, 2.0, 3.0], 5.0);
    t.set_size(10.0);
    assert_eq!(t.size(), 10.0);
}

#[test]
fn trihedron_set_position() {
    let mut t = AisTrihedron::new([0.0, 0.0, 0.0], 1.0);
    t.set_position([4.0, 5.0, 6.0]);
    assert_eq!(t.position, [4.0, 5.0, 6.0]);
}

#[test]
fn trihedron_set_colors() {
    let mut t = AisTrihedron::new([0.0, 0.0, 0.0], 1.0);
    t.set_x_color(0.5, 0.0, 0.0);
    t.set_y_color(0.0, 0.5, 0.0);
    t.set_z_color(0.0, 0.0, 0.5);
    assert_eq!(t.x_color, [0.5, 0.0, 0.0]);
    assert_eq!(t.y_color, [0.0, 0.5, 0.0]);
    assert_eq!(t.z_color, [0.0, 0.0, 0.5]);
}

#[test]
fn trihedron_display_hide() {
    let mut t = AisTrihedron::new([0.0, 0.0, 0.0], 2.0);
    assert!(!t.is_displayed());
    t.display();
    assert!(t.is_displayed());
    t.hide();
    assert!(!t.is_displayed());
}

#[test]
fn plane_new_defaults() {
    let p = AisPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 50.0);
    assert_eq!(p.origin(), [0.0, 0.0, 0.0]);
    assert_eq!(p.normal(), [0.0, 0.0, 1.0]);
    assert_eq!(p.size, 50.0);
    assert!(!p.is_displayed());
}

#[test]
fn plane_set_color_and_display() {
    let mut p = AisPlane::new([1.0, 2.0, 3.0], [0.0, 1.0, 0.0], 10.0);
    p.set_color(0.3, 0.5, 0.7);
    assert_eq!(p.color, [0.3, 0.5, 0.7]);
    p.display();
    assert!(p.is_displayed());
    p.hide();
    assert!(!p.is_displayed());
}

#[test]
fn plane_set_size() {
    let mut p = AisPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 5.0);
    p.set_size(20.0);
    assert_eq!(p.size, 20.0);
}

#[test]
fn axis_new_and_type() {
    let a = AisAxis::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], AisTypeOfAxis::XAxis);
    assert_eq!(a.axis_type(), AisTypeOfAxis::XAxis);
    assert_eq!(a.origin, [0.0, 0.0, 0.0]);
    assert_eq!(a.direction, [1.0, 0.0, 0.0]);
    assert!(!a.is_displayed());
}

#[test]
fn axis_set_type_and_color() {
    let mut a = AisAxis::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], AisTypeOfAxis::UnknownAxis);
    a.set_axis_type(AisTypeOfAxis::YAxis);
    assert_eq!(a.axis_type(), AisTypeOfAxis::YAxis);
    a.set_color(0.0, 1.0, 0.0);
    assert_eq!(a.color, [0.0, 1.0, 0.0]);
}

#[test]
fn axis_display_hide() {
    let mut a = AisAxis::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], AisTypeOfAxis::ZAxis);
    a.display();
    assert!(a.is_displayed());
    a.hide();
    assert!(!a.is_displayed());
}

#[test]
fn dimension_new_length() {
    let d = AisDimension::new(AisDimensionType::Length, 42.5);
    assert_eq!(d.dim_type, AisDimensionType::Length);
    assert!((d.value() - 42.5).abs() < 1e-12);
    assert!(d.text.is_empty());
    assert!(!d.is_displayed);
}

#[test]
fn dimension_set_text_and_display() {
    let mut d = AisDimension::new(AisDimensionType::Angle, 90.0);
    d.set_text("90 deg");
    assert_eq!(d.text, "90 deg");
    d.display();
    assert!(d.is_displayed);
    d.hide();
    assert!(!d.is_displayed);
}

#[test]
fn dimension_types_roundtrip() {
    let types = [
        AisDimensionType::Length,
        AisDimensionType::Angle,
        AisDimensionType::Radius,
        AisDimensionType::Diameter,
    ];
    for &dt in &types {
        let d = AisDimension::new(dt, 1.0);
        assert_eq!(d.dim_type, dt);
    }
}
