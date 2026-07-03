// FILE: tests/occt_shape_fix_wire.rs
// Integration tests for shape_fix_wire module
extern crate ironstream;

use ironstream::shape_fix_wire::{
    FixFlags, ShapeAnalysisWire, ShapeAnalysisWireOrder, ShapeFixEdge, ShapeFixWire,
    WireCheckStatus,
};

#[test]
fn fix_flags_constants_and_bitops() {
    assert_eq!(FixFlags::NONE.0, 0);
    assert!(FixFlags::REORDER.0 != 0);
    assert!(FixFlags::SMALL_EDGES.0 != 0);
    assert!(FixFlags::CONNECTED.0 != 0);
    assert!(FixFlags::DEGENERACY.0 != 0);
    assert!(FixFlags::SELF_INTERSECT.0 != 0);

    let mut f = FixFlags::NONE;
    f.set(FixFlags::REORDER);
    assert!(f.has(FixFlags::REORDER));
    assert!(!f.has(FixFlags::SMALL_EDGES));

    f.set(FixFlags::SMALL_EDGES);
    assert!(f.has(FixFlags::SMALL_EDGES));
    f.clear_flag(FixFlags::REORDER);
    assert!(!f.has(FixFlags::REORDER));
    assert!(f.has(FixFlags::SMALL_EDGES));
}

#[test]
fn wire_check_status_is_fail_and_is_done() {
    assert!(WireCheckStatus::Fail1.is_fail());
    assert!(WireCheckStatus::Fail2.is_fail());
    assert!(!WireCheckStatus::Ok.is_fail());
    assert!(!WireCheckStatus::Done1.is_fail());

    assert!(WireCheckStatus::Done1.is_done());
    assert!(WireCheckStatus::Done2.is_done());
    assert!(WireCheckStatus::Done3.is_done());
    assert!(!WireCheckStatus::NotDone.is_done());
    assert!(!WireCheckStatus::Ok.is_done());
    assert!(!WireCheckStatus::Fail1.is_done());
}

#[test]
fn shape_analysis_wire_order_perform() {
    let mut wo = ShapeAnalysisWireOrder::new();
    wo.set_edges(vec![1, 2, 3, 4]);
    wo.perform(true);
    assert!(wo.is_done());
    assert_eq!(wo.nb_edges(), 4);
    assert_eq!(wo.nb_gaps(), 0);
}

#[test]
fn shape_analysis_wire_gaps_and_check_order() {
    let mut aw = ShapeAnalysisWire::new(5, 3, 1e-5);
    aw.perform();
    assert!(aw.is_done());

    aw.add_gap(1, 2, 0.01);
    aw.add_gap(2, 3, 0.02);
    assert_eq!(aw.nb_gaps(), 2);
    assert!((aw.max_gap() - 0.02).abs() < 1e-10);

    // check_order with nb_edges > 0 returns Done1
    let status = aw.check_order();
    assert!(status.is_done());
}

#[test]
fn shape_fix_edge_fix_degenerated() {
    let mut fe = ShapeFixEdge::new(7);
    assert!(!fe.is_fixed(FixFlags::DEGENERACY));
    let ok = fe.fix_degenerated_edge();
    assert!(ok);
    assert!(fe.is_fixed(FixFlags::DEGENERACY));
}

#[test]
fn shape_fix_wire_perform_and_has_fix() {
    let mut fw = ShapeFixWire::new(100, 200, 1e-6);
    fw.mode_fix_reorder(true);
    fw.mode_fix_small_edge(true);
    let ok = fw.perform();
    assert!(ok);
    assert!(fw.is_done());
    assert!(fw.has_fix(FixFlags::REORDER));
    assert!(fw.has_fix(FixFlags::SMALL_EDGES));
    assert_eq!(fw.nb_applied_fixes(), 2);
    assert!(fw.wire() > 0);

    // wire_id == 0 should fail
    let mut fw_bad = ShapeFixWire::new(0, 1, 1e-6);
    assert!(!fw_bad.perform());
    assert!(!fw_bad.is_done());
}
