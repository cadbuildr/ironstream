// FILE: rust/ironstream/crates/ironstream/tests/occt_tdf_label.rs
extern crate ironstream;
use ironstream::tdf_label::*;

// ---------------------------------------------------------------------------
// TdfEntry tests
// ---------------------------------------------------------------------------

#[test]
fn entry_new_stores_tag_entry_name() {
    let e = TdfEntry::new(1, "0:1", "MyLabel");
    assert_eq!(e.tag(), 1);
    assert_eq!(e.entry(), "0:1");
    assert_eq!(e.name(), "MyLabel");
}

#[test]
fn entry_is_null_when_name_empty() {
    let e = TdfEntry::new(0, "0", "");
    assert!(e.is_null());
}

#[test]
fn entry_is_not_null_when_name_set() {
    let e = TdfEntry::new(1, "0:1", "Part");
    assert!(!e.is_null());
}

#[test]
fn entry_set_name_changes_name() {
    let mut e = TdfEntry::new(1, "0:1", "");
    assert!(e.is_null());
    e.set_name("Assembly");
    assert_eq!(e.name(), "Assembly");
    assert!(!e.is_null());
}

#[test]
fn entry_add_child_appended_to_children() {
    let mut e = TdfEntry::new(0, "0", "root");
    e.add_child(1);
    e.add_child(2);
    assert_eq!(e.children(), &[1, 2]);
}

#[test]
fn entry_add_child_deduplicated() {
    let mut e = TdfEntry::new(0, "0", "root");
    e.add_child(5);
    e.add_child(5);
    assert_eq!(e.children().len(), 1);
}

#[test]
fn entry_children_empty_initially() {
    let e = TdfEntry::new(1, "0:1", "Leaf");
    assert!(e.children().is_empty());
}

#[test]
fn entry_set_and_get_attr() {
    let mut e = TdfEntry::new(1, "0:1", "Node");
    e.set_attr("color", "red");
    assert_eq!(e.get_attr("color"), Some("red"));
}

#[test]
fn entry_get_attr_missing_returns_none() {
    let e = TdfEntry::new(1, "0:1", "Node");
    assert!(e.get_attr("shape").is_none());
}

#[test]
fn entry_set_attr_overwrites_existing() {
    let mut e = TdfEntry::new(1, "0:1", "Node");
    e.set_attr("color", "red");
    e.set_attr("color", "blue");
    assert_eq!(e.get_attr("color"), Some("blue"));
}

#[test]
fn entry_nb_attrs_counts_unique_keys() {
    let mut e = TdfEntry::new(1, "0:1", "Node");
    assert_eq!(e.nb_attrs(), 0);
    e.set_attr("a", "1");
    e.set_attr("b", "2");
    assert_eq!(e.nb_attrs(), 2);
    // Overwrite does not add a new entry.
    e.set_attr("a", "99");
    assert_eq!(e.nb_attrs(), 2);
}

// ---------------------------------------------------------------------------
// TdfData tests
// ---------------------------------------------------------------------------

#[test]
fn data_new_has_root_and_one_label() {
    let data = TdfData::new();
    assert_eq!(data.nb_labels(), 1);
}

#[test]
fn data_root_entry_is_zero() {
    let data = TdfData::new();
    assert_eq!(data.root().entry(), "0");
}

#[test]
fn data_root_is_null_by_default() {
    let data = TdfData::new();
    assert!(data.root().is_null());
}

#[test]
fn data_new_label_under_root_returns_entry() {
    let mut data = TdfData::new();
    let e = data.new_label("0", "Part");
    assert!(!e.is_empty());
    assert!(e.starts_with("0:"));
}

#[test]
fn data_new_label_increments_nb_labels() {
    let mut data = TdfData::new();
    data.new_label("0", "A");
    data.new_label("0", "B");
    assert_eq!(data.nb_labels(), 3);
}

#[test]
fn data_new_label_registers_child_on_parent() {
    let mut data = TdfData::new();
    let child_entry = data.new_label("0", "Child");
    let root = data.root();
    let child_tag: u32 = child_entry.split(':').last().unwrap().parse().unwrap();
    assert!(root.children().contains(&child_tag));
}

#[test]
fn data_new_label_nested_entry_format() {
    let mut data = TdfData::new();
    let l1 = data.new_label("0", "Level1");
    let l2 = data.new_label(&l1, "Level2");
    // Should be "0:1:2" style — starts with parent entry.
    assert!(l2.starts_with(&format!("{}:", l1)));
}

#[test]
fn data_new_label_unknown_parent_returns_empty() {
    let mut data = TdfData::new();
    let e = data.new_label("99:99", "Orphan");
    assert!(e.is_empty());
    // No label was added.
    assert_eq!(data.nb_labels(), 1);
}

#[test]
fn data_find_label_root() {
    let data = TdfData::new();
    let label = data.find_label("0");
    assert!(label.is_some());
    assert_eq!(label.unwrap().entry(), "0");
}

#[test]
fn data_find_label_returns_none_for_missing() {
    let data = TdfData::new();
    assert!(data.find_label("0:99").is_none());
}

#[test]
fn data_find_label_after_new_label() {
    let mut data = TdfData::new();
    let e = data.new_label("0", "Solid");
    let label = data.find_label(&e);
    assert!(label.is_some());
    assert_eq!(label.unwrap().name(), "Solid");
}

#[test]
fn data_find_label_mut_allows_rename() {
    let mut data = TdfData::new();
    let e = data.new_label("0", "Old");
    {
        let label = data.find_label_mut(&e).unwrap();
        label.set_name("New");
    }
    assert_eq!(data.find_label(&e).unwrap().name(), "New");
}

#[test]
fn data_set_attr_returns_true_on_success() {
    let mut data = TdfData::new();
    let e = data.new_label("0", "Node");
    assert!(data.set_attr(&e, "material", "steel"));
}

#[test]
fn data_set_attr_returns_false_for_missing_entry() {
    let mut data = TdfData::new();
    assert!(!data.set_attr("0:99", "x", "y"));
}

#[test]
fn data_set_attr_value_readable_via_find_label() {
    let mut data = TdfData::new();
    let e = data.new_label("0", "Node");
    data.set_attr(&e, "density", "7.8");
    let label = data.find_label(&e).unwrap();
    assert_eq!(label.get_attr("density"), Some("7.8"));
}

#[test]
fn data_multiple_levels_deep() {
    let mut data = TdfData::new();
    let l1 = data.new_label("0", "L1");
    let l2 = data.new_label(&l1, "L2");
    let l3 = data.new_label(&l2, "L3");
    assert_eq!(data.nb_labels(), 4); // root + 3
    assert!(data.find_label(&l3).is_some());
    assert_eq!(data.find_label(&l3).unwrap().name(), "L3");
}

#[test]
fn data_default_equals_new() {
    let d1 = TdfData::new();
    let d2 = TdfData::default();
    assert_eq!(d1.nb_labels(), d2.nb_labels());
    assert_eq!(d1.root().entry(), d2.root().entry());
}
