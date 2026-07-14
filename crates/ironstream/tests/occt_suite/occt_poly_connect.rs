// FILE: rust/ironstream/crates/ironstream/tests/occt_poly_connect.rs
//! Integration tests for `ironstream::poly_connect`, mirroring the behaviour
//! of OpenCascade's `Poly_Triangle` and `Poly_Connect`.

extern crate ironstream;
use ironstream::poly_connect::*;

// ─── PolyTriangle ────────────────────────────────────────────────────────────

#[test]
fn poly_triangle_new_and_nodes() {
    let t = PolyTriangle::new(3, 7, 11);
    assert_eq!(t.nodes(), [3, 7, 11]);
}

#[test]
fn poly_triangle_node_accessor() {
    let t = PolyTriangle::new(10, 20, 30);
    assert_eq!(t.node(0), 10);
    assert_eq!(t.node(1), 20);
    assert_eq!(t.node(2), 30);
}

#[test]
#[should_panic(expected = "out of range")]
fn poly_triangle_node_out_of_range() {
    let t = PolyTriangle::new(0, 1, 2);
    let _ = t.node(3);
}

#[test]
fn poly_triangle_set_nodes() {
    let mut t = PolyTriangle::new(0, 0, 0);
    t.set_nodes(5, 6, 7);
    assert_eq!(t.nodes(), [5, 6, 7]);
    assert_eq!(t.node(0), 5);
    assert_eq!(t.node(1), 6);
    assert_eq!(t.node(2), 7);
}

#[test]
fn poly_triangle_equality() {
    let a = PolyTriangle::new(1, 2, 3);
    let b = PolyTriangle::new(1, 2, 3);
    let c = PolyTriangle::new(1, 2, 4);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn poly_triangle_copy() {
    let a = PolyTriangle::new(4, 5, 6);
    let b = a; // Copy
    assert_eq!(a, b);
}

// ─── PolyConnect: empty mesh ─────────────────────────────────────────────────

#[test]
fn poly_connect_empty() {
    let pc = PolyConnect::new(vec![], 0);
    assert_eq!(pc.nb_triangles(), 0);
    assert_eq!(pc.nb_nodes(), 0);
}

// ─── PolyConnect: single triangle ────────────────────────────────────────────

#[test]
fn poly_connect_single_triangle() {
    let tris = vec![PolyTriangle::new(0, 1, 2)];
    let pc = PolyConnect::new(tris, 3);

    assert_eq!(pc.nb_triangles(), 1);
    assert_eq!(pc.nb_nodes(), 3);

    // A lone triangle has no neighbours.
    assert_eq!(pc.neighbours(0), [None, None, None]);

    // The triangle accessor works.
    assert_eq!(*pc.triangle(0), PolyTriangle::new(0, 1, 2));
}

// ─── PolyConnect: two triangles sharing one edge ─────────────────────────────
//
// Mesh layout:
//
//   3
//   |\
//   | \   t1 = (1,3,2)
//   |  \
//   0---1---2
//       |  /
//       | /   t0 = (1,2,3) ... wait, let's use a simpler square.
//
// Use the classic unit-square split into two right triangles:
//
//   3 --- 2
//   |  \  |
//   |   \ |
//   0 --- 1
//
//   t0 = (0, 1, 3)   edges: (0,1), (1,3), (3,0)
//   t1 = (1, 2, 3)   edges: (1,2), (2,3), (3,1)
//
// Shared edge: (1,3) — that is edge index 1 of t0, and edge index 2 of t1.

fn unit_square_tris() -> Vec<PolyTriangle> {
    vec![
        PolyTriangle::new(0, 1, 3), // t0
        PolyTriangle::new(1, 2, 3), // t1
    ]
}

#[test]
fn poly_connect_two_triangles_nb() {
    let pc = PolyConnect::new(unit_square_tris(), 4);
    assert_eq!(pc.nb_triangles(), 2);
    assert_eq!(pc.nb_nodes(), 4);
}

#[test]
fn poly_connect_two_triangles_adjacency() {
    let pc = PolyConnect::new(unit_square_tris(), 4);

    let n0 = pc.neighbours(0);
    let n1 = pc.neighbours(1);

    // t0 edge 1 = (1,3), t1 edge 2 = (3,1) — they are neighbours.
    assert_eq!(n0[1], Some(1), "t0 edge 1 should neighbour t1");
    assert_eq!(n1[2], Some(0), "t1 edge 2 should neighbour t0");

    // All other edges are boundary edges.
    assert_eq!(n0[0], None, "t0 edge 0 is a boundary edge");
    assert_eq!(n0[2], None, "t0 edge 2 is a boundary edge");
    assert_eq!(n1[0], None, "t1 edge 0 is a boundary edge");
    assert_eq!(n1[1], None, "t1 edge 1 is a boundary edge");
}

// ─── PolyConnect: triangles_around_node ──────────────────────────────────────

#[test]
fn triangles_around_corner_node() {
    let pc = PolyConnect::new(unit_square_tris(), 4);
    // Node 0 is only in t0.
    assert_eq!(pc.triangles_around_node(0), vec![0]);
    // Node 2 is only in t1.
    assert_eq!(pc.triangles_around_node(2), vec![1]);
}

#[test]
fn triangles_around_shared_nodes() {
    let pc = PolyConnect::new(unit_square_tris(), 4);
    // Node 1 is in t0 and t1.
    assert_eq!(pc.triangles_around_node(1), vec![0, 1]);
    // Node 3 is in t0 and t1.
    assert_eq!(pc.triangles_around_node(3), vec![0, 1]);
}

#[test]
#[should_panic(expected = "out of range")]
fn triangles_around_node_out_of_range() {
    let pc = PolyConnect::new(unit_square_tris(), 4);
    let _ = pc.triangles_around_node(4);
}

// ─── PolyConnect: fan of four triangles sharing a central vertex ──────────────
//
// Central vertex = 0.  Outer vertices 1..=4 arranged in a fan.
//
//   t0 = (0,1,2)
//   t1 = (0,2,3)
//   t2 = (0,3,4)
//   t3 = (0,4,1)   (closes the fan)

fn fan_tris() -> Vec<PolyTriangle> {
    vec![
        PolyTriangle::new(0, 1, 2), // t0: edges (0,1),(1,2),(2,0)
        PolyTriangle::new(0, 2, 3), // t1: edges (0,2),(2,3),(3,0)
        PolyTriangle::new(0, 3, 4), // t2: edges (0,3),(3,4),(4,0)
        PolyTriangle::new(0, 4, 1), // t3: edges (0,4),(4,1),(1,0)
    ]
}

#[test]
fn fan_nb_triangles_and_nodes() {
    let pc = PolyConnect::new(fan_tris(), 5);
    assert_eq!(pc.nb_triangles(), 4);
    assert_eq!(pc.nb_nodes(), 5);
}

#[test]
fn fan_all_four_triangles_around_center() {
    let pc = PolyConnect::new(fan_tris(), 5);
    assert_eq!(pc.triangles_around_node(0), vec![0, 1, 2, 3]);
}

#[test]
fn fan_outer_node_in_two_triangles() {
    let pc = PolyConnect::new(fan_tris(), 5);
    // Node 1 appears in t0=(0,1,2) and t3=(0,4,1).
    assert_eq!(pc.triangles_around_node(1), vec![0, 3]);
    // Node 2 appears in t0=(0,1,2) and t1=(0,2,3).
    assert_eq!(pc.triangles_around_node(2), vec![0, 1]);
}

#[test]
fn fan_adjacency_ring() {
    let pc = PolyConnect::new(fan_tris(), 5);

    // t0=(0,1,2):  edge0=(0,1), edge1=(1,2), edge2=(2,0)
    //   edge2 is (2,0)=sorted(0,2) — shared with t1 edge0=(0,2)=sorted(0,2) → neighbour
    //   edge0 is (0,1)=sorted(0,1) — shared with t3 edge2=(1,0)=sorted(0,1) → neighbour
    //   edge1 is (1,2) — boundary
    let n0 = pc.neighbours(0);
    assert!(n0.contains(&Some(1)), "t0 should neighbour t1 (shared edge (0,2))");
    assert!(n0.contains(&Some(3)), "t0 should neighbour t3 (shared edge (0,1))");
    assert!(n0.contains(&None),   "t0 has one boundary edge");

    // t2=(0,3,4): edge0=(0,3), edge1=(3,4), edge2=(4,0)
    //   edge0 shared with t1 edge2=(3,0)
    //   edge2 shared with t3 edge0=(0,4)
    //   edge1=(3,4) boundary
    let n2 = pc.neighbours(2);
    assert!(n2.contains(&Some(1)), "t2 should neighbour t1");
    assert!(n2.contains(&Some(3)), "t2 should neighbour t3");
    assert!(n2.contains(&None),   "t2 has one boundary edge");
}

// ─── PolyConnect: triangle accessor panics ───────────────────────────────────

#[test]
#[should_panic(expected = "out of range")]
fn triangle_accessor_out_of_range() {
    let pc = PolyConnect::new(unit_square_tris(), 4);
    let _ = pc.triangle(2);
}

#[test]
#[should_panic(expected = "out of range")]
fn neighbours_out_of_range() {
    let pc = PolyConnect::new(unit_square_tris(), 4);
    let _ = pc.neighbours(5);
}

// ─── PolyConnect: triangle accessor retrieves correct data ───────────────────

#[test]
fn triangle_accessor_correctness() {
    let tris = unit_square_tris();
    let pc = PolyConnect::new(tris.clone(), 4);
    assert_eq!(*pc.triangle(0), tris[0]);
    assert_eq!(*pc.triangle(1), tris[1]);
}

// ─── PolyConnect: non-manifold edge (3 triangles sharing one edge) ────────────
//
// This is pathological geometry; the contract says we leave those neighbours
// as None (no link) rather than panicking.

#[test]
fn non_manifold_edge_does_not_panic() {
    // All three triangles share edge (0,1).
    let tris = vec![
        PolyTriangle::new(0, 1, 2),
        PolyTriangle::new(0, 1, 3),
        PolyTriangle::new(0, 1, 4),
    ];
    let pc = PolyConnect::new(tris, 5);
    // Must not panic; adjacency for the shared edge is left as None.
    let _ = pc.neighbours(0);
    let _ = pc.neighbours(1);
    let _ = pc.neighbours(2);
}
