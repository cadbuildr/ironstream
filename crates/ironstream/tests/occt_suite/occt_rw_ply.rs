use ironstream::rw_ply::*;

#[test]
fn ply_status_default_is_ok() {
    assert_eq!(PlyStatus::default(), PlyStatus::Ok);
    assert!(PlyStatus::Ok.is_ok());
    assert!(!PlyStatus::ParseError.is_ok());
}

#[test]
fn ply_property_scalar_vs_list() {
    let s = PlyProperty::scalar("x", "float");
    assert!(!s.is_list);
    assert_eq!(s.name, "x");
    let l = PlyProperty::list("vertex_indices", "uint");
    assert!(l.is_list);
    assert_eq!(l.type_name, "uint");
}

#[test]
fn ply_mesh_normals_colors_uvs() {
    let mut m = PlyMesh::new();
    m.add_vertex([1.0, 0.0, 0.0]);
    m.add_normal([0.0, 1.0, 0.0]);
    m.add_color([255, 0, 0, 255]);
    m.add_uv([0.5, 0.5]);
    m.add_triangle([0, 0, 0]);
    assert!(m.has_normals);
    assert!(m.has_colors);
    assert!(m.has_uvs);
    assert_eq!(m.nb_vertices(), 1);
    assert_eq!(m.nb_triangles(), 1);
}

#[test]
fn ply_reader_read_string_nonempty() {
    let mut r = RWPlyReader::new();
    let st = r.read_string("ply data");
    assert!(st.is_ok());
    assert!(r.is_done());
    assert!(r.mesh().is_some());
}

#[test]
fn ply_writer_write_mesh_roundtrip() {
    let mut mesh = PlyMesh::new();
    mesh.add_vertex([0.0, 0.0, 0.0]);
    mesh.add_vertex([2.0, 0.0, 0.0]);
    mesh.add_vertex([0.0, 2.0, 0.0]);
    mesh.add_triangle([0, 1, 2]);
    let w = RWPlyWriter::new();
    let output = w.write_string(&mesh);
    assert!(output.contains("element vertex 3"));
    assert!(output.contains("3 0 1 2"));
}

#[test]
fn ply_writer_write_file_ok() {
    let mut mesh = PlyMesh::new();
    mesh.add_vertex([0.0, 0.0, 0.0]);
    mesh.add_vertex([1.0, 0.0, 0.0]);
    mesh.add_vertex([0.0, 1.0, 0.0]);
    mesh.add_triangle([0, 1, 2]);
    let mut w = RWPlyWriter::new();
    w.set_write_normals(false);
    let st = w.write_file(&mesh, "/tmp/test.ply");
    assert!(st.is_ok());
    assert!(w.is_done());
}
