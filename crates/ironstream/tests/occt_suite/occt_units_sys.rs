use ironstream::units_sys::*;

#[test]
fn registry_find_mm() {
    let r = UnitsRegistry::new();
    let mm = r.find_by_symbol("mm").unwrap();
    assert_eq!(mm.dimension, DimensionKind::Length);
    assert!((mm.to_si - 1e-3).abs() < 1e-15);
}

#[test]
fn registry_convert_length() {
    let r = UnitsRegistry::new();
    let v = r.convert(1000.0, "mm", "m").unwrap();
    assert!((v - 1.0).abs() < 1e-10);
}

#[test]
fn registry_wrong_dim() {
    let r = UnitsRegistry::new();
    assert!(r.convert(1.0, "m", "kg").is_none());
}

#[test]
fn registry_angle() {
    use std::f64::consts::PI;
    let r = UnitsRegistry::new();
    let rad = r.convert(180.0, "deg", "rad").unwrap();
    assert!((rad - PI).abs() < 1e-10);
}

#[test]
fn units_api_round_trip() {
    let si = UnitsApi::any_to_ls(1000.0, "mm");
    let back = UnitsApi::ls_to_any(si, "mm");
    assert!((back - 1000.0).abs() < 1e-6);
}
