// FILE: tests/occt_units_api.rs
extern crate ironstream;

use ironstream::units_api::{UnitsApi, UnitKind};

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

#[test]
fn to_si_mm_1000() {
    near(UnitsApi::to_si(1000.0, "mm").unwrap(), 1.0, 1e-12);
}

#[test]
fn to_si_in_one_inch() {
    near(UnitsApi::to_si(1.0, "in").unwrap(), 0.0254, 1e-10);
}

#[test]
fn to_si_ft_one_foot() {
    near(UnitsApi::to_si(1.0, "ft").unwrap(), 0.3048, 1e-10);
}

#[test]
fn to_si_deg_360_is_two_pi() {
    near(UnitsApi::to_si(360.0, "deg").unwrap(), 2.0 * std::f64::consts::PI, 1e-12);
}

#[test]
fn to_si_mpa_to_pa() {
    near(UnitsApi::to_si(1.0, "MPa").unwrap(), 1_000_000.0, 1e-6);
}

#[test]
fn to_si_kn_to_n() {
    near(UnitsApi::to_si(2.5, "kN").unwrap(), 2500.0, 1e-10);
}

#[test]
fn to_si_unknown_unit_is_none() {
    assert!(UnitsApi::to_si(1.0, "lightyear").is_none());
}

#[test]
fn convert_ft_to_m() {
    near(UnitsApi::convert(1.0, "ft", "m").unwrap(), 0.3048, 1e-10);
}

#[test]
fn convert_deg_to_rad_90() {
    near(UnitsApi::convert(90.0, "deg", "rad").unwrap(), std::f64::consts::FRAC_PI_2, 1e-12);
}

#[test]
fn convert_mm_to_in() {
    near(UnitsApi::convert(25.4, "mm", "in").unwrap(), 1.0, 1e-9);
}

#[test]
fn from_si_roundtrip_kg_to_lb() {
    let si = UnitsApi::to_si(1.0, "lb").unwrap();
    let back = UnitsApi::from_si(si, "lb").unwrap();
    near(back, 1.0, 1e-10);
}

#[test]
fn unit_kind_length_variants() {
    assert_eq!(UnitsApi::unit_kind("mm"), Some(UnitKind::Length));
    assert_eq!(UnitsApi::unit_kind("cm"), Some(UnitKind::Length));
    assert_eq!(UnitsApi::unit_kind("m"),  Some(UnitKind::Length));
    assert_eq!(UnitsApi::unit_kind("in"), Some(UnitKind::Length));
    assert_eq!(UnitsApi::unit_kind("ft"), Some(UnitKind::Length));
}

#[test]
fn unit_kind_angle_variants() {
    assert_eq!(UnitsApi::unit_kind("deg"), Some(UnitKind::Angle));
    assert_eq!(UnitsApi::unit_kind("rad"), Some(UnitKind::Angle));
}

#[test]
fn unit_kind_mass_force_pressure() {
    assert_eq!(UnitsApi::unit_kind("kg"),  Some(UnitKind::Mass));
    assert_eq!(UnitsApi::unit_kind("lb"),  Some(UnitKind::Mass));
    assert_eq!(UnitsApi::unit_kind("N"),   Some(UnitKind::Force));
    assert_eq!(UnitsApi::unit_kind("kN"),  Some(UnitKind::Force));
    assert_eq!(UnitsApi::unit_kind("Pa"),  Some(UnitKind::Pressure));
    assert_eq!(UnitsApi::unit_kind("GPa"), Some(UnitKind::Pressure));
}

#[test]
fn is_length_and_angle_helpers() {
    assert!(UnitsApi::is_length_unit("ft"));
    assert!(!UnitsApi::is_length_unit("rad"));
    assert!(UnitsApi::is_angle_unit("deg"));
    assert!(!UnitsApi::is_angle_unit("mm"));
}
