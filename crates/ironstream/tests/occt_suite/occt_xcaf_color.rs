// FILE: rust/ironstream/crates/ironstream/tests/occt_xcaf_color.rs

extern crate ironstream;
use ironstream::xcaf_color::*;

// --- XcafColorType ---

#[test]
fn test_color_type_variants_are_distinct() {
    assert_ne!(XcafColorType::Gen, XcafColorType::Surf);
    assert_ne!(XcafColorType::Surf, XcafColorType::Curve);
    assert_ne!(XcafColorType::Gen, XcafColorType::Curve);
}

#[test]
fn test_color_type_copy() {
    let t = XcafColorType::Gen;
    let t2 = t;
    assert_eq!(t, t2);
}

// --- XcafColorEntry ---

#[test]
fn test_entry_new_stores_label() {
    let e = XcafColorEntry::new("layer1", 1.0, 0.0, 0.0, 1.0, XcafColorType::Gen);
    assert_eq!(e.label(), "layer1");
}

#[test]
fn test_entry_new_stores_rgba() {
    let e = XcafColorEntry::new("x", 0.1, 0.2, 0.3, 0.4, XcafColorType::Surf);
    assert_eq!(e.rgba(), [0.1_f32, 0.2, 0.3, 0.4]);
}

#[test]
fn test_entry_new_stores_color_type() {
    let e = XcafColorEntry::new("edge", 0.0, 0.0, 1.0, 1.0, XcafColorType::Curve);
    assert_eq!(e.color_type(), XcafColorType::Curve);
}

#[test]
fn test_entry_set_rgba_updates_values() {
    let mut e = XcafColorEntry::new("a", 1.0, 0.0, 0.0, 1.0, XcafColorType::Gen);
    e.set_rgba([0.5, 0.5, 0.5, 0.5]);
    assert_eq!(e.rgba(), [0.5_f32, 0.5, 0.5, 0.5]);
}

#[test]
fn test_entry_label_with_empty_string() {
    let e = XcafColorEntry::new("", 0.0, 0.0, 0.0, 0.0, XcafColorType::Surf);
    assert_eq!(e.label(), "");
}

#[test]
fn test_entry_clone() {
    let e = XcafColorEntry::new("orig", 0.3, 0.6, 0.9, 1.0, XcafColorType::Curve);
    let e2 = e.clone();
    assert_eq!(e2.label(), "orig");
    assert_eq!(e2.rgba(), [0.3_f32, 0.6, 0.9, 1.0]);
    assert_eq!(e2.color_type(), XcafColorType::Curve);
}

// --- XcafColorTool ---

#[test]
fn test_tool_new_is_empty() {
    let tool = XcafColorTool::new();
    assert_eq!(tool.nb_colors(), 0);
}

#[test]
fn test_tool_add_color_returns_index() {
    let mut tool = XcafColorTool::new();
    let e = XcafColorEntry::new("red", 1.0, 0.0, 0.0, 1.0, XcafColorType::Gen);
    let idx = tool.add_color(e);
    assert_eq!(idx, 0);
}

#[test]
fn test_tool_add_multiple_returns_sequential_indices() {
    let mut tool = XcafColorTool::new();
    let i0 = tool.add_color(XcafColorEntry::new("a", 1.0, 0.0, 0.0, 1.0, XcafColorType::Gen));
    let i1 = tool.add_color(XcafColorEntry::new("b", 0.0, 1.0, 0.0, 1.0, XcafColorType::Surf));
    let i2 = tool.add_color(XcafColorEntry::new("c", 0.0, 0.0, 1.0, 1.0, XcafColorType::Curve));
    assert_eq!(i0, 0);
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
}

#[test]
fn test_tool_nb_colors_increments() {
    let mut tool = XcafColorTool::new();
    assert_eq!(tool.nb_colors(), 0);
    tool.add_color(XcafColorEntry::new("x", 0.0, 0.0, 0.0, 1.0, XcafColorType::Gen));
    assert_eq!(tool.nb_colors(), 1);
    tool.add_color(XcafColorEntry::new("y", 1.0, 1.0, 1.0, 1.0, XcafColorType::Gen));
    assert_eq!(tool.nb_colors(), 2);
}

#[test]
fn test_tool_get_color_valid_index() {
    let mut tool = XcafColorTool::new();
    tool.add_color(XcafColorEntry::new("surf_red", 1.0, 0.0, 0.0, 1.0, XcafColorType::Surf));
    let entry = tool.get_color(0).unwrap();
    assert_eq!(entry.label(), "surf_red");
    assert_eq!(entry.rgba(), [1.0_f32, 0.0, 0.0, 1.0]);
    assert_eq!(entry.color_type(), XcafColorType::Surf);
}

#[test]
fn test_tool_get_color_out_of_bounds_returns_none() {
    let tool = XcafColorTool::new();
    assert!(tool.get_color(0).is_none());
}

#[test]
fn test_tool_get_color_preserves_label_and_type() {
    let mut tool = XcafColorTool::new();
    tool.add_color(XcafColorEntry::new("wire", 0.2, 0.4, 0.6, 0.8, XcafColorType::Curve));
    let e = tool.get_color(0).unwrap();
    assert_eq!(e.label(), "wire");
    assert_eq!(e.color_type(), XcafColorType::Curve);
}

#[test]
fn test_tool_find_color_exact_match() {
    let mut tool = XcafColorTool::new();
    tool.add_color(XcafColorEntry::new("a", 0.5, 0.5, 0.5, 1.0, XcafColorType::Gen));
    let found = tool.find_color([0.5, 0.5, 0.5, 1.0]);
    assert_eq!(found, Some(0));
}

#[test]
fn test_tool_find_color_within_tolerance() {
    let mut tool = XcafColorTool::new();
    tool.add_color(XcafColorEntry::new("a", 0.5, 0.5, 0.5, 1.0, XcafColorType::Gen));
    // 5e-5 is within 1e-4 tolerance
    let found = tool.find_color([0.5 + 5e-5, 0.5 - 5e-5, 0.5, 1.0]);
    assert_eq!(found, Some(0));
}

#[test]
fn test_tool_find_color_outside_tolerance_returns_none() {
    let mut tool = XcafColorTool::new();
    tool.add_color(XcafColorEntry::new("a", 0.5, 0.5, 0.5, 1.0, XcafColorType::Gen));
    // 2e-4 exceeds 1e-4 tolerance
    let found = tool.find_color([0.5 + 2e-4, 0.5, 0.5, 1.0]);
    assert!(found.is_none());
}

#[test]
fn test_tool_find_color_returns_first_match() {
    let mut tool = XcafColorTool::new();
    tool.add_color(XcafColorEntry::new("first", 1.0, 0.0, 0.0, 1.0, XcafColorType::Gen));
    tool.add_color(XcafColorEntry::new("second", 1.0, 0.0, 0.0, 1.0, XcafColorType::Surf));
    let found = tool.find_color([1.0, 0.0, 0.0, 1.0]);
    assert_eq!(found, Some(0));
}

#[test]
fn test_tool_find_color_no_entries_returns_none() {
    let tool = XcafColorTool::new();
    assert!(tool.find_color([0.0, 0.0, 0.0, 1.0]).is_none());
}

#[test]
fn test_tool_remove_color_valid_index_returns_true() {
    let mut tool = XcafColorTool::new();
    tool.add_color(XcafColorEntry::new("del", 0.0, 0.0, 0.0, 1.0, XcafColorType::Gen));
    assert!(tool.remove_color(0));
}

#[test]
fn test_tool_remove_color_decrements_count() {
    let mut tool = XcafColorTool::new();
    tool.add_color(XcafColorEntry::new("a", 1.0, 0.0, 0.0, 1.0, XcafColorType::Gen));
    tool.add_color(XcafColorEntry::new("b", 0.0, 1.0, 0.0, 1.0, XcafColorType::Gen));
    tool.remove_color(0);
    assert_eq!(tool.nb_colors(), 1);
}

#[test]
fn test_tool_remove_color_invalid_index_returns_false() {
    let mut tool = XcafColorTool::new();
    assert!(!tool.remove_color(0));
}

#[test]
fn test_tool_remove_color_shifts_remaining_entries() {
    let mut tool = XcafColorTool::new();
    tool.add_color(XcafColorEntry::new("first", 1.0, 0.0, 0.0, 1.0, XcafColorType::Gen));
    tool.add_color(XcafColorEntry::new("second", 0.0, 1.0, 0.0, 1.0, XcafColorType::Gen));
    tool.remove_color(0);
    // "second" is now at index 0
    let e = tool.get_color(0).unwrap();
    assert_eq!(e.label(), "second");
}

#[test]
fn test_tool_default_is_empty() {
    let tool = XcafColorTool::default();
    assert_eq!(tool.nb_colors(), 0);
}

#[test]
fn test_tool_add_then_find_then_remove_round_trip() {
    let mut tool = XcafColorTool::new();
    let rgba = [0.25_f32, 0.50, 0.75, 1.0];
    let idx = tool.add_color(XcafColorEntry::new("layer", rgba[0], rgba[1], rgba[2], rgba[3], XcafColorType::Surf));
    let found = tool.find_color(rgba).unwrap();
    assert_eq!(found, idx);
    assert!(tool.remove_color(found));
    assert_eq!(tool.nb_colors(), 0);
    assert!(tool.find_color(rgba).is_none());
}
