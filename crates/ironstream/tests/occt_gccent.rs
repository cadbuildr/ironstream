// FILE: rust/ironstream/crates/ironstream/tests/occt_gccent.rs
//! Integration tests for `gccent` — GccEnt constraint qualification types.
//!
//! Tests exercise the public API only (no access to private fields).

extern crate ironstream;
use ironstream::gccent::*;

// ─────────────────────────── GccentQualifier ─────────────────────────────────

#[test]
fn qualifier_variants_are_distinct() {
    let variants = [
        GccentQualifier::Unqualified,
        GccentQualifier::Enclosing,
        GccentQualifier::Enclosed,
        GccentQualifier::Outside,
        GccentQualifier::OnCurve,
    ];
    for (i, a) in variants.iter().enumerate() {
        for (j, b) in variants.iter().enumerate() {
            if i == j {
                assert_eq!(a, b);
            } else {
                assert_ne!(a, b);
            }
        }
    }
}

#[test]
fn qualifier_copy_and_clone() {
    let q = GccentQualifier::Enclosing;
    let q2 = q;       // Copy
    let q3 = q.clone(); // Clone
    assert_eq!(q, q2);
    assert_eq!(q, q3);
}

#[test]
fn qualifier_debug() {
    let s = format!("{:?}", GccentQualifier::Outside);
    assert!(s.contains("Outside"));
}

// ─────────────────────────── GccentQualifiedCirc ─────────────────────────────

#[test]
fn qualified_circ_new_roundtrips_fields() {
    let center = [1.0, 2.0, 3.0];
    let radius = 5.0;
    let q = GccentQualifier::Enclosing;
    let circ = GccentQualifiedCirc::new(center, radius, q);

    assert_eq!(circ.center(), center);
    assert_eq!(circ.radius(), radius);
    assert_eq!(circ.qualifier(), q);
}

#[test]
fn qualified_circ_unqualified_constructor() {
    let center = [0.0, 0.0, 0.0];
    let circ = GccentQualifiedCirc::unqualified(center, 3.0);
    assert_eq!(circ.qualifier(), GccentQualifier::Unqualified);
    assert_eq!(circ.radius(), 3.0);
    assert_eq!(circ.center(), center);
}

#[test]
fn qualified_circ_enclosing_constructor() {
    let center = [1.0, 0.0, 0.0];
    let circ = GccentQualifiedCirc::enclosing(center, 7.5);
    assert_eq!(circ.qualifier(), GccentQualifier::Enclosing);
    assert_eq!(circ.radius(), 7.5);
}

#[test]
fn qualified_circ_enclosed_constructor() {
    let center = [0.0, 1.0, 0.0];
    let circ = GccentQualifiedCirc::enclosed(center, 2.0);
    assert_eq!(circ.qualifier(), GccentQualifier::Enclosed);
    assert_eq!(circ.radius(), 2.0);
}

#[test]
fn qualified_circ_outside_constructor() {
    let center = [0.0, 0.0, 1.0];
    let circ = GccentQualifiedCirc::outside(center, 4.0);
    assert_eq!(circ.qualifier(), GccentQualifier::Outside);
    assert_eq!(circ.radius(), 4.0);
}

#[test]
fn qualified_circ_copy_and_clone() {
    let circ = GccentQualifiedCirc::unqualified([0.0, 0.0, 0.0], 1.0);
    let circ2 = circ;         // Copy
    let circ3 = circ.clone(); // Clone
    assert_eq!(circ, circ2);
    assert_eq!(circ, circ3);
}

#[test]
fn qualified_circ_partial_eq() {
    let a = GccentQualifiedCirc::enclosing([1.0, 2.0, 3.0], 5.0);
    let b = GccentQualifiedCirc::enclosing([1.0, 2.0, 3.0], 5.0);
    let c = GccentQualifiedCirc::enclosed([1.0, 2.0, 3.0], 5.0);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn qualified_circ_debug() {
    let circ = GccentQualifiedCirc::outside([0.0, 0.0, 0.0], 1.0);
    let s = format!("{:?}", circ);
    assert!(s.contains("Outside"));
}

#[test]
fn qualified_circ_zero_radius() {
    let circ = GccentQualifiedCirc::unqualified([0.0, 0.0, 0.0], 0.0);
    assert_eq!(circ.radius(), 0.0);
}

#[test]
fn qualified_circ_negative_radius_stored_as_given() {
    // GccEnt does not validate radius sign; it is the caller's responsibility.
    let circ = GccentQualifiedCirc::unqualified([0.0, 0.0, 0.0], -1.0);
    assert_eq!(circ.radius(), -1.0);
}

// ─────────────────────────── GccentQualifiedLin ──────────────────────────────

#[test]
fn qualified_lin_new_roundtrips_fields() {
    let origin = [1.0, 2.0, 3.0];
    let direction = [0.0, 1.0, 0.0];
    let q = GccentQualifier::Outside;
    let lin = GccentQualifiedLin::new(origin, direction, q);

    assert_eq!(lin.origin(), origin);
    assert_eq!(lin.direction(), direction);
    assert_eq!(lin.qualifier(), q);
}

#[test]
fn qualified_lin_copy_and_clone() {
    let lin = GccentQualifiedLin::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], GccentQualifier::Unqualified);
    let lin2 = lin;
    let lin3 = lin.clone();
    assert_eq!(lin, lin2);
    assert_eq!(lin, lin3);
}

#[test]
fn qualified_lin_partial_eq() {
    let a = GccentQualifiedLin::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], GccentQualifier::Outside);
    let b = GccentQualifiedLin::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], GccentQualifier::Outside);
    let c = GccentQualifiedLin::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], GccentQualifier::Enclosing);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn qualified_lin_debug() {
    let lin = GccentQualifiedLin::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], GccentQualifier::OnCurve);
    let s = format!("{:?}", lin);
    assert!(s.contains("OnCurve"));
}

#[test]
fn qualified_lin_on_curve_qualifier() {
    let lin = GccentQualifiedLin::new([3.0, 4.0, 0.0], [1.0, 0.0, 0.0], GccentQualifier::OnCurve);
    assert_eq!(lin.qualifier(), GccentQualifier::OnCurve);
    assert_eq!(lin.origin(), [3.0, 4.0, 0.0]);
    assert_eq!(lin.direction(), [1.0, 0.0, 0.0]);
}

#[test]
fn qualified_lin_direction_not_required_to_be_unit() {
    // IronStream stores the direction as-is; normalisation is the algorithm's job.
    let dir = [3.0, 4.0, 0.0]; // length 5, not 1
    let lin = GccentQualifiedLin::new([0.0, 0.0, 0.0], dir, GccentQualifier::Unqualified);
    assert_eq!(lin.direction(), dir);
}
