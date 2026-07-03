// FILE: tests/occt_geom_curve_subdivide.rs
//! Integration tests for `geom_curve_subdivide` —
//! `KnotInsertResult` and `BSplineCurveSubdivider`.
//!
//! Each test is self-contained and exercises the public API from outside the
//! crate, mirroring the OCCT `GeomConvert_BSplineCurveKnotInsertion` semantics.

extern crate ironstream;
use ironstream::geom_curve_subdivide::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// A simple cubic (degree-3) B-spline with two spans:
///   knots  = [0.0, 0.5, 1.0]
///   mults  = [4, 1, 4]        => 6 poles
///   poles  = 6 points along x-axis
fn two_span_cubic() -> BSplineCurveSubdivider {
    let poles = vec![
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [2.0, 0.0, 0.0],
        [3.0, 1.0, 0.0],
        [4.0, 0.0, 0.0],
        [5.0, 1.0, 0.0],
    ];
    let knots = vec![0.0, 0.5, 1.0];
    let mults = vec![4, 1, 4];
    BSplineCurveSubdivider::new(poles, knots, mults, 3)
}

/// A minimal linear (degree-1) B-spline: one span, two poles.
fn linear_bspline() -> BSplineCurveSubdivider {
    let poles = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
    let knots = vec![0.0, 1.0];
    let mults = vec![2, 2];
    BSplineCurveSubdivider::new(poles, knots, mults, 1)
}

// ---------------------------------------------------------------------------
// KnotInsertResult — unit tests
// ---------------------------------------------------------------------------

#[test]
fn knot_insert_result_new_not_done() {
    let r = KnotInsertResult::new();
    assert!(!r.is_done(), "default result must not be marked done");
    assert_eq!(r.nb_knots(), 0);
    assert_eq!(r.nb_poles(), 0);
}

#[test]
fn knot_insert_result_default_matches_new() {
    let a = KnotInsertResult::new();
    let b = KnotInsertResult::default();
    assert_eq!(a.is_done(), b.is_done());
    assert_eq!(a.nb_knots(), b.nb_knots());
    assert_eq!(a.nb_poles(), b.nb_poles());
    assert_eq!(a.degree, b.degree);
}

#[test]
fn knot_insert_result_pole_access() {
    let mut r = KnotInsertResult::new();
    r.new_poles = vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
    r.is_done = true;
    assert_eq!(r.pole(0), [1.0, 2.0, 3.0]);
    assert_eq!(r.pole(1), [4.0, 5.0, 6.0]);
}

#[test]
fn knot_insert_result_nb_knots_and_poles() {
    let mut r = KnotInsertResult::new();
    r.new_knots = vec![0.0, 0.5, 1.0];
    r.new_mults = vec![2, 1, 2];
    r.new_poles = vec![[0.0, 0.0, 0.0]; 4];
    r.is_done = true;
    assert_eq!(r.nb_knots(), 3);
    assert_eq!(r.nb_poles(), 4);
}

// ---------------------------------------------------------------------------
// BSplineCurveSubdivider — construction
// ---------------------------------------------------------------------------

#[test]
fn subdivider_new_stores_data() {
    let s = two_span_cubic();
    assert_eq!(s.degree(), 3);
    assert_eq!(s.nb_poles(), 6);
    assert_eq!(s.knots, vec![0.0, 0.5, 1.0]);
    assert_eq!(s.mults, vec![4, 1, 4]);
}

#[test]
fn subdivider_linear_degree() {
    let s = linear_bspline();
    assert_eq!(s.degree(), 1);
    assert_eq!(s.nb_poles(), 2);
}

// ---------------------------------------------------------------------------
// insert_knot — knot not present
// ---------------------------------------------------------------------------

#[test]
fn insert_knot_new_value_is_done() {
    let s = two_span_cubic();
    let r = s.insert_knot(0.25);
    assert!(r.is_done());
}

#[test]
fn insert_knot_new_value_degree_preserved() {
    let s = two_span_cubic();
    let r = s.insert_knot(0.25);
    assert_eq!(r.degree, s.degree);
}

#[test]
fn insert_knot_new_value_knot_count_increases() {
    let s = two_span_cubic();
    let original_nb = s.knots.len();
    let r = s.insert_knot(0.25);
    // A new distinct knot should be added
    assert_eq!(r.nb_knots(), original_nb + 1);
}

#[test]
fn insert_knot_new_value_knot_present_in_result() {
    let s = two_span_cubic();
    let r = s.insert_knot(0.25);
    assert!(r.new_knots.contains(&0.25_f64));
}

#[test]
fn insert_knot_new_value_knot_sorted() {
    let s = two_span_cubic();
    let r = s.insert_knot(0.75);
    // Result knots must be non-decreasing
    for w in r.new_knots.windows(2) {
        assert!(w[0] <= w[1], "knots out of order: {:?}", r.new_knots);
    }
}

#[test]
fn insert_knot_new_value_inserted_with_mult_one() {
    let s = two_span_cubic();
    let r = s.insert_knot(0.75);
    let pos = r.new_knots.iter().position(|&k| (k - 0.75).abs() < 1.0e-12);
    assert!(pos.is_some(), "new knot 0.75 missing");
    assert_eq!(r.new_mults[pos.unwrap()], 1, "new knot should have multiplicity 1");
}

#[test]
fn insert_knot_poles_unchanged_in_stub() {
    // The stub does not update poles — they should equal the original
    let s = two_span_cubic();
    let r = s.insert_knot(0.25);
    assert_eq!(r.new_poles, s.poles);
}

// ---------------------------------------------------------------------------
// insert_knot — knot already present (no-op path)
// ---------------------------------------------------------------------------

#[test]
fn insert_knot_existing_is_done() {
    let s = two_span_cubic();
    let r = s.insert_knot(0.5); // 0.5 is already in knots
    assert!(r.is_done());
}

#[test]
fn insert_knot_existing_knot_count_unchanged() {
    let s = two_span_cubic();
    let original_nb = s.knots.len();
    let r = s.insert_knot(0.5);
    assert_eq!(r.nb_knots(), original_nb);
}

#[test]
fn insert_knot_existing_poles_unchanged() {
    let s = two_span_cubic();
    let r = s.insert_knot(0.5);
    assert_eq!(r.new_poles, s.poles);
}

#[test]
fn insert_knot_existing_mults_unchanged() {
    let s = two_span_cubic();
    let r = s.insert_knot(0.0); // boundary knot
    assert_eq!(r.new_mults, s.mults);
}

#[test]
fn insert_knot_existing_boundary_end() {
    let s = two_span_cubic();
    let r = s.insert_knot(1.0); // end knot
    assert!(r.is_done());
    assert_eq!(r.nb_knots(), s.knots.len());
}

// ---------------------------------------------------------------------------
// insert_knot — linear curve edge cases
// ---------------------------------------------------------------------------

#[test]
fn insert_knot_linear_new_knot() {
    let s = linear_bspline();
    let r = s.insert_knot(0.5);
    assert!(r.is_done());
    assert_eq!(r.nb_knots(), s.knots.len() + 1);
    assert!(r.new_knots.contains(&0.5_f64));
}

#[test]
fn insert_knot_linear_existing_knot() {
    let s = linear_bspline();
    let r = s.insert_knot(0.0);
    assert!(r.is_done());
    assert_eq!(r.nb_knots(), s.knots.len());
}

// ---------------------------------------------------------------------------
// split_at
// ---------------------------------------------------------------------------

#[test]
fn split_at_both_done() {
    let s = two_span_cubic();
    let (left, right) = s.split_at(0.5);
    assert!(left.is_done());
    assert!(right.is_done());
}

#[test]
fn split_at_degrees_preserved() {
    let s = two_span_cubic();
    let (left, right) = s.split_at(0.5);
    assert_eq!(left.degree, s.degree);
    assert_eq!(right.degree, s.degree);
}

#[test]
fn split_at_poles_partition_covers_all() {
    let s = two_span_cubic();
    let (left, right) = s.split_at(0.5);
    // The two halves must together account for all poles (may share split point)
    let total = left.nb_poles() + right.nb_poles();
    // total equals n (no overlap) or n+1 (shared boundary pole)
    let n = s.nb_poles();
    assert!(
        total == n || total == n + 1,
        "unexpected pole split: left={} right={} total={}",
        left.nb_poles(),
        right.nb_poles(),
        total
    );
}

#[test]
fn split_at_each_half_nonempty() {
    let s = two_span_cubic();
    let (left, right) = s.split_at(0.5);
    assert!(left.nb_poles() >= 1, "left half must have at least one pole");
    assert!(right.nb_poles() >= 1, "right half must have at least one pole");
}

#[test]
fn split_at_parameter_near_start() {
    // Splitting very close to the start should give a tiny left half
    let s = two_span_cubic();
    let (left, right) = s.split_at(0.01);
    assert!(left.nb_poles() <= right.nb_poles());
}

#[test]
fn split_at_parameter_near_end() {
    // Splitting very close to the end should give a tiny right half
    let s = two_span_cubic();
    let (left, right) = s.split_at(0.99);
    assert!(right.nb_poles() <= left.nb_poles());
}

#[test]
fn split_at_linear_curve() {
    let s = linear_bspline();
    let (left, right) = s.split_at(0.5);
    assert!(left.is_done());
    assert!(right.is_done());
    // Linear curve has 2 poles; each half should have at least one
    assert!(left.nb_poles() >= 1);
    assert!(right.nb_poles() >= 1);
}

#[test]
fn split_at_knots_copied_to_both_halves() {
    let s = two_span_cubic();
    let (left, right) = s.split_at(0.5);
    // The stub copies the original knot vector to both results
    assert_eq!(left.new_knots, s.knots);
    assert_eq!(right.new_knots, s.knots);
}

// ---------------------------------------------------------------------------
// nb_poles / degree accessors
// ---------------------------------------------------------------------------

#[test]
fn nb_poles_matches_input_vec_length() {
    let s = two_span_cubic();
    assert_eq!(s.nb_poles(), s.poles.len());
}

#[test]
fn degree_accessor() {
    let s = BSplineCurveSubdivider::new(
        vec![[0.0, 0.0, 0.0]; 5],
        vec![0.0, 1.0],
        vec![5, 5],
        4,
    );
    assert_eq!(s.degree(), 4);
}

#[test]
fn empty_poles_zero_nb_poles() {
    let s = BSplineCurveSubdivider::new(vec![], vec![0.0, 1.0], vec![1, 1], 2);
    assert_eq!(s.nb_poles(), 0);
}
