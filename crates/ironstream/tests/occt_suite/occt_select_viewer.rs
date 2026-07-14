use ironstream::select_viewer::*;

#[test]
fn selection_mode_values() {
    assert_eq!(SelectionMode::whole().0, 0);
    assert_eq!(SelectionMode::vertex().0, 1);
    assert_eq!(SelectionMode::edge().0, 2);
    assert_eq!(SelectionMode::face().0, 4);
    assert_eq!(SelectionMode::shell().0, 5);
    assert_eq!(SelectionMode::solid().0, 6);
}

#[test]
fn select_mgr_selection_activate_deactivate() {
    let mut sel = SelectMgrSelection::new(SelectionMode::face());
    assert!(!sel.is_active());
    sel.activate();
    assert!(sel.is_active());
    sel.deactivate();
    assert!(!sel.is_active());
}

#[test]
fn select_mgr_sensitive_entity_bounding_box() {
    let mut e = SelectMgrSensitiveEntity::new(1, 100, 5);
    // no bounding box yet -> not inside any query
    assert!(!e.is_inside_box([0.0; 3], [1.0, 1.0, 1.0]));

    e.set_bounding_box([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    assert!(e.is_inside_box([0.5, 0.5, 0.5], [2.0, 2.0, 2.0]));
    assert!(e.is_inside_box([-1.0, -1.0, -1.0], [0.5, 0.5, 0.5]));
    assert!(!e.is_inside_box([2.0, 2.0, 2.0], [3.0, 3.0, 3.0]));
}

#[test]
fn select_mgr_volume_from_point_and_box() {
    let vol = SelectMgrVolume::from_point([5.0, 5.0, 5.0], 1.0);
    assert_eq!(vol.min, [4.0, 4.0, 4.0]);
    assert_eq!(vol.max, [6.0, 6.0, 6.0]);

    let vol2 = SelectMgrVolume::from_box([0.0; 3], [10.0, 10.0, 10.0]);
    assert_eq!(vol2.min, [0.0; 3]);
    assert_eq!(vol2.max, [10.0, 10.0, 10.0]);
}

#[test]
fn viewer_selector_pick_returns_sorted_by_priority() {
    let mut vs = SelectMgrViewerSelector::new();

    let mut sel = SelectMgrSelection::new(SelectionMode::whole());
    let mut e1 = SelectMgrSensitiveEntity::new(1, 10, 5);
    e1.set_bounding_box([0.0; 3], [1.0, 1.0, 1.0]);
    let mut e2 = SelectMgrSensitiveEntity::new(2, 20, 10);
    e2.set_bounding_box([0.0; 3], [1.0, 1.0, 1.0]);
    sel.add(e1);
    sel.add(e2);
    sel.activate();
    vs.add_selection(sel);

    let vol = SelectMgrVolume::from_box([0.0; 3], [2.0, 2.0, 2.0]);
    vs.pick(&vol);
    assert_eq!(vs.nb_picked(), 2);
    // highest priority first
    assert_eq!(vs.closest_picked().unwrap().owner_id, 20);
}

#[test]
fn viewer_selector_deactivate_all_prevents_pick() {
    let mut vs = SelectMgrViewerSelector::new();
    let mut sel = SelectMgrSelection::new(SelectionMode::whole());
    let mut e = SelectMgrSensitiveEntity::new(1, 10, 0);
    e.set_bounding_box([0.0; 3], [1.0, 1.0, 1.0]);
    sel.add(e);
    sel.activate();
    vs.add_selection(sel);
    vs.deactivate_all();

    let vol = SelectMgrVolume::from_point([0.5, 0.5, 0.5], 1.0);
    vs.pick(&vol);
    assert_eq!(vs.nb_picked(), 0);
    assert!(vs.closest_picked().is_none());
}
