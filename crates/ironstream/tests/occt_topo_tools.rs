// FILE: crates/ironstream/tests/occt_topo_tools.rs
extern crate ironstream;

use ironstream::topo_tools::{
    TopToolsHSequenceOfShape, TopToolsListOfShape, TopToolsMapOfShape, TopToolsSequenceOfShape,
    TopToolsShapeSet,
};

// --- TopToolsMapOfShape ---

#[test]
fn map_add_returns_true_when_new() {
    let mut m = TopToolsMapOfShape::new();
    assert!(m.add("Solid"));
}

#[test]
fn map_add_returns_false_when_duplicate() {
    let mut m = TopToolsMapOfShape::new();
    m.add("Face");
    assert!(!m.add("Face"));
}

#[test]
fn map_contains_after_add() {
    let mut m = TopToolsMapOfShape::new();
    m.add("Edge");
    assert!(m.contains("Edge"));
    assert!(!m.contains("Wire"));
}

#[test]
fn map_remove_existing() {
    let mut m = TopToolsMapOfShape::new();
    m.add("Vertex");
    assert!(m.remove("Vertex"));
    assert!(!m.contains("Vertex"));
}

#[test]
fn map_remove_missing_returns_false() {
    let mut m = TopToolsMapOfShape::new();
    assert!(!m.remove("Shell"));
}

#[test]
fn map_size_and_clear() {
    let mut m = TopToolsMapOfShape::new();
    m.add("A");
    m.add("B");
    m.add("C");
    assert_eq!(m.size(), 3);
    m.clear();
    assert_eq!(m.size(), 0);
    assert!(m.is_empty());
}

// --- TopToolsListOfShape ---

#[test]
fn list_append_and_first_last() {
    let mut l = TopToolsListOfShape::new();
    l.append("Alpha".to_string());
    l.append("Beta".to_string());
    assert_eq!(l.first(), Some("Alpha"));
    assert_eq!(l.last(), Some("Beta"));
}

#[test]
fn list_prepend_puts_at_front() {
    let mut l = TopToolsListOfShape::new();
    l.append("Second".to_string());
    l.prepend("First".to_string());
    assert_eq!(l.first(), Some("First"));
    assert_eq!(l.size(), 2);
}

#[test]
fn list_remove_first_returns_and_shrinks() {
    let mut l = TopToolsListOfShape::new();
    l.append("X".to_string());
    l.append("Y".to_string());
    let removed = l.remove_first();
    assert_eq!(removed, Some("X".to_string()));
    assert_eq!(l.size(), 1);
    assert_eq!(l.first(), Some("Y"));
}

#[test]
fn list_remove_first_empty_returns_none() {
    let mut l = TopToolsListOfShape::new();
    assert_eq!(l.remove_first(), None);
}

#[test]
fn list_is_empty_and_clear() {
    let mut l = TopToolsListOfShape::new();
    assert!(l.is_empty());
    l.append("Z".to_string());
    assert!(!l.is_empty());
    l.clear();
    assert!(l.is_empty());
}

// --- TopToolsSequenceOfShape ---

#[test]
fn seq_value_is_one_based() {
    let mut s = TopToolsSequenceOfShape::new();
    s.append("First".to_string());
    s.append("Second".to_string());
    assert_eq!(s.value(1), Some("First"));
    assert_eq!(s.value(2), Some("Second"));
    assert_eq!(s.value(0), None);
    assert_eq!(s.value(3), None);
}

#[test]
fn seq_set_value_replaces() {
    let mut s = TopToolsSequenceOfShape::new();
    s.append("Old".to_string());
    s.set_value(1, "New".to_string());
    assert_eq!(s.value(1), Some("New"));
}

#[test]
fn seq_remove_one_based() {
    let mut s = TopToolsSequenceOfShape::new();
    s.append("A".to_string());
    s.append("B".to_string());
    s.append("C".to_string());
    s.remove(2);
    assert_eq!(s.length(), 2);
    assert_eq!(s.value(1), Some("A"));
    assert_eq!(s.value(2), Some("C"));
}

#[test]
fn seq_prepend_and_first_last() {
    let mut s = TopToolsSequenceOfShape::new();
    s.append("End".to_string());
    s.prepend("Start".to_string());
    assert_eq!(s.first(), Some("Start"));
    assert_eq!(s.last(), Some("End"));
}

#[test]
fn hseq_delegates_correctly() {
    let mut h = TopToolsHSequenceOfShape::new();
    h.append("P".to_string());
    h.append("Q".to_string());
    assert_eq!(h.length(), 2);
    assert_eq!(h.value(1), Some("P"));
    h.remove(1);
    assert_eq!(h.value(1), Some("Q"));
    assert_eq!(h.length(), 1);
}

// --- TopToolsShapeSet ---

#[test]
fn shape_set_add_returns_one_based_index() {
    let mut ss = TopToolsShapeSet::new();
    let i1 = ss.add("Shell".to_string());
    let i2 = ss.add("Solid".to_string());
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
}

#[test]
fn shape_set_add_duplicate_returns_existing_index() {
    let mut ss = TopToolsShapeSet::new();
    ss.add("Face".to_string());
    let idx = ss.add("Face".to_string());
    assert_eq!(idx, 1);
    assert_eq!(ss.nb_shapes(), 1);
}

#[test]
fn shape_set_find_and_shape() {
    let mut ss = TopToolsShapeSet::new();
    ss.add("Wire".to_string());
    ss.add("Edge".to_string());
    assert_eq!(ss.find("Wire"), Some(1));
    assert_eq!(ss.find("Edge"), Some(2));
    assert_eq!(ss.find("Missing"), None);
    assert_eq!(ss.shape(1), Some("Wire"));
    assert_eq!(ss.shape(2), Some("Edge"));
    assert_eq!(ss.shape(0), None);
    assert_eq!(ss.shape(3), None);
}

#[test]
fn shape_set_clear_resets() {
    let mut ss = TopToolsShapeSet::new();
    ss.add("X".to_string());
    ss.clear();
    assert_eq!(ss.nb_shapes(), 0);
    assert_eq!(ss.find("X"), None);
}
