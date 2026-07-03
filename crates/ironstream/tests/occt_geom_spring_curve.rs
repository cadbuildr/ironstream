// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_spring_curve.rs
//! Tests for `geom_spring_curve` — the Geom_TrimmedCurve-based spring/composite
//! curve module.

extern crate ironstream;
use ironstream::geom_spring_curve::*;

// ─────────────────────── SpringCurveSegment ──────────────────────────────────

#[test]
fn segment_fields_round_trip() {
    let seg = SpringCurveSegment::new(0.0, 2.5, "coil-1");
    assert_eq!(seg.u_start(), 0.0);
    assert_eq!(seg.u_end(), 2.5);
    assert_eq!(seg.label(), "coil-1");
}

#[test]
fn segment_length() {
    let seg = SpringCurveSegment::new(1.0, 4.0, "seg");
    assert!((seg.length() - 3.0).abs() < 1e-12);
}

#[test]
fn segment_length_zero_when_equal_params() {
    let seg = SpringCurveSegment::new(3.0, 3.0, "point");
    assert_eq!(seg.length(), 0.0);
}

#[test]
fn segment_mid_param() {
    let seg = SpringCurveSegment::new(1.0, 3.0, "mid");
    assert!((seg.mid_param() - 2.0).abs() < 1e-12);
}

#[test]
fn segment_mid_param_at_origin() {
    let seg = SpringCurveSegment::new(0.0, 4.0, "s");
    assert!((seg.mid_param() - 2.0).abs() < 1e-12);
}

// ─────────────────────── trim_curve free function ────────────────────────────

#[test]
fn trim_curve_produces_correct_segment() {
    let seg = trim_curve(0.5, 1.5, "trim");
    assert!((seg.u_start() - 0.5).abs() < 1e-12);
    assert!((seg.u_end() - 1.5).abs() < 1e-12);
    assert_eq!(seg.label(), "trim");
    assert!((seg.length() - 1.0).abs() < 1e-12);
}

// ─────────────────────── SpringCurve — empty ─────────────────────────────────

#[test]
fn empty_curve_after_build() {
    let mut c = SpringCurve::new();
    c.build();
    assert_eq!(c.nb_segments(), 0);
    assert!((c.total_length() - 0.0).abs() < 1e-12);
    assert!(!c.is_closed());
}

#[test]
fn default_is_same_as_new() {
    let mut c = SpringCurve::default();
    c.build();
    assert_eq!(c.nb_segments(), 0);
    assert!(!c.is_closed());
}

// ─────────────────────── SpringCurve — single segment ────────────────────────

#[test]
fn single_segment_total_length() {
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(0.0, 5.0, "only"));
    c.build();
    assert!((c.total_length() - 5.0).abs() < 1e-12);
    assert_eq!(c.nb_segments(), 1);
}

#[test]
fn single_segment_not_closed() {
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(0.0, 5.0, "seg"));
    c.build();
    // u_start=0, u_end=5 — not equal, so not closed.
    assert!(!c.is_closed());
}

#[test]
fn single_segment_trivially_closed() {
    // A degenerate single-segment where u_start == u_end; closed by definition
    // because first.u_start ≈ last.u_end.
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(3.0, 3.0, "point"));
    c.build();
    assert!(c.is_closed());
}

// ─────────────────────── SpringCurve — multiple segments ─────────────────────

#[test]
fn three_segments_total_length() {
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(0.0, 1.0, "a"));
    c.add_segment(SpringCurveSegment::new(1.0, 3.0, "b"));
    c.add_segment(SpringCurveSegment::new(3.0, 6.0, "c"));
    c.build();
    // lengths: 1 + 2 + 3 = 6
    assert!((c.total_length() - 6.0).abs() < 1e-12);
}

#[test]
fn three_segments_open() {
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(0.0, 1.0, "a"));
    c.add_segment(SpringCurveSegment::new(1.0, 2.0, "b"));
    c.add_segment(SpringCurveSegment::new(2.0, 3.0, "c"));
    c.build();
    // first.u_start=0, last.u_end=3 — not equal.
    assert!(!c.is_closed());
}

#[test]
fn segments_form_closed_loop() {
    // Simulate a periodic spring: last u_end matches first u_start.
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(0.0, 2.0, "a"));
    c.add_segment(SpringCurveSegment::new(2.0, 4.0, "b"));
    c.add_segment(SpringCurveSegment::new(4.0, 0.0, "c")); // wraps back
    c.build();
    assert!(c.is_closed());
}

#[test]
fn segment_access_by_index() {
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(0.0, 1.0, "first"));
    c.add_segment(SpringCurveSegment::new(1.0, 2.5, "second"));
    c.build();
    assert_eq!(c.segment(0).label(), "first");
    assert_eq!(c.segment(1).label(), "second");
    assert!((c.segment(1).u_end() - 2.5).abs() < 1e-12);
}

#[test]
#[should_panic]
fn segment_out_of_bounds_panics() {
    let c = SpringCurve::new();
    let _ = c.segment(0); // empty — must panic
}

// ─────────────────────── parameter_at_length ─────────────────────────────────

#[test]
fn parameter_at_length_zero_returns_first_u_start() {
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(2.0, 5.0, "s"));
    c.build();
    let p = c.parameter_at_length(0.0);
    assert!((p - 2.0).abs() < 1e-12);
}

#[test]
fn parameter_at_length_full_returns_last_u_end() {
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(0.0, 3.0, "s"));
    c.build();
    let p = c.parameter_at_length(3.0);
    assert!((p - 3.0).abs() < 1e-12);
}

#[test]
fn parameter_at_length_within_first_segment() {
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(0.0, 4.0, "a"));
    c.add_segment(SpringCurveSegment::new(4.0, 8.0, "b"));
    c.build();
    // arc_length=2.0 is 2 units into the first segment [0,4], so param = 0+2 = 2.
    let p = c.parameter_at_length(2.0);
    assert!((p - 2.0).abs() < 1e-12);
}

#[test]
fn parameter_at_length_exactly_at_segment_boundary() {
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(0.0, 3.0, "a"));
    c.add_segment(SpringCurveSegment::new(3.0, 7.0, "b"));
    c.build();
    // arc_length=3.0 lands exactly at the end of segment "a" / start of "b".
    // The walk consumes remaining==3.0 with seg_len==3.0, so it returns
    // seg.u_start + remaining = 0 + 3 = 3.
    let p = c.parameter_at_length(3.0);
    assert!((p - 3.0).abs() < 1e-12);
}

#[test]
fn parameter_at_length_within_second_segment() {
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(0.0, 3.0, "a"));
    c.add_segment(SpringCurveSegment::new(3.0, 7.0, "b"));
    c.build();
    // arc_length=5.0: first seg consumes 3, then 2 remain in [3,7], param=3+2=5.
    let p = c.parameter_at_length(5.0);
    assert!((p - 5.0).abs() < 1e-12);
}

#[test]
fn parameter_at_length_beyond_total_clamps_to_end() {
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(0.0, 4.0, "s"));
    c.build();
    let p = c.parameter_at_length(100.0);
    assert!((p - 4.0).abs() < 1e-12);
}

#[test]
fn parameter_at_length_on_empty_curve_returns_zero() {
    let mut c = SpringCurve::new();
    c.build();
    let p = c.parameter_at_length(1.0);
    assert!((p - 0.0).abs() < 1e-12);
}

#[test]
fn parameter_at_length_three_non_contiguous_params() {
    // Segments whose u_start/u_end don't share boundaries (gap-style).
    let mut c = SpringCurve::new();
    c.add_segment(SpringCurveSegment::new(10.0, 12.0, "p1")); // len=2
    c.add_segment(SpringCurveSegment::new(20.0, 25.0, "p2")); // len=5
    c.add_segment(SpringCurveSegment::new(30.0, 31.0, "p3")); // len=1
    c.build();
    // total_length = 8
    assert!((c.total_length() - 8.0).abs() < 1e-12);
    // arc_length=0 → inside p1: 10+0=10
    assert!((c.parameter_at_length(0.0) - 10.0).abs() < 1e-12);
    // arc_length=2 → boundary of p1: 10+2=12
    assert!((c.parameter_at_length(2.0) - 12.0).abs() < 1e-12);
    // arc_length=3 → 1 unit into p2: 20+1=21
    assert!((c.parameter_at_length(3.0) - 21.0).abs() < 1e-12);
    // arc_length=7 → end of p2: 20+5=25
    assert!((c.parameter_at_length(7.0) - 25.0).abs() < 1e-12);
    // arc_length=7.5 → 0.5 into p3: 30+0.5=30.5
    assert!((c.parameter_at_length(7.5) - 30.5).abs() < 1e-12);
    // arc_length=8 → end of p3: 31
    assert!((c.parameter_at_length(8.0) - 31.0).abs() < 1e-12);
}
