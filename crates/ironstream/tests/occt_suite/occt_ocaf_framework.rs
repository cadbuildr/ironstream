// FILE: tests/occt_ocaf_framework.rs
extern crate ironstream;

use ironstream::ocaf_framework::{
    TdfLabel, TdfData, TdfAttribute,
    TDataStdName, TDataStdInteger, TDataStdReal,
    TDocStdDocument,
};

// ---------------------------------------------------------------------------
// TdfLabel structural tests
// ---------------------------------------------------------------------------

#[test]
fn root_label_properties() {
    let root = TdfLabel::new_root();
    assert!(root.is_root());
    assert_eq!(root.depth(), 0);
    assert!(!root.is_null());
    assert!(root.parent().is_none());
}

#[test]
fn child_label_has_correct_depth_and_tag() {
    let root = TdfLabel::new_root();
    let child = root.child(5);
    assert!(!child.is_root());
    assert_eq!(child.depth(), 1);
    assert_eq!(child.tag(), 5);
}

#[test]
fn nested_child_path() {
    let root = TdfLabel::new_root();
    let a = root.child(1);
    let b = a.child(2);
    let c = b.child(3);
    assert_eq!(c.depth(), 3);
    assert_eq!(c.tag(), 3);
    assert_eq!(c.parent().unwrap().tag(), 2);
}

#[test]
fn parent_chain_resolves_to_root() {
    let label = TdfLabel::new_root().child(10).child(20).child(30);
    let p1 = label.parent().unwrap();
    let p2 = p1.parent().unwrap();
    let p3 = p2.parent().unwrap();
    assert!(p3.is_root());
    assert!(p3.parent().is_none());
}

#[test]
fn is_descendant_of_various_depths() {
    let root = TdfLabel::new_root();
    let lvl1 = root.child(1);
    let lvl2 = lvl1.child(1);
    let lvl3 = lvl2.child(1);

    assert!(lvl3.is_descendant_of(&root));
    assert!(lvl3.is_descendant_of(&lvl1));
    assert!(lvl3.is_descendant_of(&lvl2));
    assert!(!lvl3.is_descendant_of(&lvl3));
    assert!(!lvl2.is_descendant_of(&lvl3));
    assert!(!root.is_descendant_of(&lvl1));
}

#[test]
fn sibling_labels_not_descendants_of_each_other() {
    let root = TdfLabel::new_root();
    let a = root.child(1);
    let b = root.child(2);
    assert!(!a.is_descendant_of(&b));
    assert!(!b.is_descendant_of(&a));
}

// ---------------------------------------------------------------------------
// Attribute type name tests
// ---------------------------------------------------------------------------

#[test]
fn attribute_type_names_are_correct() {
    let name = TDataStdName::new("hello");
    let int_attr = TDataStdInteger { value: 0 };
    let real_attr = TDataStdReal { value: 0.0 };
    assert_eq!(name.attribute_type_name(), "TDataStd_Name");
    assert_eq!(int_attr.attribute_type_name(), "TDataStd_Integer");
    assert_eq!(real_attr.attribute_type_name(), "TDataStd_Real");
}

// ---------------------------------------------------------------------------
// TdfData attachment and retrieval
// ---------------------------------------------------------------------------

#[test]
fn set_and_get_name_attribute() {
    let mut data = TdfData::new();
    let label = data.root().child(1);
    data.set_attribute(&label, TDataStdName::new("Shaft"));
    let attr = data.get_attribute(&label, "TDataStd_Name");
    assert!(attr.is_some());
}

#[test]
fn multiple_attributes_on_same_label() {
    let mut data = TdfData::new();
    let label = data.root().child(1);
    data.set_attribute(&label, TDataStdName::new("Gear"));
    data.set_attribute(&label, TDataStdInteger { value: 7 });
    data.set_attribute(&label, TDataStdReal { value: 2.5 });
    assert!(data.has_attribute(&label, "TDataStd_Name"));
    assert!(data.has_attribute(&label, "TDataStd_Integer"));
    assert!(data.has_attribute(&label, "TDataStd_Real"));
}

#[test]
fn attribute_on_different_labels_are_independent() {
    let mut data = TdfData::new();
    let la = data.root().child(1);
    let lb = data.root().child(2);
    data.set_attribute(&la, TDataStdName::new("A"));
    assert!(data.has_attribute(&la, "TDataStd_Name"));
    assert!(!data.has_attribute(&lb, "TDataStd_Name"));
}

#[test]
fn replacing_attribute_overwrites() {
    let mut data = TdfData::new();
    let label = data.root().child(1);
    data.set_attribute(&label, TDataStdInteger { value: 10 });
    data.set_attribute(&label, TDataStdInteger { value: 99 });
    // Should still have exactly one Integer attribute (the map key is stable).
    assert!(data.has_attribute(&label, "TDataStd_Integer"));
}

// ---------------------------------------------------------------------------
// TDocStdDocument tests
// ---------------------------------------------------------------------------

#[test]
fn document_format_and_main_label() {
    let doc = TDocStdDocument::new("XmlOCAF");
    assert_eq!(doc.format(), "XmlOCAF");
    let ml = doc.main_label();
    assert_eq!(ml.depth(), 1);
    assert_eq!(ml.tag(), 1);
}

#[test]
fn document_data_allows_attribute_attachment() {
    let mut doc = TDocStdDocument::new("BinOCAF");
    let ml = doc.main_label();
    doc.data_mut().set_attribute(&ml, TDataStdName::new("MyDocument"));
    assert!(doc.data().has_attribute(&ml, "TDataStd_Name"));
}
