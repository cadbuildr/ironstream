// FILE: tests/occt_brepalgo_bool.rs
extern crate ironstream;
use ironstream::brepalgo_bool::*;

#[test]
fn test_bool_op_type_default() {
    let t = BoolOpType::default();
    assert_eq!(t, BoolOpType::Fuse);
}

#[test]
fn test_bool_op_status_is_done() {
    assert!(BoolOpStatus::Done.is_done());
    assert!(!BoolOpStatus::NotDone.is_done());
    assert!(!BoolOpStatus::ErrorNullArgument.is_done());
    assert!(!BoolOpStatus::Error.is_done());
}

#[test]
fn test_brepalgo_bool_op_fuse_success() {
    let mut op = BrepAlgoBooleanOp::new(BoolOpType::Fuse);
    op.add_argument(10);
    op.add_argument(20);
    op.add_tool(30);
    op.set_fuzzy_value(1e-7);
    op.set_run_parallel(true);
    op.set_nondestructive(true);
    op.build();

    assert!(op.is_done());
    assert!(op.shape() > 0);
    assert_eq!(op.nb_arguments(), 2);
    assert_eq!(op.nb_tools(), 1);
    assert_eq!(op.op_type, BoolOpType::Fuse);
}

#[test]
fn test_brepalgo_bool_op_null_argument_error() {
    let mut op = BrepAlgoBooleanOp::new(BoolOpType::Cut);
    // No arguments added
    op.build();
    assert!(!op.is_done());
    assert_eq!(op.status, BoolOpStatus::ErrorNullArgument);
}

#[test]
fn test_brepalgo_section_basic() {
    let mut sec = BrepAlgoSection::new(5, 10);
    assert_eq!(sec.shape1_id, 5);
    assert_eq!(sec.shape2_id, 10);

    sec.set_approximation(true);
    sec.set_compute_pcurves(false, true);
    sec.build();

    assert!(sec.is_done());
    assert!(sec.nb_section_edges() > 0);
    let edge = sec.section_edge(0);
    assert!(edge.is_some());
    assert_eq!(sec.shape(), 5 + 10 + 70000);
}

#[test]
fn test_geomssplitter_perform() {
    let mut sp = GeomsplitterOp::new();
    sp.add_argument(1);
    sp.add_argument(2);
    sp.add_tool(100);
    sp.set_fuzzy_value(1e-6);
    sp.perform();

    assert!(sp.is_done());
    // 2 args * 1 tool = 2 results
    assert_eq!(sp.nb_results(), 2);
    assert_eq!(sp.result(0), Some(1 + 100 + 80000));
    assert_eq!(sp.result(1), Some(2 + 100 + 80000));
    assert_eq!(sp.result(99), None);
}
