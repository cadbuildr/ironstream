// FILE: tests/occt_brep_sweep.rs
extern crate ironstream;

use ironstream::brep_sweep::*;
use std::f64::consts::PI;

// ── BRepSweepPrism ────────────────────────────────────────────────────────────

#[test]
fn test_prism_z_axis_basic() {
    let p = BRepSweepPrism::new([0.0, 0.0, 1.0], 5.0).unwrap();
    assert!(p.is_done());
    assert!(!p.is_infinite());
    assert!((p.length() - 5.0).abs() < 1e-12);
    let d = p.direction();
    assert!((d[0]).abs() < 1e-12);
    assert!((d[1]).abs() < 1e-12);
    assert!((d[2] - 1.0).abs() < 1e-12);
}

#[test]
fn test_prism_unnormalized_direction_is_normalized() {
    let p = BRepSweepPrism::new([0.0, 0.0, 3.0], 2.0).unwrap();
    let d = p.direction();
    let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    assert!((len - 1.0).abs() < 1e-12, "direction not unit: len={len}");
    assert!((d[2] - 1.0).abs() < 1e-12);
}

#[test]
fn test_prism_diagonal_direction_normalized() {
    let p = BRepSweepPrism::new([1.0, 1.0, 1.0], 10.0).unwrap();
    let d = p.direction();
    let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    assert!((len - 1.0).abs() < 1e-12);
    let expected = 1.0_f64 / 3.0_f64.sqrt();
    assert!((d[0] - expected).abs() < 1e-12);
    assert!((d[1] - expected).abs() < 1e-12);
    assert!((d[2] - expected).abs() < 1e-12);
}

#[test]
fn test_prism_error_zero_length() {
    assert!(matches!(
        BRepSweepPrism::new([0.0, 0.0, 1.0], 0.0),
        Err(SweepError::ZeroDistance)
    ));
}

#[test]
fn test_prism_error_negative_length() {
    assert!(matches!(
        BRepSweepPrism::new([0.0, 1.0, 0.0], -3.0),
        Err(SweepError::ZeroDistance)
    ));
}

#[test]
fn test_prism_error_zero_direction() {
    assert!(matches!(
        BRepSweepPrism::new([0.0, 0.0, 0.0], 1.0),
        Err(SweepError::InvalidDirection)
    ));
}

#[test]
fn test_prism_infinite() {
    let p = BRepSweepPrism::infinite([0.0, 1.0, 0.0]).unwrap();
    assert!(p.is_done());
    assert!(p.is_infinite());
    assert!(p.length().is_infinite());
    let d = p.direction();
    assert!((d[1] - 1.0).abs() < 1e-12);
}

#[test]
fn test_prism_infinite_error_zero_direction() {
    assert!(matches!(
        BRepSweepPrism::infinite([0.0, 0.0, 0.0]),
        Err(SweepError::InvalidDirection)
    ));
}

// ── BRepSweepRevolution ───────────────────────────────────────────────────────

#[test]
fn test_revolution_full_y_axis() {
    let r = BRepSweepRevolution::full_revolution([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]).unwrap();
    assert!(r.is_done());
    assert!(r.is_full());
    assert!((r.angle() - 2.0 * PI).abs() < 1e-9);
    let origin = r.axis_origin();
    assert!(origin[0].abs() < 1e-12 && origin[1].abs() < 1e-12 && origin[2].abs() < 1e-12);
    let dir = r.axis_direction();
    assert!((dir[1] - 1.0).abs() < 1e-12);
}

#[test]
fn test_revolution_partial_quarter_turn() {
    let r = BRepSweepRevolution::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 2.0).unwrap();
    assert!(r.is_done());
    assert!(!r.is_full());
    assert!((r.angle() - PI / 2.0).abs() < 1e-12);
}

#[test]
fn test_revolution_axis_direction_normalized() {
    let r = BRepSweepRevolution::new([0.0, 0.0, 0.0], [0.0, 0.0, 5.0], PI).unwrap();
    let d = r.axis_direction();
    let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    assert!((len - 1.0).abs() < 1e-12, "direction not unit: len={len}");
    assert!((d[2] - 1.0).abs() < 1e-12);
}

#[test]
fn test_revolution_off_center_axis_origin() {
    let origin = [3.0, 0.0, -1.0];
    let r = BRepSweepRevolution::new(origin, [0.0, 0.0, 1.0], PI).unwrap();
    let o = r.axis_origin();
    assert!((o[0] - 3.0).abs() < 1e-12);
    assert!((o[1]).abs() < 1e-12);
    assert!((o[2] - (-1.0)).abs() < 1e-12);
}

#[test]
fn test_revolution_error_zero_angle() {
    assert!(matches!(
        BRepSweepRevolution::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0),
        Err(SweepError::ZeroAngle)
    ));
}

#[test]
fn test_revolution_error_near_zero_angle() {
    assert!(matches!(
        BRepSweepRevolution::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1e-13),
        Err(SweepError::ZeroAngle)
    ));
}

#[test]
fn test_revolution_error_zero_direction() {
    assert!(matches!(
        BRepSweepRevolution::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], PI),
        Err(SweepError::InvalidDirection)
    ));
}

#[test]
fn test_revolution_full_flag_for_large_angle() {
    let r = BRepSweepRevolution::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 3.0 * PI).unwrap();
    assert!(r.is_full(), "angle > 2π should be treated as full");
}

#[test]
fn test_sweep_error_variants_distinct() {
    let e1 = SweepError::EmptyShape;
    let e2 = SweepError::InvalidDirection;
    let e3 = SweepError::ZeroAngle;
    let e4 = SweepError::ZeroDistance;
    assert_ne!(e1, e2);
    assert_ne!(e2, e3);
    assert_ne!(e3, e4);
    assert_ne!(e1, e4);
}
