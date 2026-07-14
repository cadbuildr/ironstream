// Integration tests for caf_label
use ironstream::caf_label::*;

#[test]
fn tag_path_root_depth_and_child() {
    let root = TagPath::root();
    assert_eq!(root.depth(), 0);
    assert_eq!(root.tag(), None);

    let child = root.child(5);
    assert_eq!(child.depth(), 1);
    assert_eq!(child.tag(), Some(5));

    let grandchild = child.child(7);
    assert_eq!(grandchild.depth(), 2);
    assert_eq!(grandchild.tag(), Some(7));
}

#[test]
fn tag_path_father_path() {
    let path = TagPath::root().child(1).child(2);
    let father = path.father_path().expect("should have a father");
    assert_eq!(father.depth(), 1);
    assert_eq!(father.tag(), Some(1));

    // Root has no father
    assert!(TagPath::root().father_path().is_none());
}

#[test]
fn tdf_label_null_and_non_null() {
    let null = TdfLabel::null();
    assert!(null.is_null());
    assert_eq!(null.depth(), 0);
    assert_eq!(null.tag(), None);

    let label = TdfLabel::new(TagPath::root().child(3));
    assert!(!label.is_null());
    assert_eq!(label.depth(), 1);
    assert_eq!(label.tag(), Some(3));
}

#[test]
fn tdf_label_attributes_add_remove_has() {
    let mut label = TdfLabel::new(TagPath::root().child(1));
    assert!(!label.has_attribute(42));
    label.add_attribute(42);
    label.add_attribute(99);
    assert!(label.has_attribute(42));
    assert!(label.has_attribute(99));
    assert_eq!(label.nb_attributes(), 2);

    // Adding duplicate should not increase count
    label.add_attribute(42);
    assert_eq!(label.nb_attributes(), 2);

    label.remove_attribute(42);
    assert!(!label.has_attribute(42));
    assert_eq!(label.nb_attributes(), 1);
}

#[test]
fn tdf_label_descendant_and_equality() {
    let root = TdfLabel::new(TagPath::root());
    let child = TdfLabel::new(TagPath::root().child(1));
    let grandchild = TdfLabel::new(TagPath::root().child(1).child(2));
    let sibling = TdfLabel::new(TagPath::root().child(2));

    assert!(grandchild.is_descendant_of(&root));
    assert!(grandchild.is_descendant_of(&child));
    assert!(!grandchild.is_descendant_of(&sibling));
    assert!(!root.is_descendant_of(&grandchild));

    let child2 = TdfLabel::new(TagPath::root().child(1));
    assert!(child.is_equal(&child2));
    assert!(!child.is_equal(&sibling));
}

#[test]
fn tdf_label_sequence_and_tag_source() {
    // LabelSequence: 1-based indexing
    let mut seq = TdfLabelSequence::new();
    assert_eq!(seq.length(), 0);
    assert!(seq.value(1).is_none());

    seq.append(TdfLabel::new(TagPath::new(vec![10])));
    seq.append(TdfLabel::new(TagPath::new(vec![20])));
    assert_eq!(seq.length(), 2);
    assert_eq!(seq.value(1).unwrap().tag(), Some(10));
    assert_eq!(seq.value(2).unwrap().tag(), Some(20));
    assert!(seq.value(0).is_none()); // 0 is invalid

    // TagSource: generates sequential child tags
    let mut ts = TdfTagSource::new(TagPath::root());
    let c1 = ts.new_child_label();
    let c2 = ts.new_child_label();
    let c3 = ts.new_child_label();
    assert_eq!(c1.tag(), Some(1));
    assert_eq!(c2.tag(), Some(2));
    assert_eq!(c3.tag(), Some(3));
    assert_eq!(ts.last_tag(), 3);
}
