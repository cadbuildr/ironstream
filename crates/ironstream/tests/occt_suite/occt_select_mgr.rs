// FILE: tests/occt_select_mgr.rs
extern crate ironstream;

use ironstream::select_mgr::{
    SelectionMode, SelectionState, SensitiveEntity,
    SelectMgrSelection, SensitiveEntitySet, SelectMgrSelectionManager,
};

#[test]
fn selection_mode_constants() {
    assert_eq!(SelectionMode::SHAPE.value(), 0);
    assert_eq!(SelectionMode::VERTEX.value(), 1);
    assert_eq!(SelectionMode::EDGE.value(), 2);
    assert_eq!(SelectionMode::FACE.value(), 4);
    assert_eq!(SelectionMode::SOLID.value(), 6);
}

#[test]
fn sensitive_entity_activate_deactivate() {
    let mut e = SensitiveEntity::new(42, SelectionMode::FACE);
    assert!(e.is_active);
    e.deactivate();
    assert!(!e.is_active);
    e.activate();
    assert!(e.is_active);
}

#[test]
fn selection_add_and_query() {
    let mut sel = SelectMgrSelection::new(SelectionMode::EDGE);
    assert!(sel.is_empty());
    sel.add(SensitiveEntity::new(1, SelectionMode::EDGE));
    sel.add(SensitiveEntity::new(2, SelectionMode::EDGE));
    assert_eq!(sel.nb_sensitive(), 2);
    assert_eq!(sel.sensitive(0).map(|e| e.entity_id), Some(1));
    assert!(sel.sensitive(10).is_none());
}

#[test]
fn selection_state_transitions() {
    let mut sel = SelectMgrSelection::new(SelectionMode::WIRE);
    assert_eq!(sel.state(), SelectionState::Off);
    sel.set_state(SelectionState::On);
    assert_eq!(sel.state(), SelectionState::On);
    sel.set_state(SelectionState::Waiting);
    assert_eq!(sel.state(), SelectionState::Waiting);
}

#[test]
fn sensitive_entity_set_bvh_and_remove() {
    let mut set = SensitiveEntitySet::new();
    set.add(SensitiveEntity::new(10, SelectionMode::FACE));
    set.add(SensitiveEntity::new(11, SelectionMode::FACE));
    assert_eq!(set.size(), 2);
    assert!(!set.bvh_built);
    set.build_bvh();
    assert!(set.bvh_built);
    let removed = set.remove_by_id(10);
    assert!(removed);
    assert_eq!(set.size(), 1);
    assert!(!set.bvh_built); // invalidated
    assert!(!set.remove_by_id(99)); // non-existent
}

#[test]
fn selection_manager_activate_deactivate_remove() {
    let mut mgr = SelectMgrSelectionManager::new();
    mgr.activate(1, SelectionMode::FACE);
    mgr.activate(1, SelectionMode::EDGE);
    assert!(mgr.is_activated(1, SelectionMode::FACE));
    assert!(mgr.is_activated(1, SelectionMode::EDGE));
    assert_eq!(mgr.nb_selected_modes(1), 2);
    mgr.deactivate(1, SelectionMode::FACE);
    assert!(!mgr.is_activated(1, SelectionMode::FACE));
    assert_eq!(mgr.nb_objects(), 1);
    assert!(mgr.remove(1));
    assert_eq!(mgr.nb_objects(), 0);
}
