extern crate ironstream;
use ironstream::rw_obj_writer::*;

#[test]
fn test_rw_obj_writer_basic() {
    let nodes: Vec<[f64; 3]> = vec![
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];
    let tris: Vec<[u32; 3]> = vec![[0, 1, 2], [0, 1, 3]];

    // ObjWriter basic write
    let out = "/tmp/occt_rw_obj_test.obj";
    let w = ObjWriter::new(out);
    assert_eq!(w.coordinate_system, 0);
    w.write(&nodes, &tris, None).unwrap();

    // with coord system
    let w2 = ObjWriter::new(out).with_coord_system(1);
    assert_eq!(w2.coordinate_system, 1);

    // write with normals
    let normals: Vec<[f64; 3]> = vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];
    ObjWriter::new(out).write(&nodes, &tris, Some(&normals)).unwrap();

    // write_with_materials
    ObjWriter::new(out)
        .write_with_materials(&nodes, &tris, "materials.mtl")
        .unwrap();

    // MtlWriter
    let mtl_out = "/tmp/occt_rw_obj_test.mtl";
    let mw = MtlWriter::new(mtl_out);
    mw.write_material("Gold", [1.0, 0.84, 0.0], [1.0, 1.0, 1.0], 64.0).unwrap();

    // free function
    write_obj(&nodes, &tris, "/tmp/occt_rw_obj_free.obj").unwrap();
}
