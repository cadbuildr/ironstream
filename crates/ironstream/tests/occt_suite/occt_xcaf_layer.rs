// FILE: rust/ironstream/crates/ironstream/tests/occt_xcaf_layer.rs

extern crate ironstream;
use ironstream::xcaf_layer::*;

// --- XcafLayer ---

#[test]
fn test_layer_new_stores_id_and_name() {
    let layer = XcafLayer::new(3, "walls");
    assert_eq!(layer.id(), 3);
    assert_eq!(layer.name(), "walls");
}

#[test]
fn test_layer_new_is_visible_by_default() {
    let layer = XcafLayer::new(0, "default");
    assert!(layer.is_visible());
}

#[test]
fn test_layer_set_visible_false() {
    let mut layer = XcafLayer::new(1, "hidden");
    layer.set_visible(false);
    assert!(!layer.is_visible());
}

#[test]
fn test_layer_set_visible_true_after_false() {
    let mut layer = XcafLayer::new(2, "toggled");
    layer.set_visible(false);
    layer.set_visible(true);
    assert!(layer.is_visible());
}

#[test]
fn test_layer_set_name_updates_name() {
    let mut layer = XcafLayer::new(0, "old");
    layer.set_name("new");
    assert_eq!(layer.name(), "new");
}

#[test]
fn test_layer_set_name_empty_string() {
    let mut layer = XcafLayer::new(0, "something");
    layer.set_name("");
    assert_eq!(layer.name(), "");
}

#[test]
fn test_layer_clone_is_independent() {
    let layer = XcafLayer::new(7, "original");
    let mut cloned = layer.clone();
    cloned.set_name("clone");
    cloned.set_visible(false);
    assert_eq!(layer.name(), "original");
    assert!(layer.is_visible());
}

#[test]
fn test_layer_id_unchanged_after_mutation() {
    let mut layer = XcafLayer::new(42, "test");
    layer.set_name("renamed");
    layer.set_visible(false);
    assert_eq!(layer.id(), 42);
}

// --- XcafLayerTool ---

#[test]
fn test_tool_new_is_empty() {
    let tool = XcafLayerTool::new();
    assert_eq!(tool.nb_layers(), 0);
}

#[test]
fn test_tool_default_is_empty() {
    let tool = XcafLayerTool::default();
    assert_eq!(tool.nb_layers(), 0);
}

#[test]
fn test_tool_add_layer_returns_id() {
    let mut tool = XcafLayerTool::new();
    let id = tool.add_layer("layer0");
    assert_eq!(tool.nb_layers(), 1);
    let layer = tool.get_layer(id).unwrap();
    assert_eq!(layer.name(), "layer0");
}

#[test]
fn test_tool_add_multiple_layers_returns_distinct_ids() {
    let mut tool = XcafLayerTool::new();
    let id0 = tool.add_layer("a");
    let id1 = tool.add_layer("b");
    let id2 = tool.add_layer("c");
    assert_ne!(id0, id1);
    assert_ne!(id1, id2);
    assert_ne!(id0, id2);
}

#[test]
fn test_tool_nb_layers_increments() {
    let mut tool = XcafLayerTool::new();
    assert_eq!(tool.nb_layers(), 0);
    tool.add_layer("x");
    assert_eq!(tool.nb_layers(), 1);
    tool.add_layer("y");
    assert_eq!(tool.nb_layers(), 2);
}

#[test]
fn test_tool_find_layer_by_name() {
    let mut tool = XcafLayerTool::new();
    let id = tool.add_layer("walls");
    let found = tool.find_layer("walls");
    assert_eq!(found, Some(id));
}

#[test]
fn test_tool_find_layer_not_found_returns_none() {
    let mut tool = XcafLayerTool::new();
    tool.add_layer("floors");
    assert!(tool.find_layer("ceilings").is_none());
}

#[test]
fn test_tool_find_layer_empty_tool_returns_none() {
    let tool = XcafLayerTool::new();
    assert!(tool.find_layer("anything").is_none());
}

#[test]
fn test_tool_find_layer_returns_first_match() {
    let mut tool = XcafLayerTool::new();
    let id0 = tool.add_layer("dup");
    tool.add_layer("dup");
    let found = tool.find_layer("dup");
    assert_eq!(found, Some(id0));
}

#[test]
fn test_tool_get_layer_valid_id() {
    let mut tool = XcafLayerTool::new();
    let id = tool.add_layer("roofs");
    let layer = tool.get_layer(id).unwrap();
    assert_eq!(layer.id(), id);
    assert_eq!(layer.name(), "roofs");
}

#[test]
fn test_tool_get_layer_invalid_id_returns_none() {
    let tool = XcafLayerTool::new();
    assert!(tool.get_layer(999).is_none());
}

#[test]
fn test_tool_remove_layer_returns_true_when_present() {
    let mut tool = XcafLayerTool::new();
    let id = tool.add_layer("temp");
    assert!(tool.remove_layer(id));
}

#[test]
fn test_tool_remove_layer_decrements_count() {
    let mut tool = XcafLayerTool::new();
    let id = tool.add_layer("bye");
    tool.remove_layer(id);
    assert_eq!(tool.nb_layers(), 0);
}

#[test]
fn test_tool_remove_layer_returns_false_when_absent() {
    let mut tool = XcafLayerTool::new();
    assert!(!tool.remove_layer(0));
}

#[test]
fn test_tool_remove_layer_get_returns_none_afterwards() {
    let mut tool = XcafLayerTool::new();
    let id = tool.add_layer("gone");
    tool.remove_layer(id);
    assert!(tool.get_layer(id).is_none());
}

#[test]
fn test_tool_remove_layer_leaves_others_intact() {
    let mut tool = XcafLayerTool::new();
    let id0 = tool.add_layer("keep");
    let id1 = tool.add_layer("remove");
    tool.remove_layer(id1);
    assert!(tool.get_layer(id0).is_some());
    assert!(tool.get_layer(id1).is_none());
}

// --- shape_layers associations ---

#[test]
fn test_set_layer_for_shape_and_retrieve() {
    let mut tool = XcafLayerTool::new();
    let lid = tool.add_layer("doors");
    tool.set_layer_for_shape("shape:0:1:2", lid);
    assert_eq!(tool.layer_for_shape("shape:0:1:2"), Some(lid));
}

#[test]
fn test_layer_for_shape_unknown_label_returns_none() {
    let tool = XcafLayerTool::new();
    assert!(tool.layer_for_shape("unknown").is_none());
}

#[test]
fn test_set_layer_for_shape_replaces_prior_association() {
    let mut tool = XcafLayerTool::new();
    let lid0 = tool.add_layer("first");
    let lid1 = tool.add_layer("second");
    tool.set_layer_for_shape("s:1", lid0);
    tool.set_layer_for_shape("s:1", lid1);
    assert_eq!(tool.layer_for_shape("s:1"), Some(lid1));
}

#[test]
fn test_shapes_in_layer_returns_associated_labels() {
    let mut tool = XcafLayerTool::new();
    let lid = tool.add_layer("windows");
    tool.set_layer_for_shape("shape:1", lid);
    tool.set_layer_for_shape("shape:2", lid);
    let mut shapes = tool.shapes_in_layer(lid);
    shapes.sort();
    assert_eq!(shapes, vec!["shape:1", "shape:2"]);
}

#[test]
fn test_shapes_in_layer_empty_when_none_assigned() {
    let mut tool = XcafLayerTool::new();
    let lid = tool.add_layer("empty_layer");
    assert!(tool.shapes_in_layer(lid).is_empty());
}

#[test]
fn test_shapes_in_layer_does_not_include_other_layers() {
    let mut tool = XcafLayerTool::new();
    let lid0 = tool.add_layer("A");
    let lid1 = tool.add_layer("B");
    tool.set_layer_for_shape("s:1", lid0);
    tool.set_layer_for_shape("s:2", lid1);
    let in_a = tool.shapes_in_layer(lid0);
    assert_eq!(in_a, vec!["s:1"]);
    let in_b = tool.shapes_in_layer(lid1);
    assert_eq!(in_b, vec!["s:2"]);
}

#[test]
fn test_remove_layer_also_removes_shape_associations() {
    let mut tool = XcafLayerTool::new();
    let lid = tool.add_layer("transient");
    tool.set_layer_for_shape("s:99", lid);
    tool.remove_layer(lid);
    assert!(tool.layer_for_shape("s:99").is_none());
}

#[test]
fn test_round_trip_add_find_assign_shapes_remove() {
    let mut tool = XcafLayerTool::new();
    let lid = tool.add_layer("round_trip");
    assert_eq!(tool.find_layer("round_trip"), Some(lid));
    tool.set_layer_for_shape("label:1", lid);
    tool.set_layer_for_shape("label:2", lid);
    assert_eq!(tool.shapes_in_layer(lid).len(), 2);
    tool.remove_layer(lid);
    assert_eq!(tool.nb_layers(), 0);
    assert!(tool.find_layer("round_trip").is_none());
    assert!(tool.shapes_in_layer(lid).is_empty());
}
