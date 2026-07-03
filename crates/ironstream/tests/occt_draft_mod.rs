// FILE: tests/occt_draft_mod.rs
extern crate ironstream;

use ironstream::draft_mod::{
    DraftAngleError, DraftFaceEntry, DraftModification, DraftModificationStatus,
};
use std::f64::consts::FRAC_PI_4;

fn simple_entry(face_id: u32, angle: f64) -> DraftFaceEntry {
    DraftFaceEntry::new(face_id, angle, [0.0, 1.0, 0.0], 0.0)
}

#[test]
fn new_modification_is_not_done() {
    let dm = DraftModification::new();
    assert_eq!(dm.status(), DraftModificationStatus::NotDone);
    assert!(!dm.is_done());
}

#[test]
fn new_modification_has_zero_faces() {
    let dm = DraftModification::new();
    assert_eq!(dm.nb_faces(), 0);
}

#[test]
fn add_valid_face_returns_true() {
    let mut dm = DraftModification::new();
    assert!(dm.add_face(simple_entry(1, FRAC_PI_4)));
}

#[test]
fn add_face_at_exactly_pi_over_2_is_rejected() {
    let mut dm = DraftModification::new();
    let result = dm.add_face(simple_entry(1, std::f64::consts::FRAC_PI_2 + 1e-10));
    assert!(!result);
    assert_eq!(dm.nb_faces(), 0);
}

#[test]
fn add_face_negative_large_angle_is_rejected() {
    let mut dm = DraftModification::new();
    let result = dm.add_face(simple_entry(2, -(std::f64::consts::FRAC_PI_2 + 0.1)));
    assert!(!result);
}

#[test]
fn perform_with_faces_yields_done_status() {
    let mut dm = DraftModification::new();
    dm.add_face(simple_entry(1, FRAC_PI_4));
    dm.perform();
    assert_eq!(dm.status(), DraftModificationStatus::Done);
    assert!(dm.is_done());
}

#[test]
fn perform_empty_yields_error_status() {
    let mut dm = DraftModification::new();
    dm.perform();
    assert_eq!(dm.status(), DraftModificationStatus::Error);
    assert!(!dm.is_done());
}

#[test]
fn error_status_remains_no_error_after_successful_perform() {
    let mut dm = DraftModification::new();
    dm.add_face(simple_entry(10, 0.1));
    dm.perform();
    assert_eq!(dm.error_status(), DraftAngleError::NoError);
}

#[test]
fn remove_face_reduces_count() {
    let mut dm = DraftModification::new();
    dm.add_face(simple_entry(1, FRAC_PI_4));
    dm.add_face(simple_entry(2, FRAC_PI_4));
    dm.remove_face(1);
    assert_eq!(dm.nb_faces(), 1);
}

#[test]
fn remove_nonexistent_face_is_safe() {
    let mut dm = DraftModification::new();
    dm.add_face(simple_entry(5, FRAC_PI_4));
    dm.remove_face(999);
    assert_eq!(dm.nb_faces(), 1);
}

#[test]
fn clear_empties_face_list_and_resets_status() {
    let mut dm = DraftModification::new();
    dm.add_face(simple_entry(3, FRAC_PI_4));
    dm.perform();
    dm.clear();
    assert_eq!(dm.nb_faces(), 0);
    assert_eq!(dm.status(), DraftModificationStatus::NotDone);
}

#[test]
fn draft_face_entry_stores_all_fields() {
    let normal = [0.0, 0.0, 1.0];
    let d = 5.0;
    let e = DraftFaceEntry::new(42, FRAC_PI_4, normal, d);
    assert_eq!(e.face_id, 42);
    assert!((e.draft_angle - FRAC_PI_4).abs() < 1e-12);
    assert!((e.neutral_plane[2] - 1.0).abs() < 1e-12);
    assert!((e.neutral_plane[3] - 5.0).abs() < 1e-12);
}

#[test]
fn add_many_faces_then_perform() {
    let mut dm = DraftModification::new();
    for id in 0..8u32 {
        let angle = FRAC_PI_4 * (id as f64 / 8.0);
        assert!(dm.add_face(simple_entry(id, angle)));
    }
    assert_eq!(dm.nb_faces(), 8);
    dm.perform();
    assert!(dm.is_done());
}

#[test]
fn perform_then_clear_then_perform_gives_error() {
    let mut dm = DraftModification::new();
    dm.add_face(simple_entry(1, FRAC_PI_4));
    dm.perform();
    assert!(dm.is_done());
    dm.clear();
    dm.perform();
    assert_eq!(dm.status(), DraftModificationStatus::Error);
}
