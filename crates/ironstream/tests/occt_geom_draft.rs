// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_draft.rs
extern crate ironstream;
use ironstream::geom_draft::*;

// --- DraftFace tests ---

#[test]
fn draft_face_new_stores_fields() {
    let f = DraftFace::new("top", 5.0_f64.to_radians(), [0.0, 0.0, 1.0]);
    assert_eq!(f.label(), "top");
    assert!((f.angle() - 5.0_f64.to_radians()).abs() < 1e-12);
    assert_eq!(f.direction(), [0.0, 0.0, 1.0]);
    assert!((f.neutral_plane_dist() - 0.0).abs() < 1e-12);
}

#[test]
fn draft_face_set_neutral_plane_dist() {
    let mut f = DraftFace::new("side", 3.0_f64.to_radians(), [1.0, 0.0, 0.0]);
    f.set_neutral_plane_dist(12.5);
    assert!((f.neutral_plane_dist() - 12.5).abs() < 1e-12);
}

#[test]
fn draft_face_default_neutral_plane_dist_is_zero() {
    let f = DraftFace::new("base", 1.0, [0.0, 1.0, 0.0]);
    assert_eq!(f.neutral_plane_dist(), 0.0);
}

#[test]
fn draft_face_label_roundtrip() {
    let f = DraftFace::new("my_face_label", 0.1, [0.0, 0.0, 1.0]);
    assert_eq!(f.label(), "my_face_label");
}

#[test]
fn draft_face_direction_stored_correctly() {
    let dir = [0.5773, 0.5773, 0.5773];
    let f = DraftFace::new("diag", 2.0, dir);
    assert_eq!(f.direction(), dir);
}

#[test]
fn draft_face_negative_angle_stored() {
    let f = DraftFace::new("neg", -3.0_f64.to_radians(), [0.0, 0.0, 1.0]);
    assert!(f.angle() < 0.0);
}

#[test]
fn draft_face_neutral_plane_dist_overwrite() {
    let mut f = DraftFace::new("f", 1.0, [0.0, 0.0, 1.0]);
    f.set_neutral_plane_dist(5.0);
    f.set_neutral_plane_dist(-3.0);
    assert!((f.neutral_plane_dist() - (-3.0)).abs() < 1e-12);
}

// --- DraftStatus tests ---

#[test]
fn draft_status_variants_are_distinct() {
    assert_ne!(DraftStatus::NoError, DraftStatus::NotDone);
    assert_ne!(DraftStatus::FaceRecomputed, DraftStatus::EdgeRecomputed);
    assert_ne!(DraftStatus::EdgeRecomputed, DraftStatus::VertexRecomputed);
}

#[test]
fn draft_status_copy() {
    let s = DraftStatus::FaceRecomputed;
    let t = s;
    assert_eq!(s, t);
}

// --- DraftResult tests ---

#[test]
fn draft_result_new_is_not_done() {
    let r = DraftResult::new();
    assert!(!r.is_done());
    assert_eq!(r.status(), DraftStatus::NotDone);
    assert_eq!(r.nb_faces_modified(), 0);
}

#[test]
fn draft_result_default_equals_new() {
    let r: DraftResult = Default::default();
    assert!(!r.is_done());
    assert_eq!(r.status(), DraftStatus::NotDone);
}

#[test]
fn draft_result_build_sets_done() {
    let mut r = DraftResult::new();
    r.build(3);
    assert!(r.is_done());
}

#[test]
fn draft_result_build_sets_face_recomputed_status() {
    let mut r = DraftResult::new();
    r.build(1);
    assert_eq!(r.status(), DraftStatus::FaceRecomputed);
}

#[test]
fn draft_result_build_stores_nb_faces_modified() {
    let mut r = DraftResult::new();
    r.build(7);
    assert_eq!(r.nb_faces_modified(), 7);
}

#[test]
fn draft_result_build_zero_faces_still_done() {
    let mut r = DraftResult::new();
    r.build(0);
    assert!(r.is_done());
    assert_eq!(r.nb_faces_modified(), 0);
}

// --- MakeDraftAngle tests ---

#[test]
fn make_draft_angle_new_empty() {
    let m = MakeDraftAngle::new();
    assert_eq!(m.nb_faces(), 0);
    assert!(!m.is_done());
}

#[test]
fn make_draft_angle_default_empty() {
    let m: MakeDraftAngle = Default::default();
    assert_eq!(m.nb_faces(), 0);
    assert!(!m.is_done());
}

#[test]
fn make_draft_angle_add_face_increments_count() {
    let mut m = MakeDraftAngle::new();
    m.add_face(DraftFace::new("f1", 2.0_f64.to_radians(), [0.0, 0.0, 1.0]));
    assert_eq!(m.nb_faces(), 1);
}

#[test]
fn make_draft_angle_add_multiple_faces() {
    let mut m = MakeDraftAngle::new();
    for i in 0..4 {
        m.add_face(DraftFace::new(&format!("f{}", i), 1.0, [0.0, 0.0, 1.0]));
    }
    assert_eq!(m.nb_faces(), 4);
}

#[test]
fn make_draft_angle_build_sets_is_done() {
    let mut m = MakeDraftAngle::new();
    m.add_face(DraftFace::new("top", 5.0_f64.to_radians(), [0.0, 1.0, 0.0]));
    m.build();
    assert!(m.is_done());
}

#[test]
fn make_draft_angle_build_no_faces_still_done() {
    let mut m = MakeDraftAngle::new();
    m.build();
    assert!(m.is_done());
}

#[test]
fn make_draft_angle_result_nb_faces_matches_added() {
    let mut m = MakeDraftAngle::new();
    m.add_face(DraftFace::new("a", 1.0, [1.0, 0.0, 0.0]));
    m.add_face(DraftFace::new("b", 2.0, [0.0, 1.0, 0.0]));
    m.add_face(DraftFace::new("c", 3.0, [0.0, 0.0, 1.0]));
    m.build();
    assert_eq!(m.result().nb_faces_modified(), 3);
}

#[test]
fn make_draft_angle_result_status_face_recomputed_after_build() {
    let mut m = MakeDraftAngle::new();
    m.add_face(DraftFace::new("f", 1.0, [0.0, 0.0, 1.0]));
    m.build();
    assert_eq!(m.result().status(), DraftStatus::FaceRecomputed);
}

#[test]
fn make_draft_angle_result_not_done_before_build() {
    let m = MakeDraftAngle::new();
    assert!(!m.result().is_done());
    assert_eq!(m.result().status(), DraftStatus::NotDone);
}

#[test]
fn make_draft_angle_is_done_reflects_result() {
    let mut m = MakeDraftAngle::new();
    assert!(!m.is_done());
    m.add_face(DraftFace::new("f", 1.0, [0.0, 0.0, 1.0]));
    m.build();
    assert_eq!(m.is_done(), m.result().is_done());
}
