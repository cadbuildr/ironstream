// FILE: tests/occt_shape_extend.rs
extern crate ironstream;
use ironstream::shape_extend::*;

// ---------------------------------------------------------------------------
// ShapeExtendStatus tests
// ---------------------------------------------------------------------------

#[test]
fn status_variants_are_distinct() {
    let statuses = [
        ShapeExtendStatus::Ok,
        ShapeExtendStatus::Done,
        ShapeExtendStatus::Done1,
        ShapeExtendStatus::Done2,
        ShapeExtendStatus::Done3,
        ShapeExtendStatus::Done4,
        ShapeExtendStatus::Done5,
        ShapeExtendStatus::Done6,
        ShapeExtendStatus::Done7,
        ShapeExtendStatus::Done8,
        ShapeExtendStatus::Fail,
        ShapeExtendStatus::Fail1,
        ShapeExtendStatus::Fail2,
        ShapeExtendStatus::Fail3,
        ShapeExtendStatus::Fail4,
    ];
    // Every pair must be distinct.
    for i in 0..statuses.len() {
        for j in 0..statuses.len() {
            if i == j {
                assert_eq!(statuses[i], statuses[j]);
            } else {
                assert_ne!(statuses[i], statuses[j]);
            }
        }
    }
}

#[test]
fn status_copy_and_clone() {
    let s = ShapeExtendStatus::Done3;
    let copied = s;
    let cloned = s.clone();
    assert_eq!(s, copied);
    assert_eq!(s, cloned);
}

// ---------------------------------------------------------------------------
// ShapeExtendMsg tests
// ---------------------------------------------------------------------------

#[test]
fn msg_new_sets_fields() {
    let msg = ShapeExtendMsg::new(ShapeExtendStatus::Done1, "edge repaired");
    assert_eq!(msg.status(), ShapeExtendStatus::Done1);
    assert_eq!(msg.text(), "edge repaired");
    assert_eq!(msg.nb_shapes(), 0);
}

#[test]
fn msg_set_nb_shapes() {
    let mut msg = ShapeExtendMsg::new(ShapeExtendStatus::Fail2, "degenerate edge");
    assert_eq!(msg.nb_shapes(), 0);
    msg.set_nb_shapes(7);
    assert_eq!(msg.nb_shapes(), 7);
}

#[test]
fn msg_clone_is_independent() {
    let mut original = ShapeExtendMsg::new(ShapeExtendStatus::Ok, "ok");
    original.set_nb_shapes(3);
    let mut cloned = original.clone();
    cloned.set_nb_shapes(99);
    assert_eq!(original.nb_shapes(), 3);
    assert_eq!(cloned.nb_shapes(), 99);
}

#[test]
fn msg_with_empty_text() {
    let msg = ShapeExtendMsg::new(ShapeExtendStatus::Fail, "");
    assert_eq!(msg.text(), "");
    assert_eq!(msg.status(), ShapeExtendStatus::Fail);
}

// ---------------------------------------------------------------------------
// ShapeExtendWireData tests
// ---------------------------------------------------------------------------

#[test]
fn wire_new_is_empty() {
    let wire = ShapeExtendWireData::new();
    assert_eq!(wire.nb_edges(), 0);
    assert!(wire.is_manifold());
}

#[test]
fn wire_default_is_empty() {
    let wire = ShapeExtendWireData::default();
    assert_eq!(wire.nb_edges(), 0);
}

#[test]
fn wire_add_edge_at_end() {
    let mut wire = ShapeExtendWireData::new();
    wire.add_edge("e1", true);
    wire.add_edge("e2", true);
    wire.add_edge("e3", true);
    assert_eq!(wire.nb_edges(), 3);
    assert_eq!(wire.edge(1), "e1");
    assert_eq!(wire.edge(2), "e2");
    assert_eq!(wire.edge(3), "e3");
}

#[test]
fn wire_add_edge_at_start() {
    let mut wire = ShapeExtendWireData::new();
    wire.add_edge("e1", false);
    wire.add_edge("e2", false);
    // prepend: e2 should now be first
    assert_eq!(wire.nb_edges(), 2);
    assert_eq!(wire.edge(1), "e2");
    assert_eq!(wire.edge(2), "e1");
}

#[test]
fn wire_add_edge_mixed() {
    let mut wire = ShapeExtendWireData::new();
    wire.add_edge("middle", true);
    wire.add_edge("first", false);
    wire.add_edge("last", true);
    assert_eq!(wire.nb_edges(), 3);
    assert_eq!(wire.edge(1), "first");
    assert_eq!(wire.edge(2), "middle");
    assert_eq!(wire.edge(3), "last");
}

#[test]
fn wire_nb_edges_matches_actual_count() {
    let mut wire = ShapeExtendWireData::new();
    for i in 0..10 {
        wire.add_edge(&format!("e{}", i), true);
        assert_eq!(wire.nb_edges(), i + 1);
    }
}

#[test]
fn wire_reverse() {
    let mut wire = ShapeExtendWireData::new();
    wire.add_edge("a", true);
    wire.add_edge("b", true);
    wire.add_edge("c", true);
    wire.reverse();
    assert_eq!(wire.edge(1), "c");
    assert_eq!(wire.edge(2), "b");
    assert_eq!(wire.edge(3), "a");
}

#[test]
fn wire_reverse_twice_is_identity() {
    let mut wire = ShapeExtendWireData::new();
    wire.add_edge("x", true);
    wire.add_edge("y", true);
    wire.reverse();
    wire.reverse();
    assert_eq!(wire.edge(1), "x");
    assert_eq!(wire.edge(2), "y");
}

#[test]
fn wire_clear() {
    let mut wire = ShapeExtendWireData::new();
    wire.add_edge("a", true);
    wire.add_edge("b", true);
    wire.clear();
    assert_eq!(wire.nb_edges(), 0);
}

#[test]
fn wire_clear_then_readd() {
    let mut wire = ShapeExtendWireData::new();
    wire.add_edge("a", true);
    wire.clear();
    wire.add_edge("z", true);
    assert_eq!(wire.nb_edges(), 1);
    assert_eq!(wire.edge(1), "z");
}

#[test]
fn wire_set_manifold() {
    let mut wire = ShapeExtendWireData::new();
    assert!(wire.is_manifold());
    wire.set_manifold(false);
    assert!(!wire.is_manifold());
    wire.set_manifold(true);
    assert!(wire.is_manifold());
}

#[test]
#[should_panic]
fn wire_edge_index_zero_panics() {
    let mut wire = ShapeExtendWireData::new();
    wire.add_edge("e1", true);
    let _ = wire.edge(0);
}

#[test]
#[should_panic]
fn wire_edge_index_out_of_range_panics() {
    let mut wire = ShapeExtendWireData::new();
    wire.add_edge("e1", true);
    let _ = wire.edge(2);
}

#[test]
fn wire_clone_is_independent() {
    let mut wire = ShapeExtendWireData::new();
    wire.add_edge("original", true);
    let mut cloned = wire.clone();
    cloned.add_edge("extra", true);
    assert_eq!(wire.nb_edges(), 1);
    assert_eq!(cloned.nb_edges(), 2);
}

// ---------------------------------------------------------------------------
// ShapeExtendEdge tests
// ---------------------------------------------------------------------------

#[test]
fn edge_new_defaults() {
    let e = ShapeExtendEdge::new("edge_42");
    assert_eq!(e.edge_label(), "edge_42");
    assert!(!e.has_pcurve_on_s1());
    assert!(!e.has_pcurve_on_s2());
    assert!(!e.is_free());
}

#[test]
fn edge_set_pcurve_on_s1() {
    let mut e = ShapeExtendEdge::new("e");
    e.set_pcurve_on_s1(true);
    assert!(e.has_pcurve_on_s1());
    e.set_pcurve_on_s1(false);
    assert!(!e.has_pcurve_on_s1());
}

#[test]
fn edge_set_free() {
    let mut e = ShapeExtendEdge::new("e");
    e.set_free(true);
    assert!(e.is_free());
    e.set_free(false);
    assert!(!e.is_free());
}

#[test]
fn edge_pcurve_on_s2_is_read_only_via_has() {
    // pcurve_on_s2 has no public setter; defaults to false and stays false.
    let e = ShapeExtendEdge::new("e");
    assert!(!e.has_pcurve_on_s2());
}

#[test]
fn edge_label_preserved_with_special_chars() {
    let e = ShapeExtendEdge::new("wire/face[0]#edge");
    assert_eq!(e.edge_label(), "wire/face[0]#edge");
}

#[test]
fn edge_clone_is_independent() {
    let mut original = ShapeExtendEdge::new("base");
    let mut cloned = original.clone();
    cloned.set_free(true);
    cloned.set_pcurve_on_s1(true);
    assert!(!original.is_free());
    assert!(!original.has_pcurve_on_s1());
    assert!(cloned.is_free());
    assert!(cloned.has_pcurve_on_s1());
    // Original label unchanged.
    assert_eq!(original.edge_label(), "base");
}

// ---------------------------------------------------------------------------
// Cross-type integration
// ---------------------------------------------------------------------------

#[test]
fn msg_done_status_roundtrip() {
    for &st in &[
        ShapeExtendStatus::Done1,
        ShapeExtendStatus::Done8,
        ShapeExtendStatus::Fail4,
    ] {
        let msg = ShapeExtendMsg::new(st, "test");
        assert_eq!(msg.status(), st);
    }
}

#[test]
fn wire_with_many_edges_reversal_correctness() {
    let mut wire = ShapeExtendWireData::new();
    let labels: Vec<String> = (1..=8).map(|i| format!("e{}", i)).collect();
    for lbl in &labels {
        wire.add_edge(lbl, true);
    }
    wire.reverse();
    for i in 1..=8usize {
        assert_eq!(wire.edge(i), labels[8 - i]);
    }
}
