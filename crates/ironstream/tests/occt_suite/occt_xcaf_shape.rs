// FILE: rust/ironstream/crates/ironstream/tests/occt_xcaf_shape.rs

extern crate ironstream;
use ironstream::xcaf_shape::*;

// --- XcafShapeEntry ---

#[test]
fn test_entry_new_stores_label_and_name() {
    let e = XcafShapeEntry::new("0:1:1:1", "Bolt");
    assert_eq!(e.label(), "0:1:1:1");
    assert_eq!(e.name(), "Bolt");
}

#[test]
fn test_entry_new_default_flags_are_false() {
    let e = XcafShapeEntry::new("0:1", "Root");
    assert!(!e.is_free());
    assert!(!e.is_simple());
    assert!(!e.is_assembly());
    assert!(!e.is_component());
    assert!(!e.is_compound());
}

#[test]
fn test_entry_new_default_nb_components_is_zero() {
    let e = XcafShapeEntry::new("0:1", "Root");
    assert_eq!(e.nb_components(), 0);
}

#[test]
fn test_entry_mark_as_free() {
    let mut e = XcafShapeEntry::new("0:1:1", "Part");
    e.mark_as_free();
    assert!(e.is_free());
}

#[test]
fn test_entry_mark_as_assembly() {
    let mut e = XcafShapeEntry::new("0:1:1", "Assy");
    e.mark_as_assembly();
    assert!(e.is_assembly());
}

#[test]
fn test_entry_mark_as_free_does_not_affect_assembly() {
    let mut e = XcafShapeEntry::new("0:1:1", "X");
    e.mark_as_free();
    assert!(!e.is_assembly());
}

#[test]
fn test_entry_mark_as_assembly_does_not_affect_free() {
    let mut e = XcafShapeEntry::new("0:1:1", "X");
    e.mark_as_assembly();
    assert!(!e.is_free());
}

#[test]
fn test_entry_set_nb_components() {
    let mut e = XcafShapeEntry::new("0:1:1", "Assy");
    e.set_nb_components(5);
    assert_eq!(e.nb_components(), 5);
}

#[test]
fn test_entry_set_nb_components_zero() {
    let mut e = XcafShapeEntry::new("0:1:1", "Assy");
    e.set_nb_components(3);
    e.set_nb_components(0);
    assert_eq!(e.nb_components(), 0);
}

#[test]
fn test_entry_clone_preserves_all_fields() {
    let mut e = XcafShapeEntry::new("0:1:2", "Widget");
    e.mark_as_free();
    e.mark_as_assembly();
    e.set_nb_components(4);
    let e2 = e.clone();
    assert_eq!(e2.label(), "0:1:2");
    assert_eq!(e2.name(), "Widget");
    assert!(e2.is_free());
    assert!(e2.is_assembly());
    assert_eq!(e2.nb_components(), 4);
}

#[test]
fn test_entry_empty_label_and_name() {
    let e = XcafShapeEntry::new("", "");
    assert_eq!(e.label(), "");
    assert_eq!(e.name(), "");
}

// --- XcafShapeTool ---

#[test]
fn test_tool_new_is_empty() {
    let tool = XcafShapeTool::new();
    assert_eq!(tool.nb_shapes(), 0);
}

#[test]
fn test_tool_default_is_empty() {
    let tool = XcafShapeTool::default();
    assert_eq!(tool.nb_shapes(), 0);
}

#[test]
fn test_tool_add_shape_returns_zero_for_first() {
    let mut tool = XcafShapeTool::new();
    let idx = tool.add_shape(XcafShapeEntry::new("0:1:1", "A"));
    assert_eq!(idx, 0);
}

#[test]
fn test_tool_add_shape_returns_sequential_indices() {
    let mut tool = XcafShapeTool::new();
    let i0 = tool.add_shape(XcafShapeEntry::new("0:1:1", "A"));
    let i1 = tool.add_shape(XcafShapeEntry::new("0:1:2", "B"));
    let i2 = tool.add_shape(XcafShapeEntry::new("0:1:3", "C"));
    assert_eq!(i0, 0);
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
}

#[test]
fn test_tool_nb_shapes_increments() {
    let mut tool = XcafShapeTool::new();
    assert_eq!(tool.nb_shapes(), 0);
    tool.add_shape(XcafShapeEntry::new("0:1:1", "A"));
    assert_eq!(tool.nb_shapes(), 1);
    tool.add_shape(XcafShapeEntry::new("0:1:2", "B"));
    assert_eq!(tool.nb_shapes(), 2);
}

#[test]
fn test_tool_get_shape_valid_index() {
    let mut tool = XcafShapeTool::new();
    tool.add_shape(XcafShapeEntry::new("0:1:1", "Gear"));
    let e = tool.get_shape(0).unwrap();
    assert_eq!(e.label(), "0:1:1");
    assert_eq!(e.name(), "Gear");
}

#[test]
fn test_tool_get_shape_out_of_bounds_returns_none() {
    let tool = XcafShapeTool::new();
    assert!(tool.get_shape(0).is_none());
}

#[test]
fn test_tool_get_shape_second_entry() {
    let mut tool = XcafShapeTool::new();
    tool.add_shape(XcafShapeEntry::new("0:1:1", "First"));
    tool.add_shape(XcafShapeEntry::new("0:1:2", "Second"));
    assert_eq!(tool.get_shape(1).unwrap().name(), "Second");
}

#[test]
fn test_tool_find_shape_exact_label() {
    let mut tool = XcafShapeTool::new();
    tool.add_shape(XcafShapeEntry::new("0:1:1", "A"));
    assert_eq!(tool.find_shape("0:1:1"), Some(0));
}

#[test]
fn test_tool_find_shape_no_match_returns_none() {
    let mut tool = XcafShapeTool::new();
    tool.add_shape(XcafShapeEntry::new("0:1:1", "A"));
    assert!(tool.find_shape("0:1:99").is_none());
}

#[test]
fn test_tool_find_shape_empty_tool_returns_none() {
    let tool = XcafShapeTool::new();
    assert!(tool.find_shape("0:1:1").is_none());
}

#[test]
fn test_tool_find_shape_returns_first_match() {
    let mut tool = XcafShapeTool::new();
    tool.add_shape(XcafShapeEntry::new("same", "First"));
    tool.add_shape(XcafShapeEntry::new("same", "Second"));
    assert_eq!(tool.find_shape("same"), Some(0));
}

#[test]
fn test_tool_find_shape_second_entry() {
    let mut tool = XcafShapeTool::new();
    tool.add_shape(XcafShapeEntry::new("0:1:1", "A"));
    tool.add_shape(XcafShapeEntry::new("0:1:2", "B"));
    assert_eq!(tool.find_shape("0:1:2"), Some(1));
}

#[test]
fn test_tool_free_shapes_empty_when_none_free() {
    let mut tool = XcafShapeTool::new();
    tool.add_shape(XcafShapeEntry::new("0:1:1", "A"));
    tool.add_shape(XcafShapeEntry::new("0:1:2", "B"));
    assert!(tool.free_shapes().is_empty());
}

#[test]
fn test_tool_free_shapes_returns_correct_indices() {
    let mut tool = XcafShapeTool::new();
    let mut e0 = XcafShapeEntry::new("0:1:1", "A");
    e0.mark_as_free();
    tool.add_shape(e0);
    tool.add_shape(XcafShapeEntry::new("0:1:2", "B"));
    let mut e2 = XcafShapeEntry::new("0:1:3", "C");
    e2.mark_as_free();
    tool.add_shape(e2);
    let free = tool.free_shapes();
    assert_eq!(free, vec![0, 2]);
}

#[test]
fn test_tool_free_shapes_all_free() {
    let mut tool = XcafShapeTool::new();
    for i in 0..3usize {
        let mut e = XcafShapeEntry::new(format!("0:1:{}", i), format!("S{}", i));
        e.mark_as_free();
        tool.add_shape(e);
    }
    assert_eq!(tool.free_shapes(), vec![0, 1, 2]);
}

#[test]
fn test_tool_is_free_true() {
    let mut tool = XcafShapeTool::new();
    let mut e = XcafShapeEntry::new("0:1:1", "Free");
    e.mark_as_free();
    tool.add_shape(e);
    assert!(tool.is_free(0));
}

#[test]
fn test_tool_is_free_false() {
    let mut tool = XcafShapeTool::new();
    tool.add_shape(XcafShapeEntry::new("0:1:1", "Bound"));
    assert!(!tool.is_free(0));
}

#[test]
fn test_tool_is_free_out_of_bounds_returns_false() {
    let tool = XcafShapeTool::new();
    assert!(!tool.is_free(0));
}

#[test]
fn test_tool_remove_shape_valid_returns_true() {
    let mut tool = XcafShapeTool::new();
    tool.add_shape(XcafShapeEntry::new("0:1:1", "A"));
    assert!(tool.remove_shape(0));
}

#[test]
fn test_tool_remove_shape_decrements_count() {
    let mut tool = XcafShapeTool::new();
    tool.add_shape(XcafShapeEntry::new("0:1:1", "A"));
    tool.add_shape(XcafShapeEntry::new("0:1:2", "B"));
    tool.remove_shape(0);
    assert_eq!(tool.nb_shapes(), 1);
}

#[test]
fn test_tool_remove_shape_invalid_returns_false() {
    let mut tool = XcafShapeTool::new();
    assert!(!tool.remove_shape(0));
}

#[test]
fn test_tool_remove_shape_shifts_remaining() {
    let mut tool = XcafShapeTool::new();
    tool.add_shape(XcafShapeEntry::new("0:1:1", "First"));
    tool.add_shape(XcafShapeEntry::new("0:1:2", "Second"));
    tool.remove_shape(0);
    assert_eq!(tool.get_shape(0).unwrap().name(), "Second");
}

#[test]
fn test_tool_remove_shape_then_find_gone() {
    let mut tool = XcafShapeTool::new();
    tool.add_shape(XcafShapeEntry::new("0:1:1", "A"));
    tool.remove_shape(0);
    assert!(tool.find_shape("0:1:1").is_none());
}

#[test]
fn test_tool_add_find_get_remove_round_trip() {
    let mut tool = XcafShapeTool::new();
    let mut e = XcafShapeEntry::new("0:1:5", "Sprocket");
    e.mark_as_free();
    e.mark_as_assembly();
    e.set_nb_components(3);
    let idx = tool.add_shape(e);
    let found = tool.find_shape("0:1:5").unwrap();
    assert_eq!(found, idx);
    let fetched = tool.get_shape(found).unwrap();
    assert_eq!(fetched.name(), "Sprocket");
    assert!(fetched.is_free());
    assert!(fetched.is_assembly());
    assert_eq!(fetched.nb_components(), 3);
    assert!(tool.remove_shape(found));
    assert_eq!(tool.nb_shapes(), 0);
    assert!(tool.find_shape("0:1:5").is_none());
}
