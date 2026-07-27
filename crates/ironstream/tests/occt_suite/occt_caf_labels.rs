extern crate ironstream;
use ironstream::caf_labels::*;

#[test]
fn label_hierarchy() {
    let root = Label::root();
    assert!(root.is_root());
    assert_eq!(root.depth(), 0);
    let child = root.find_child(1, true).unwrap();
    assert_eq!(child.depth(), 1);
    assert_eq!(child.tag(), 1);
    let parent = child.father().unwrap();
    assert_eq!(parent, root);
}

#[test]
fn label_entry_format() {
    let l = Label::new(vec![0, 2, 4]);
    assert_eq!(l.entry(), "0:2:4");
    let root = Label::root();
    assert_eq!(root.entry(), "0");
}

#[test]
fn label_descendant_check() {
    let root = Label::root();
    let child = root.find_child(3, true).unwrap();
    let grandchild = child.find_child(7, true).unwrap();
    assert!(grandchild.is_descendant_of(&root));
    assert!(grandchild.is_descendant_of(&child));
    assert!(!root.is_descendant_of(&child));
}

#[test]
fn label_map_dedup() {
    let mut m = LabelMap::new();
    let a = Label::new(vec![0, 1]);
    assert!(m.add(a.clone()));
    assert!(!m.add(a.clone())); // duplicate rejected
    assert_eq!(m.extent(), 1);
    m.clear();
    assert_eq!(m.extent(), 0);
}

#[test]
fn tag_source_sequential() {
    let root = Label::root();
    let mut ts = TagSource::new();
    let c1 = ts.new_child_label(&root);
    let c2 = ts.new_child_label(&root);
    let c3 = ts.new_child_label(&root);
    assert_ne!(c1, c2);
    assert_ne!(c2, c3);
    assert_eq!(ts.get(), 3);
    ts.set(10);
    assert_eq!(ts.get(), 10);
}
