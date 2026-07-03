// FILE: tests/occt_imesh_complex.rs
extern crate ironstream;
use ironstream::imesh_complex::*;

#[test]
fn mesh_vertex_on_boundary() {
    let v = MeshVertex::new(0, [0.0, 0.0], [0.0, 0.0, 0.0]).on_boundary();
    assert!(v.is_on_boundary);
    let v2 = MeshVertex::new(1, [1.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(!v2.is_on_boundary);
}

#[test]
fn mesh_edge_reversed() {
    let e = MeshEdgeDef::new(0, 1);
    let rev = e.reversed();
    assert_eq!(rev.v0, 1);
    assert_eq!(rev.v1, 0);
}

#[test]
fn mesh_triangle_contains_vertex() {
    let t = MeshTriangle::new([0, 1, 2], [10, 11, 12]);
    assert!(t.contains_vertex(0));
    assert!(t.contains_vertex(2));
    assert!(!t.contains_vertex(5));
}

#[test]
fn mesh_face_add_vertex_edge_triangle() {
    let mut f = IMeshFace::new(1);
    let v0 = f.add_vertex(MeshVertex::new(0, [0.0, 0.0], [0.0, 0.0, 0.0]).on_boundary());
    let v1 = f.add_vertex(MeshVertex::new(1, [1.0, 0.0], [1.0, 0.0, 0.0]).on_boundary());
    let v2 = f.add_vertex(MeshVertex::new(2, [0.5, 1.0], [0.5, 1.0, 0.0]));
    let e0 = f.add_edge(MeshEdgeDef::new(v0, v1).boundary());
    let e1 = f.add_edge(MeshEdgeDef::new(v1, v2));
    let e2 = f.add_edge(MeshEdgeDef::new(v2, v0));
    f.add_triangle(MeshTriangle::new([v0, v1, v2], [e0, e1, e2]));
    f.is_done = true;
    assert_eq!(f.nb_vertices(), 3);
    assert_eq!(f.nb_triangles(), 1);
    assert_eq!(f.boundary_vertices().len(), 2);
    assert_eq!(f.boundary_edges().len(), 1);
}

#[test]
fn imesh_model_totals() {
    let mut model = IMeshModel::new();
    let mut face = IMeshFace::new(0);
    face.add_vertex(MeshVertex::new(0, [0.0, 0.0], [0.0; 3]));
    face.add_vertex(MeshVertex::new(1, [1.0, 0.0], [1.0, 0.0, 0.0]));
    face.add_vertex(MeshVertex::new(2, [0.0, 1.0], [0.0, 1.0, 0.0]));
    face.add_triangle(MeshTriangle::new([0, 1, 2], [0, 1, 2]));
    face.is_done = true;
    model.add_face(face);
    assert_eq!(model.total_vertices(), 3);
    assert_eq!(model.total_triangles(), 1);
    assert!(model.is_done());
}
