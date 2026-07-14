extern crate ironstream;
use ironstream::rw_gltf_writer::*;

#[test]
fn test_rw_gltf_writer_basic() {
    // GltfWriteParams
    let params = GltfWriteParams::new();
    assert!(!params.transform_from_occt);
    assert!(!params.merge_faces);
    assert!(!params.split_by_material);

    let _default_params = GltfWriteParams::default();

    // GltfWriter - JSON format
    let w = GltfWriter::new("/tmp/test.gltf");
    assert_eq!(w.format, GltfFormat::Gltf);
    assert_eq!(w.file_path, "/tmp/test.gltf");

    // GltfWriter - GLB format
    let wb = GltfWriter::binary("/tmp/test.glb");
    assert_eq!(wb.format, GltfFormat::Glb);

    // write shapes to a temp path
    let out = "/tmp/occt_rw_gltf_test.gltf";
    let writer = GltfWriter::new(out);
    writer.write(&["shape1".to_string(), "shape2".to_string()]).unwrap();

    // write_gltf free function
    let out2 = "/tmp/occt_rw_gltf_test2.gltf";
    write_gltf(&["shapeA".to_string()], out2).unwrap();

    // GltfFormat variants
    assert_ne!(GltfFormat::Glb, GltfFormat::Gltf);
}
