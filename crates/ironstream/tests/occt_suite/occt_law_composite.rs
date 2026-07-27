// FILE: rust/ironstream/crates/ironstream/tests/occt_law_composite.rs
//! Integration tests for the `law_composite` module.
//!
//! Mirrors the behaviour expected of OCCT's `Law_Constant`, `Law_Linear`,
//! and `Law_Composite`.

extern crate ironstream;
use ironstream::law_composite::*;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn near(a: f64, b: f64, tol: f64, label: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{label}: expected {b}, got {a}, diff={diff} > tol={tol}",
        diff = (a - b).abs()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// LawFunctionType
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn law_function_type_variants_are_distinct() {
    assert_ne!(LawFunctionType::Constant, LawFunctionType::Linear);
    assert_ne!(LawFunctionType::Linear, LawFunctionType::BSpline);
    assert_ne!(LawFunctionType::BSpline, LawFunctionType::Composite);
}

// ─────────────────────────────────────────────────────────────────────────────
// LawConstant
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn constant_value_is_constant() {
    let law = LawConstant::new(3.14, 0.0, 1.0);
    near(law.value(), 3.14, 1e-15, "value()");
    near(law.evaluate(0.0), 3.14, 1e-15, "evaluate(0)");
    near(law.evaluate(0.5), 3.14, 1e-15, "evaluate(0.5)");
    near(law.evaluate(1.0), 3.14, 1e-15, "evaluate(1)");
    // Out-of-range still returns the constant.
    near(law.evaluate(-99.0), 3.14, 1e-15, "evaluate(oob low)");
    near(law.evaluate(999.0), 3.14, 1e-15, "evaluate(oob high)");
}

#[test]
fn constant_bounds() {
    let law = LawConstant::new(1.0, 2.0, 5.0);
    assert_eq!(law.bounds(), (2.0, 5.0));
}

#[test]
fn constant_zero_value() {
    let law = LawConstant::new(0.0, -1.0, 1.0);
    near(law.evaluate(0.0), 0.0, 1e-15, "zero constant");
}

#[test]
fn constant_negative_value() {
    let law = LawConstant::new(-7.5, 0.0, 10.0);
    near(law.evaluate(5.0), -7.5, 1e-15, "negative constant");
}

#[test]
#[should_panic]
fn constant_invalid_bounds_panics() {
    let _law = LawConstant::new(1.0, 5.0, 3.0); // last < first
}

// ─────────────────────────────────────────────────────────────────────────────
// LawLinear
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn linear_endpoint_values() {
    let law = LawLinear::new(0.0, 1.0, 2.0, 8.0);
    near(law.evaluate(0.0), 2.0, 1e-12, "f(first)=start");
    near(law.evaluate(1.0), 8.0, 1e-12, "f(last)=end_val");
}

#[test]
fn linear_midpoint() {
    let law = LawLinear::new(0.0, 2.0, 0.0, 4.0);
    near(law.evaluate(1.0), 2.0, 1e-12, "midpoint");
}

#[test]
fn linear_quarter_point() {
    let law = LawLinear::new(0.0, 4.0, 0.0, 8.0);
    near(law.evaluate(1.0), 2.0, 1e-12, "quarter point");
    near(law.evaluate(3.0), 6.0, 1e-12, "three-quarter point");
}

#[test]
fn linear_clamping_below() {
    let law = LawLinear::new(1.0, 3.0, 10.0, 20.0);
    // t=0 is below domain; should clamp to first=1 => start=10
    near(law.evaluate(0.0), 10.0, 1e-12, "clamp below");
}

#[test]
fn linear_clamping_above() {
    let law = LawLinear::new(1.0, 3.0, 10.0, 20.0);
    // t=5 is above domain; should clamp to last=3 => end_val=20
    near(law.evaluate(5.0), 20.0, 1e-12, "clamp above");
}

#[test]
fn linear_constant_special_case() {
    // start == end_val => constant law
    let law = LawLinear::new(0.0, 10.0, 5.0, 5.0);
    for t in [0.0, 2.5, 5.0, 7.5, 10.0] {
        near(law.evaluate(t), 5.0, 1e-12, "constant linear");
    }
}

#[test]
fn linear_bounds() {
    let law = LawLinear::new(-3.0, 7.0, 0.0, 1.0);
    assert_eq!(law.bounds(), (-3.0, 7.0));
}

#[test]
#[should_panic]
fn linear_invalid_bounds_panics() {
    let _law = LawLinear::new(5.0, 2.0, 0.0, 1.0); // last < first
}

#[test]
fn linear_finite_difference_slope() {
    let law = LawLinear::new(0.0, 10.0, 2.0, 12.0);
    // slope = (12 - 2) / 10 = 1.0
    let h = 1e-6;
    let t = 5.0;
    let fd = (law.evaluate(t + h) - law.evaluate(t - h)) / (2.0 * h);
    near(fd, 1.0, 1e-5, "finite diff slope");
}

// ─────────────────────────────────────────────────────────────────────────────
// LawSegment
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn segment_constant_delegates() {
    let seg = LawSegment::Constant(LawConstant::new(42.0, 0.0, 5.0));
    near(seg.evaluate(2.5), 42.0, 1e-15, "segment constant evaluate");
    assert_eq!(seg.first(), 0.0);
    assert_eq!(seg.last(), 5.0);
}

#[test]
fn segment_linear_delegates() {
    let seg = LawSegment::Linear(LawLinear::new(0.0, 4.0, 0.0, 8.0));
    near(seg.evaluate(2.0), 4.0, 1e-12, "segment linear evaluate midpoint");
    assert_eq!(seg.first(), 0.0);
    assert_eq!(seg.last(), 4.0);
}

// ─────────────────────────────────────────────────────────────────────────────
// LawComposite — basic structure
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn composite_empty_nb_laws() {
    let comp = LawComposite::new();
    assert_eq!(comp.nb_laws(), 0);
}

#[test]
fn composite_single_constant_segment() {
    let mut comp = LawComposite::new();
    comp.add_constant(0.0, 1.0, 7.0);
    assert_eq!(comp.nb_laws(), 1);
    near(comp.evaluate(0.0), 7.0, 1e-15, "start");
    near(comp.evaluate(0.5), 7.0, 1e-15, "mid");
    near(comp.evaluate(1.0), 7.0, 1e-15, "end");
}

#[test]
fn composite_single_linear_segment() {
    let mut comp = LawComposite::new();
    comp.add_linear(0.0, 1.0, 0.0, 10.0);
    assert_eq!(comp.nb_laws(), 1);
    near(comp.evaluate(0.0), 0.0, 1e-12, "start");
    near(comp.evaluate(0.5), 5.0, 1e-12, "mid");
    near(comp.evaluate(1.0), 10.0, 1e-12, "end");
}

// ─────────────────────────────────────────────────────────────────────────────
// LawComposite — multiple segments
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn composite_two_constant_segments() {
    let mut comp = LawComposite::new();
    comp.add_constant(0.0, 1.0, 3.0);
    comp.add_constant(1.0, 2.0, 9.0);
    assert_eq!(comp.nb_laws(), 2);
    near(comp.evaluate(0.5), 3.0, 1e-15, "first segment");
    near(comp.evaluate(1.5), 9.0, 1e-15, "second segment");
}

#[test]
fn composite_two_linear_segments_stitched() {
    let mut comp = LawComposite::new();
    comp.add_linear(0.0, 1.0, 0.0, 1.0);
    comp.add_linear(1.0, 2.0, 1.0, 3.0);
    // First segment: f(0)=0, f(1)=1
    near(comp.evaluate(0.0), 0.0, 1e-12, "t=0");
    near(comp.evaluate(0.5), 0.5, 1e-12, "t=0.5");
    near(comp.evaluate(1.0), 1.0, 1e-10, "junction t=1");
    // Second segment: f(1)=1, f(2)=3
    near(comp.evaluate(1.5), 2.0, 1e-12, "t=1.5");
    near(comp.evaluate(2.0), 3.0, 1e-12, "t=2");
}

#[test]
fn composite_mixed_constant_then_linear() {
    let mut comp = LawComposite::new();
    comp.add_constant(0.0, 1.0, 5.0);
    comp.add_linear(1.0, 3.0, 5.0, 9.0);
    near(comp.evaluate(0.5), 5.0, 1e-15, "constant part");
    near(comp.evaluate(1.0), 5.0, 1e-12, "junction");
    near(comp.evaluate(2.0), 7.0, 1e-12, "linear midpoint");
    near(comp.evaluate(3.0), 9.0, 1e-12, "linear end");
}

#[test]
fn composite_mixed_linear_then_constant() {
    let mut comp = LawComposite::new();
    comp.add_linear(0.0, 2.0, 0.0, 4.0);
    comp.add_constant(2.0, 5.0, 4.0);
    near(comp.evaluate(1.0), 2.0, 1e-12, "linear midpoint");
    near(comp.evaluate(2.0), 4.0, 1e-12, "junction");
    near(comp.evaluate(3.5), 4.0, 1e-15, "constant plateau");
    near(comp.evaluate(5.0), 4.0, 1e-15, "plateau end");
}

#[test]
fn composite_three_segments() {
    let mut comp = LawComposite::new();
    comp.add_constant(0.0, 1.0, 1.0);
    comp.add_linear(1.0, 2.0, 1.0, 3.0);
    comp.add_constant(2.0, 4.0, 3.0);
    assert_eq!(comp.nb_laws(), 3);
    near(comp.evaluate(0.5), 1.0, 1e-15, "first plateau");
    near(comp.evaluate(1.5), 2.0, 1e-12, "ramp midpoint");
    near(comp.evaluate(3.0), 3.0, 1e-15, "second plateau");
}

// ─────────────────────────────────────────────────────────────────────────────
// LawComposite — bounds and clamping
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn composite_bounds_two_segments() {
    let mut comp = LawComposite::new();
    comp.add_constant(1.0, 2.0, 0.0);
    comp.add_linear(2.0, 5.0, 0.0, 6.0);
    assert_eq!(comp.bounds(), (1.0, 5.0));
}

#[test]
fn composite_clamp_below() {
    let mut comp = LawComposite::new();
    comp.add_linear(2.0, 4.0, 10.0, 20.0);
    // t=0 is below domain; clamps to first=2 => start=10
    near(comp.evaluate(-10.0), 10.0, 1e-12, "clamp below domain");
}

#[test]
fn composite_clamp_above() {
    let mut comp = LawComposite::new();
    comp.add_linear(2.0, 4.0, 10.0, 20.0);
    // t=100 is above domain; clamps to last=4 => end_val=20
    near(comp.evaluate(100.0), 20.0, 1e-12, "clamp above domain");
}

#[test]
fn composite_at_exact_start_boundary() {
    let mut comp = LawComposite::new();
    comp.add_linear(0.0, 3.0, 0.0, 6.0);
    near(comp.evaluate(0.0), 0.0, 1e-12, "exact start");
}

#[test]
fn composite_at_exact_end_boundary() {
    let mut comp = LawComposite::new();
    comp.add_linear(0.0, 3.0, 0.0, 6.0);
    near(comp.evaluate(3.0), 6.0, 1e-12, "exact end");
}

#[test]
fn composite_at_interior_junction() {
    let mut comp = LawComposite::new();
    comp.add_constant(0.0, 1.0, 2.0);
    comp.add_constant(1.0, 2.0, 4.0);
    // Junction at t=1 should evaluate with either segment; both give a
    // contiguous result when the law values match at the junction.
    // Here first segment gives 2 and second gives 4 — the composite
    // picks whichever segment `find_segment` resolves to.
    let v = comp.evaluate(1.0);
    assert!(v == 2.0 || v == 4.0, "junction value must be from one of the two segments");
}

// ─────────────────────────────────────────────────────────────────────────────
// LawComposite — nb_laws
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn composite_nb_laws_increments() {
    let mut comp = LawComposite::new();
    assert_eq!(comp.nb_laws(), 0);
    comp.add_constant(0.0, 1.0, 1.0);
    assert_eq!(comp.nb_laws(), 1);
    comp.add_constant(1.0, 2.0, 2.0);
    assert_eq!(comp.nb_laws(), 2);
    comp.add_linear(2.0, 3.0, 2.0, 5.0);
    assert_eq!(comp.nb_laws(), 3);
}

// ─────────────────────────────────────────────────────────────────────────────
// LawComposite — panic guards
// ─────────────────────────────────────────────────────────────────────────────

#[test]
#[should_panic]
fn composite_evaluate_empty_panics() {
    let comp = LawComposite::new();
    let _ = comp.evaluate(0.5);
}

#[test]
#[should_panic]
fn composite_bounds_empty_panics() {
    let comp = LawComposite::new();
    let _ = comp.bounds();
}

#[test]
#[should_panic]
fn composite_gap_between_segments_panics() {
    let mut comp = LawComposite::new();
    comp.add_constant(0.0, 1.0, 1.0);
    comp.add_constant(2.0, 3.0, 2.0); // gap: [1,2] not covered
}

#[test]
#[should_panic]
fn composite_reversed_segment_panics() {
    let mut comp = LawComposite::new();
    comp.add_constant(5.0, 3.0, 1.0); // last < first
}
