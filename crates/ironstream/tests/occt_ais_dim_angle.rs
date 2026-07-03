use ironstream::ais_dim_angle::*;

#[test]
fn angle_dim_90_degrees_computed() {
    let d = AisAngleDimension::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    assert!((d.computed_value().to_degrees() - 90.0).abs() < 1e-10);
    assert!((d.value_deg() - 90.0).abs() < 1e-10);
}

#[test]
fn angle_dim_custom_value_overrides_computed() {
    let mut d = AisAngleDimension::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    d.set_custom_value(std::f64::consts::PI / 6.0);
    assert!((d.value_deg() - 30.0).abs() < 1e-10);
    d.unset_custom_value();
    assert!((d.value_deg() - 90.0).abs() < 1e-10);
}

#[test]
fn angle_dim_kind_is_angle() {
    let d = AisAngleDimension::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    assert_eq!(d.kind(), AisDimensionKind::Angle);
}

#[test]
fn radius_dim_value_and_kind() {
    let r = AisRadiusDimension::new([0.0, 0.0, 0.0], 7.5);
    assert!((r.value() - 7.5).abs() < 1e-10);
    assert_eq!(r.kind(), AisDimensionKind::Radius);
    let mut r2 = AisRadiusDimension::new([0.0, 0.0, 0.0], 5.0);
    r2.set_custom_value(3.0);
    assert!((r2.value() - 3.0).abs() < 1e-10);
}

#[test]
fn diameter_dim_value_and_radius() {
    let d = AisDiameterDimension::new([0.0, 0.0, 0.0], 4.0);
    assert!((d.value() - 8.0).abs() < 1e-10);
    assert!((d.radius() - 4.0).abs() < 1e-10);
    assert_eq!(d.kind(), AisDimensionKind::Diameter);
}

#[test]
fn dimension_aspect_defaults_and_setters() {
    let mut a = AisDimensionAspect::new();
    assert!((a.text_size - 1.0).abs() < 1e-10);
    assert!((a.arrow_size - 0.5).abs() < 1e-10);
    assert!(a.is_units_displayed);
    assert_eq!(a.units_string, "mm");
    a.set_text_size(2.5);
    a.set_arrow_size(0.8);
    a.set_units_string("in");
    assert!((a.text_size - 2.5).abs() < 1e-10);
    assert_eq!(a.units_string, "in");
}
