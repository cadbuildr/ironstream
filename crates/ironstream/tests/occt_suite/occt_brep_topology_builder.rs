// FILE: rust/ironstream/crates/ironstream/tests/occt_brep_topology_builder.rs
extern crate ironstream;
use ironstream::brep_topology_builder::*;

// ---------------------------------------------------------------------------
// BrepTopoShapeType — enum identity
// ---------------------------------------------------------------------------

#[test]
fn shape_type_vertex_is_vertex() {
    let t = BrepTopoShapeType::Vertex;
    assert_eq!(t, BrepTopoShapeType::Vertex);
}

#[test]
fn shape_type_all_variants_distinct() {
    use BrepTopoShapeType::*;
    let all = [Vertex, Edge, Wire, Face, Shell, Solid, CompSolid, Compound];
    for i in 0..all.len() {
        for j in 0..all.len() {
            if i == j {
                assert_eq!(all[i], all[j]);
            } else {
                assert_ne!(all[i], all[j]);
            }
        }
    }
}

#[test]
fn shape_type_copy_semantics() {
    let t = BrepTopoShapeType::Shell;
    let t2 = t;
    assert_eq!(t, t2);
}

// ---------------------------------------------------------------------------
// BrepTopoShape — construction and accessors
// ---------------------------------------------------------------------------

#[test]
fn brep_topo_shape_new_stores_id() {
    let s = BrepTopoShape::new(42, BrepTopoShapeType::Vertex, "pt");
    assert_eq!(s.id(), 42);
}

#[test]
fn brep_topo_shape_new_stores_shape_type() {
    let s = BrepTopoShape::new(0, BrepTopoShapeType::Face, "f");
    assert_eq!(s.shape_type(), BrepTopoShapeType::Face);
}

#[test]
fn brep_topo_shape_new_stores_label() {
    let s = BrepTopoShape::new(0, BrepTopoShapeType::Solid, "my_solid");
    assert_eq!(s.label(), "my_solid");
}

#[test]
fn brep_topo_shape_new_orientation_is_forward() {
    let s = BrepTopoShape::new(0, BrepTopoShapeType::Edge, "e");
    assert_eq!(s.orientation(), 0);
}

#[test]
fn brep_topo_shape_new_children_empty() {
    let s = BrepTopoShape::new(0, BrepTopoShapeType::Wire, "w");
    assert!(s.children().is_empty());
}

#[test]
fn brep_topo_shape_add_child_appends() {
    let mut s = BrepTopoShape::new(0, BrepTopoShapeType::Wire, "w");
    s.add_child(3);
    s.add_child(7);
    assert_eq!(s.children(), &[3usize, 7]);
}

#[test]
fn brep_topo_shape_set_orientation_reversed() {
    let mut s = BrepTopoShape::new(0, BrepTopoShapeType::Edge, "e");
    s.set_orientation(1);
    assert_eq!(s.orientation(), 1);
}

#[test]
fn brep_topo_shape_set_orientation_roundtrip() {
    let mut s = BrepTopoShape::new(0, BrepTopoShapeType::Face, "f");
    s.set_orientation(1);
    s.set_orientation(0);
    assert_eq!(s.orientation(), 0);
}

// ---------------------------------------------------------------------------
// BrepTopoBuilder — new / nb_shapes
// ---------------------------------------------------------------------------

#[test]
fn builder_new_is_empty() {
    let b = BrepTopoBuilder::new();
    assert_eq!(b.nb_shapes(), 0);
}

#[test]
fn builder_shape_on_empty_returns_none() {
    let b = BrepTopoBuilder::new();
    assert!(b.shape(0).is_none());
    assert!(b.shape(100).is_none());
}

// ---------------------------------------------------------------------------
// BrepTopoBuilder — make_vertex
// ---------------------------------------------------------------------------

#[test]
fn make_vertex_first_id_is_zero() {
    let mut b = BrepTopoBuilder::new();
    let id = b.make_vertex("A");
    assert_eq!(id, 0);
}

#[test]
fn make_vertex_increments_id() {
    let mut b = BrepTopoBuilder::new();
    let a = b.make_vertex("A");
    let c = b.make_vertex("B");
    assert_eq!(c, a + 1);
}

#[test]
fn make_vertex_nb_shapes_increases() {
    let mut b = BrepTopoBuilder::new();
    b.make_vertex("v");
    assert_eq!(b.nb_shapes(), 1);
    b.make_vertex("v2");
    assert_eq!(b.nb_shapes(), 2);
}

#[test]
fn make_vertex_shape_type_is_vertex() {
    let mut b = BrepTopoBuilder::new();
    let id = b.make_vertex("pt");
    assert_eq!(b.shape(id).unwrap().shape_type(), BrepTopoShapeType::Vertex);
}

#[test]
fn make_vertex_label_stored() {
    let mut b = BrepTopoBuilder::new();
    let id = b.make_vertex("corner");
    assert_eq!(b.shape(id).unwrap().label(), "corner");
}

#[test]
fn make_vertex_has_no_children() {
    let mut b = BrepTopoBuilder::new();
    let id = b.make_vertex("v");
    assert!(b.shape(id).unwrap().children().is_empty());
}

// ---------------------------------------------------------------------------
// BrepTopoBuilder — make_edge
// ---------------------------------------------------------------------------

#[test]
fn make_edge_shape_type_is_edge() {
    let mut b = BrepTopoBuilder::new();
    let v0 = b.make_vertex("v0");
    let v1 = b.make_vertex("v1");
    let e = b.make_edge(v0, v1);
    assert_eq!(b.shape(e).unwrap().shape_type(), BrepTopoShapeType::Edge);
}

#[test]
fn make_edge_children_are_the_two_vertices() {
    let mut b = BrepTopoBuilder::new();
    let v0 = b.make_vertex("v0");
    let v1 = b.make_vertex("v1");
    let e = b.make_edge(v0, v1);
    assert_eq!(b.shape(e).unwrap().children(), &[v0, v1]);
}

#[test]
fn make_edge_id_follows_vertices() {
    let mut b = BrepTopoBuilder::new();
    let v0 = b.make_vertex("v0");
    let v1 = b.make_vertex("v1");
    let e = b.make_edge(v0, v1);
    assert_eq!(e, 2);
}

// ---------------------------------------------------------------------------
// BrepTopoBuilder — make_wire
// ---------------------------------------------------------------------------

#[test]
fn make_wire_shape_type_is_wire() {
    let mut b = BrepTopoBuilder::new();
    let v0 = b.make_vertex("v0");
    let v1 = b.make_vertex("v1");
    let e = b.make_edge(v0, v1);
    let w = b.make_wire(&[e]);
    assert_eq!(b.shape(w).unwrap().shape_type(), BrepTopoShapeType::Wire);
}

#[test]
fn make_wire_children_match_edges_slice() {
    let mut b = BrepTopoBuilder::new();
    let v0 = b.make_vertex("v0");
    let v1 = b.make_vertex("v1");
    let v2 = b.make_vertex("v2");
    let e0 = b.make_edge(v0, v1);
    let e1 = b.make_edge(v1, v2);
    let e2 = b.make_edge(v2, v0);
    let w = b.make_wire(&[e0, e1, e2]);
    assert_eq!(b.shape(w).unwrap().children(), &[e0, e1, e2]);
}

#[test]
fn make_wire_empty_edges_has_no_children() {
    let mut b = BrepTopoBuilder::new();
    let w = b.make_wire(&[]);
    assert!(b.shape(w).unwrap().children().is_empty());
}

// ---------------------------------------------------------------------------
// BrepTopoBuilder — make_face
// ---------------------------------------------------------------------------

#[test]
fn make_face_shape_type_is_face() {
    let mut b = BrepTopoBuilder::new();
    let v0 = b.make_vertex("v0");
    let v1 = b.make_vertex("v1");
    let e = b.make_edge(v0, v1);
    let w = b.make_wire(&[e]);
    let f = b.make_face(w);
    assert_eq!(b.shape(f).unwrap().shape_type(), BrepTopoShapeType::Face);
}

#[test]
fn make_face_child_is_wire() {
    let mut b = BrepTopoBuilder::new();
    let v0 = b.make_vertex("v0");
    let v1 = b.make_vertex("v1");
    let e = b.make_edge(v0, v1);
    let w = b.make_wire(&[e]);
    let f = b.make_face(w);
    assert_eq!(b.shape(f).unwrap().children(), &[w]);
}

// ---------------------------------------------------------------------------
// BrepTopoBuilder — make_shell
// ---------------------------------------------------------------------------

#[test]
fn make_shell_shape_type_is_shell() {
    let mut b = BrepTopoBuilder::new();
    let v0 = b.make_vertex("v0");
    let v1 = b.make_vertex("v1");
    let e = b.make_edge(v0, v1);
    let w = b.make_wire(&[e]);
    let f = b.make_face(w);
    let sh = b.make_shell(&[f]);
    assert_eq!(b.shape(sh).unwrap().shape_type(), BrepTopoShapeType::Shell);
}

#[test]
fn make_shell_children_match_faces_slice() {
    let mut b = BrepTopoBuilder::new();
    let v0 = b.make_vertex("v0");
    let v1 = b.make_vertex("v1");
    let e = b.make_edge(v0, v1);
    let w = b.make_wire(&[e]);
    let f0 = b.make_face(w);
    let f1 = b.make_face(w);
    let f2 = b.make_face(w);
    let sh = b.make_shell(&[f0, f1, f2]);
    assert_eq!(b.shape(sh).unwrap().children(), &[f0, f1, f2]);
}

// ---------------------------------------------------------------------------
// BrepTopoBuilder — make_solid
// ---------------------------------------------------------------------------

#[test]
fn make_solid_shape_type_is_solid() {
    let mut b = BrepTopoBuilder::new();
    let v0 = b.make_vertex("v0");
    let v1 = b.make_vertex("v1");
    let e = b.make_edge(v0, v1);
    let w = b.make_wire(&[e]);
    let f = b.make_face(w);
    let sh = b.make_shell(&[f]);
    let sol = b.make_solid(sh);
    assert_eq!(b.shape(sol).unwrap().shape_type(), BrepTopoShapeType::Solid);
}

#[test]
fn make_solid_child_is_shell() {
    let mut b = BrepTopoBuilder::new();
    let v0 = b.make_vertex("v0");
    let v1 = b.make_vertex("v1");
    let e = b.make_edge(v0, v1);
    let w = b.make_wire(&[e]);
    let f = b.make_face(w);
    let sh = b.make_shell(&[f]);
    let sol = b.make_solid(sh);
    assert_eq!(b.shape(sol).unwrap().children(), &[sh]);
}

// ---------------------------------------------------------------------------
// BrepTopoBuilder — full topology chain
// ---------------------------------------------------------------------------

#[test]
fn full_chain_nb_shapes_counts_all_nodes() {
    let mut b = BrepTopoBuilder::new();
    // 3 vertices + 3 edges + 1 wire + 1 face + 1 shell + 1 solid = 10
    let v0 = b.make_vertex("v0");
    let v1 = b.make_vertex("v1");
    let v2 = b.make_vertex("v2");
    let e0 = b.make_edge(v0, v1);
    let e1 = b.make_edge(v1, v2);
    let e2 = b.make_edge(v2, v0);
    let w = b.make_wire(&[e0, e1, e2]);
    let f = b.make_face(w);
    let sh = b.make_shell(&[f]);
    let _sol = b.make_solid(sh);
    assert_eq!(b.nb_shapes(), 10);
}

#[test]
fn full_chain_shape_ids_are_contiguous() {
    let mut b = BrepTopoBuilder::new();
    let v0 = b.make_vertex("v0");
    let v1 = b.make_vertex("v1");
    let e = b.make_edge(v0, v1);
    let w = b.make_wire(&[e]);
    let f = b.make_face(w);
    let sh = b.make_shell(&[f]);
    let sol = b.make_solid(sh);
    // ids should be 0,1,2,3,4,5,6
    assert_eq!(v0, 0);
    assert_eq!(v1, 1);
    assert_eq!(e, 2);
    assert_eq!(w, 3);
    assert_eq!(f, 4);
    assert_eq!(sh, 5);
    assert_eq!(sol, 6);
}

#[test]
fn shape_returns_none_for_out_of_range_id() {
    let mut b = BrepTopoBuilder::new();
    b.make_vertex("v");
    assert!(b.shape(1).is_none());
    assert!(b.shape(999).is_none());
}
