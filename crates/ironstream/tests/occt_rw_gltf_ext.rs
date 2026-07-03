// Integration tests for rw_gltf_ext module
extern crate ironstream;

use ironstream::rw_gltf_ext::*;

#[test]
fn gltf_status_predicates() {
    assert!(GltfStatus::Ok.is_ok());
    assert!(!GltfStatus::FileNotFound.is_ok());
    assert!(!GltfStatus::ParseError.is_ok());
    assert!(!GltfStatus::WriteError.is_ok());
    assert!(!GltfStatus::NoMesh.is_ok());
    assert!(!GltfStatus::InvalidJson.is_ok());
    assert!(!GltfStatus::UnsupportedVersion.is_ok());
    let s = GltfStatus::default();
    assert_eq!(s, GltfStatus::Ok);
}

#[test]
fn gltf_primitive_construction_and_accessors() {
    let mut p = GltfPrimitive::new();
    assert_eq!(p.material_idx, -1);
    assert!(p.is_empty());
    assert_eq!(p.nb_positions(), 0);
    assert_eq!(p.nb_indices(), 0);

    p.positions.push([0.0, 0.0, 0.0]);
    p.positions.push([1.0, 0.0, 0.0]);
    p.positions.push([0.0, 1.0, 0.0]);
    p.indices.extend_from_slice(&[0, 1, 2]);
    p.normals.push([0.0, 0.0, 1.0]);

    assert!(!p.is_empty());
    assert_eq!(p.nb_positions(), 3);
    assert_eq!(p.nb_indices(), 3);
    assert_eq!(p.normals.len(), 1);
}

#[test]
fn gltf_node_construction() {
    let mut n = GltfNode::new("root");
    assert_eq!(n.name, "root");
    assert_eq!(n.mesh_idx, -1);
    assert!(!n.has_mesh());
    assert_eq!(n.children.len(), 0);

    n.mesh_idx = 0;
    assert!(n.has_mesh());
    n.add_child(1);
    n.add_child(2);
    assert_eq!(n.children.len(), 2);
    // default rotation is unit quaternion
    assert!((n.rotation[3] - 1.0).abs() < 1e-6);
    // default scale is 1
    assert!((n.scale[0] - 1.0).abs() < 1e-6);
}

#[test]
fn gltf_material_construction_and_color() {
    let mut mat = GltfMaterial::new("Metal");
    assert_eq!(mat.name, "Metal");
    assert_eq!(mat.base_color, [1.0, 1.0, 1.0, 1.0]);
    assert!((mat.roughness_factor - 0.5).abs() < 1e-6);
    assert!(!mat.is_transparent());

    mat.set_base_color(0.8, 0.2, 0.0, 0.5);
    assert_eq!(mat.base_color, [0.8, 0.2, 0.0, 0.5]);

    mat.alpha_mode = 1;
    assert!(mat.is_transparent());
}

#[test]
fn gltf_reader_read_file_stub() {
    let mut r = RWGltfReader::new();
    assert!(r.read_normals);
    assert!(r.read_colors);
    assert!(r.read_uvs);
    assert_eq!(r.memory_limit, 512 * 1024 * 1024);

    let st = r.read_file("model.gltf");
    assert!(st.is_ok());
    assert!(r.is_done());
    assert_eq!(r.nb_primitives(), 1);
    assert_eq!(r.nb_nodes(), 1);
    assert_eq!(r.nb_materials(), 0);

    let prim = &r.primitives[0];
    assert_eq!(prim.nb_positions(), 3);
    assert_eq!(prim.nb_indices(), 3);
    assert!(!prim.is_empty());

    let node = &r.nodes[0];
    assert!(node.has_mesh());
    assert_eq!(r.root_nodes[0], 0);

    // settings
    r.set_read_normals(false);
    assert!(!r.read_normals);
    r.set_read_colors(false);
    r.set_memory_limit(1024);
    assert_eq!(r.memory_limit, 1024);
}

#[test]
fn gltf_writer_write_and_no_mesh() {
    let mut w = RWGltfWriter::new();
    w.set_binary(true);
    assert!(w.use_binary);
    w.set_force_uvs(true);
    assert!(w.force_uvs);

    // write with empty primitives → NoMesh
    let st = w.write("out.glb", &[]);
    assert_eq!(st, GltfStatus::NoMesh);
    assert!(!w.is_done());

    // write with data
    let mut prim = GltfPrimitive::new();
    prim.positions.push([0.0, 0.0, 0.0]);
    let st2 = w.write("out.glb", &[prim]);
    assert!(st2.is_ok());
    assert!(w.is_done());
    assert_eq!(w.nb_nodes_written(), 1);

    // write multiple primitives
    let p1 = GltfPrimitive::new();
    let mut p2 = GltfPrimitive::new();
    p2.positions.push([1.0, 0.0, 0.0]);
    // empty primitive counts as node in stub
    let st3 = w.write("out2.glb", &[p1, p2]);
    assert!(st3.is_ok());
    assert_eq!(w.nb_nodes_written(), 2);
}
