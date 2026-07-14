// FILE: tests/occt_hlr_brep.rs
extern crate ironstream;
use ironstream::hlr_brep::*;

#[test]
fn algo_starts_empty_and_stale() {
    let algo = HlrBrepAlgo::new();
    assert_eq!(algo.edge_count(), 0);
    assert!(!algo.is_up_to_date());
}

#[test]
fn algo_default_equals_new() {
    let algo = HlrBrepAlgo::default();
    assert_eq!(algo.edge_count(), 0);
    assert!(!algo.is_up_to_date());
}

#[test]
fn update_marks_algo_current() {
    let mut algo = HlrBrepAlgo::new();
    algo.update();
    assert!(algo.is_up_to_date());
}

#[test]
fn adding_edge_after_update_marks_stale() {
    let mut algo = HlrBrepAlgo::new();
    algo.update();
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleSharp, 0.0, 1.0));
    assert!(!algo.is_up_to_date());
}

#[test]
fn edge_count_tracks_additions() {
    let mut algo = HlrBrepAlgo::new();
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleSharp, 0.0, 1.0));
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::HiddenSmooth, 0.0, 1.0));
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleSewing, 0.0, 1.0));
    assert_eq!(algo.edge_count(), 3);
}

#[test]
fn visible_edges_returns_only_visible() {
    let mut algo = HlrBrepAlgo::new();
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleSharp, 0.0, 1.0));
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleSmooth, 0.1, 0.9));
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::HiddenSharp, 0.0, 1.0));
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::HiddenOutline, 0.0, 1.0));
    algo.update();
    let extractor = HlrBrepHLRToShape::new(&algo);
    let vis = extractor.visible_edges();
    assert_eq!(vis.len(), 2);
    assert!(vis.iter().all(|e| e.edge_type.is_visible()));
}

#[test]
fn hidden_edges_returns_only_hidden() {
    let mut algo = HlrBrepAlgo::new();
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::HiddenSharp, 0.0, 1.0));
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::HiddenSewing, 0.0, 0.5));
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::HiddenOutline, 0.0, 1.0));
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleSharp, 0.0, 1.0));
    algo.update();
    let extractor = HlrBrepHLRToShape::new(&algo);
    let hid = extractor.hidden_edges();
    assert_eq!(hid.len(), 3);
    assert!(hid.iter().all(|e| e.edge_type.is_hidden()));
}

#[test]
fn visible_outline_edges_excludes_hidden_outline() {
    let mut algo = HlrBrepAlgo::new();
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleOutline, 0.0, 1.0));
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleOutline, 0.2, 0.8));
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::HiddenOutline, 0.0, 1.0));
    algo.add_edge(HlrBrepEdgeData::new(HlrBrepEdgeType::VisibleSharp, 0.0, 1.0));
    algo.update();
    let extractor = HlrBrepHLRToShape::new(&algo);
    let outlines = extractor.visible_outline_edges();
    assert_eq!(outlines.len(), 2);
    for e in &outlines {
        assert_eq!(e.edge_type, HlrBrepEdgeType::VisibleOutline);
    }
}

#[test]
fn empty_algo_yields_empty_filter_results() {
    let algo = HlrBrepAlgo::new();
    let extractor = HlrBrepHLRToShape::new(&algo);
    assert!(extractor.visible_edges().is_empty());
    assert!(extractor.hidden_edges().is_empty());
    assert!(extractor.visible_outline_edges().is_empty());
}

#[test]
fn edge_data_params_are_preserved() {
    let e = HlrBrepEdgeData::new(HlrBrepEdgeType::HiddenSmooth, 0.3, 0.7);
    assert_eq!(e.edge_type, HlrBrepEdgeType::HiddenSmooth);
    assert!((e.start_param - 0.3).abs() < 1e-15);
    assert!((e.end_param - 0.7).abs() < 1e-15);
}

#[test]
fn all_eight_edge_type_variants_are_distinct() {
    let variants = [
        HlrBrepEdgeType::VisibleSharp,
        HlrBrepEdgeType::VisibleSmooth,
        HlrBrepEdgeType::VisibleSewing,
        HlrBrepEdgeType::VisibleOutline,
        HlrBrepEdgeType::HiddenSharp,
        HlrBrepEdgeType::HiddenSmooth,
        HlrBrepEdgeType::HiddenSewing,
        HlrBrepEdgeType::HiddenOutline,
    ];
    assert_eq!(variants.len(), 8);
    for i in 0..variants.len() {
        for j in 0..variants.len() {
            if i == j {
                assert_eq!(variants[i], variants[j]);
            } else {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }
}

#[test]
fn edge_type_copy_clone_and_debug() {
    let t = HlrBrepEdgeType::VisibleSharp;
    let t2 = t;
    assert_eq!(t, t2);
    let s = format!("{:?}", HlrBrepEdgeType::HiddenOutline);
    assert!(!s.is_empty());
    assert_eq!(s, "HiddenOutline");
}

#[test]
fn visible_hidden_partition_covers_all_types() {
    let visible_types = [
        HlrBrepEdgeType::VisibleSharp,
        HlrBrepEdgeType::VisibleSmooth,
        HlrBrepEdgeType::VisibleSewing,
        HlrBrepEdgeType::VisibleOutline,
    ];
    let hidden_types = [
        HlrBrepEdgeType::HiddenSharp,
        HlrBrepEdgeType::HiddenSmooth,
        HlrBrepEdgeType::HiddenSewing,
        HlrBrepEdgeType::HiddenOutline,
    ];
    for t in &visible_types {
        assert!(t.is_visible(), "{:?} should be visible", t);
        assert!(!t.is_hidden(), "{:?} should not be hidden", t);
    }
    for t in &hidden_types {
        assert!(!t.is_visible(), "{:?} should not be visible", t);
        assert!(t.is_hidden(), "{:?} should be hidden", t);
    }
}
