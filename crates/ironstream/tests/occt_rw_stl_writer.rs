extern crate ironstream;
use ironstream::rw_stl_writer::*;

#[test]
fn test_rw_stl_writer_basic() {
    let nodes: Vec<[f64; 3]> = vec![
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];
    let tris: Vec<[u32; 3]> = vec![[0, 1, 2], [0, 1, 3]];
    let normals: Vec<[f64; 3]> = vec![
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];

    // StlWriter - binary
    let w = StlWriter::new();
    assert!(w.binary);
    let out = "/tmp/occt_rw_stl_test.stl";
    w.write(&nodes, &tris, Some(&normals), out).unwrap();

    // StlWriter - ascii
    let wa = StlWriter::ascii();
    assert!(!wa.binary);
    wa.write(&nodes, &tris, Some(&normals), "/tmp/occt_rw_stl_ascii.stl").unwrap();

    // to_bytes (static method)
    let bytes = StlWriter::to_bytes(&nodes, &tris);
    assert!(!bytes.is_empty());
    // binary STL header is 80 bytes + 4 byte count
    assert!(bytes.len() >= 84);

    // StlReader round-trip
    let mut reader = StlReader::new();
    reader.read_file(out).unwrap();
    assert_eq!(reader.triangle_count(), 2);

    // write_stl_binary free function
    write_stl_binary(&nodes, &tris, "/tmp/occt_rw_stl_free.stl").unwrap();
}
