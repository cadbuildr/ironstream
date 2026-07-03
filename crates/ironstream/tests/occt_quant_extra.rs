extern crate ironstream;
use ironstream::quant_extra::*;

#[test]
fn test_name_of_color_rgb_values() {
    assert_eq!(NameOfColor::Red.rgb_u8(), [255, 0, 0]);
    assert_eq!(NameOfColor::Blue.rgb_u8(), [0, 0, 255]);
    assert_eq!(NameOfColor::White.rgb_u8(), [255, 255, 255]);
}

#[test]
fn test_name_of_color_luminance_and_dark() {
    assert!(NameOfColor::White.luminance() > 0.9);
    assert!(!NameOfColor::White.is_dark());
    assert!(NameOfColor::Black.luminance() < 0.01);
    assert!(NameOfColor::Black.is_dark());
}

#[test]
fn test_physical_quantity_base_vs_derived() {
    assert!(PhysicalQuantity::Mass.is_base());
    assert!(PhysicalQuantity::Length.is_base());
    assert!(PhysicalQuantity::Force.is_derived());
    assert!(PhysicalQuantity::Energy.is_derived());
    assert_eq!(PhysicalQuantity::Pressure.si_unit_symbol(), "Pa");
}

#[test]
fn test_quantity_factor_mm_to_m() {
    let f = QuantitySystem::mm_to_m();
    assert!((f.convert(1000.0) - 1.0).abs() < 1e-10);
    assert!((f.inverse_convert(1.0) - 1000.0).abs() < 1e-10);
}

#[test]
fn test_quantity_factor_deg_to_rad() {
    let r = QuantitySystem::deg_to_rad();
    assert!((r.convert(180.0) - std::f64::consts::PI).abs() < 1e-10);
    assert!((r.convert(360.0) - std::f64::consts::TAU).abs() < 1e-10);
}

#[test]
fn test_period_from_seconds_and_add() {
    let p = Period::from_seconds(3661.0);
    assert_eq!(p.hours, 1);
    assert_eq!(p.minutes, 1);
    let p2 = Period::new(0, 0, 60.0);
    let sum = p.add(&p2);
    assert!((sum.to_seconds() - 3721.0).abs() < 1e-6);
}
