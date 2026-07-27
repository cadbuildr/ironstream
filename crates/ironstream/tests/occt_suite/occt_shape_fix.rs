// FILE: tests/occt_shape_fix.rs
//! Integration tests for the `shape_fix` module.
//!
//! Tests exercise the public API of `ShapeFix_Shape`, `ShapeFix_Wire`,
//! `ShapeFix_Face`, and `ShapeFix_Edge` using only publicly visible types.

extern crate ironstream;

use ironstream::shape_fix::{
    FixReport, ShapeFix_Edge, ShapeFix_Face, ShapeFix_Shape, ShapeFix_Wire, FIX_TOLERANCE,
};
use ironstream::top_exp::{
    TopoDS_Edge, TopoDS_Face, TopoDS_Shape, TopoDS_Shell, TopoDS_Solid, TopoDS_Vertex, TopoDS_Wire,
};

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn vtx(id: u64, x: f64, y: f64, z: f64) -> TopoDS_Vertex {
    TopoDS_Vertex { id, x, y, z }
}

fn edge(id: u64, ax: f64, ay: f64, az: f64, bx: f64, by: f64, bz: f64) -> TopoDS_Edge {
    TopoDS_Edge {
        id,
        vertices: vec![vtx(id * 10, ax, ay, az), vtx(id * 10 + 1, bx, by, bz)],
    }
}

/// Closed equilateral triangle wire in the XY plane.
fn triangle_wire(id: u64) -> TopoDS_Wire {
    let (p0, p1, p2) = ((0.0f64, 0.0f64, 0.0f64), (1.0f64, 0.0f64, 0.0f64), (0.5f64, 1.0f64, 0.0f64));
    TopoDS_Wire {
        id,
        edges: vec![
            TopoDS_Edge {
                id: id * 10,
                vertices: vec![vtx(id * 100, p0.0, p0.1, p0.2), vtx(id * 100 + 1, p1.0, p1.1, p1.2)],
            },
            TopoDS_Edge {
                id: id * 10 + 1,
                vertices: vec![vtx(id * 100 + 1, p1.0, p1.1, p1.2), vtx(id * 100 + 2, p2.0, p2.1, p2.2)],
            },
            TopoDS_Edge {
                id: id * 10 + 2,
                vertices: vec![vtx(id * 100 + 2, p2.0, p2.1, p2.2), vtx(id * 100, p0.0, p0.1, p0.2)],
            },
        ],
    }
}

fn triangle_face(id: u64) -> TopoDS_Face {
    TopoDS_Face { id, wires: vec![triangle_wire(id * 10)] }
}

fn tetra_shell(id: u64) -> TopoDS_Shell {
    TopoDS_Shell {
        id,
        faces: (0..4u64).map(|i| triangle_face(id * 10 + i)).collect(),
    }
}

// ---------------------------------------------------------------------------
// ShapeFix_Edge integration tests
// ---------------------------------------------------------------------------

/// A valid edge requires no fixes.
#[test]
fn test_edge_valid_requires_no_fixes() {
    let e = edge(1, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0);
    let mut ef = ShapeFix_Edge::new(e);
    let fixes = ef.perform();
    assert_eq!(fixes, 0);
    assert!(!ef.is_degenerate);
    // endpoint_gap should match the actual 3.0 distance.
    assert!((ef.endpoint_gap - 3.0).abs() < 1e-10);
}

/// A NaN coordinate is reset to 0 and reported as a fix.
#[test]
fn test_edge_nan_coordinate_is_repaired() {
    let e = TopoDS_Edge {
        id: 2,
        vertices: vec![vtx(1, f64::NAN, 1.0, 0.0), vtx(2, 5.0, 1.0, 0.0)],
    };
    let mut ef = ShapeFix_Edge::new(e);
    let fixes = ef.perform();
    assert!(fixes > 0);
    let repaired = ef.edge();
    assert!(repaired.vertices[0].x.is_finite(), "x should be finite after repair");
}

/// An edge whose endpoints coincide is flagged as degenerate.
#[test]
fn test_edge_zero_length_is_degenerate() {
    let e = TopoDS_Edge {
        id: 3,
        vertices: vec![vtx(1, 7.0, 3.0, -1.0), vtx(2, 7.0, 3.0, -1.0)],
    };
    let mut ef = ShapeFix_Edge::new(e);
    ef.perform();
    assert!(ef.is_degenerate);
    assert!(ef.endpoint_gap < FIX_TOLERANCE);
}

/// An edge with only one vertex gets a second vertex padded in.
#[test]
fn test_edge_one_vertex_padded() {
    let e = TopoDS_Edge { id: 4, vertices: vec![vtx(1, 2.0, 2.0, 2.0)] };
    let mut ef = ShapeFix_Edge::new(e);
    ef.perform();
    assert_eq!(ef.edge().vertices.len(), 2, "padded vertex should be added");
}

/// Custom tolerance can be set and retrieved.
#[test]
fn test_edge_custom_tolerance() {
    let e = edge(5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let mut ef = ShapeFix_Edge::new(e);
    ef.set_tolerance(1e-3);
    assert!((ef.tolerance() - 1e-3).abs() < 1e-15);
}

// ---------------------------------------------------------------------------
// ShapeFix_Wire integration tests
// ---------------------------------------------------------------------------

/// A valid triangular wire needs no repairs.
#[test]
fn test_wire_valid_triangle_no_repairs() {
    let w = triangle_wire(1);
    let edge_count = w.edges.len();
    let mut wf = ShapeFix_Wire::new(w);
    let fixes = wf.perform();
    assert_eq!(fixes, 0);
    assert_eq!(wf.wire().edges.len(), edge_count);
}

/// A degenerate (zero-length) edge inside a wire is removed.
#[test]
fn test_wire_degenerate_edge_removed() {
    let w = TopoDS_Wire {
        id: 10,
        edges: vec![
            edge(1, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            TopoDS_Edge {
                id: 2,
                vertices: vec![vtx(21, 1.5, 0.0, 0.0), vtx(22, 1.5, 0.0, 0.0)], // degenerate
            },
            edge(3, 1.5, 0.0, 0.0, 2.0, 0.0, 0.0),
        ],
    };
    let before = w.edges.len();
    let mut wf = ShapeFix_Wire::new(w);
    wf.perform();
    assert!(wf.wire().edges.len() < before, "degenerate edge should be removed");
}

/// Edges with a tiny gap are snapped together.
#[test]
fn test_wire_small_gap_closed() {
    let gap = FIX_TOLERANCE * 0.3; // smaller than 2×tolerance
    let w = TopoDS_Wire {
        id: 20,
        edges: vec![
            TopoDS_Edge {
                id: 1,
                vertices: vec![vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)],
            },
            TopoDS_Edge {
                id: 2,
                vertices: vec![vtx(3, 1.0 + gap, 0.0, 0.0), vtx(4, 2.0, 0.0, 0.0)],
            },
        ],
    };
    let mut wf = ShapeFix_Wire::new(w);
    wf.set_tolerance(FIX_TOLERANCE * 2.0);
    wf.perform();
    let w = wf.wire();
    let end_of_first = w.edges[0].vertices.last().unwrap().x;
    let start_of_second = w.edges[1].vertices.first().unwrap().x;
    assert!(
        (end_of_first - start_of_second).abs() < 1e-12,
        "gap should be closed: end={end_of_first} start={start_of_second}"
    );
}

/// Out-of-order edges are reordered by the greedy pass.
#[test]
fn test_wire_out_of_order_edges_reordered() {
    // Correct connectivity: 1→2, 2→3, 3→4  but supplied as  1→2, 3→4, 2→3.
    let w = TopoDS_Wire {
        id: 30,
        edges: vec![
            TopoDS_Edge { id: 1, vertices: vec![vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)] },
            TopoDS_Edge { id: 3, vertices: vec![vtx(5, 2.0, 0.0, 0.0), vtx(6, 3.0, 0.0, 0.0)] },
            TopoDS_Edge { id: 2, vertices: vec![vtx(3, 1.0, 0.0, 0.0), vtx(4, 2.0, 0.0, 0.0)] },
        ],
    };
    let mut wf = ShapeFix_Wire::new(w);
    wf.perform();
    let edges = &wf.wire().edges;
    // After greedy reorder: 1 (0→1), 2 (1→2), 3 (2→3)
    assert_eq!(edges[0].id, 1, "first edge should be id=1");
    assert_eq!(edges[1].id, 2, "second edge should be id=2 (reordered)");
    assert_eq!(edges[2].id, 3, "third edge should be id=3");
}

// ---------------------------------------------------------------------------
// ShapeFix_Face integration tests
// ---------------------------------------------------------------------------

/// A valid triangular face needs no repairs.
#[test]
fn test_face_valid_no_repairs() {
    let f = triangle_face(1);
    let mut ff = ShapeFix_Face::new(f);
    let fixes = ff.perform();
    assert_eq!(fixes, 0);
}

/// A face containing a wire with fewer than 3 edges has that wire removed.
#[test]
fn test_face_tiny_inner_wire_removed() {
    let outer = triangle_wire(1);
    // Inner wire with only 2 edges — too small to be a valid hole.
    let inner = TopoDS_Wire {
        id: 99,
        edges: vec![
            edge(10, 0.1, 0.1, 0.0, 0.2, 0.1, 0.0),
            edge(11, 0.2, 0.1, 0.0, 0.1, 0.1, 0.0),
        ],
    };
    let f = TopoDS_Face { id: 5, wires: vec![outer, inner] };
    let mut ff = ShapeFix_Face::new(f);
    ff.perform();
    assert_eq!(ff.face().wires.len(), 1, "tiny inner wire should be removed");
}

/// Duplicate inner wires (same edge ids) are deduplicated.
#[test]
fn test_face_duplicate_holes_deduplicated() {
    let outer = triangle_wire(1);
    let hole_a = triangle_wire(2);
    let hole_b = triangle_wire(2); // exact duplicate of hole_a
    let f = TopoDS_Face { id: 6, wires: vec![outer, hole_a, hole_b] };
    let mut ff = ShapeFix_Face::new(f);
    ff.perform();
    assert_eq!(
        ff.face().wires.len(),
        2,
        "duplicate hole should be removed; should have outer + 1 hole"
    );
}

// ---------------------------------------------------------------------------
// ShapeFix_Shape integration tests
// ---------------------------------------------------------------------------

/// A fully valid solid produces a clean FixReport.
#[test]
fn test_shape_solid_clean_report() {
    let shape = TopoDS_Shape::Solid(TopoDS_Solid {
        id: 1,
        shells: vec![tetra_shell(1)],
    });
    let mut sf = ShapeFix_Shape::new(shape);
    let report = sf.perform();
    assert!(report.is_clean(), "valid solid should need no repairs");
}

/// A vertex with NaN coordinate is repaired and the result is finite.
#[test]
fn test_shape_vertex_nan_repaired() {
    let bad_v = TopoDS_Shape::Vertex(vtx(1, f64::NAN, 2.0, 3.0));
    let mut sf = ShapeFix_Shape::new(bad_v);
    let report = sf.perform();
    assert!(report.total_fixes > 0);
    if let TopoDS_Shape::Vertex(v) = sf.shape() {
        assert!(v.x.is_finite(), "x should be finite after repair");
        // y and z should be unchanged.
        assert!((v.y - 2.0).abs() < 1e-12);
        assert!((v.z - 3.0).abs() < 1e-12);
    } else {
        panic!("expected vertex");
    }
}

/// A vertex with Inf coordinate is repaired.
#[test]
fn test_shape_vertex_inf_repaired() {
    let bad_v = TopoDS_Shape::Vertex(vtx(2, 0.0, f64::NEG_INFINITY, 0.0));
    let mut sf = ShapeFix_Shape::new(bad_v);
    sf.perform();
    if let TopoDS_Shape::Vertex(v) = sf.shape() {
        assert!(v.y.is_finite());
    } else {
        panic!("expected vertex");
    }
}

/// into_shape() transfers ownership of the repaired shape.
#[test]
fn test_shape_into_shape_ownership() {
    let v = TopoDS_Shape::Vertex(vtx(3, 1.0, 1.0, 1.0));
    let mut sf = ShapeFix_Shape::new(v);
    sf.perform();
    let owned = sf.into_shape();
    match owned {
        TopoDS_Shape::Vertex(v) => {
            assert!((v.x - 1.0).abs() < 1e-12);
        }
        _ => panic!("expected vertex"),
    }
}

/// FixReport::is_clean() returns true when total_fixes == 0.
#[test]
fn test_fix_report_is_clean_zero() {
    let r = FixReport::default();
    assert!(r.is_clean());
}

/// FixReport::is_clean() returns false when fixes > 0.
#[test]
fn test_fix_report_not_clean_when_fixes() {
    let r = FixReport {
        total_fixes: 3,
        ..Default::default()
    };
    assert!(!r.is_clean());
}
