use ironstream::prs3d_shape::*;

#[test]
fn brep_mesh_params_defaults() {
    let p = BrepMeshParams::default();
    assert!(p.in_relative);
    assert!(!p.is_parallel);
    assert!((p.linear_deflection - 0.001).abs() < 1e-14);
    assert!(p.internal_vertices_mode);
}

#[test]
fn brep_mesh_params_builder() {
    let p = BrepMeshParams::new(0.005)
        .with_angular(0.1)
        .with_relative(false)
        .with_parallel(true);
    assert!(!p.in_relative);
    assert!(p.is_parallel);
    assert!((p.angular_deflection - 0.1).abs() < 1e-14);
    assert!((p.linear_deflection - 0.005).abs() < 1e-14);
}

#[test]
fn incremental_mesh_perform_nb_faces() {
    let params = BrepMeshParams::new(0.01);
    let mut m = BrepMeshIncrementalMesh::new(7, params);
    assert!(!m.is_done());
    m.perform();
    assert!(m.is_done());
    assert_eq!(m.nb_faces_meshed(), 7);
}

#[test]
fn poly_triangulation_add_nodes_and_triangles() {
    let mut t = PolyTriangulation::new();
    let a = t.add_node([0.0, 0.0, 0.0]);
    let b = t.add_node([1.0, 0.0, 0.0]);
    let c = t.add_node([0.0, 1.0, 0.0]);
    t.add_triangle(MeshTriangle::new(a, b, c));
    assert_eq!(t.nb_nodes(), 3);
    assert_eq!(t.nb_triangles(), 1);
    assert_eq!(t.triangle(0).unwrap().indices, [0, 1, 2]);
    assert_eq!(t.node(1).unwrap(), [1.0, 0.0, 0.0]);
}

#[test]
fn poly_triangulation_normals() {
    let mut t = PolyTriangulation::new();
    assert!(!t.has_normals());
    t.add_node([0.0; 3]);
    t.set_normal(0, [0.0, 0.0, 1.0]);
    assert!(t.has_normals());
    assert_eq!(t.normals[0], [0.0, 0.0, 1.0]);
}

#[test]
fn std_prs_stubs() {
    let mut t = PolyTriangulation::new();
    let a = t.add_node([0.0; 3]);
    let b = t.add_node([1.0, 0.0, 0.0]);
    let c = t.add_node([0.0, 1.0, 0.0]);
    t.add_triangle(MeshTriangle::new(a, b, c));
    assert_eq!(StdPrsShadedShape::add(&t), 1);
    assert_eq!(StdPrsWFShape::add(&t), 1); // 3 nodes / 2 = 1
    let pt = StdPrsVertex::add([1.0, 2.0, 3.0], 0.5);
    assert_eq!(pt, [1.0, 2.0, 3.0]);
}
