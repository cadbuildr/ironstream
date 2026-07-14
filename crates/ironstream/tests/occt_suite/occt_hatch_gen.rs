// FILE: tests/occt_hatch_gen.rs
//! Integration tests for `hatch_gen` — HatchGen hatching domain types.
//!
//! All tests exercise the public API only via the `ironstream` crate.
//! No shapes are required; every test is self-contained with numeric data.

extern crate ironstream;

use ironstream::hatch_gen::{
    build_domains, sort_points_on_hatching, HatchGen_Domain, HatchGen_IntersectionPoint,
    HatchGen_IntersectionType, HatchGen_PointOnElement, HatchGen_PointOnHatching,
};

const TOL: f64 = 1e-9;
const RTOL: f64 = 1e-6;

fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= TOL,
        "{label}: expected {b} but got {a}, diff = {}",
        (a - b).abs()
    );
}

fn near_r(a: f64, b: f64, eps: f64, label: &str) {
    assert!(
        (a - b).abs() <= eps,
        "{label}: expected {b} but got {a}, diff = {}",
        (a - b).abs()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1: IntersectionType variants are all distinct and clone-able.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn intersection_type_variants_distinct() {
    let types = [
        HatchGen_IntersectionType::IN,
        HatchGen_IntersectionType::OUT,
        HatchGen_IntersectionType::TOUCH_IN,
        HatchGen_IntersectionType::TOUCH_OUT,
        HatchGen_IntersectionType::IDENTICAL,
    ];
    // Every pair of distinct variants must compare unequal.
    for i in 0..types.len() {
        for j in 0..types.len() {
            if i == j {
                assert_eq!(types[i], types[j], "same variant must be equal");
            } else {
                assert_ne!(types[i], types[j], "distinct variants must differ");
            }
        }
    }
    // Ensure Clone is implemented (no compile error).
    let _copy = types[0];
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2: HatchGen_IntersectionPoint constructor and all accessors.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn intersection_point_constructor_and_accessors() {
    let p = HatchGen_IntersectionPoint::new(5, -1.25, HatchGen_IntersectionType::OUT);
    assert_eq!(p.index(), 5, "index");
    near(p.parameter(), -1.25, "parameter");
    assert_eq!(p.intersection_type(), HatchGen_IntersectionType::OUT);
    assert!(!p.is_segment_beginning(), "default segment_beginning=false");
    assert!(!p.is_segment_end(), "default segment_end=false");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3: IntersectionPoint segment flags can be toggled independently.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn intersection_point_segment_flags_independent() {
    let mut p = HatchGen_IntersectionPoint::new(1, 0.0, HatchGen_IntersectionType::IDENTICAL);
    p.set_segment_beginning(true);
    assert!(p.is_segment_beginning(), "beginning flag set");
    assert!(!p.is_segment_end(), "end flag still false");
    p.set_segment_end(true);
    assert!(p.is_segment_beginning() && p.is_segment_end(), "both flags set");
    p.set_segment_beginning(false);
    assert!(!p.is_segment_beginning(), "beginning cleared");
    assert!(p.is_segment_end(), "end still set");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4: HatchGen_PointOnHatching — basic accessors and inner intersection.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn point_on_hatching_basic_accessors() {
    let ph = HatchGen_PointOnHatching::new(6.28, HatchGen_IntersectionType::TOUCH_OUT, 3);
    near_r(ph.parameter(), 6.28, RTOL, "parameter");
    assert_eq!(ph.intersection_type(), HatchGen_IntersectionType::TOUCH_OUT);
    assert_eq!(ph.element_index(), 3, "element index");
    // The inner intersection_point should carry the same data.
    let ip = ph.intersection_point();
    near_r(ip.parameter(), 6.28, RTOL, "inner param");
    assert_eq!(ip.index(), 3, "inner index mirrors element_index");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5: HatchGen_PointOnElement — basic accessors.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn point_on_element_basic_accessors() {
    let pe = HatchGen_PointOnElement::new(0.75, HatchGen_IntersectionType::IN, 9);
    near_r(pe.parameter(), 0.75, RTOL, "element param");
    assert_eq!(pe.intersection_type(), HatchGen_IntersectionType::IN);
    assert_eq!(pe.hatch_index(), 9, "hatch index");
    let ip = pe.intersection_point();
    near_r(ip.parameter(), 0.75, RTOL, "inner param");
    assert_eq!(ip.index(), 9, "inner index mirrors hatch_index");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6: HatchGen_Domain — bounded domain properties.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn domain_bounded_properties() {
    let p1 = HatchGen_PointOnHatching::new(2.0, HatchGen_IntersectionType::IN, 1);
    let p2 = HatchGen_PointOnHatching::new(10.0, HatchGen_IntersectionType::OUT, 2);
    let d = HatchGen_Domain::from_points(p1, p2);

    assert!(d.has_first_point(), "has first");
    assert!(d.has_last_point(), "has last");
    assert!(d.is_closed(), "closed domain");
    assert!(!d.is_infinite(), "not infinite");
    near(d.length().unwrap(), 8.0, "length");
    near(d.midpoint().unwrap(), 6.0, "midpoint");
    assert!(d.is_valid(TOL), "valid with positive length");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7: HatchGen_Domain — open domain (half-infinite).
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn domain_open_at_end() {
    let p = HatchGen_PointOnHatching::new(4.0, HatchGen_IntersectionType::IN, 5);
    let d = HatchGen_Domain::from_first_point(p);
    assert!(d.has_first_point(), "has first");
    assert!(!d.has_last_point(), "no last");
    assert!(!d.is_closed(), "not closed");
    assert!(d.length().is_none(), "no finite length");
    assert!(d.midpoint().is_none(), "no midpoint");
    // contains: everything above 4.0+tol is inside.
    assert!(d.contains(1e9, TOL), "far right inside open domain");
    assert!(!d.contains(3.0, TOL), "3.0 below start");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 8: HatchGen_Domain set_first_point / set_last_point / clear.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn domain_mutation_setters_and_clear() {
    let mut d = HatchGen_Domain::new();
    assert!(!d.has_first_point());
    assert!(!d.has_last_point());
    assert!(d.is_infinite(), "empty domain is infinite");

    let p1 = HatchGen_PointOnHatching::new(1.0, HatchGen_IntersectionType::IN, 1);
    d.set_first_point(p1);
    assert!(d.has_first_point());
    assert!(!d.has_last_point());

    let p2 = HatchGen_PointOnHatching::new(5.0, HatchGen_IntersectionType::OUT, 2);
    d.set_last_point(p2);
    assert!(d.is_closed());
    near(d.length().unwrap(), 4.0, "length after set");

    d.clear_first_point();
    assert!(!d.has_first_point(), "first cleared");
    assert!(d.has_last_point(), "last still present");

    d.clear_last_point();
    assert!(!d.has_last_point(), "last cleared");
    assert!(d.is_infinite(), "back to infinite");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9: sort_points_on_hatching produces ascending order.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn sort_produces_ascending_order() {
    let mut pts = vec![
        HatchGen_PointOnHatching::new(5.0, HatchGen_IntersectionType::OUT, 3),
        HatchGen_PointOnHatching::new(1.0, HatchGen_IntersectionType::IN, 1),
        HatchGen_PointOnHatching::new(3.0, HatchGen_IntersectionType::IN, 2),
        HatchGen_PointOnHatching::new(9.0, HatchGen_IntersectionType::OUT, 4),
        HatchGen_PointOnHatching::new(-1.0, HatchGen_IntersectionType::IN, 5),
    ];
    sort_points_on_hatching(&mut pts);
    let params: Vec<f64> = pts.iter().map(|p| p.parameter()).collect();
    for i in 1..params.len() {
        assert!(
            params[i - 1] <= params[i] + TOL,
            "not sorted: {} > {} at index {}",
            params[i - 1],
            params[i],
            i
        );
    }
    near(params[0], -1.0, "first after sort");
    near(params[4], 9.0, "last after sort");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10: build_domains with multiple IN/OUT pairs and a TOUCH_IN interspersed.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn build_domains_multiple_pairs_with_touch() {
    // Sequence: IN@1, TOUCH_IN@2.5, OUT@4, IN@6, OUT@10
    let mut pts = vec![
        HatchGen_PointOnHatching::new(1.0, HatchGen_IntersectionType::IN, 1),
        HatchGen_PointOnHatching::new(2.5, HatchGen_IntersectionType::TOUCH_IN, 2),
        HatchGen_PointOnHatching::new(4.0, HatchGen_IntersectionType::OUT, 3),
        HatchGen_PointOnHatching::new(6.0, HatchGen_IntersectionType::IN, 4),
        HatchGen_PointOnHatching::new(10.0, HatchGen_IntersectionType::OUT, 5),
    ];
    sort_points_on_hatching(&mut pts);
    let domains = build_domains(&pts);

    // Expect exactly two closed domains: [1, 4] and [6, 10].
    assert_eq!(domains.len(), 2, "two closed domains expected");

    near(domains[0].first_point().unwrap().parameter(), 1.0, "d0 start");
    near(domains[0].last_point().unwrap().parameter(), 4.0, "d0 end");
    near(domains[0].length().unwrap(), 3.0, "d0 length");

    near(domains[1].first_point().unwrap().parameter(), 6.0, "d1 start");
    near(domains[1].last_point().unwrap().parameter(), 10.0, "d1 end");
    near(domains[1].length().unwrap(), 4.0, "d1 length");

    // Both domains should be valid and closed.
    assert!(domains[0].is_closed(), "d0 closed");
    assert!(domains[1].is_closed(), "d1 closed");
    assert!(domains[0].is_valid(TOL), "d0 valid");
    assert!(domains[1].is_valid(TOL), "d1 valid");

    // Midpoints.
    near_r(domains[0].midpoint().unwrap(), 2.5, RTOL, "d0 midpoint");
    near_r(domains[1].midpoint().unwrap(), 8.0, RTOL, "d1 midpoint");
}
