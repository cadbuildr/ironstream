// FILE: tests/occt_rw_step.rs
//! Integration tests for `ironstream::rw_step` — the STEP I/O framework
//! mirroring OpenCascade's `STEPControl` package.
//!
//! These tests exercise the public API from an external-crate perspective,
//! verifying that `STEPControl_Reader`, `STEPControl_Writer`, and
//! `STEPControl_StepModelType` behave consistently end-to-end.

extern crate ironstream;

use ironstream::rw_step::{
    IFSelect_ReturnStatus, STEPControl_Reader, STEPControl_StepModelType, STEPControl_Writer,
    StepEntity, StepModel,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a minimal but structurally valid ISO-10303-21 string that contains
/// two `CARTESIAN_POINT` entities and one `CLOSED_SHELL` root.
fn make_step_string() -> String {
    [
        "ISO-10303-21;",
        "HEADER;",
        "FILE_DESCRIPTION(('integration test'),'2;1');",
        "FILE_NAME('model.step','',('author'),('org'),'IronStream','IronStream','');",
        "FILE_SCHEMA(('CONFIG_CONTROL_DESIGN'));",
        "ENDSEC;",
        "DATA;",
        "#1 = CARTESIAN_POINT('origin',(0.000000,0.000000,0.000000));",
        "#2 = CARTESIAN_POINT('p_apex',(0.000000,0.000000,10.000000));",
        "#3 = CARTESIAN_POINT('p_base',(5.000000,0.000000,0.000000));",
        "#4 = CLOSED_SHELL('outer_shell',(#1,#2,#3));",
        "ENDSEC;",
        "END-ISO-10303-21;",
    ]
    .join("\n")
}

/// Build a STEP string with no DATA section at all.
fn make_step_no_data() -> String {
    [
        "ISO-10303-21;",
        "HEADER;",
        "FILE_DESCRIPTION(('empty'),'2;1');",
        "FILE_NAME('empty.step','',(''),(''),'','','');",
        "FILE_SCHEMA(('CONFIG_CONTROL_DESIGN'));",
        "ENDSEC;",
        "DATA;",
        "ENDSEC;",
        "END-ISO-10303-21;",
    ]
    .join("\n")
}

// ---------------------------------------------------------------------------
// STEPControl_StepModelType
// ---------------------------------------------------------------------------

#[test]
fn model_type_all_variants_have_display() {
    let variants = [
        STEPControl_StepModelType::FacetedBrep,
        STEPControl_StepModelType::ShellBasedSurfaceModel,
        STEPControl_StepModelType::ManifoldSolidBrep,
        STEPControl_StepModelType::GeometricCurveSet,
        STEPControl_StepModelType::AsIs,
    ];
    for v in &variants {
        let s = v.to_string();
        assert!(!s.is_empty(), "variant {:?} produced empty display string", v);
    }
}

#[test]
fn model_type_default_is_as_is() {
    let t: STEPControl_StepModelType = Default::default();
    assert_eq!(t, STEPControl_StepModelType::AsIs);
}

#[test]
fn model_type_clone_and_eq() {
    let a = STEPControl_StepModelType::FacetedBrep;
    let b = a;
    assert_eq!(a, b);
    assert_ne!(a, STEPControl_StepModelType::ManifoldSolidBrep);
}

// ---------------------------------------------------------------------------
// IFSelect_ReturnStatus
// ---------------------------------------------------------------------------

#[test]
fn return_status_display_all_variants() {
    let pairs = [
        (IFSelect_ReturnStatus::RetDone,    "RetDone"),
        (IFSelect_ReturnStatus::RetWarning, "RetWarning"),
        (IFSelect_ReturnStatus::RetFail,    "RetFail"),
        (IFSelect_ReturnStatus::RetError,   "RetError"),
        (IFSelect_ReturnStatus::RetVoid,    "RetVoid"),
    ];
    for (status, expected) in &pairs {
        assert_eq!(status.to_string(), *expected);
    }
}

// ---------------------------------------------------------------------------
// STEPControl_Reader — loading
// ---------------------------------------------------------------------------

#[test]
fn reader_read_file_success_returns_ret_done() {
    let mut reader = STEPControl_Reader::new();
    let status = reader.read_file(&make_step_string());
    assert_eq!(status, IFSelect_ReturnStatus::RetDone);
    assert!(reader.is_loaded());
    assert!(reader.errors().is_empty());
}

#[test]
fn reader_reads_correct_entity_count() {
    let mut reader = STEPControl_Reader::new();
    reader.read_file(&make_step_string());
    // 3 CARTESIAN_POINTs + 1 CLOSED_SHELL = 4 entities.
    assert_eq!(reader.model().nb_entities(), 4);
}

#[test]
fn reader_invalid_content_returns_ret_fail() {
    let mut reader = STEPControl_Reader::new();
    let status = reader.read_file("garbage data, not a STEP file at all");
    assert_eq!(status, IFSelect_ReturnStatus::RetFail);
    assert!(!reader.is_loaded());
    assert!(!reader.errors().is_empty());
}

#[test]
fn reader_empty_data_section_is_warning() {
    let mut reader = STEPControl_Reader::new();
    let status = reader.read_file(&make_step_no_data());
    // The reader loaded successfully but produced no entities → warning.
    assert!(
        status == IFSelect_ReturnStatus::RetDone
            || status == IFSelect_ReturnStatus::RetWarning,
        "expected RetDone or RetWarning, got {:?}",
        status
    );
    assert!(reader.is_loaded());
}

// ---------------------------------------------------------------------------
// STEPControl_Reader — transfer and shape access
// ---------------------------------------------------------------------------

#[test]
fn reader_nb_roots_for_transfer_positive_after_load() {
    let mut reader = STEPControl_Reader::new();
    reader.read_file(&make_step_string());
    // There is at least the CLOSED_SHELL root.
    assert!(reader.nb_roots_for_transfer() >= 1);
}

#[test]
fn reader_transfer_roots_returns_transferred_count() {
    let mut reader = STEPControl_Reader::new();
    reader.read_file(&make_step_string());
    let n = reader.transfer_roots();
    assert!(n >= 1);
    assert_eq!(reader.nb_shapes(), n);
}

#[test]
fn reader_shape_access_by_rank() {
    let mut reader = STEPControl_Reader::new();
    reader.read_file(&make_step_string());
    reader.transfer_roots();
    assert!(reader.nb_shapes() > 0);
    let first = reader.shape(1).expect("shape(1) must be Some after transfer");
    assert!(!first.keyword.is_empty());
    // Out-of-range rank returns None.
    assert!(reader.shape(9999).is_none());
    assert!(reader.shape(0).is_none());
}

#[test]
fn reader_one_shape_collects_all_transferred() {
    let mut reader = STEPControl_Reader::new();
    reader.read_file(&make_step_string());
    reader.transfer_roots();
    let compound = reader.one_shape();
    assert_eq!(compound.len(), reader.nb_shapes());
}

#[test]
fn reader_cartesian_points_decoded_correctly() {
    let mut reader = STEPControl_Reader::new();
    reader.read_file(&make_step_string());
    let pts = reader.cartesian_points();
    // Expect the three CARTESIAN_POINT entities.
    assert_eq!(pts.len(), 3);
    // origin (#1) should be at (0,0,0).
    let origin = pts.iter().find(|(id, _)| *id == 1).expect("id 1 must exist");
    assert!((origin.1[0]).abs() < 1e-9);
    assert!((origin.1[1]).abs() < 1e-9);
    assert!((origin.1[2]).abs() < 1e-9);
    // apex (#2) should be at (0,0,10).
    let apex = pts.iter().find(|(id, _)| *id == 2).expect("id 2 must exist");
    assert!((apex.1[2] - 10.0).abs() < 1e-9);
}

#[test]
fn reader_clear_resets_to_empty_state() {
    let mut reader = STEPControl_Reader::new();
    reader.read_file(&make_step_string());
    reader.transfer_roots();
    reader.clear();
    assert!(!reader.is_loaded());
    assert_eq!(reader.nb_shapes(), 0);
    assert_eq!(reader.model().nb_entities(), 0);
}

// ---------------------------------------------------------------------------
// STEPControl_Writer
// ---------------------------------------------------------------------------

#[test]
fn writer_default_is_empty() {
    let writer = STEPControl_Writer::new();
    assert_eq!(writer.nb_entities(), 0);
    assert!(writer.write_file().is_empty());
}

#[test]
fn writer_transfer_entity_increments_count() {
    let mut writer = STEPControl_Writer::new();
    let e = StepEntity::new(0, "CARTESIAN_POINT", "'p',(1.0,2.0,3.0)");
    let status = writer.transfer_entity(e);
    assert_eq!(status, IFSelect_ReturnStatus::RetDone);
    assert_eq!(writer.nb_entities(), 1);
}

#[test]
fn writer_transfer_empty_keyword_returns_error() {
    let mut writer = STEPControl_Writer::new();
    let bad = StepEntity::new(1, "", "params");
    let status = writer.transfer_entity(bad);
    assert_eq!(status, IFSelect_ReturnStatus::RetError);
    assert_eq!(writer.nb_entities(), 0);
}

#[test]
fn writer_write_file_produces_valid_step_envelope() {
    let mut writer = STEPControl_Writer::new();
    writer.transfer_point("origin", 0.0, 0.0, 0.0);
    writer.transfer_point("top", 0.0, 0.0, 50.0);
    let step = writer.write_file_named("test_solid");
    assert!(step.starts_with("ISO-10303-21;"), "must start with ISO sentinel");
    assert!(step.contains("HEADER;"), "must contain HEADER section");
    assert!(step.contains("DATA;"), "must contain DATA section");
    assert!(step.contains("CARTESIAN_POINT"), "must contain written entities");
    assert!(
        step.trim_end().ends_with("END-ISO-10303-21;"),
        "must end with ISO-10303-21 terminator"
    );
}

#[test]
fn writer_round_trip_readable_by_reader() {
    // Write two points and a shell entity, then parse the output with the reader.
    let mut writer = STEPControl_Writer::new();
    writer.set_mode(STEPControl_StepModelType::FacetedBrep);
    writer.transfer_point("v0", 0.0, 0.0, 0.0);
    writer.transfer_point("v1", 1.0, 0.0, 0.0);
    writer.transfer_point("v2", 0.0, 1.0, 0.0);
    let e = StepEntity::new(4, "CLOSED_SHELL", "'tri',(#1,#2,#3)");
    writer.transfer_entity(e);
    let step_text = writer.write_file_named("round_trip");

    let mut reader = STEPControl_Reader::new();
    let status = reader.read_file(&step_text);
    assert_eq!(status, IFSelect_ReturnStatus::RetDone);
    // The reader must find at least the 4 user entities.
    assert!(reader.model().nb_entities() >= 4);
    // The 3 CARTESIAN_POINTs must decode.
    let pts = reader.cartesian_points();
    assert_eq!(pts.len(), 3);
}

#[test]
fn writer_mode_affects_file_description_header() {
    let mut writer = STEPControl_Writer::with_mode(STEPControl_StepModelType::ManifoldSolidBrep);
    writer.transfer_point("p", 1.0, 2.0, 3.0);
    let step = writer.write_file();
    // The mode string must appear somewhere in the FILE_DESCRIPTION.
    assert!(step.contains("ManifoldSolidBrep"));
}

#[test]
fn writer_tolerance_embedded_in_output() {
    let mut writer = STEPControl_Writer::new();
    writer.set_tolerance(1.0e-3);
    writer.transfer_point("p", 0.0, 0.0, 0.0);
    let step = writer.write_file();
    assert!(step.contains("1.00E-3") || step.contains("1E-3") || step.contains("0.001") || step.contains("1.00E"));
}

#[test]
fn writer_clear_resets_staged_entities() {
    let mut writer = STEPControl_Writer::new();
    writer.transfer_point("a", 1.0, 0.0, 0.0);
    writer.transfer_point("b", 2.0, 0.0, 0.0);
    assert_eq!(writer.nb_entities(), 2);
    writer.clear();
    assert_eq!(writer.nb_entities(), 0);
    assert!(writer.write_file().is_empty());
}
