// FILE: tests/occt_brep_mesh_tools.rs
extern crate ironstream;
use ironstream::brep_mesh_tools::*;

// ---------------------------------------------------------------------------
// MeshVertex tests
// ---------------------------------------------------------------------------

#[test]
fn test_mesh_vertex_new_stores_coordinates() {
    let v = MeshVertex::new([0.25, 0.75], [1.0, 2.0, 3.0]);
    assert_eq!(v.point_2d(), [0.25, 0.75]);
    assert_eq!(v.point_3d(), [1.0, 2.0, 3.0]);
}

#[test]
fn test_mesh_vertex_uv_location_defaults_to_point_2d() {
    let pt2d = [0.1, 0.9];
    let v = MeshVertex::new(pt2d, [0.0, 0.0, 0.0]);
    assert_eq!(v.uv_location(), pt2d);
}

#[test]
fn test_mesh_vertex_fields_are_independent() {
    let v = MeshVertex::new([1.0, 2.0], [3.0, 4.0, 5.0]);
    // point_2d and point_3d hold different dimensional data.
    assert_ne!(v.point_2d()[0], v.point_3d()[0]);
}

#[test]
fn test_mesh_vertex_zero_coordinates() {
    let v = MeshVertex::new([0.0, 0.0], [0.0, 0.0, 0.0]);
    assert_eq!(v.point_2d(), [0.0, 0.0]);
    assert_eq!(v.point_3d(), [0.0, 0.0, 0.0]);
    assert_eq!(v.uv_location(), [0.0, 0.0]);
}

#[test]
fn test_mesh_vertex_negative_coordinates() {
    let v = MeshVertex::new([-1.0, -2.0], [-3.0, -4.0, -5.0]);
    assert_eq!(v.point_2d(), [-1.0, -2.0]);
    assert_eq!(v.point_3d(), [-3.0, -4.0, -5.0]);
}

#[test]
fn test_mesh_vertex_copy() {
    let v = MeshVertex::new([0.5, 0.5], [1.0, 1.0, 1.0]);
    let w = v;
    assert_eq!(v.point_3d(), w.point_3d());
}

// ---------------------------------------------------------------------------
// MeshEdge tests
// ---------------------------------------------------------------------------

#[test]
fn test_mesh_edge_new_sets_endpoints_and_is_free() {
    let e = MeshEdge::new(3, 7);
    assert_eq!(e.v0(), 3);
    assert_eq!(e.v1(), 7);
    assert!(e.is_free(), "new edge must start as free");
}

#[test]
fn test_mesh_edge_set_free_false() {
    let mut e = MeshEdge::new(0, 1);
    e.set_free(false);
    assert!(!e.is_free());
}

#[test]
fn test_mesh_edge_set_free_round_trip() {
    let mut e = MeshEdge::new(0, 1);
    e.set_free(false);
    e.set_free(true);
    assert!(e.is_free());
}

#[test]
fn test_mesh_edge_length_unit_axis_aligned() {
    // Edge along X axis of length 1.
    let verts = [
        MeshVertex::new([0.0, 0.0], [0.0, 0.0, 0.0]),
        MeshVertex::new([1.0, 0.0], [1.0, 0.0, 0.0]),
    ];
    let e = MeshEdge::new(0, 1);
    assert!((e.length(&verts) - 1.0).abs() < 1e-12);
}

#[test]
fn test_mesh_edge_length_diagonal() {
    // Edge from origin to (3, 4, 0) — length should be 5.
    let verts = [
        MeshVertex::new([0.0, 0.0], [0.0, 0.0, 0.0]),
        MeshVertex::new([3.0, 4.0], [3.0, 4.0, 0.0]),
    ];
    let e = MeshEdge::new(0, 1);
    assert!((e.length(&verts) - 5.0).abs() < 1e-12);
}

#[test]
fn test_mesh_edge_length_3d_diagonal() {
    // Edge from origin to (1, 1, 1) — length = sqrt(3).
    let verts = [
        MeshVertex::new([0.0, 0.0], [0.0, 0.0, 0.0]),
        MeshVertex::new([1.0, 1.0], [1.0, 1.0, 1.0]),
    ];
    let e = MeshEdge::new(0, 1);
    let expected = 3f64.sqrt();
    assert!((e.length(&verts) - expected).abs() < 1e-12);
}

#[test]
fn test_mesh_edge_length_zero_for_coincident_vertices() {
    let verts = [
        MeshVertex::new([0.5, 0.5], [2.0, 3.0, 4.0]),
        MeshVertex::new([0.5, 0.5], [2.0, 3.0, 4.0]),
    ];
    let e = MeshEdge::new(0, 1);
    assert!(e.length(&verts) < 1e-12);
}

#[test]
fn test_mesh_edge_uses_3d_coordinates_for_length() {
    // 2-D coords deliberately differ from 3-D; length must use point_3d.
    let verts = [
        MeshVertex::new([0.0, 0.0], [0.0, 0.0, 0.0]),
        MeshVertex::new([999.0, 999.0], [3.0, 0.0, 0.0]),
    ];
    let e = MeshEdge::new(0, 1);
    assert!((e.length(&verts) - 3.0).abs() < 1e-12);
}

// ---------------------------------------------------------------------------
// MeshFaceData tests
// ---------------------------------------------------------------------------

#[test]
fn test_mesh_face_data_new_is_empty_and_not_done() {
    let fd = MeshFaceData::new(0.05);
    assert_eq!(fd.nb_vertices(), 0);
    assert_eq!(fd.nb_edges(), 0);
    assert_eq!(fd.nb_triangles(), 0);
    assert!(!fd.is_done());
    assert!((fd.deflection - 0.05).abs() < 1e-12);
}

#[test]
fn test_mesh_face_data_add_vertex_increments_count() {
    let mut fd = MeshFaceData::new(0.1);
    let i0 = fd.add_vertex(MeshVertex::new([0.0, 0.0], [0.0, 0.0, 0.0]));
    let i1 = fd.add_vertex(MeshVertex::new([1.0, 0.0], [1.0, 0.0, 0.0]));
    assert_eq!(i0, 0);
    assert_eq!(i1, 1);
    assert_eq!(fd.nb_vertices(), 2);
}

#[test]
fn test_mesh_face_data_vertex_accessor_returns_correct_data() {
    let mut fd = MeshFaceData::new(0.1);
    let v = MeshVertex::new([0.3, 0.7], [1.0, 2.0, 3.0]);
    fd.add_vertex(v);
    let retrieved = fd.vertex(0);
    assert_eq!(retrieved.point_2d(), [0.3, 0.7]);
    assert_eq!(retrieved.point_3d(), [1.0, 2.0, 3.0]);
}

#[test]
fn test_mesh_face_data_add_edge_increments_count() {
    let mut fd = MeshFaceData::new(0.1);
    let e0 = fd.add_edge(MeshEdge::new(0, 1));
    let e1 = fd.add_edge(MeshEdge::new(1, 2));
    assert_eq!(e0, 0);
    assert_eq!(e1, 1);
    assert_eq!(fd.nb_edges(), 2);
}

#[test]
fn test_mesh_face_data_add_triangle_increments_count() {
    let mut fd = MeshFaceData::new(0.1);
    fd.add_triangle([0, 1, 2]);
    fd.add_triangle([0, 2, 3]);
    assert_eq!(fd.nb_triangles(), 2);
}

#[test]
fn test_mesh_face_data_triangles_store_correct_indices() {
    let mut fd = MeshFaceData::new(0.1);
    fd.add_triangle([10, 20, 30]);
    assert_eq!(fd.triangles[0], [10, 20, 30]);
}

#[test]
fn test_mesh_face_data_mark_done() {
    let mut fd = MeshFaceData::new(0.1);
    assert!(!fd.is_done());
    fd.mark_done();
    assert!(fd.is_done());
}

#[test]
fn test_mesh_face_data_mark_done_is_idempotent() {
    let mut fd = MeshFaceData::new(0.1);
    fd.mark_done();
    fd.mark_done();
    assert!(fd.is_done());
}

#[test]
fn test_mesh_face_data_full_workflow() {
    let mut fd = MeshFaceData::new(0.01);

    // Add three vertices for a triangle.
    fd.add_vertex(MeshVertex::new([0.0, 0.0], [0.0, 0.0, 0.0]));
    fd.add_vertex(MeshVertex::new([1.0, 0.0], [1.0, 0.0, 0.0]));
    fd.add_vertex(MeshVertex::new([0.5, 1.0], [0.5, 1.0, 0.0]));

    // Add three boundary edges.
    fd.add_edge(MeshEdge::new(0, 1));
    fd.add_edge(MeshEdge::new(1, 2));
    fd.add_edge(MeshEdge::new(2, 0));

    // Add the single triangle.
    fd.add_triangle([0, 1, 2]);

    fd.mark_done();

    assert_eq!(fd.nb_vertices(), 3);
    assert_eq!(fd.nb_edges(), 3);
    assert_eq!(fd.nb_triangles(), 1);
    assert!(fd.is_done());
}

#[test]
fn test_mesh_face_data_edge_length_via_vertex_slice() {
    let mut fd = MeshFaceData::new(0.1);
    fd.add_vertex(MeshVertex::new([0.0, 0.0], [0.0, 0.0, 0.0]));
    fd.add_vertex(MeshVertex::new([4.0, 3.0], [4.0, 3.0, 0.0]));
    let ei = fd.add_edge(MeshEdge::new(0, 1));

    let len = fd.edges[ei].length(&fd.vertices);
    assert!((len - 5.0).abs() < 1e-12, "expected 5.0, got {len}");
}

#[test]
fn test_mesh_face_data_multiple_vertices_retrieval() {
    let mut fd = MeshFaceData::new(0.2);
    let pts = [
        ([0.0, 0.0], [0.0, 0.0, 0.0]),
        ([1.0, 0.0], [1.0, 0.0, 0.0]),
        ([1.0, 1.0], [1.0, 1.0, 0.0]),
        ([0.0, 1.0], [0.0, 1.0, 0.0]),
    ];
    for (p2, p3) in pts.iter() {
        fd.add_vertex(MeshVertex::new(*p2, *p3));
    }
    assert_eq!(fd.nb_vertices(), 4);
    for (i, (p2, p3)) in pts.iter().enumerate() {
        assert_eq!(fd.vertex(i).point_2d(), *p2);
        assert_eq!(fd.vertex(i).point_3d(), *p3);
    }
}

#[test]
fn test_mesh_face_data_free_edge_flag_preserved() {
    let mut fd = MeshFaceData::new(0.1);
    let mut e = MeshEdge::new(0, 1);
    e.set_free(false);
    fd.add_edge(e);
    assert!(!fd.edges[0].is_free());
}
