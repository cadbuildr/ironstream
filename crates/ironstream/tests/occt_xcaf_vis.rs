extern crate ironstream;
use ironstream::xcaf_vis::*;

#[test]
fn xcaf_prs_style_default_is_visible_and_opaque() {
    let s = XcafPrsStyle::default();
    assert!(s.is_visible);
    assert!((s.transparency - 0.0).abs() < 1e-10);
    assert!(!s.is_transparent());
    assert!(!s.is_set()); // no colors set by default
}

#[test]
fn xcaf_prs_style_with_transparency_clamps_to_0_1() {
    // Value within range
    let s1 = XcafPrsStyle::new().with_transparency(0.5);
    assert!((s1.transparency - 0.5).abs() < 1e-6);
    // Value above 1 should be clamped to 1
    let s2 = XcafPrsStyle::new().with_transparency(2.0);
    assert!((s2.transparency - 1.0).abs() < 1e-6);
    // Negative value should be clamped to 0
    let s3 = XcafPrsStyle::new().with_transparency(-0.5);
    assert!((s3.transparency - 0.0).abs() < 1e-6);
}

#[test]
fn xcaf_document_explorer_nb_roots_counts_depth_0_entries() {
    let mut exp = XcafDocumentExplorer::new(1, 10);
    exp.init(vec![
        ExplorerEntry { label: 10, depth: 0, style: Default::default(), instance_path: vec![] },
        ExplorerEntry { label: 11, depth: 1, style: Default::default(), instance_path: vec![10] },
        ExplorerEntry { label: 20, depth: 0, style: Default::default(), instance_path: vec![] },
        ExplorerEntry { label: 21, depth: 1, style: Default::default(), instance_path: vec![20] },
        ExplorerEntry { label: 22, depth: 2, style: Default::default(), instance_path: vec![20, 21] },
    ]);
    assert_eq!(exp.nb_roots(), 2);
}

#[test]
fn xcaf_document_explorer_collect_leaves() {
    let mut exp = XcafDocumentExplorer::new(1, 10);
    // Simple hierarchy: 10 -> 11, 10 -> 12 (11 and 12 are leaves, 10 is a parent)
    exp.init(vec![
        ExplorerEntry { label: 10, depth: 0, style: Default::default(), instance_path: vec![] },
        ExplorerEntry { label: 11, depth: 1, style: Default::default(), instance_path: vec![10] },
        ExplorerEntry { label: 12, depth: 1, style: Default::default(), instance_path: vec![10] },
    ]);
    let leaves = exp.collect_leaves();
    // 11 and 12 are leaves (nothing has them in instance_path at deeper depth)
    assert!(leaves.contains(&11));
    assert!(leaves.contains(&12));
}

#[test]
fn xcaf_prs_ais_object_nb_sub_styles_after_set_sub_style() {
    let mut obj = XcafPrsAISObject::new(42, 1);
    assert_eq!(obj.nb_sub_styles(), 0);
    obj.set_sub_style(10, XcafPrsStyle::new().with_surface_color(1.0, 0.0, 0.0));
    assert_eq!(obj.nb_sub_styles(), 1);
    obj.set_sub_style(20, XcafPrsStyle::new().with_surface_color(0.0, 1.0, 0.0));
    obj.set_sub_style(30, XcafPrsStyle::new().with_curve_color(0.0, 0.0, 1.0));
    assert_eq!(obj.nb_sub_styles(), 3);
    // Overwriting an existing sub_label should not increase count
    obj.set_sub_style(10, XcafPrsStyle::new().with_surface_color(0.5, 0.5, 0.5));
    assert_eq!(obj.nb_sub_styles(), 3);
}

#[test]
fn xcaf_prs_style_is_set_returns_false_when_no_colors() {
    let s = XcafPrsStyle::new();
    assert!(!s.is_set());
    assert!(!s.has_surface_color());
    assert!(!s.has_curve_color());
    // After setting a surface color, is_set returns true
    let s2 = s.with_surface_color(0.3, 0.3, 0.3);
    assert!(s2.is_set());
    assert!(s2.has_surface_color());
}
