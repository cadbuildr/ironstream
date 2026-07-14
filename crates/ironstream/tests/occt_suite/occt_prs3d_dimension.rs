// FILE: tests/occt_prs3d_dimension.rs
extern crate ironstream;
use ironstream::prs3d_dimension::*;

#[test]
fn test_length_dimension_computed_value() {
    let d = LengthDimension::new([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
    assert!((d.computed_value() - 5.0).abs() < 1e-12);
    let mid = d.midpoint();
    assert!((mid[0] - 1.5).abs() < 1e-12);
    assert!((mid[1] - 2.0).abs() < 1e-12);
}

#[test]
fn test_length_dimension_display_mm() {
    let d = LengthDimension::new([0.0, 0.0, 0.0], [10.0, 0.0, 0.0]);
    let s = d.display_string();
    assert!(s.contains("10"), "display should contain '10': {}", s);
    assert!(s.contains("mm"), "display should contain 'mm': {}", s);
}

#[test]
fn test_angle_dimension_90_degrees() {
    let d = AngleDimension::new(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
    );
    assert!((d.computed_angle_deg() - 90.0).abs() < 1e-10);
    assert!((d.computed_angle_rad() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
}

#[test]
fn test_radius_dimension() {
    let r = RadiusDimension::new([0.0, 0.0, 0.0], 7.5);
    assert!(r.is_valid);
    let s = r.display_string();
    assert!(s.starts_with('R'), "radius display should start with R: {}", s);
    assert!((r.radius - 7.5).abs() < 1e-12);
}

#[test]
fn test_diameter_dimension() {
    let d = DiameterDimension::new([1.0, 2.0, 3.0], 5.0);
    assert!((d.diameter() - 10.0).abs() < 1e-12);
    assert!(d.is_valid);
}

#[test]
fn test_dimension_unit_factors_and_suffixes() {
    assert!((DimensionUnit::Mm.factor_from_mm() - 1.0).abs() < 1e-15);
    assert!((DimensionUnit::Cm.factor_from_mm() - 0.1).abs() < 1e-15);
    assert!((DimensionUnit::M.factor_from_mm() - 0.001).abs() < 1e-18);
    assert_eq!(DimensionUnit::Inch.suffix(), "\"");
    assert_eq!(DimensionUnit::Mm.suffix(), " mm");
}
