// FILE: tests/occt_ais_local.rs
// Integration tests for ais_local module
extern crate ironstream;

use ironstream::ais_local::{
    AisLocalContext, AisLocalState, AisLocalStatus, AisPickResult, AisSelMode, AisSelection,
};

#[test]
fn ais_sel_mode_enum_values() {
    assert_eq!(AisSelMode::Shape as i32, 0);
    assert_eq!(AisSelMode::Vertex as i32, 1);
    assert_eq!(AisSelMode::Edge as i32, 2);
    assert_eq!(AisSelMode::Wire as i32, 3);
    assert_eq!(AisSelMode::Face as i32, 4);
    assert_eq!(AisSelMode::Shell as i32, 5);
    assert_eq!(AisSelMode::Solid as i32, 6);
    assert_eq!(AisSelMode::CompSolid as i32, 7);
    assert_eq!(AisSelMode::Compound as i32, 8);
}

#[test]
fn ais_local_status_add_remove_modes() {
    let mut s = AisLocalStatus::new(false, 1);
    s.add_selection_mode(AisSelMode::Face);
    s.add_selection_mode(AisSelMode::Edge);
    s.add_selection_mode(AisSelMode::Face); // duplicate → ignored
    assert_eq!(s.nb_selection_modes(), 2);
    assert!(s.is_activated(AisSelMode::Face));
    assert!(s.is_activated(AisSelMode::Edge));
    s.remove_selection_mode(AisSelMode::Face);
    assert!(!s.is_activated(AisSelMode::Face));
    assert_eq!(s.nb_selection_modes(), 1);
}

#[test]
fn ais_local_status_state_and_selected() {
    let mut s = AisLocalStatus::new(true, 2);
    assert!(!s.is_selected());
    s.set_state(AisLocalState::Selected);
    assert!(s.is_selected());
    s.set_state(AisLocalState::Active);
    assert!(!s.is_selected());
}

#[test]
fn ais_selection_select_deselect_clear() {
    let mut sel = AisSelection::new("test_sel");
    sel.select(10);
    sel.select(20);
    sel.select(10); // duplicate
    assert_eq!(sel.nb_selected(), 2);
    assert!(sel.is_selected(10));
    sel.deselect(10);
    assert!(!sel.is_selected(10));
    assert_eq!(sel.nb_selected(), 1);
    sel.clear();
    assert_eq!(sel.nb_selected(), 0);
}

#[test]
fn ais_local_context_load_activate_deactivate() {
    let mut ctx = AisLocalContext::new(1);
    assert!(ctx.is_open());
    assert_eq!(ctx.nb_loaded(), 0);

    ctx.load_object(42, false);
    ctx.load_object(43, true);
    assert_eq!(ctx.nb_loaded(), 2);

    ctx.activate_mode(42, AisSelMode::Edge);
    ctx.activate_mode(42, AisSelMode::Face);
    let s = ctx.status(42).unwrap();
    assert!(s.is_activated(AisSelMode::Edge));
    assert!(s.is_activated(AisSelMode::Face));
    assert_eq!(s.nb_selection_modes(), 2);

    ctx.deactivate_mode(42, AisSelMode::Edge);
    assert!(!ctx.status(42).unwrap().is_activated(AisSelMode::Edge));
}

#[test]
fn ais_local_context_pick_and_select_detected() {
    let mut ctx = AisLocalContext::new(2);
    ctx.load_object(10, false);
    ctx.activate_mode(10, AisSelMode::Shape);

    let r1 = AisPickResult::new(10, 0, 5.0, AisSelMode::Shape);
    let r2 = AisPickResult::new(99, 1, 1.0, AisSelMode::Face); // not loaded
    ctx.add_pick_result(r1);
    ctx.add_pick_result(r2);
    assert_eq!(ctx.nb_detected(), 2);

    ctx.select_detected();
    // obj 10 should now be selected
    assert_eq!(ctx.nb_selected_objects(), 1);
    assert!(ctx.status(10).unwrap().is_selected());

    ctx.clear_pick_results();
    assert_eq!(ctx.nb_detected(), 0);

    ctx.close();
    assert!(!ctx.is_open());
    ctx.open();
    assert!(ctx.is_open());
}
