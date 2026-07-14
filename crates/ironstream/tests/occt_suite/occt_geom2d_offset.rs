// FILE: rust/ironstream/crates/ironstream/tests/occt_geom2d_offset.rs

//! Integration tests for `Geom2d_OffsetCurve` (stub module).
//!
//! Exercises the constructor, accessors, mutators, stub evaluation, and
//! continuity query defined in `ironstream::geom2d_offset`.

extern crate ironstream;
use ironstream::geom2d_offset::*;

// ─────────────────────────────────────────────────────────────────────────────
// Construction and basic accessors
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn new_stores_all_fields() {
    let c = Geom2dOffsetCurve::new(3.5, 3, 10, 0.0, 1.0);
    assert_eq!(c.offset(), 3.5);
    assert_eq!(c.degree(), 3);
    assert_eq!(c.nb_poles(), 10);
    assert_eq!(c.first_param(), 0.0);
    assert_eq!(c.last_param(), 1.0);
}

#[test]
fn new_is_not_periodic() {
    let c = Geom2dOffsetCurve::new(0.0, 1, 2, -1.0, 1.0);
    assert!(!c.is_periodic());
}

// ─────────────────────────────────────────────────────────────────────────────
// Offset getter / setter
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn set_offset_updates_value() {
    let mut c = Geom2dOffsetCurve::new(1.0, 2, 5, 0.0, 2.0);
    c.set_offset(-4.0);
    assert_eq!(c.offset(), -4.0);
}

#[test]
fn offset_zero_roundtrip() {
    let mut c = Geom2dOffsetCurve::new(5.0, 1, 2, 0.0, 1.0);
    c.set_offset(0.0);
    assert_eq!(c.offset(), 0.0);
}

// ─────────────────────────────────────────────────────────────────────────────
// Parameter range: set_range
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn set_range_updates_first_and_last() {
    let mut c = Geom2dOffsetCurve::new(0.0, 3, 4, 0.0, 1.0);
    c.set_range(-5.0, 10.0);
    assert_eq!(c.first_param(), -5.0);
    assert_eq!(c.last_param(), 10.0);
}

#[test]
fn initial_range_preserved() {
    let c = Geom2dOffsetCurve::new(2.0, 2, 3, -3.14, 3.14);
    assert!((c.first_param() - (-3.14)).abs() < 1e-15);
    assert!((c.last_param() - 3.14).abs() < 1e-15);
}

// ─────────────────────────────────────────────────────────────────────────────
// Stub evaluation: evaluate()
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn evaluate_returns_t_and_offset_in_y() {
    let offset = 7.0;
    let c = Geom2dOffsetCurve::new(offset, 2, 3, 0.0, 1.0);
    let p = c.evaluate(0.5);
    assert_eq!(p[0], 0.5, "x should equal t");
    assert_eq!(p[1], offset, "y should equal offset");
}

#[test]
fn evaluate_at_negative_t() {
    let c = Geom2dOffsetCurve::new(-2.0, 1, 2, -10.0, 10.0);
    let p = c.evaluate(-3.0);
    assert_eq!(p[0], -3.0);
    assert_eq!(p[1], -2.0);
}

#[test]
fn evaluate_zero_offset_gives_y_zero() {
    let c = Geom2dOffsetCurve::new(0.0, 1, 2, 0.0, 1.0);
    let p = c.evaluate(42.0);
    assert_eq!(p[0], 42.0);
    assert_eq!(p[1], 0.0);
}

// ─────────────────────────────────────────────────────────────────────────────
// Stub evaluation: d1()
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn d1_value_matches_evaluate() {
    let c = Geom2dOffsetCurve::new(5.0, 3, 6, 0.0, 1.0);
    let t = 0.25;
    let (val, _tan) = c.d1(t);
    let ev = c.evaluate(t);
    assert_eq!(val, ev, "d1 value must equal evaluate");
}

#[test]
fn d1_tangent_is_unit_x() {
    let c = Geom2dOffsetCurve::new(1.0, 2, 4, 0.0, 1.0);
    let (_val, tan) = c.d1(0.5);
    assert_eq!(tan, [1.0, 0.0], "stub tangent must be [1, 0]");
}

#[test]
fn d1_tangent_constant_across_parameters() {
    let c = Geom2dOffsetCurve::new(3.0, 3, 5, 0.0, 10.0);
    for i in 0..=10 {
        let t = i as f64;
        let (_val, tan) = c.d1(t);
        assert_eq!(
            tan,
            [1.0, 0.0],
            "tangent should be [1, 0] at t={t}"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// continuity()
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn continuity_degree_zero_gives_zero() {
    let c = Geom2dOffsetCurve::new(0.0, 0, 1, 0.0, 1.0);
    assert_eq!(c.continuity(), 0);
}

#[test]
fn continuity_degree_one_gives_one() {
    let c = Geom2dOffsetCurve::new(0.0, 1, 2, 0.0, 1.0);
    assert_eq!(c.continuity(), 1);
}

#[test]
fn continuity_degree_two_gives_two() {
    let c = Geom2dOffsetCurve::new(0.0, 2, 3, 0.0, 1.0);
    assert_eq!(c.continuity(), 2);
}

#[test]
fn continuity_degree_three_capped_at_two() {
    let c = Geom2dOffsetCurve::new(0.0, 3, 4, 0.0, 1.0);
    assert_eq!(c.continuity(), 2, "continuity is capped at 2");
}

#[test]
fn continuity_high_degree_capped_at_two() {
    let c = Geom2dOffsetCurve::new(0.0, 10, 11, 0.0, 1.0);
    assert_eq!(c.continuity(), 2, "continuity is capped at 2 for any degree >= 2");
}

// ─────────────────────────────────────────────────────────────────────────────
// Clone / PartialEq
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn clone_produces_equal_value() {
    let c = Geom2dOffsetCurve::new(1.5, 3, 7, -1.0, 1.0);
    let d = c.clone();
    assert_eq!(c, d);
}

#[test]
fn mutated_clone_differs() {
    let c = Geom2dOffsetCurve::new(1.5, 3, 7, -1.0, 1.0);
    let mut d = c.clone();
    d.set_offset(99.0);
    assert_ne!(c, d);
}
