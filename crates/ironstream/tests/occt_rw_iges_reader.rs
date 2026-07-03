extern crate ironstream;
use ironstream::rw_iges_reader::*;

#[test]
fn test_rw_iges_reader_basic() {
    let mut reader = IgesReader::new();
    assert_eq!(reader.root_count(), 0);

    let result = reader.read_file("model.igs");
    assert!(result.is_ok());
    assert_eq!(reader.root_count(), 1);

    let n = reader.transfer_roots();
    assert!(n >= 1);

    let shape = reader.one_shape();
    assert!(!shape.is_empty());

    reader.clear();
    assert_eq!(reader.root_count(), 0);

    let converter = IgesToBrep::new(1e-4);
    let surf = converter.transfer_surface("face_1");
    assert!(surf.contains("brep_surface"));
    let curve = converter.transfer_curve("edge_1");
    assert!(curve.contains("brep_curve"));

    let shapes = read_iges("model.iges");
    assert!(shapes.is_ok());
}
