// FILE: rust/ironstream/crates/ironstream/tests/occt_hatch_build.rs
//! Integration tests for `hatch_build` — HatchDomain, HatchLine, HatchBuilder.
//!
//! All tests exercise the public API only via the `ironstream` crate.
//! No shapes or external dependencies are required; every test is self-contained
//! with numeric data.

extern crate ironstream;

use ironstream::hatch_build::{HatchBuilder, HatchDomain, HatchLine};

const TOL: f64 = 1e-9;

fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= TOL,
        "{label}: expected {b} but got {a}, diff = {}",
        (a - b).abs()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1: HatchDomain::new_bounded stores first and last params correctly.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_domain_bounded_accessors() {
    let d = HatchDomain::new_bounded(2.0, 7.0);
    assert!(d.has_first(), "has_first should be true");
    assert!(d.has_last(), "has_last should be true");
    near(d.first_param(), 2.0, "first_param");
    near(d.last_param(), 7.0, "last_param");
    near(d.length(), 5.0, "length");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2: HatchDomain::new_bounded swaps reversed endpoints.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_domain_bounded_swaps_reversed() {
    let d = HatchDomain::new_bounded(9.0, 3.0);
    near(d.first_param(), 3.0, "first_param after swap");
    near(d.last_param(), 9.0, "last_param after swap");
    near(d.length(), 6.0, "length after swap");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3: HatchDomain::new_open reports no endpoints and infinite length.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_domain_open_flags_and_length() {
    let d = HatchDomain::new_open();
    assert!(!d.has_first(), "open domain: has_first should be false");
    assert!(!d.has_last(), "open domain: has_last should be false");
    assert!(
        d.length().is_infinite(),
        "open domain length must be infinite"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4: HatchDomain::new_bounded with identical endpoints yields length 0.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_domain_zero_length() {
    let d = HatchDomain::new_bounded(4.0, 4.0);
    assert!(d.has_first());
    assert!(d.has_last());
    near(d.length(), 0.0, "zero-length domain");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5: HatchLine::new stores origin and direction; starts with 0 domains.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_line_new_and_accessors() {
    let origin = [1.0_f64, 2.0];
    let direction = [0.0_f64, 1.0];
    let line = HatchLine::new(origin, direction);

    assert_eq!(line.origin(), [1.0, 2.0], "origin mismatch");
    assert_eq!(line.direction(), [0.0, 1.0], "direction mismatch");
    assert_eq!(line.nb_domains(), 0, "newly created line has no domains");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6: HatchLine::add_domain appends domains; domain() returns correct refs.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_line_add_and_retrieve_domains() {
    let mut line = HatchLine::new([0.0, 0.0], [1.0, 0.0]);

    line.add_domain(HatchDomain::new_bounded(0.0, 3.0));
    line.add_domain(HatchDomain::new_bounded(5.0, 8.0));

    assert_eq!(line.nb_domains(), 2, "expected 2 domains");

    let d0 = line.domain(0);
    near(d0.first_param(), 0.0, "d0 first");
    near(d0.last_param(), 3.0, "d0 last");

    let d1 = line.domain(1);
    near(d1.first_param(), 5.0, "d1 first");
    near(d1.last_param(), 8.0, "d1 last");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7: HatchBuilder::new starts empty with the given tolerance.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_builder_new_empty() {
    let hb = HatchBuilder::new(1e-7);
    assert_eq!(hb.nb_lines(), 0, "builder should start with no lines");
    near(hb.tolerance(), 1e-7, "tolerance stored correctly");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 8: HatchBuilder::add_line appends lines accessible via line().
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_builder_add_and_retrieve_lines() {
    let mut hb = HatchBuilder::new(1e-9);
    hb.add_line([0.0, 0.0], [1.0, 0.0]);
    hb.add_line([0.0, 1.0], [1.0, 0.0]);
    hb.add_line([0.0, 2.0], [1.0, 0.0]);

    assert_eq!(hb.nb_lines(), 3, "expected 3 lines");

    assert_eq!(hb.line(0).origin(), [0.0, 0.0]);
    assert_eq!(hb.line(1).origin(), [0.0, 1.0]);
    assert_eq!(hb.line(2).origin(), [0.0, 2.0]);

    for i in 0..3 {
        assert_eq!(hb.line(i).direction(), [1.0, 0.0]);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9: HatchBuilder::trim_by_domain adds a domain to the specified line.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_builder_trim_by_domain_basic() {
    let mut hb = HatchBuilder::new(1e-9);
    hb.add_line([0.0, 0.0], [1.0, 0.0]);

    hb.trim_by_domain(0, 1.0, 5.0);

    let line = hb.line(0);
    assert_eq!(line.nb_domains(), 1, "one domain expected after trim");
    let d = line.domain(0);
    near(d.first_param(), 1.0, "trim first");
    near(d.last_param(), 5.0, "trim last");
    near(d.length(), 4.0, "trim length");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10: trim_by_domain swaps reversed (first > last) arguments.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_builder_trim_swaps_reversed() {
    let mut hb = HatchBuilder::new(1e-9);
    hb.add_line([0.0, 0.0], [0.0, 1.0]);

    hb.trim_by_domain(0, 8.0, 2.0); // reversed

    let d = hb.line(0).domain(0);
    near(d.first_param(), 2.0, "first after swap");
    near(d.last_param(), 8.0, "last after swap");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 11: trim_by_domain discards degenerate (zero-length) domains.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_builder_trim_discards_degenerate() {
    let tolerance = 1e-6;
    let mut hb = HatchBuilder::new(tolerance);
    hb.add_line([0.0, 0.0], [1.0, 0.0]);

    // Domain length 0 — should be discarded.
    hb.trim_by_domain(0, 3.0, 3.0);
    // Domain length smaller than tolerance — should also be discarded.
    hb.trim_by_domain(0, 3.0, 3.0 + tolerance * 0.5);

    assert_eq!(
        hb.line(0).nb_domains(),
        0,
        "degenerate domains must be discarded"
    );

    // Domain just above tolerance — should be kept.
    hb.trim_by_domain(0, 0.0, tolerance * 2.0);
    assert_eq!(hb.line(0).nb_domains(), 1, "valid domain must be kept");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 12: Multiple lines each receive independent trim domains.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_builder_multiple_lines_independent_domains() {
    let mut hb = HatchBuilder::new(1e-9);

    // Three horizontal hatch lines at y = 0, 1, 2.
    for y in 0..3_usize {
        hb.add_line([0.0, y as f64], [1.0, 0.0]);
    }

    // Trim each line with a different interval.
    hb.trim_by_domain(0, 0.0, 4.0);
    hb.trim_by_domain(1, 1.0, 3.0);
    hb.trim_by_domain(2, 0.5, 2.5);

    assert_eq!(hb.line(0).nb_domains(), 1);
    assert_eq!(hb.line(1).nb_domains(), 1);
    assert_eq!(hb.line(2).nb_domains(), 1);

    near(hb.line(0).domain(0).length(), 4.0, "line0 domain length");
    near(hb.line(1).domain(0).length(), 2.0, "line1 domain length");
    near(hb.line(2).domain(0).length(), 2.0, "line2 domain length");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 13: A single line may have multiple non-overlapping trim domains.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_line_multiple_domains_per_line() {
    let mut hb = HatchBuilder::new(1e-9);
    hb.add_line([0.0, 0.0], [1.0, 0.0]);

    hb.trim_by_domain(0, 0.0, 2.0);
    hb.trim_by_domain(0, 4.0, 6.0);
    hb.trim_by_domain(0, 9.0, 11.0);

    let line = hb.line(0);
    assert_eq!(line.nb_domains(), 3, "expected 3 domains");

    near(line.domain(0).first_param(), 0.0, "d0 first");
    near(line.domain(0).last_param(), 2.0, "d0 last");
    near(line.domain(1).first_param(), 4.0, "d1 first");
    near(line.domain(1).last_param(), 6.0, "d1 last");
    near(line.domain(2).first_param(), 9.0, "d2 first");
    near(line.domain(2).last_param(), 11.0, "d2 last");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 14: HatchLine with a diagonal direction retains that direction.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_line_diagonal_direction() {
    let dir = [1.0_f64 / 2.0_f64.sqrt(), 1.0_f64 / 2.0_f64.sqrt()];
    let line = HatchLine::new([3.0, -1.0], dir);
    let got = line.direction();
    assert!(
        (got[0] - dir[0]).abs() < TOL && (got[1] - dir[1]).abs() < TOL,
        "diagonal direction not stored correctly"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 15: HatchBuilder simulates a simple rectangular hatch pattern.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn hatch_builder_rectangular_pattern() {
    // Simulate hatching a unit square [0,1]x[0,1] with 5 horizontal lines
    // at y = 0.0, 0.25, 0.5, 0.75, 1.0, each trimmed to x in [0, 1].
    let mut hb = HatchBuilder::new(1e-9);
    let ys = [0.0_f64, 0.25, 0.5, 0.75, 1.0];
    for &y in &ys {
        hb.add_line([0.0, y], [1.0, 0.0]);
    }
    for i in 0..hb.nb_lines() {
        hb.trim_by_domain(i, 0.0, 1.0);
    }

    assert_eq!(hb.nb_lines(), 5, "5 horizontal lines");
    for i in 0..5 {
        let line = hb.line(i);
        assert_eq!(line.nb_domains(), 1, "line {i}: one domain");
        near(line.domain(0).first_param(), 0.0, &format!("line {i} first"));
        near(line.domain(0).last_param(), 1.0, &format!("line {i} last"));
        near(line.domain(0).length(), 1.0, &format!("line {i} length"));
        near(line.origin()[1], ys[i], &format!("line {i} y-origin"));
    }
}
