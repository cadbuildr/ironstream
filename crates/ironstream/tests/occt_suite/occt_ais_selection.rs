// FILE: tests/occt_ais_selection.rs
extern crate ironstream;
use ironstream::ais_selection::*;

#[test]
fn test_entity_owner_new() {
    let o = EntityOwner::new(1, 42);
    assert_eq!(o.owner_id, 1);
    assert_eq!(o.selectable_id, 42);
    assert!(!o.is_selected);
    assert_eq!(o.priority, 0);
}

#[test]
fn test_entity_owner_select_toggle() {
    let mut o = EntityOwner::new(5, 0).with_priority(3);
    assert_eq!(o.priority, 3);
    o.select();
    assert!(o.is_selected);
    o.toggle();
    assert!(!o.is_selected);
    o.toggle();
    assert!(o.is_selected);
    o.deselect();
    assert!(!o.is_selected);
}

#[test]
fn test_ais_selection_replace_scheme() {
    let mut sel = AisSelection::new();
    sel.select(EntityOwner::new(1, 0), SelectionScheme::Replace);
    sel.select(EntityOwner::new(2, 0), SelectionScheme::Replace);
    assert_eq!(sel.nb_selected(), 1);
    assert!(sel.is_selected(2));
    assert!(!sel.is_selected(1));
}

#[test]
fn test_ais_selection_add_and_xor() {
    let mut sel = AisSelection::new();
    sel.select(EntityOwner::new(10, 0), SelectionScheme::Add);
    sel.select(EntityOwner::new(20, 0), SelectionScheme::Add);
    sel.select(EntityOwner::new(10, 0), SelectionScheme::Add); // duplicate ignored
    assert_eq!(sel.nb_selected(), 2);

    sel.select(EntityOwner::new(10, 0), SelectionScheme::XOR); // toggle off
    assert!(!sel.is_selected(10));
    sel.select(EntityOwner::new(10, 0), SelectionScheme::XOR); // toggle on
    assert!(sel.is_selected(10));
}

#[test]
fn test_ais_selection_remove_and_clear() {
    let mut sel = AisSelection::new();
    sel.select(EntityOwner::new(1, 0), SelectionScheme::Add);
    sel.select(EntityOwner::new(2, 0), SelectionScheme::Add);
    sel.select(EntityOwner::new(1, 0), SelectionScheme::Remove);
    assert!(!sel.is_selected(1));
    assert_eq!(sel.nb_selected(), 1);
    sel.clear();
    assert!(sel.is_empty());
}

#[test]
fn test_indexed_map_of_owner() {
    let mut m = IndexedMapOfOwner::new();
    let idx1 = m.add(EntityOwner::new(100, 0));
    let idx2 = m.add(EntityOwner::new(200, 0));
    assert_eq!(m.extent(), 2);
    assert!(m.contains(100));
    assert_eq!(m.find_index(200), Some(idx2));
    let e = m.find_from_index(idx1).unwrap();
    assert_eq!(e.owner_id, 100);
    m.clear();
    assert_eq!(m.extent(), 0);
}
