// FILE: tests/occt_tdf_data.rs
extern crate ironstream;

use ironstream::tdf_data::{LabelTag, TdfData, TdfLabel, TdfTransaction};

#[test]
fn label_tag_child_depth_parent() {
    let root = LabelTag::root();
    assert_eq!(root.depth(), 0);
    assert!(root.parent().is_none());

    let child = root.child(1);
    assert_eq!(child.depth(), 1);
    assert_eq!(child.tag(), Some(1));
    assert_eq!(child.parent(), Some(root.clone()));

    let grandchild = child.child(3);
    assert_eq!(grandchild.depth(), 2);
    assert_eq!(grandchild.tag(), Some(3));
    assert_eq!(grandchild.parent(), Some(child));
}

#[test]
fn tdf_label_set_attr_get_attr() {
    let mut lbl = TdfLabel::new(LabelTag::root());
    assert!(lbl.is_root());
    assert_eq!(lbl.nb_attrs(), 0);
    lbl.set_attr("Name", "RootPart");
    lbl.set_attr("Color", "red");
    assert_eq!(lbl.get_attr("Name"), Some("RootPart"));
    assert!(lbl.has_attr("Color"));
    assert!(!lbl.has_attr("Missing"));
    assert_eq!(lbl.nb_attrs(), 2);
}

#[test]
fn tdf_data_new_label_find_label() {
    let mut data = TdfData::new();
    let root = LabelTag::root();
    let child1 = data.new_label(&root, 1);
    let child2 = data.new_label(&root, 2);
    let grandchild = data.new_label(&child1, 1);

    assert_eq!(child1.depth(), 1);
    assert!(data.find_label(&child1).is_some());
    assert!(data.find_label(&child2).is_some());
    assert!(data.find_label(&grandchild).is_some());
    assert_eq!(data.nb_labels, data.labels.len());
}

#[test]
fn tdf_transaction_commit() {
    let mut tx = TdfTransaction::new("add_part", 1);
    assert!(tx.is_open);
    tx.mark_modified(LabelTag::root().child(1));
    tx.mark_modified(LabelTag::root().child(2));
    // duplicate mark should not add twice
    tx.mark_modified(LabelTag::root().child(1));
    assert_eq!(tx.nb_modified(), 2);
    let delta = tx.commit();
    assert!(!tx.is_open);
    assert_eq!(delta.nb_changes(), 2);
}
