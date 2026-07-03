use ironstream::rw_stl::*;

#[test]
fn reader_success_stl_path_100_triangles() {
    let mut r = StlReader::new();
    assert!(r.read("model.stl", 1));
    assert!(r.is_done());
    assert_eq!(r.nb_triangles(), 100);
}

#[test]
fn reader_failure_obj_path() {
    let mut r = StlReader::new();
    assert!(!r.read("model.obj", 1));
    assert!(!r.is_done());
    assert_eq!(r.nb_triangles(), 0);
}

#[test]
fn writer_write_mesh_id_gt_0_and_path_returns_true() {
    let mut w = StlWriter::new();
    assert!(w.write("output.stl", 1));
    assert!(w.is_done());
}

#[test]
fn writer_fail_empty_path() {
    let mut w = StlWriter::new();
    assert!(!w.write("", 1));
    assert!(!w.is_done());
}

#[test]
fn stl_file_info_probe_stl_nb_triangles_gt_0_is_binary() {
    let info = StlFileInfo::probe("part.stl");
    assert!(info.nb_triangles_hint() > 0);
    assert!(info.is_binary());
}
