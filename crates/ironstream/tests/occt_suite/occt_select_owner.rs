use ironstream::select_owner::*;

#[test]
fn entity_owner_defaults() {
    let o = SelectMgrEntityOwner::new(1, 10);
    assert!(!o.is_selected());
    assert!(!o.is_detected());
    assert_eq!(o.selectable_id(), 10);
    assert_eq!(o.shape_id(), 0);
    assert_eq!(o.priority(), 0);
}

#[test]
fn entity_owner_state_transitions() {
    let mut o = SelectMgrEntityOwner::new(1, 10);
    o.set_detected();
    assert!(o.is_detected());
    assert!(!o.is_selected());
    o.set_selected();
    assert!(o.is_selected());
    o.clear_state();
    assert!(!o.is_selected());
    assert!(!o.is_detected());
}

#[test]
fn selectable_object_modes_dedup_and_deactivate() {
    let mut obj = SelectMgrSelectableObject::new(1);
    obj.activate_mode(0);
    obj.activate_mode(1);
    obj.activate_mode(0); // duplicate — should not increase count
    assert_eq!(obj.selection_modes.len(), 2, "duplicate mode should not be added");
    obj.deactivate_mode(0);
    assert!(!obj.is_mode_active(0));
    assert!(obj.is_mode_active(1));
}

#[test]
fn selectable_object_owners_add_select_clear() {
    let mut obj = SelectMgrSelectableObject::new(1);
    let mut o1 = SelectMgrEntityOwner::new(1, 1).with_shape(10);
    o1.set_selected();
    obj.add_owner(o1);
    obj.add_owner(SelectMgrEntityOwner::new(2, 1).with_shape(20));
    assert_eq!(obj.nb_owners(), 2);
    assert_eq!(obj.selected_owners().len(), 1);
    obj.clear_selection();
    assert_eq!(obj.selected_owners().len(), 0);
}

#[test]
fn selection_mgr_activate_deactivate() {
    let mut mgr = SelectMgrSelectionMgr::new();
    mgr.add(SelectMgrSelectableObject::new(1));
    mgr.activate(1, 0);
    mgr.activate(1, 1);
    {
        let obj = mgr.find(1).unwrap();
        assert!(obj.is_mode_active(0));
        assert!(obj.is_mode_active(1));
    }
    mgr.deactivate(1, 0);
    let obj = mgr.find(1).unwrap();
    assert!(!obj.is_mode_active(0));
    assert!(obj.is_mode_active(1));
}

#[test]
fn selection_mgr_find_unknown_returns_none() {
    let mgr = SelectMgrSelectionMgr::new();
    assert!(mgr.find(999).is_none());
    assert_eq!(mgr.nb_objects(), 0);
}
