use ironstream::ais_dim_length::*;

#[test]
fn length_dim_measured_value_3_4_5_triangle() {
    let d = AisLengthDimension::new(1, [0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
    assert!(d.is_valid());
    assert!((d.measured_value() - 5.0).abs() < 1e-10);
}

#[test]
fn length_dim_formatted_with_unit_suffix() {
    let mut d = AisLengthDimension::new(1, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    d.set_unit_suffix("mm");
    let s = d.formatted_value(2);
    assert!(s.contains("mm"), "formatted_value should contain 'mm', got: {}", s);
}

#[test]
fn angle_dim_90_degrees() {
    let d = AisAngleDimension::new(1, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    assert!(d.is_valid());
    assert!((d.measured_value_deg() - 90.0).abs() < 1e-8);
}

#[test]
fn angle_dim_invalid_zero_length_edge() {
    // vertex and p1 are the same => zero-length edge => invalid
    let d = AisAngleDimension::new(1, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(!d.is_valid());
}

#[test]
fn radius_dim_valid() {
    let d = AisRadiusDimension::new(1, [0.0, 0.0, 0.0], 5.0);
    assert!(d.is_valid());
    assert!((d.measured_value() - 5.0).abs() < 1e-10);
}
