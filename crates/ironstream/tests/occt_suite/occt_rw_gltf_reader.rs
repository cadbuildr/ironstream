extern crate ironstream;

use ironstream::rw_gltf_reader::{GltfAccessor, GltfAccessorType, GltfNode, GltfReader};

#[test]
fn gltf_accessor_type_nb_components() {
    assert_eq!(GltfAccessorType::Scalar.nb_components(), 1);
    assert_eq!(GltfAccessorType::Vec2.nb_components(), 2);
    assert_eq!(GltfAccessorType::Vec3.nb_components(), 3);
    assert_eq!(GltfAccessorType::Mat4.nb_components(), 16);
}

#[test]
fn gltf_accessor_new() {
    let acc = GltfAccessor::new(0, 100, GltfAccessorType::Vec3);
    assert_eq!(acc.count, 100);
    assert_eq!(acc.component_type, 5126); // GL_FLOAT
    assert_eq!(acc.byte_offset, 0);
}

#[test]
fn gltf_node_default() {
    let n = GltfNode::new("Root");
    assert_eq!(n.name, "Root");
    assert!(n.mesh_index.is_none());
    assert!(n.children.is_empty());
    assert_eq!(n.rotation, [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(n.scale, [1.0, 1.0, 1.0]);
}

#[test]
fn gltf_reader_stub_read_file() {
    let mut reader = GltfReader::new();
    assert!(!reader.is_done());
    assert!(!reader.read_file("nonexistent.gltf"));
    assert_eq!(reader.nb_errors, 1);
    assert_eq!(reader.nb_nodes(), 0);
    assert_eq!(reader.nb_meshes(), 0);
}

#[test]
fn gltf_reader_multiple_read_increments_errors() {
    let mut reader = GltfReader::new();
    reader.read_file("a.gltf");
    reader.read_file("b.gltf");
    assert_eq!(reader.nb_errors, 2);
}
