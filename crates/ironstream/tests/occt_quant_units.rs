extern crate ironstream;
use ironstream::quant_units::*;

#[test]
fn units_catalogue_meter_factor_is_1_0() {
    let m = UnitsCatalogue::meter();
    assert!((m.factor_to_si - 1.0).abs() < 1e-15);
    assert_eq!(m.kind, QuantityKind::Length);
    assert_eq!(m.symbol, "m");
}

#[test]
fn units_catalogue_kilogram_kind_is_mass() {
    let kg = UnitsCatalogue::kilogram();
    assert_eq!(kg.kind, QuantityKind::Mass);
    assert!((kg.factor_to_si - 1.0).abs() < 1e-15);
    assert_eq!(kg.symbol, "kg");
}

#[test]
fn units_catalogue_fahrenheit_32_degrees_is_0_celsius() {
    let f = UnitsCatalogue::fahrenheit();
    let c = UnitsCatalogue::celsius();
    // 32°F should convert to 0°C
    let result = f.convert_to(32.0, &c);
    assert!((result - 0.0).abs() < 1e-4);
}

#[test]
fn units_api_factor_mm_to_m_returns_0_001() {
    let mm = UnitsCatalogue::millimeter();
    let m = UnitsCatalogue::meter();
    let factor = UnitsApi::factor(&mm, &m);
    assert!((factor - 0.001).abs() < 1e-15);
}

#[test]
fn units_dimensions_for_kind_length_has_length_1() {
    let dim = UnitsDimensions::for_kind(QuantityKind::Length);
    assert_eq!(dim.length, 1);
    assert_eq!(dim.mass, 0);
    assert_eq!(dim.time, 0);
    assert!(!dim.is_dimensionless());
}

#[test]
fn units_dimensions_invert_negates_all_exponents() {
    let force = UnitsDimensions::for_kind(QuantityKind::Force);
    // Force: M^1 L^1 T^-2
    assert_eq!(force.mass, 1);
    assert_eq!(force.length, 1);
    assert_eq!(force.time, -2);
    let inv = force.invert();
    assert_eq!(inv.mass, -1);
    assert_eq!(inv.length, -1);
    assert_eq!(inv.time, 2);
    // Dimensionless dimensions invert to stay dimensionless
    let dl = UnitsDimensions::dimensionless();
    let dl_inv = dl.invert();
    assert!(dl_inv.is_dimensionless());
}
