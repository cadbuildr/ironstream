// FILE: tests/occt_topo_shape_ext.rs
extern crate ironstream;
use ironstream::topo_shape_ext::*;

#[test]
fn status_done_fail() {
    assert!(ShapeExtendStatus::Done1.is_done());
    assert!(!ShapeExtendStatus::Done1.is_fail());
    assert!(ShapeExtendStatus::Fail2.is_fail());
    assert!(!ShapeExtendStatus::OK.is_done());
}

#[test]
fn msg_registrator() {
    let mut r = ShapeMsgRegistrator::new();
    r.send(1, "ok trace", ShapeMsgGravity::Trace);
    r.send(1, "bad edge", ShapeMsgGravity::Fail);
    r.send(2, "check warning", ShapeMsgGravity::Warning);
    assert_eq!(r.nb_messages(), 3);
    assert!(r.has_failures());
    assert!(r.has_warnings());
    assert_eq!(r.messages_for(1).len(), 2);
}

#[test]
fn history_modifications() {
    let mut h = BrepToolsHistory::new();
    h.add_modification(10, 20);
    h.add_modification(10, 21);
    h.add_deletion(5);
    h.add_deletion(5); // dup → should only count once
    assert_eq!(h.modified(10).len(), 2);
    assert!(h.is_removed(5));
    assert!(!h.is_removed(99));
    assert_eq!(h.nb_deletions(), 1);
}

#[test]
fn history_merge() {
    let mut h1 = BrepToolsHistory::new();
    h1.add_modification(1, 10);
    let mut h2 = BrepToolsHistory::new();
    h2.add_modification(2, 20);
    h2.add_deletion(3);
    h1.merge(&h2);
    assert_eq!(h1.nb_modifications(), 2);
    assert_eq!(h1.nb_deletions(), 1);
}

#[test]
fn shape_mapper_bind_find() {
    let mut m = BrepShapeMapper::new();
    m.bind(1, 100);
    assert_eq!(m.find(1), Some(100));
    // Override binding
    m.bind(1, 101);
    assert_eq!(m.find(1), Some(101));
    assert!(m.is_bound(1));
}
