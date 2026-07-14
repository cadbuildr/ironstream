// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_wire_builder.rs
//! Integration tests for `geom_wire_builder`.
//!
//! Exercises [`WireEdge`], [`WireBuilderStatus`], and [`WireBuilder`]
//! through their public API only.

extern crate ironstream;
use ironstream::geom_wire_builder::*;

// ─────────────────────────────────────────────────────────────────────────────
// WireEdge
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn wire_edge_new_stores_fields() {
    let e = WireEdge::new(7, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
    assert_eq!(e.id(), 7);
    assert_eq!(e.start(), [1.0, 2.0, 3.0]);
    assert_eq!(e.end(), [4.0, 5.0, 6.0]);
    assert!(!e.is_reversed());
}

#[test]
fn wire_edge_length_axis_aligned() {
    // Edge along X axis of length 5.
    let e = WireEdge::new(0, [0.0, 0.0, 0.0], [5.0, 0.0, 0.0]);
    assert!((e.length() - 5.0).abs() < 1e-12);
}

#[test]
fn wire_edge_length_diagonal() {
    // 3-4-5 right triangle in XY plane.
    let e = WireEdge::new(1, [0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
    assert!((e.length() - 5.0).abs() < 1e-12);
}

#[test]
fn wire_edge_length_zero_for_degenerate_edge() {
    let e = WireEdge::new(2, [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]);
    assert!((e.length()).abs() < 1e-15);
}

#[test]
fn wire_edge_reversed_swaps_endpoints() {
    let e = WireEdge::new(3, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let r = e.reversed();
    assert_eq!(r.start(), [1.0, 0.0, 0.0]);
    assert_eq!(r.end(), [0.0, 0.0, 0.0]);
}

#[test]
fn wire_edge_reversed_toggles_is_reversed_flag() {
    let e = WireEdge::new(4, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(!e.is_reversed());
    let r = e.reversed();
    assert!(r.is_reversed());
    let rr = r.reversed();
    assert!(!rr.is_reversed());
}

#[test]
fn wire_edge_reversed_preserves_id() {
    let e = WireEdge::new(42, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert_eq!(e.reversed().id(), 42);
}

#[test]
fn wire_edge_reversed_preserves_length() {
    let e = WireEdge::new(5, [1.0, 2.0, 3.0], [4.0, 6.0, 3.0]);
    let orig_len = e.length();
    let rev_len = e.reversed().length();
    assert!((orig_len - rev_len).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// WireBuilderStatus
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn wire_builder_status_variants_are_distinct() {
    assert_ne!(WireBuilderStatus::WireDone, WireBuilderStatus::EmptyWire);
    assert_ne!(WireBuilderStatus::WireDone, WireBuilderStatus::DisconnectedWire);
    assert_ne!(WireBuilderStatus::WireDone, WireBuilderStatus::NonManifoldWire);
    assert_ne!(WireBuilderStatus::EmptyWire, WireBuilderStatus::DisconnectedWire);
    assert_ne!(WireBuilderStatus::EmptyWire, WireBuilderStatus::NonManifoldWire);
    assert_ne!(WireBuilderStatus::DisconnectedWire, WireBuilderStatus::NonManifoldWire);
}

// ─────────────────────────────────────────────────────────────────────────────
// WireBuilder — initial state
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn wire_builder_new_status_is_empty_wire() {
    let wb = WireBuilder::new();
    assert_eq!(wb.status(), WireBuilderStatus::EmptyWire);
}

#[test]
fn wire_builder_new_is_done_is_false() {
    let wb = WireBuilder::new();
    assert!(!wb.is_done());
}

#[test]
fn wire_builder_new_nb_edges_is_zero() {
    let wb = WireBuilder::new();
    assert_eq!(wb.nb_edges(), 0);
}

#[test]
fn wire_builder_new_total_length_is_zero() {
    let wb = WireBuilder::new();
    assert!((wb.total_length()).abs() < 1e-15);
}

#[test]
fn wire_builder_new_is_not_closed() {
    let wb = WireBuilder::new();
    assert!(!wb.is_closed());
}

// ─────────────────────────────────────────────────────────────────────────────
// WireBuilder — single-edge wire
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn wire_builder_add_first_edge_always_accepted() {
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    assert_eq!(wb.status(), WireBuilderStatus::WireDone);
    assert!(wb.is_done());
    assert_eq!(wb.nb_edges(), 1);
}

#[test]
fn wire_builder_single_edge_total_length() {
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [3.0, 4.0, 0.0]));
    assert!((wb.total_length() - 5.0).abs() < 1e-12);
}

#[test]
fn wire_builder_single_edge_not_closed() {
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    assert!(!wb.is_closed());
}

// ─────────────────────────────────────────────────────────────────────────────
// WireBuilder — connectivity (natural and reversed)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn wire_builder_add_connected_edges_in_natural_order() {
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    wb.add(WireEdge::new(1, [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]));
    assert_eq!(wb.status(), WireBuilderStatus::WireDone);
    assert_eq!(wb.nb_edges(), 2);
}

#[test]
fn wire_builder_add_reversed_edge_accepted_and_flipped() {
    // e1 ends at (1,0,0); e2 is supplied end-first so must be reversed.
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    // e2 naturally goes (2,0,0)->(1,0,0) — reversed connects to (1,0,0).
    wb.add(WireEdge::new(1, [2.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    assert_eq!(wb.status(), WireBuilderStatus::WireDone);
    assert_eq!(wb.nb_edges(), 2);
}

#[test]
fn wire_builder_disconnected_edge_sets_status() {
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    // Gap: new edge starts at (5,0,0), far from (1,0,0).
    wb.add(WireEdge::new(1, [5.0, 0.0, 0.0], [6.0, 0.0, 0.0]));
    assert_eq!(wb.status(), WireBuilderStatus::DisconnectedWire);
    assert!(!wb.is_done());
}

#[test]
fn wire_builder_disconnected_state_rejects_further_edges() {
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    wb.add(WireEdge::new(1, [5.0, 0.0, 0.0], [6.0, 0.0, 0.0])); // disconnected
    let count_before = wb.nb_edges();
    // Try to add a validly-connecting edge after the broken state.
    wb.add(WireEdge::new(2, [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]));
    assert_eq!(wb.nb_edges(), count_before, "no new edge should be stored after disconnect");
    assert_eq!(wb.status(), WireBuilderStatus::DisconnectedWire);
}

#[test]
fn wire_builder_within_tolerance_is_connected() {
    // Endpoint differs by 5e-8, well within 1e-7 tolerance.
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    wb.add(WireEdge::new(1, [1.0 + 5e-8, 0.0, 0.0], [2.0, 0.0, 0.0]));
    assert_eq!(wb.status(), WireBuilderStatus::WireDone);
}

#[test]
fn wire_builder_beyond_tolerance_is_disconnected() {
    // Endpoint differs by 2e-7, just outside 1e-7 tolerance.
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    wb.add(WireEdge::new(1, [1.0 + 2e-7, 0.0, 0.0], [2.0, 0.0, 0.0]));
    assert_eq!(wb.status(), WireBuilderStatus::DisconnectedWire);
}

// ─────────────────────────────────────────────────────────────────────────────
// WireBuilder — total_length
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn wire_builder_total_length_sums_all_edges() {
    let mut wb = WireBuilder::new();
    // Unit square edges along X and Y.
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0])); // len 1
    wb.add(WireEdge::new(1, [1.0, 0.0, 0.0], [1.0, 1.0, 0.0])); // len 1
    wb.add(WireEdge::new(2, [1.0, 1.0, 0.0], [0.0, 1.0, 0.0])); // len 1
    assert!((wb.total_length() - 3.0).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// WireBuilder — close()
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn wire_builder_close_on_empty_is_noop() {
    let mut wb = WireBuilder::new();
    wb.close();
    assert_eq!(wb.status(), WireBuilderStatus::EmptyWire);
}

#[test]
fn wire_builder_close_open_wire_gives_done_when_loops() {
    // Triangle: (0,0)-(1,0)-(0.5,1)-(0,0).
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    wb.add(WireEdge::new(1, [1.0, 0.0, 0.0], [0.5, 1.0, 0.0]));
    wb.add(WireEdge::new(2, [0.5, 1.0, 0.0], [0.0, 0.0, 0.0]));
    wb.close();
    assert_eq!(wb.status(), WireBuilderStatus::WireDone);
    assert!(wb.is_closed());
}

#[test]
fn wire_builder_close_open_wire_gives_disconnected_when_gap() {
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    wb.add(WireEdge::new(1, [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]));
    // The wire ends at (2,0,0) but started at (0,0,0) — gap of 2.
    wb.close();
    assert_eq!(wb.status(), WireBuilderStatus::DisconnectedWire);
    assert!(!wb.is_closed());
}

#[test]
fn wire_builder_is_closed_true_for_closed_triangle() {
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    wb.add(WireEdge::new(1, [1.0, 0.0, 0.0], [0.5, 1.0, 0.0]));
    wb.add(WireEdge::new(2, [0.5, 1.0, 0.0], [0.0, 0.0, 0.0]));
    assert!(wb.is_closed());
}

#[test]
fn wire_builder_is_closed_false_for_open_chain() {
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    wb.add(WireEdge::new(1, [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]));
    assert!(!wb.is_closed());
}

#[test]
fn wire_builder_total_length_closed_square() {
    let mut wb = WireBuilder::new();
    wb.add(WireEdge::new(0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    wb.add(WireEdge::new(1, [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]));
    wb.add(WireEdge::new(2, [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]));
    wb.add(WireEdge::new(3, [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]));
    wb.close();
    assert_eq!(wb.status(), WireBuilderStatus::WireDone);
    assert!(wb.is_closed());
    assert!((wb.total_length() - 4.0).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// WireBuilder — Default
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn wire_builder_default_equals_new() {
    let wb = WireBuilder::default();
    assert_eq!(wb.status(), WireBuilderStatus::EmptyWire);
    assert_eq!(wb.nb_edges(), 0);
}
