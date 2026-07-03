// FILE: rust/ironstream/crates/ironstream/tests/occt_geom2d_gccent.rs
//! Tests for `geom2d_gccent` — 2-D Gcc constraint qualification types and
//! the `Geom2dGcc_Circ2d2TanRad` stub solver.
//!
//! Mirrors the intent of the OpenCascade `Geom2dGcc` GTests while staying
//! entirely within safe, zero-dependency Rust.

extern crate ironstream;
use ironstream::geom2d_gccent::*;

// ---------------------------------------------------------------------------
// Geom2dGccQualifier
// ---------------------------------------------------------------------------

#[test]
fn qualifier_variants_are_distinct() {
    assert_ne!(Geom2dGccQualifier::Unqualified, Geom2dGccQualifier::Enclosing);
    assert_ne!(Geom2dGccQualifier::Enclosing, Geom2dGccQualifier::Enclosed);
    assert_ne!(Geom2dGccQualifier::Enclosed, Geom2dGccQualifier::Outside);
    assert_ne!(Geom2dGccQualifier::Outside, Geom2dGccQualifier::OnCurve);
}

#[test]
fn qualifier_copy_and_eq() {
    let q = Geom2dGccQualifier::Outside;
    let q2 = q;
    assert_eq!(q, q2);
}

// ---------------------------------------------------------------------------
// Geom2dGccQualifiedCirc — construction helpers
// ---------------------------------------------------------------------------

#[test]
fn qualified_circ_new_stores_fields() {
    let c = Geom2dGccQualifiedCirc::new([1.0, 2.0], 3.5, Geom2dGccQualifier::Outside);
    assert_eq!(c.center(), [1.0, 2.0]);
    assert_eq!(c.radius(), 3.5);
    assert_eq!(c.qualifier(), Geom2dGccQualifier::Outside);
}

#[test]
fn qualified_circ_unqualified() {
    let c = Geom2dGccQualifiedCirc::unqualified([0.0, 0.0], 1.0);
    assert_eq!(c.qualifier(), Geom2dGccQualifier::Unqualified);
    assert_eq!(c.center(), [0.0, 0.0]);
    assert_eq!(c.radius(), 1.0);
}

#[test]
fn qualified_circ_enclosed() {
    let c = Geom2dGccQualifiedCirc::enclosed([2.0, -1.0], 4.0);
    assert_eq!(c.qualifier(), Geom2dGccQualifier::Enclosed);
    assert_eq!(c.center(), [2.0, -1.0]);
    assert_eq!(c.radius(), 4.0);
}

#[test]
fn qualified_circ_enclosing() {
    let c = Geom2dGccQualifiedCirc::enclosing([-3.0, 5.0], 7.0);
    assert_eq!(c.qualifier(), Geom2dGccQualifier::Enclosing);
    assert_eq!(c.center(), [-3.0, 5.0]);
    assert_eq!(c.radius(), 7.0);
}

#[test]
fn qualified_circ_outside() {
    let c = Geom2dGccQualifiedCirc::outside([10.0, 10.0], 2.0);
    assert_eq!(c.qualifier(), Geom2dGccQualifier::Outside);
    assert_eq!(c.center(), [10.0, 10.0]);
    assert_eq!(c.radius(), 2.0);
}

#[test]
fn qualified_circ_copy_independence() {
    let c1 = Geom2dGccQualifiedCirc::outside([0.0, 0.0], 1.0);
    let c2 = c1; // Copy
    assert_eq!(c1, c2);
}

// ---------------------------------------------------------------------------
// Geom2dGccTanCirc — result circle
// ---------------------------------------------------------------------------

#[test]
fn tan_circ_new_stores_fields() {
    let t = Geom2dGccTanCirc::new(
        [3.0, 4.0],
        5.0,
        Geom2dGccQualifier::Outside,
        Geom2dGccQualifier::Enclosed,
    );
    assert_eq!(t.center(), [3.0, 4.0]);
    assert_eq!(t.radius(), 5.0);
    assert_eq!(t.qualifier1(), Geom2dGccQualifier::Outside);
    assert_eq!(t.qualifier2(), Geom2dGccQualifier::Enclosed);
}

#[test]
fn tan_circ_zero_radius() {
    let t = Geom2dGccTanCirc::new(
        [0.0, 0.0],
        0.0,
        Geom2dGccQualifier::Unqualified,
        Geom2dGccQualifier::Unqualified,
    );
    assert_eq!(t.radius(), 0.0);
}

#[test]
fn tan_circ_copy() {
    let t1 = Geom2dGccTanCirc::new(
        [1.0, 2.0],
        3.0,
        Geom2dGccQualifier::Enclosing,
        Geom2dGccQualifier::OnCurve,
    );
    let t2 = t1;
    assert_eq!(t1, t2);
}

// ---------------------------------------------------------------------------
// Geom2dGcc2TanRad — solver lifecycle
// ---------------------------------------------------------------------------

#[test]
fn solver_new_not_done() {
    let solver = Geom2dGcc2TanRad::new();
    assert!(!solver.is_done());
    assert_eq!(solver.nb_solutions(), 0);
}

#[test]
fn solver_default_not_done() {
    let solver = Geom2dGcc2TanRad::default();
    assert!(!solver.is_done());
}

#[test]
fn solver_solve_sets_done() {
    let c1 = Geom2dGccQualifiedCirc::outside([0.0, 0.0], 1.0);
    let c2 = Geom2dGccQualifiedCirc::outside([5.0, 0.0], 1.0);
    let mut solver = Geom2dGcc2TanRad::new();
    solver.solve(&c1, &c2, 2.0);
    assert!(solver.is_done());
}

#[test]
fn solver_stub_returns_no_solutions() {
    let c1 = Geom2dGccQualifiedCirc::unqualified([0.0, 0.0], 1.0);
    let c2 = Geom2dGccQualifiedCirc::unqualified([4.0, 0.0], 1.0);
    let mut solver = Geom2dGcc2TanRad::new();
    solver.solve(&c1, &c2, 3.0);
    assert_eq!(solver.nb_solutions(), 0);
    assert_eq!(solver.solutions().len(), 0);
}

#[test]
fn solver_solve_is_idempotent() {
    let c1 = Geom2dGccQualifiedCirc::enclosed([0.0, 0.0], 2.0);
    let c2 = Geom2dGccQualifiedCirc::enclosing([6.0, 0.0], 2.0);
    let mut solver = Geom2dGcc2TanRad::new();
    solver.solve(&c1, &c2, 5.0);
    assert!(solver.is_done());
    // Second call should still succeed and remain done.
    solver.solve(&c1, &c2, 5.0);
    assert!(solver.is_done());
    assert_eq!(solver.nb_solutions(), 0);
}

#[test]
fn solver_clone() {
    let c1 = Geom2dGccQualifiedCirc::outside([0.0, 0.0], 1.0);
    let c2 = Geom2dGccQualifiedCirc::outside([3.0, 0.0], 1.0);
    let mut solver = Geom2dGcc2TanRad::new();
    solver.solve(&c1, &c2, 1.5);
    let cloned = solver.clone();
    assert_eq!(cloned.is_done(), solver.is_done());
    assert_eq!(cloned.nb_solutions(), solver.nb_solutions());
}

#[test]
fn solver_with_on_curve_qualifier() {
    let c1 = Geom2dGccQualifiedCirc::new([0.0, 0.0], 1.0, Geom2dGccQualifier::OnCurve);
    let c2 = Geom2dGccQualifiedCirc::new([4.0, 0.0], 1.0, Geom2dGccQualifier::OnCurve);
    let mut solver = Geom2dGcc2TanRad::new();
    solver.solve(&c1, &c2, 2.0);
    assert!(solver.is_done());
}

#[test]
fn solver_negative_radius_stub_still_done() {
    // The stub does not validate radius; it always sets is_done = true.
    let c1 = Geom2dGccQualifiedCirc::unqualified([0.0, 0.0], 1.0);
    let c2 = Geom2dGccQualifiedCirc::unqualified([3.0, 0.0], 1.0);
    let mut solver = Geom2dGcc2TanRad::new();
    solver.solve(&c1, &c2, -1.0);
    assert!(solver.is_done());
}
