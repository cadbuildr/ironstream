// FILE: tests/occt_shape_heal.rs
extern crate ironstream;

use ironstream::shape_heal::{
    HealFixType, HealStatus, HealFix, FixSmallFace, FixRemoveSmallEdges, ShapeHealing,
};

#[test]
fn heal_status_is_done() {
    assert!(HealStatus::Done.is_done());
    assert!(!HealStatus::NotDone.is_done());
    assert!(!HealStatus::Failed.is_done());
    assert!(!HealStatus::NoFixes.is_done());
}

#[test]
fn fix_small_face_removes_faces() {
    let mut f = FixSmallFace::new(10);
    f.set_tolerance(1e-6);
    f.perform(vec![1, 2, 3]);
    assert!(f.is_done());
    assert_eq!(f.nb_removed_faces(), 3);
    assert_eq!(f.shape(), 10 + 40000);
}

#[test]
fn fix_small_face_null_shape_fails() {
    let mut f = FixSmallFace::new(0);
    f.perform(vec![1]);
    assert_eq!(f.status, HealStatus::Failed);
    assert!(!f.is_done());
}

#[test]
fn fix_small_face_empty_list_no_fixes() {
    let mut f = FixSmallFace::new(20);
    f.perform(vec![]);
    assert_eq!(f.status, HealStatus::NoFixes);
    assert_eq!(f.shape(), 20); // returns original shape
}

#[test]
fn fix_remove_small_edges_removes() {
    let mut f = FixRemoveSmallEdges::new(30);
    f.set_tolerance(5e-7);
    let ok = f.remove_small_edges(vec![5, 6, 7]);
    assert!(ok);
    assert!(f.is_done());
    assert_eq!(f.nb_removed(), 3);
    assert_eq!(f.shape(), 30 + 41000);
}

#[test]
fn shape_healing_apply_and_perform() {
    let mut h = ShapeHealing::new(50);
    h.set_tolerance_3d(1e-5);
    h.set_tolerance_angular(1e-4);
    h.apply_fix(HealFixType::SmallFace, vec![1, 2]);
    h.apply_fix(HealFixType::SewEdges, vec![3]);
    assert_eq!(h.nb_fixes(), 3);
    h.perform();
    assert!(h.is_done());
    assert_eq!(h.shape(), 50 + 50000);
}
