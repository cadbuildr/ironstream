// FILE: rust/ironstream/crates/ironstream/tests/occt_topods_compound_builder.rs
extern crate ironstream;
use ironstream::topods_compound_builder::*;

// ---------------------------------------------------------------------------
// TopoShapeRef — construction and accessors
// ---------------------------------------------------------------------------

#[test]
fn topo_shape_ref_stores_id() {
    let r = TopoShapeRef::new(5, "part", "Solid");
    assert_eq!(r.id(), 5);
}

#[test]
fn topo_shape_ref_stores_label() {
    let r = TopoShapeRef::new(0, "my_face", "Face");
    assert_eq!(r.label(), "my_face");
}

#[test]
fn topo_shape_ref_stores_shape_type() {
    let r = TopoShapeRef::new(0, "e", "Edge");
    assert_eq!(r.shape_type(), "Edge");
}

#[test]
fn topo_shape_ref_id_zero() {
    let r = TopoShapeRef::new(0, "origin", "Vertex");
    assert_eq!(r.id(), 0);
}

#[test]
fn topo_shape_ref_large_id() {
    let r = TopoShapeRef::new(usize::MAX, "big", "Shell");
    assert_eq!(r.id(), usize::MAX);
}

#[test]
fn topo_shape_ref_empty_label() {
    let r = TopoShapeRef::new(1, "", "Solid");
    assert_eq!(r.label(), "");
}

#[test]
fn topo_shape_ref_empty_shape_type() {
    let r = TopoShapeRef::new(2, "x", "");
    assert_eq!(r.shape_type(), "");
}

// ---------------------------------------------------------------------------
// CompoundBuilder — new / is_empty / nb_shapes
// ---------------------------------------------------------------------------

#[test]
fn compound_builder_new_is_empty() {
    let b = CompoundBuilder::new();
    assert!(b.is_empty());
}

#[test]
fn compound_builder_new_nb_shapes_zero() {
    let b = CompoundBuilder::new();
    assert_eq!(b.nb_shapes(), 0);
}

#[test]
fn compound_builder_default_equals_new() {
    let b: CompoundBuilder = Default::default();
    assert!(b.is_empty());
    assert_eq!(b.nb_shapes(), 0);
}

// ---------------------------------------------------------------------------
// CompoundBuilder — add_shape
// ---------------------------------------------------------------------------

#[test]
fn add_shape_first_id_is_zero() {
    let mut b = CompoundBuilder::new();
    let id = b.add_shape("face_0", "Face");
    assert_eq!(id, 0);
}

#[test]
fn add_shape_second_id_is_one() {
    let mut b = CompoundBuilder::new();
    b.add_shape("s0", "Solid");
    let id = b.add_shape("s1", "Solid");
    assert_eq!(id, 1);
}

#[test]
fn add_shape_increments_nb_shapes() {
    let mut b = CompoundBuilder::new();
    b.add_shape("a", "Face");
    assert_eq!(b.nb_shapes(), 1);
    b.add_shape("b", "Face");
    assert_eq!(b.nb_shapes(), 2);
    b.add_shape("c", "Solid");
    assert_eq!(b.nb_shapes(), 3);
}

#[test]
fn add_shape_clears_is_empty() {
    let mut b = CompoundBuilder::new();
    assert!(b.is_empty());
    b.add_shape("x", "Edge");
    assert!(!b.is_empty());
}

#[test]
fn add_shape_label_stored_via_lookup() {
    let mut b = CompoundBuilder::new();
    let id = b.add_shape("wing", "Shell");
    assert_eq!(b.shape(id).unwrap().label(), "wing");
}

#[test]
fn add_shape_type_stored_via_lookup() {
    let mut b = CompoundBuilder::new();
    let id = b.add_shape("w", "Wire");
    assert_eq!(b.shape(id).unwrap().shape_type(), "Wire");
}

#[test]
fn add_multiple_shapes_ids_sequential() {
    let mut b = CompoundBuilder::new();
    let ids: Vec<usize> = (0..10).map(|i| b.add_shape(&format!("s{}", i), "Face")).collect();
    assert_eq!(ids, (0..10).collect::<Vec<usize>>());
}

// ---------------------------------------------------------------------------
// CompoundBuilder — remove_shape
// ---------------------------------------------------------------------------

#[test]
fn remove_shape_returns_true_on_success() {
    let mut b = CompoundBuilder::new();
    let id = b.add_shape("f", "Face");
    assert!(b.remove_shape(id));
}

#[test]
fn remove_shape_decrements_nb_shapes() {
    let mut b = CompoundBuilder::new();
    let id = b.add_shape("f", "Face");
    b.remove_shape(id);
    assert_eq!(b.nb_shapes(), 0);
}

#[test]
fn remove_shape_makes_builder_empty_when_last() {
    let mut b = CompoundBuilder::new();
    let id = b.add_shape("only", "Solid");
    b.remove_shape(id);
    assert!(b.is_empty());
}

#[test]
fn remove_shape_returns_false_for_unknown_id() {
    let mut b = CompoundBuilder::new();
    assert!(!b.remove_shape(0));
    assert!(!b.remove_shape(999));
}

#[test]
fn remove_shape_returns_false_on_second_call() {
    let mut b = CompoundBuilder::new();
    let id = b.add_shape("x", "Face");
    b.remove_shape(id);
    assert!(!b.remove_shape(id));
}

#[test]
fn remove_shape_leaves_other_shapes_intact() {
    let mut b = CompoundBuilder::new();
    let id0 = b.add_shape("keep", "Solid");
    let id1 = b.add_shape("drop", "Face");
    b.remove_shape(id1);
    assert_eq!(b.nb_shapes(), 1);
    assert_eq!(b.shape(id0).unwrap().label(), "keep");
}

#[test]
fn remove_shape_middle_element() {
    let mut b = CompoundBuilder::new();
    let id0 = b.add_shape("a", "Face");
    let id1 = b.add_shape("b", "Face");
    let id2 = b.add_shape("c", "Face");
    assert!(b.remove_shape(id1));
    assert_eq!(b.nb_shapes(), 2);
    assert!(b.shape(id0).is_some());
    assert!(b.shape(id1).is_none());
    assert!(b.shape(id2).is_some());
}

// ---------------------------------------------------------------------------
// CompoundBuilder — shape lookup
// ---------------------------------------------------------------------------

#[test]
fn shape_returns_none_for_unknown_id() {
    let b = CompoundBuilder::new();
    assert!(b.shape(0).is_none());
    assert!(b.shape(42).is_none());
}

#[test]
fn shape_returns_correct_ref_after_multiple_adds() {
    let mut b = CompoundBuilder::new();
    b.add_shape("alpha", "Edge");
    let id = b.add_shape("beta", "Wire");
    b.add_shape("gamma", "Face");
    let s = b.shape(id).unwrap();
    assert_eq!(s.id(), id);
    assert_eq!(s.label(), "beta");
    assert_eq!(s.shape_type(), "Wire");
}

// ---------------------------------------------------------------------------
// CompoundBuilder — clear
// ---------------------------------------------------------------------------

#[test]
fn clear_on_empty_builder_is_idempotent() {
    let mut b = CompoundBuilder::new();
    b.clear();
    assert!(b.is_empty());
    assert_eq!(b.nb_shapes(), 0);
}

#[test]
fn clear_removes_all_shapes() {
    let mut b = CompoundBuilder::new();
    b.add_shape("a", "Face");
    b.add_shape("b", "Solid");
    b.clear();
    assert!(b.is_empty());
    assert_eq!(b.nb_shapes(), 0);
}

#[test]
fn clear_resets_id_counter_to_zero() {
    let mut b = CompoundBuilder::new();
    b.add_shape("x", "Face");
    b.add_shape("y", "Face");
    b.clear();
    let id = b.add_shape("z", "Solid");
    assert_eq!(id, 0);
}

#[test]
fn clear_then_add_works_normally() {
    let mut b = CompoundBuilder::new();
    b.add_shape("old", "Face");
    b.clear();
    let id = b.add_shape("new", "Solid");
    assert_eq!(b.nb_shapes(), 1);
    assert_eq!(b.shape(id).unwrap().label(), "new");
}

// ---------------------------------------------------------------------------
// CompoundBuilder — shape_ids_of_type
// ---------------------------------------------------------------------------

#[test]
fn shape_ids_of_type_empty_when_no_shapes() {
    let b = CompoundBuilder::new();
    assert!(b.shape_ids_of_type("Face").is_empty());
}

#[test]
fn shape_ids_of_type_empty_when_no_match() {
    let mut b = CompoundBuilder::new();
    b.add_shape("s", "Solid");
    assert!(b.shape_ids_of_type("Face").is_empty());
}

#[test]
fn shape_ids_of_type_returns_matching_ids() {
    let mut b = CompoundBuilder::new();
    let f0 = b.add_shape("f0", "Face");
    b.add_shape("s0", "Solid");
    let f1 = b.add_shape("f1", "Face");
    let ids = b.shape_ids_of_type("Face");
    assert_eq!(ids, vec![f0, f1]);
}

#[test]
fn shape_ids_of_type_returns_all_when_all_match() {
    let mut b = CompoundBuilder::new();
    let id0 = b.add_shape("e0", "Edge");
    let id1 = b.add_shape("e1", "Edge");
    let id2 = b.add_shape("e2", "Edge");
    let ids = b.shape_ids_of_type("Edge");
    assert_eq!(ids, vec![id0, id1, id2]);
}

#[test]
fn shape_ids_of_type_excludes_removed_shapes() {
    let mut b = CompoundBuilder::new();
    let f0 = b.add_shape("f0", "Face");
    let f1 = b.add_shape("f1", "Face");
    b.remove_shape(f0);
    let ids = b.shape_ids_of_type("Face");
    assert_eq!(ids, vec![f1]);
}

#[test]
fn shape_ids_of_type_case_sensitive() {
    let mut b = CompoundBuilder::new();
    b.add_shape("f", "Face");
    assert!(b.shape_ids_of_type("face").is_empty());
    assert!(b.shape_ids_of_type("FACE").is_empty());
}

// ---------------------------------------------------------------------------
// CompoundExplorer — empty compound
// ---------------------------------------------------------------------------

#[test]
fn explorer_new_empty_more_is_false() {
    let ex = CompoundExplorer::new(Vec::new());
    assert!(!ex.more());
}

#[test]
fn explorer_new_empty_reset_stays_false() {
    let mut ex = CompoundExplorer::new(Vec::new());
    ex.reset();
    assert!(!ex.more());
}

// ---------------------------------------------------------------------------
// CompoundExplorer — single-element compound
// ---------------------------------------------------------------------------

#[test]
fn explorer_single_more_is_true_initially() {
    let shapes = vec![TopoShapeRef::new(0, "only", "Face")];
    let ex = CompoundExplorer::new(shapes);
    assert!(ex.more());
}

#[test]
fn explorer_single_current_returns_ref() {
    let shapes = vec![TopoShapeRef::new(0, "only", "Solid")];
    let ex = CompoundExplorer::new(shapes);
    assert_eq!(ex.current().label(), "only");
}

#[test]
fn explorer_single_more_false_after_next() {
    let shapes = vec![TopoShapeRef::new(0, "only", "Edge")];
    let mut ex = CompoundExplorer::new(shapes);
    ex.next();
    assert!(!ex.more());
}

#[test]
fn explorer_single_reset_restores_cursor() {
    let shapes = vec![TopoShapeRef::new(0, "x", "Wire")];
    let mut ex = CompoundExplorer::new(shapes);
    ex.next();
    assert!(!ex.more());
    ex.reset();
    assert!(ex.more());
    assert_eq!(ex.current().label(), "x");
}

// ---------------------------------------------------------------------------
// CompoundExplorer — multi-element compound
// ---------------------------------------------------------------------------

#[test]
fn explorer_iterates_all_shapes_in_order() {
    let shapes = vec![
        TopoShapeRef::new(0, "alpha", "Face"),
        TopoShapeRef::new(1, "beta", "Solid"),
        TopoShapeRef::new(2, "gamma", "Edge"),
    ];
    let mut ex = CompoundExplorer::new(shapes);
    let mut collected: Vec<String> = Vec::new();
    while ex.more() {
        collected.push(ex.current().label().to_string());
        ex.next();
    }
    assert_eq!(collected, vec!["alpha", "beta", "gamma"]);
}

#[test]
fn explorer_iterates_correct_ids_in_order() {
    let shapes = vec![
        TopoShapeRef::new(10, "a", "Face"),
        TopoShapeRef::new(20, "b", "Face"),
    ];
    let mut ex = CompoundExplorer::new(shapes);
    assert_eq!(ex.current().id(), 10);
    ex.next();
    assert_eq!(ex.current().id(), 20);
    ex.next();
    assert!(!ex.more());
}

#[test]
fn explorer_reset_allows_second_pass() {
    let shapes = vec![
        TopoShapeRef::new(0, "p", "Vertex"),
        TopoShapeRef::new(1, "q", "Vertex"),
    ];
    let mut ex = CompoundExplorer::new(shapes);
    let mut first_pass: Vec<String> = Vec::new();
    while ex.more() {
        first_pass.push(ex.current().label().to_string());
        ex.next();
    }
    ex.reset();
    let mut second_pass: Vec<String> = Vec::new();
    while ex.more() {
        second_pass.push(ex.current().label().to_string());
        ex.next();
    }
    assert_eq!(first_pass, second_pass);
}

#[test]
fn explorer_next_past_end_does_not_panic() {
    let shapes = vec![TopoShapeRef::new(0, "x", "Face")];
    let mut ex = CompoundExplorer::new(shapes);
    ex.next();
    // calling next() when more() is false should be safe
    ex.next();
    ex.next();
    assert!(!ex.more());
}

#[test]
fn explorer_shape_types_accessible() {
    let shapes = vec![
        TopoShapeRef::new(0, "f", "Face"),
        TopoShapeRef::new(1, "s", "Solid"),
    ];
    let mut ex = CompoundExplorer::new(shapes);
    assert_eq!(ex.current().shape_type(), "Face");
    ex.next();
    assert_eq!(ex.current().shape_type(), "Solid");
}

// ---------------------------------------------------------------------------
// Combined workflows
// ---------------------------------------------------------------------------

#[test]
fn builder_snapshot_explored_in_order() {
    let mut b = CompoundBuilder::new();
    let id0 = b.add_shape("body_a", "Solid");
    let id1 = b.add_shape("body_b", "Solid");
    let id2 = b.add_shape("patch", "Face");
    // take snapshot from the builder's internal vec via shape_ids_of_type
    // then construct refs for the explorer
    let all_ids = {
        let mut v = b.shape_ids_of_type("Solid");
        v.extend(b.shape_ids_of_type("Face"));
        v
    };
    assert_eq!(all_ids, vec![id0, id1, id2]);
}

#[test]
fn builder_remove_then_type_filter_consistent() {
    let mut b = CompoundBuilder::new();
    let f0 = b.add_shape("f0", "Face");
    let f1 = b.add_shape("f1", "Face");
    let _s0 = b.add_shape("s0", "Solid");
    b.remove_shape(f0);
    let face_ids = b.shape_ids_of_type("Face");
    assert_eq!(face_ids, vec![f1]);
    assert_eq!(b.nb_shapes(), 2);
}

#[test]
fn full_workflow_add_explore_clear_readd() {
    let mut b = CompoundBuilder::new();
    b.add_shape("a", "Face");
    b.add_shape("b", "Solid");

    // explore first batch
    let snapshot: Vec<TopoShapeRef> = b.shape_ids_of_type("Face")
        .iter()
        .map(|&id| b.shape(id).unwrap().clone())
        .collect();
    let mut ex = CompoundExplorer::new(snapshot);
    assert!(ex.more());
    assert_eq!(ex.current().shape_type(), "Face");

    // clear and re-add
    b.clear();
    let new_id = b.add_shape("c", "Wire");
    assert_eq!(new_id, 0);
    assert_eq!(b.nb_shapes(), 1);
}
