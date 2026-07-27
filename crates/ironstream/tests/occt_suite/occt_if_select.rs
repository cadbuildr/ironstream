// FILE: tests/occt_if_select.rs
extern crate ironstream;
use ironstream::if_select::*;

#[test]
fn work_model_add_and_find_entity() {
    let mut m = WorkModel::new();
    m.add_entity(1, "CARTESIAN_POINT");
    m.add_entity(2, "DIRECTION");
    assert_eq!(m.nb_entities(), 2);
    assert!(m.find(1).is_some());
    assert!(m.find(99).is_none());
}

#[test]
fn work_model_entity_type_lookup() {
    let mut m = WorkModel::new();
    m.add_entity(5, "SHAPE_DEFINITION");
    assert_eq!(m.entity_type(5).as_deref(), Some("SHAPE_DEFINITION"));
    assert!(m.entity_type(0).is_none());
}

#[test]
fn work_model_referencing() {
    let mut m = WorkModel::new();
    m.add_entity(1, "A");
    m.add_entity(2, "B");
    m.add_ref(2, 1);
    let refs = m.referencing(1);
    assert_eq!(refs, vec![2]);
}

#[test]
fn work_session_load_step_file() {
    let mut s = WorkSession::new();
    let status = s.load_file("model.stp");
    assert_eq!(status, WorkSessionStatus::Done);
    assert!(s.is_done());
}

#[test]
fn work_session_apply_select_all() {
    let mut s = WorkSession::new();
    s.model.add_entity(1, "A");
    s.model.add_entity(2, "B");
    s.model.add_entity(3, "A");
    let ids = s.apply_selector(&SelectAll);
    assert_eq!(ids.len(), 3);
}

#[test]
fn work_session_apply_select_by_type() {
    let mut s = WorkSession::new();
    s.model.add_entity(1, "CARTESIAN_POINT");
    s.model.add_entity(2, "DIRECTION");
    s.model.add_entity(3, "CARTESIAN_POINT");
    let sel = SelectByType { type_name: "CARTESIAN_POINT".into() };
    let ids = s.apply_selector(&sel);
    assert_eq!(ids.len(), 2);
}

#[test]
fn type_name_signature_matches() {
    let sig = TypeNameSignature;
    assert_eq!(sig.name(), "TypeName");
    assert!(sig.matches(1, "ENTITY", true));
    assert!(!sig.matches(1, "NOTENTITY", true));
    assert!(sig.matches(1, "ENT", false));
}
