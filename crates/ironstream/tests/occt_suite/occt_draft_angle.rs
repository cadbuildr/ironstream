// Integration tests for draft_angle
use ironstream::draft_angle::*;

#[test]
fn draft_angle_multiple_faces() {
    let mut da = BrepOffsetApiDraftAngle::new(42);
    da.add(5, [0.0, 0.0, 1.0], 3.0_f64.to_radians(), 10);
    da.add(6, [0.0, 0.0, 1.0], 3.0_f64.to_radians(), 10);
    da.add(7, [0.0, 0.0, 1.0], 3.0_f64.to_radians(), 10);
    assert_eq!(da.nb_added_faces(), 3);
    da.build();
    assert!(da.is_done());
    assert_eq!(da.shape(), 42 + 8000);
}

#[test]
fn draft_angle_null_face_not_added() {
    let mut da = BrepOffsetApiDraftAngle::new(1);
    let ok = da.add(0, [0.0, 0.0, 1.0], 0.1, 5);
    assert!(!ok);
    assert_eq!(da.nb_added_faces(), 0);
}

#[test]
fn draft_angle_error_face_initially_zero() {
    let da = BrepOffsetApiDraftAngle::new(10);
    assert_eq!(da.error_face(), 0);
}

#[test]
fn draft_modification_face_entry_fields() {
    let mut dm = DraftModification::new();
    dm.add_face(3, 7, [1.0, 0.0, 0.0], 10.0_f64.to_radians());
    let entry = dm.face(0).unwrap();
    assert_eq!(entry.face_id, 3);
    assert_eq!(entry.neutral_plane_id, 7);
    assert_eq!(entry.result_face_id, 3 + 5000);
}

#[test]
fn draft_edge_info_ids() {
    let info = DraftEdgeInfo::new(99);
    assert_eq!(info.edge_id, 99);
    assert_eq!(info.curve_id, 99 + 100);
    assert!(!info.is_tangent_on_s1);
    assert!(!info.is_tangent_on_s2);
}

#[test]
fn draft_status_variants() {
    assert!(DraftStatus::Done.is_done());
    assert!(!DraftStatus::OverFlow.is_done());
    assert!(!DraftStatus::NullFace.is_done());
    assert_eq!(DraftStatus::default(), DraftStatus::NotDone);
}
