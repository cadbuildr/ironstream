// FILE: tests/occt_step_control.rs
extern crate ironstream;
use ironstream::step_control::{
    StepControlReader, StepControlWriter, StepModelType, StepReadStatus, StepWriteStatus,
};

#[test]
fn reader_initial_state_not_loaded() {
    let r = StepControlReader::new();
    assert!(!r.is_loaded());
    assert_eq!(r.filename(), None);
    assert_eq!(r.nb_shapes(), 0);
}

#[test]
fn reader_read_file_returns_done() {
    let mut r = StepControlReader::new();
    let status = r.read_file("component.step");
    assert_eq!(status, StepReadStatus::Done);
}

#[test]
fn reader_read_file_sets_loaded_and_filename() {
    let mut r = StepControlReader::new();
    r.read_file("assembly.stp");
    assert!(r.is_loaded());
    assert_eq!(r.filename(), Some("assembly.stp"));
}

#[test]
fn reader_transfer_roots_after_read() {
    let mut r = StepControlReader::new();
    r.read_file("shape.step");
    assert_eq!(r.transfer_roots(), 1);
}

#[test]
fn reader_clear_resets_to_initial() {
    let mut r = StepControlReader::new();
    r.read_file("model.step");
    r.clear();
    assert!(!r.is_loaded());
    assert_eq!(r.filename(), None);
    assert_eq!(r.nb_shapes(), 0);
}

#[test]
fn reader_transfer_roots_after_clear_is_zero() {
    let mut r = StepControlReader::new();
    r.read_file("model.step");
    r.clear();
    assert_eq!(r.transfer_roots(), 0);
}

#[test]
fn writer_initial_model_type_is_as_is() {
    let w = StepControlWriter::new();
    assert_eq!(w.model_type(), StepModelType::AsIs);
}

#[test]
fn writer_set_model_type_manifold() {
    let mut w = StepControlWriter::new();
    w.set_model_type(StepModelType::ManifoldSolidBrep);
    assert_eq!(w.model_type(), StepModelType::ManifoldSolidBrep);
}

#[test]
fn writer_set_model_type_shell_based() {
    let mut w = StepControlWriter::new();
    w.set_model_type(StepModelType::ShellBasedSurfaceModel);
    assert_eq!(w.model_type(), StepModelType::ShellBasedSurfaceModel);
}

#[test]
fn writer_transfer_increments_count() {
    let mut w = StepControlWriter::new();
    assert_eq!(w.nb_shapes(), 0);
    w.transfer();
    assert_eq!(w.nb_shapes(), 1);
    w.transfer();
    assert_eq!(w.nb_shapes(), 2);
}

#[test]
fn writer_transfer_returns_done() {
    let mut w = StepControlWriter::new();
    assert_eq!(w.transfer(), StepWriteStatus::Done);
}

#[test]
fn writer_write_stub_returns_done() {
    let w = StepControlWriter::new();
    assert_eq!(w.write("output.step"), StepWriteStatus::Done);
}

#[test]
fn step_read_status_debug_and_eq() {
    let s = StepReadStatus::Error;
    assert_eq!(format!("{:?}", s), "Error");
    assert_eq!(s, StepReadStatus::Error);
    assert_ne!(s, StepReadStatus::Done);
}

#[test]
fn step_write_status_not_done_variant() {
    let s = StepWriteStatus::NotDone;
    assert_eq!(format!("{:?}", s), "NotDone");
    assert_ne!(s, StepWriteStatus::Done);
    assert_ne!(s, StepWriteStatus::Fail);
}

#[test]
fn model_type_geometric_curve_set_copy() {
    let mt = StepModelType::GeometricCurveSet;
    let mt2 = mt;
    assert_eq!(mt, mt2);
    assert_eq!(format!("{:?}", mt), "GeometricCurveSet");
}
