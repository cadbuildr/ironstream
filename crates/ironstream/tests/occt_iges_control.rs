// FILE: tests/occt_iges_control.rs
extern crate ironstream;
use ironstream::iges_control::{
    IgesControlReader, IgesControlWriter, IgesReadStatus, IgesWriteMode,
};

#[test]
fn reader_default_state_not_loaded() {
    let r = IgesControlReader::new();
    assert!(!r.is_loaded());
    assert!(r.filename().is_none());
    assert_eq!(r.nb_roots_for_transfer(), 0);
    assert_eq!(r.nb_failed_entities(), 0);
}

#[test]
fn reader_read_file_returns_done_status() {
    let mut r = IgesControlReader::new();
    let status = r.read_file("bracket.igs");
    assert_eq!(status, IgesReadStatus::Done);
}

#[test]
fn reader_loaded_flag_set_after_read() {
    let mut r = IgesControlReader::new();
    r.read_file("bolt.igs");
    assert!(r.is_loaded());
}

#[test]
fn reader_filename_matches_path_passed_in() {
    let mut r = IgesControlReader::new();
    r.read_file("gear.igs");
    assert_eq!(r.filename(), Some("gear.igs"));
}

#[test]
fn reader_transfer_roots_after_read() {
    let mut r = IgesControlReader::new();
    r.read_file("part.igs");
    let n = r.transfer_roots();
    assert_eq!(n, r.nb_roots_for_transfer());
}

#[test]
fn reader_clear_shapes_resets_roots() {
    let mut r = IgesControlReader::new();
    r.read_file("part.igs");
    r.clear_shapes();
    assert_eq!(r.nb_roots_for_transfer(), 0);
}

#[test]
fn reader_no_failed_entities_on_stub_read() {
    let mut r = IgesControlReader::new();
    r.read_file("assembly.igs");
    assert_eq!(r.nb_failed_entities(), 0);
}

#[test]
fn writer_default_mode_is_brep() {
    let w = IgesControlWriter::new();
    assert_eq!(w.write_mode(), IgesWriteMode::BRep);
}

#[test]
fn writer_new_with_mode_brewithcurves() {
    let w = IgesControlWriter::new_with_mode(IgesWriteMode::BrewithCurves);
    assert_eq!(w.write_mode(), IgesWriteMode::BrewithCurves);
}

#[test]
fn writer_new_with_mode_nobrep() {
    let w = IgesControlWriter::new_with_mode(IgesWriteMode::NoBrep);
    assert_eq!(w.write_mode(), IgesWriteMode::NoBrep);
}

#[test]
fn writer_add_shape_increments_counter() {
    let mut w = IgesControlWriter::new();
    w.add_shape();
    w.add_shape();
    w.add_shape();
    assert_eq!(w.nb_shapes(), 3);
}

#[test]
fn writer_write_stub_returns_true() {
    let mut w = IgesControlWriter::new_with_mode(IgesWriteMode::BRep);
    w.add_shape();
    assert!(w.write("result.igs"));
}

#[test]
fn read_status_debug_format() {
    assert_eq!(format!("{:?}", IgesReadStatus::Done),  "Done");
    assert_eq!(format!("{:?}", IgesReadStatus::Error), "Error");
    assert_eq!(format!("{:?}", IgesReadStatus::Void),  "Void");
    assert_eq!(format!("{:?}", IgesReadStatus::Fail),  "Fail");
}

#[test]
fn write_mode_debug_format() {
    assert_eq!(format!("{:?}", IgesWriteMode::BRep),          "BRep");
    assert_eq!(format!("{:?}", IgesWriteMode::BrewithCurves), "BrewithCurves");
    assert_eq!(format!("{:?}", IgesWriteMode::NoBrep),        "NoBrep");
}

#[test]
fn reader_writer_stub_pipeline() {
    let mut w = IgesControlWriter::new_with_mode(IgesWriteMode::BRep);
    w.add_shape();
    let wrote = w.write("pipeline.igs");
    assert!(wrote);

    let mut r = IgesControlReader::new();
    let status = r.read_file("pipeline.igs");
    assert_eq!(status, IgesReadStatus::Done);
    assert!(r.is_loaded());
    let transferred = r.transfer_roots();
    assert_eq!(transferred, r.nb_roots_for_transfer());
}
