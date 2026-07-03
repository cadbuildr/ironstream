// FILE: rust/ironstream/crates/ironstream/tests/occt_brep_algo_feature.rs

//! Tests for `brep_algo_feature` — ported BRepAlgoAPI feature removal types.

extern crate ironstream;
use ironstream::brep_algo_feature::*;

// ---------------------------------------------------------------------------
// AlgoFeature
// ---------------------------------------------------------------------------

#[test]
fn test_algo_feature_new_and_accessors() {
    let f = AlgoFeature::new(
        AlgoFeatureType::Hole,
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        3,
    );
    assert_eq!(f.feature_type(), AlgoFeatureType::Hole);
    assert_eq!(f.center(), [1.0, 2.0, 3.0]);
    assert_eq!(f.size(), [4.0, 5.0, 6.0]);
    assert_eq!(f.nb_faces(), 3);
}

#[test]
fn test_algo_feature_volume() {
    let f = AlgoFeature::new(
        AlgoFeatureType::Pocket,
        [0.0, 0.0, 0.0],
        [2.0, 3.0, 4.0],
        6,
    );
    assert_eq!(f.volume(), 24.0);
}

#[test]
fn test_algo_feature_volume_zero_dimension() {
    let f = AlgoFeature::new(
        AlgoFeatureType::Slot,
        [0.0, 0.0, 0.0],
        [5.0, 0.0, 3.0],
        4,
    );
    assert_eq!(f.volume(), 0.0);
}

#[test]
fn test_algo_feature_all_types() {
    for ft in [
        AlgoFeatureType::Hole,
        AlgoFeatureType::Pocket,
        AlgoFeatureType::Slot,
        AlgoFeatureType::Protrusion,
        AlgoFeatureType::Unknown,
    ] {
        let f = AlgoFeature::new(ft, [0.0; 3], [1.0, 1.0, 1.0], 1);
        assert_eq!(f.feature_type(), ft);
    }
}

#[test]
fn test_algo_feature_clone() {
    let f = AlgoFeature::new(
        AlgoFeatureType::Protrusion,
        [7.0, 8.0, 9.0],
        [1.0, 2.0, 3.0],
        5,
    );
    let g = f.clone();
    assert_eq!(g.feature_type(), AlgoFeatureType::Protrusion);
    assert_eq!(g.center(), [7.0, 8.0, 9.0]);
    assert_eq!(g.nb_faces(), 5);
}

// ---------------------------------------------------------------------------
// BrepAlgoFeatureResult
// ---------------------------------------------------------------------------

#[test]
fn test_brep_algo_feature_result_new() {
    let r = BrepAlgoFeatureResult::new();
    assert!(!r.is_done());
    assert_eq!(r.nb_faces_modified(), 0);
    assert_eq!(r.nb_features(), 0);
}

#[test]
fn test_brep_algo_feature_result_default() {
    let r = BrepAlgoFeatureResult::default();
    assert!(!r.is_done());
    assert_eq!(r.nb_features(), 0);
}

#[test]
fn test_brep_algo_feature_result_mark_done() {
    let mut r = BrepAlgoFeatureResult::new();
    r.mark_done(7);
    assert!(r.is_done());
    assert_eq!(r.nb_faces_modified(), 7);
}

#[test]
fn test_brep_algo_feature_result_add_and_access_feature() {
    let mut r = BrepAlgoFeatureResult::new();
    let f1 = AlgoFeature::new(AlgoFeatureType::Hole, [0.0, 0.0, 0.0], [1.0, 1.0, 2.0], 2);
    let f2 = AlgoFeature::new(AlgoFeatureType::Slot, [1.0, 0.0, 0.0], [3.0, 1.0, 1.0], 4);
    r.add_feature(f1);
    r.add_feature(f2);
    assert_eq!(r.nb_features(), 2);
    assert_eq!(r.feature(0).feature_type(), AlgoFeatureType::Hole);
    assert_eq!(r.feature(1).feature_type(), AlgoFeatureType::Slot);
}

#[test]
fn test_brep_algo_feature_result_feature_volume_via_result() {
    let mut r = BrepAlgoFeatureResult::new();
    r.add_feature(AlgoFeature::new(
        AlgoFeatureType::Pocket,
        [0.0; 3],
        [2.0, 3.0, 5.0],
        6,
    ));
    assert_eq!(r.feature(0).volume(), 30.0);
}

// ---------------------------------------------------------------------------
// BrepAlgoFeatureRemoval
// ---------------------------------------------------------------------------

#[test]
fn test_brep_algo_feature_removal_new() {
    let rem = BrepAlgoFeatureRemoval::new();
    assert_eq!(rem.nb_to_remove(), 0);
    assert!(!rem.is_done());
}

#[test]
fn test_brep_algo_feature_removal_default() {
    let rem = BrepAlgoFeatureRemoval::default();
    assert_eq!(rem.nb_to_remove(), 0);
    assert!(!rem.is_done());
}

#[test]
fn test_brep_algo_feature_removal_add_feature() {
    let mut rem = BrepAlgoFeatureRemoval::new();
    rem.add_feature_to_remove("hole_1");
    rem.add_feature_to_remove("pocket_A");
    assert_eq!(rem.nb_to_remove(), 2);
    assert_eq!(rem.features_to_remove[0], "hole_1");
    assert_eq!(rem.features_to_remove[1], "pocket_A");
}

#[test]
fn test_brep_algo_feature_removal_perform_sets_done() {
    let mut rem = BrepAlgoFeatureRemoval::new();
    rem.add_feature_to_remove("slot_X");
    rem.add_feature_to_remove("hole_Y");
    rem.add_feature_to_remove("pocket_Z");
    rem.perform();
    assert!(rem.is_done());
}

#[test]
fn test_brep_algo_feature_removal_perform_nb_faces_equals_nb_to_remove() {
    let mut rem = BrepAlgoFeatureRemoval::new();
    rem.add_feature_to_remove("f1");
    rem.add_feature_to_remove("f2");
    let expected = rem.nb_to_remove();
    rem.perform();
    assert_eq!(rem.result().nb_faces_modified(), expected);
}

#[test]
fn test_brep_algo_feature_removal_perform_zero_features() {
    let mut rem = BrepAlgoFeatureRemoval::new();
    rem.perform();
    assert!(rem.is_done());
    assert_eq!(rem.result().nb_faces_modified(), 0);
}

#[test]
fn test_brep_algo_feature_removal_result_accessor() {
    let mut rem = BrepAlgoFeatureRemoval::new();
    rem.add_feature_to_remove("r1");
    rem.perform();
    let r = rem.result();
    assert!(r.is_done());
    assert_eq!(r.nb_faces_modified(), 1);
}

#[test]
fn test_brep_algo_feature_removal_no_perform_not_done() {
    let mut rem = BrepAlgoFeatureRemoval::new();
    rem.add_feature_to_remove("will_not_be_removed");
    // perform() not called
    assert!(!rem.is_done());
    assert_eq!(rem.result().nb_faces_modified(), 0);
}
