// FILE: rust/ironstream/crates/ironstream/tests/occt_brep_spline_builder.rs
//! Unit tests for `brep_spline_builder` — exercising `BrepEdgeParams`,
//! `BrepMakeEdge`, `BrepWireBuilder`, and `edge_length`, mirroring the style
//! of OCCT's `BRepBuilderAPI_MakeEdge` / `BRepBuilderAPI_MakeWire` tests.

extern crate ironstream;
use ironstream::brep_spline_builder::*;

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol}): diff = {}",
        (a - b).abs()
    );
}

// ---------------------------------------------------------------------------
// edge_length
// ---------------------------------------------------------------------------

#[test]
fn edge_length_axis_aligned() {
    // A unit step along X.
    near(edge_length([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]), 1.0, 1e-12);
}

#[test]
fn edge_length_diagonal() {
    // (3, 4, 0) → length 5 (3-4-5 triangle).
    near(edge_length([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]), 5.0, 1e-12);
}

#[test]
fn edge_length_3d_diagonal() {
    // (1,2,2) → sqrt(1+4+4) = 3.
    near(edge_length([0.0, 0.0, 0.0], [1.0, 2.0, 2.0]), 3.0, 1e-12);
}

#[test]
fn edge_length_zero() {
    near(edge_length([1.0, 2.0, 3.0], [1.0, 2.0, 3.0]), 0.0, 1e-12);
}

// ---------------------------------------------------------------------------
// BrepEdgeParams
// ---------------------------------------------------------------------------

#[test]
fn edge_params_new_stores_points() {
    let p = BrepEdgeParams::new([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
    assert_eq!(p.start(), [1.0, 2.0, 3.0]);
    assert_eq!(p.end(), [4.0, 5.0, 6.0]);
}

#[test]
fn edge_params_default_tolerance() {
    let p = BrepEdgeParams::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    near(p.tolerance(), 1e-7, 1e-15);
}

#[test]
fn edge_params_set_tolerance() {
    let mut p = BrepEdgeParams::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    p.set_tolerance(1e-4);
    near(p.tolerance(), 1e-4, 1e-12);
}

#[test]
fn edge_params_no_range_by_default() {
    let p = BrepEdgeParams::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(!p.has_parameter_range);
}

#[test]
fn edge_params_with_range() {
    let p = BrepEdgeParams::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).with_range(0.25, 0.75);
    assert!(p.has_parameter_range);
    near(p.u_start, 0.25, 1e-12);
    near(p.u_end, 0.75, 1e-12);
}

#[test]
fn edge_params_with_range_does_not_change_points() {
    let p = BrepEdgeParams::new([1.0, 0.0, 0.0], [2.0, 0.0, 0.0]).with_range(0.0, 1.0);
    assert_eq!(p.start(), [1.0, 0.0, 0.0]);
    assert_eq!(p.end(), [2.0, 0.0, 0.0]);
}

// ---------------------------------------------------------------------------
// BrepMakeEdge
// ---------------------------------------------------------------------------

#[test]
fn make_edge_not_done_before_build() {
    let params = BrepEdgeParams::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let edge = BrepMakeEdge::new(params);
    assert!(!edge.is_done());
    near(edge.length(), 0.0, 1e-12);
}

#[test]
fn make_edge_build_marks_done() {
    let params = BrepEdgeParams::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let mut edge = BrepMakeEdge::new(params);
    edge.build();
    assert!(edge.is_done());
}

#[test]
fn make_edge_build_computes_length() {
    let params = BrepEdgeParams::new([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
    let mut edge = BrepMakeEdge::new(params);
    edge.build();
    near(edge.length(), 5.0, 1e-12);
}

#[test]
fn make_edge_build_unit_x() {
    let params = BrepEdgeParams::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let mut edge = BrepMakeEdge::new(params);
    edge.build();
    near(edge.length(), 1.0, 1e-12);
}

#[test]
fn make_edge_stores_params() {
    let params = BrepEdgeParams::new([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
    let edge = BrepMakeEdge::new(params.clone());
    assert_eq!(edge.params.start(), params.start());
    assert_eq!(edge.params.end(), params.end());
}

// ---------------------------------------------------------------------------
// BrepWireBuilder
// ---------------------------------------------------------------------------

fn built_edge(start: [f64; 3], end: [f64; 3]) -> BrepMakeEdge {
    let mut e = BrepMakeEdge::new(BrepEdgeParams::new(start, end));
    e.build();
    e
}

#[test]
fn wire_builder_new_is_empty() {
    let w = BrepWireBuilder::new();
    assert_eq!(w.nb_edges(), 0);
    assert!(!w.is_done());
    assert!(!w.is_closed());
}

#[test]
fn wire_builder_add_and_build_single_edge() {
    let mut w = BrepWireBuilder::new();
    w.add_edge(built_edge([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    w.build();
    assert!(w.is_done());
    assert_eq!(w.nb_edges(), 1);
}

#[test]
fn wire_builder_open_wire_not_closed() {
    let mut w = BrepWireBuilder::new();
    w.add_edge(built_edge([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    w.add_edge(built_edge([1.0, 0.0, 0.0], [1.0, 1.0, 0.0]));
    w.build();
    assert!(w.is_done());
    assert!(!w.is_closed());
}

#[test]
fn wire_builder_closed_triangle() {
    // Triangle: A→B→C→A (closed).
    let a = [0.0, 0.0, 0.0];
    let b = [1.0, 0.0, 0.0];
    let c = [0.5, 1.0, 0.0];

    let mut w = BrepWireBuilder::new();
    w.add_edge(built_edge(a, b));
    w.add_edge(built_edge(b, c));
    // Last edge ends exactly at the start of the first edge.
    let mut last = BrepMakeEdge::new(BrepEdgeParams::new(c, a));
    last.build();
    w.add_edge(last);
    w.build();

    assert!(w.is_done());
    assert_eq!(w.nb_edges(), 3);
    assert!(w.is_closed());
}

#[test]
fn wire_builder_is_not_done_if_edge_not_built() {
    let mut w = BrepWireBuilder::new();
    // Add an edge that has NOT been built.
    let raw = BrepMakeEdge::new(BrepEdgeParams::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    w.add_edge(raw);
    w.build();
    assert!(!w.is_done());
}

#[test]
fn wire_builder_mixed_done_not_done() {
    let mut w = BrepWireBuilder::new();
    w.add_edge(built_edge([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    // Second edge not built.
    let raw = BrepMakeEdge::new(BrepEdgeParams::new([1.0, 0.0, 0.0], [2.0, 0.0, 0.0]));
    w.add_edge(raw);
    w.build();
    assert!(!w.is_done());
    assert_eq!(w.nb_edges(), 2);
}

#[test]
fn wire_builder_single_edge_not_closed() {
    // A single edge cannot form a closed wire.
    let mut w = BrepWireBuilder::new();
    w.add_edge(built_edge([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    w.build();
    assert!(!w.is_closed());
}

#[test]
fn wire_builder_near_closed_within_tolerance() {
    // Start and end are within the default 1e-7 tolerance — should be closed.
    let a = [0.0, 0.0, 0.0];
    let b = [1.0, 0.0, 0.0];
    // End is almost at `a` but differs by 5e-8 in X (within 1e-7).
    let a_almost = [5e-8, 0.0, 0.0];

    let mut w = BrepWireBuilder::new();
    w.add_edge(built_edge(a, b));
    let mut closing = BrepMakeEdge::new(BrepEdgeParams::new(b, a_almost));
    closing.build();
    w.add_edge(closing);
    w.build();
    assert!(w.is_closed());
}

#[test]
fn wire_builder_not_closed_outside_tolerance() {
    // End differs by 1e-4 — well outside the default 1e-7 tolerance.
    let a = [0.0, 0.0, 0.0];
    let b = [1.0, 0.0, 0.0];
    let a_far = [1e-4, 0.0, 0.0];

    let mut w = BrepWireBuilder::new();
    w.add_edge(built_edge(a, b));
    let mut non_closing = BrepMakeEdge::new(BrepEdgeParams::new(b, a_far));
    non_closing.build();
    w.add_edge(non_closing);
    w.build();
    assert!(!w.is_closed());
}
